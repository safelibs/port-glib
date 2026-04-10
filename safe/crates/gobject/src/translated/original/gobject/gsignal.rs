// SAFETY: Mechanically translated from the checked-in upstream GObject sources; unsafe is retained at ABI-preserving FFI and memory-layout boundaries.
use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
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
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_array_new(zero_terminated: gboolean, clear_: gboolean, element_size: guint)
        -> *mut GArray;
    fn g_array_free(array: *mut GArray, free_segment: gboolean) -> *mut gchar;
    fn g_array_append_vals(array: *mut GArray, data: gconstpointer, len: guint) -> *mut GArray;
    fn g_quark_try_string(string: *const gchar) -> GQuark;
    fn g_quark_from_string(string: *const gchar) -> GQuark;
    fn g_intern_string(string: *const gchar) -> *const gchar;
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_getenv(variable: *const gchar) -> *const gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_realloc(mem: gpointer, n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_realloc_n(mem: gpointer, n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_hash_table_new(hash_func: GHashFunc, key_equal_func: GEqualFunc) -> *mut GHashTable;
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_add(hash_table: *mut GHashTable, key: gpointer) -> gboolean;
    fn g_hash_table_remove(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_direct_hash(v: gconstpointer) -> guint;
    fn g_hook_list_init(hook_list: *mut GHookList, hook_size: guint);
    fn g_hook_list_clear(hook_list: *mut GHookList);
    fn g_hook_alloc(hook_list: *mut GHookList) -> *mut GHook;
    fn g_hook_ref(hook_list: *mut GHookList, hook: *mut GHook) -> *mut GHook;
    fn g_hook_unref(hook_list: *mut GHookList, hook: *mut GHook);
    fn g_hook_destroy(hook_list: *mut GHookList, hook_id: gulong) -> gboolean;
    fn g_hook_destroy_link(hook_list: *mut GHookList, hook: *mut GHook);
    fn g_hook_insert_before(hook_list: *mut GHookList, sibling: *mut GHook, hook: *mut GHook);
    fn g_hook_first_valid(hook_list: *mut GHookList, may_be_in_call: gboolean) -> *mut GHook;
    fn g_hook_next_valid(
        hook_list: *mut GHookList,
        hook: *mut GHook,
        may_be_in_call: gboolean,
    ) -> *mut GHook;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_memdup2(mem: gconstpointer, byte_size: gsize) -> gpointer;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn g_type_name(type_0: GType) -> *const gchar;
    fn g_type_parent(type_0: GType) -> GType;
    fn g_type_is_a(type_0: GType, is_a_type: GType) -> gboolean;
    fn g_type_class_peek(type_0: GType) -> gpointer;
    fn g_type_interfaces(type_0: GType, n_interfaces: *mut guint) -> *mut GType;
    fn g_type_fundamental(type_id: GType) -> GType;
    fn g_type_value_table_peek(type_0: GType) -> *mut GTypeValueTable;
    fn g_type_check_instance(instance: *mut GTypeInstance) -> gboolean;
    fn g_type_check_is_value_type(type_0: GType) -> gboolean;
    fn g_type_test_flags(type_0: GType, flags: guint) -> gboolean;
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
    fn g_signal_type_cclosure_new(itype: GType, struct_offset: guint) -> *mut GClosure;
    fn g_closure_ref(closure: *mut GClosure) -> *mut GClosure;
    fn g_closure_sink(closure: *mut GClosure);
    fn g_closure_unref(closure: *mut GClosure);
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
    fn g_closure_set_marshal(closure: *mut GClosure, marshal: GClosureMarshal);
    fn g_closure_invoke(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
    );
    fn g_cclosure_marshal_generic(
        closure: *mut GClosure,
        return_gvalue: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn g_cclosure_marshal_generic_va(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args_list: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
    fn g_value_init(value: *mut GValue, g_type: GType) -> *mut GValue;
    fn g_value_copy(src_value: *const GValue, dest_value: *mut GValue);
    fn g_value_reset(value: *mut GValue) -> *mut GValue;
    fn g_value_unset(value: *mut GValue);
    fn g_value_init_from_instance(value: *mut GValue, instance: gpointer);
    fn g_value_peek_pointer(value: *const GValue) -> gpointer;
    fn g_param_spec_is_valid_name(name: *const gchar) -> gboolean;
    fn g_cclosure_marshal_VOID__VOID(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn g_cclosure_marshal_VOID__VOIDv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
    fn g_cclosure_marshal_VOID__BOOLEAN(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn g_cclosure_marshal_VOID__BOOLEANv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
    fn g_cclosure_marshal_VOID__CHAR(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn g_cclosure_marshal_VOID__CHARv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
    fn g_cclosure_marshal_VOID__UCHAR(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn g_cclosure_marshal_VOID__UCHARv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
    fn g_cclosure_marshal_VOID__INT(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn g_cclosure_marshal_VOID__INTv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
    fn g_cclosure_marshal_VOID__UINT(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn g_cclosure_marshal_VOID__UINTv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
    fn g_cclosure_marshal_VOID__LONG(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn g_cclosure_marshal_VOID__LONGv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
    fn g_cclosure_marshal_VOID__ULONG(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn g_cclosure_marshal_VOID__ULONGv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
    fn g_cclosure_marshal_VOID__ENUM(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn g_cclosure_marshal_VOID__ENUMv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
    fn g_cclosure_marshal_VOID__FLAGS(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn g_cclosure_marshal_VOID__FLAGSv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
    fn g_cclosure_marshal_VOID__FLOAT(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn g_cclosure_marshal_VOID__FLOATv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
    fn g_cclosure_marshal_VOID__DOUBLE(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn g_cclosure_marshal_VOID__DOUBLEv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
    fn g_cclosure_marshal_VOID__STRING(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn g_cclosure_marshal_VOID__STRINGv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
    fn g_cclosure_marshal_VOID__PARAM(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn g_cclosure_marshal_VOID__PARAMv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
    fn g_cclosure_marshal_VOID__BOXED(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn g_cclosure_marshal_VOID__BOXEDv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
    fn g_cclosure_marshal_VOID__POINTER(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn g_cclosure_marshal_VOID__POINTERv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
    fn g_cclosure_marshal_VOID__OBJECT(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn g_cclosure_marshal_VOID__OBJECTv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
    fn g_cclosure_marshal_VOID__VARIANT(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn g_cclosure_marshal_VOID__VARIANTv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn _g_closure_is_void(closure: *mut GClosure, instance: gpointer) -> gboolean;
    fn _g_closure_supports_invoke_va(closure: *mut GClosure) -> gboolean;
    fn _g_closure_set_va_marshal(closure: *mut GClosure, marshal: GVaClosureMarshal);
    fn _g_closure_invoke_va(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
    fn _g_object_has_signal_handler(object: *mut GObject) -> gboolean;
    fn _g_object_set_has_signal_handler(object: *mut GObject, signal_id: guint);
    fn g_value_set_boolean(value: *mut GValue, v_boolean: gboolean);
    fn g_value_get_boolean(value: *const GValue) -> gboolean;
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
pub type guint8 = ::core::ffi::c_uchar;
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
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GArray {
    pub data: *mut gchar,
    pub len: guint,
}
pub type GArray = _GArray;
pub type va_list = __builtin_va_list;
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GMutex = _GMutex;
pub type GData = _GData;
pub type GHashTable = _GHashTable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GHook {
    pub data: gpointer,
    pub next: *mut GHook,
    pub prev: *mut GHook,
    pub ref_count: guint,
    pub hook_id: gulong,
    pub flags: guint,
    pub func: gpointer,
    pub destroy: GDestroyNotify,
}
pub type GHook = _GHook;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GHookList {
    pub seq_id: gulong,
    #[bitfield(name = "hook_size", ty = "guint", bits = "0..=15")]
    #[bitfield(name = "is_setup", ty = "guint", bits = "16..=16")]
    pub hook_size_is_setup: [u8; 3],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 5],
    pub hooks: *mut GHook,
    pub dummy3: gpointer,
    pub finalize_hook: GHookFinalizeFunc,
    pub dummy: [gpointer; 2],
}
pub type GHookFinalizeFunc = Option<unsafe extern "C" fn(*mut GHookList, *mut GHook) -> ()>;
pub type GHookList = _GHookList;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const G_HOOK_FLAG_MASK: C2RustUnnamed = 15;
pub const G_HOOK_FLAG_IN_CALL: C2RustUnnamed = 2;
pub const G_HOOK_FLAG_ACTIVE: C2RustUnnamed = 1;
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
    pub data: [C2RustUnnamed_0; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const G_TYPE_FLAG_DEEP_DERIVABLE: C2RustUnnamed_1 = 8;
pub const G_TYPE_FLAG_DERIVABLE: C2RustUnnamed_1 = 4;
pub const G_TYPE_FLAG_INSTANTIATABLE: C2RustUnnamed_1 = 2;
pub const G_TYPE_FLAG_CLASSED: C2RustUnnamed_1 = 1;
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
pub type GVaClosureMarshal = Option<
    unsafe extern "C" fn(
        *mut GClosure,
        *mut GValue,
        gpointer,
        ::core::ffi::VaList,
        gpointer,
        ::core::ffi::c_int,
        *mut GType,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCClosure {
    pub closure: GClosure,
    pub callback: gpointer,
}
pub type GCClosure = _GCClosure;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSignalQuery {
    pub signal_id: guint,
    pub signal_name: *const gchar,
    pub itype: GType,
    pub signal_flags: GSignalFlags,
    pub return_type: GType,
    pub n_params: guint,
    pub param_types: *const GType,
}
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
pub type GSignalQuery = _GSignalQuery;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSignalInvocationHint {
    pub signal_id: guint,
    pub detail: GQuark,
    pub run_type: GSignalFlags,
}
pub type GSignalInvocationHint = _GSignalInvocationHint;
pub type GSignalCMarshaller = GClosureMarshal;
pub type GSignalCVaMarshaller = GVaClosureMarshal;
pub type GSignalEmissionHook = Option<
    unsafe extern "C" fn(*mut GSignalInvocationHint, guint, *const GValue, gpointer) -> gboolean,
>;
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
pub type SignalNode = _SignalNode;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _SignalNode {
    pub signal_id: guint,
    pub itype: GType,
    pub name: *const gchar,
    #[bitfield(name = "destroyed", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "flags", ty = "guint", bits = "1..=9")]
    #[bitfield(name = "n_params", ty = "guint", bits = "10..=17")]
    #[bitfield(name = "single_va_closure_is_valid", ty = "guint", bits = "18..=18")]
    #[bitfield(name = "single_va_closure_is_after", ty = "guint", bits = "19..=19")]
    pub destroyed_flags_n_params_single_va_closure_is_valid_single_va_closure_is_after: [u8; 3],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 5],
    pub param_types: *mut GType,
    pub return_type: GType,
    pub class_closure_bsa: *mut GBSearchArray,
    pub accumulator: *mut SignalAccumulator,
    pub c_marshaller: GSignalCMarshaller,
    pub va_marshaller: GSignalCVaMarshaller,
    pub emission_hooks: *mut GHookList,
    pub single_va_closure: *mut GClosure,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SignalAccumulator {
    pub func: GSignalAccumulator,
    pub data: gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union GBSearchArray {
    pub n_nodes: guint,
    pub alignment_dummy1: gpointer,
    pub alignment_dummy2: glong,
    pub alignment_dummy3: gdouble,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClassClosure {
    pub instance_type: GType,
    pub closure: *mut GClosure,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GBSearchConfig {
    pub sizeof_node: guint,
    pub cmp_nodes: GBSearchCompareFunc,
    pub flags: guint,
}
pub type GBSearchCompareFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint>;
pub const G_BSEARCH_ARRAY_ALIGN_POWER2: C2RustUnnamed_2 = 1;
pub type SignalKey = _SignalKey;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _SignalKey {
    pub itype: GType,
    pub quark: GQuark,
    pub signal_id: guint,
}
pub type Emission = _Emission;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _Emission {
    pub next: *mut Emission,
    pub instance: gpointer,
    pub ihint: GSignalInvocationHint,
    pub state: EmissionState,
    pub chain_type: GType,
}
pub type EmissionState = ::core::ffi::c_uint;
pub const EMISSION_RESTART: EmissionState = 3;
pub const EMISSION_HOOK: EmissionState = 2;
pub const EMISSION_RUN: EmissionState = 1;
pub const EMISSION_STOP: EmissionState = 0;
pub type Handler = _Handler;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _Handler {
    pub sequential_number: gulong,
    pub next: *mut Handler,
    pub prev: *mut Handler,
    pub detail: GQuark,
    pub signal_id: guint,
    pub ref_count: guint,
    #[bitfield(name = "block_count", ty = "guint", bits = "0..=15")]
    #[bitfield(name = "after", ty = "guint", bits = "16..=16")]
    #[bitfield(name = "has_invalid_closure_notify", ty = "guint", bits = "17..=17")]
    pub block_count_after_has_invalid_closure_notify: [u8; 3],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub closure: *mut GClosure,
    pub instance: gpointer,
}
pub type HandlerList = _HandlerList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _HandlerList {
    pub signal_id: guint,
    pub handlers: *mut Handler,
    pub tail_before: *mut Handler,
    pub tail_after: *mut Handler,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SignalHook {
    pub hook: GHook,
    pub detail: GQuark,
}
pub type GObject = _GObject;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObject {
    pub g_type_instance: GTypeInstance,
    pub ref_count: guint,
    pub qdata: *mut GData,
}
pub const G_VALUE_COLLECT_POINTER: C2RustUnnamed_3 = 112;
pub const G_VALUE_COLLECT_DOUBLE: C2RustUnnamed_3 = 100;
pub const G_VALUE_COLLECT_INT64: C2RustUnnamed_3 = 113;
pub const G_VALUE_COLLECT_LONG: C2RustUnnamed_3 = 108;
pub const G_VALUE_COLLECT_INT: C2RustUnnamed_3 = 105;
pub type HandlerMatch = _HandlerMatch;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _HandlerMatch {
    pub handler: *mut Handler,
    pub next: *mut HandlerMatch,
    pub signal_id: guint,
}
pub type GRealClosure = _GRealClosure;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GRealClosure {
    pub meta_marshal: GClosureMarshal,
    pub meta_marshal_data: gpointer,
    pub va_meta_marshal: GVaClosureMarshal,
    pub va_marshal: GVaClosureMarshal,
    pub closure: GClosure,
}
pub type CallbackHandlerFunc = Option<unsafe extern "C" fn(gpointer, gulong) -> ()>;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const G_BSEARCH_ARRAY_AUTO_SHRINK: C2RustUnnamed_2 = 2;
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn g_bit_storage_impl(mut number: gulong) -> guint {
    let mut n_bits: guint = 0 as guint;
    loop {
        n_bits = n_bits.wrapping_add(1);
        number >>= 1 as ::core::ffi::c_int;
        if !(number != 0) {
            break;
        }
    }
    return n_bits;
}
#[inline]
unsafe extern "C" fn g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    ref_0 = *ptr;
    *ptr = ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
    return ref_0;
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
#[inline]
unsafe extern "C" fn g_bsearch_array_create(
    mut bconfig: *const GBSearchConfig,
) -> *mut GBSearchArray {
    let mut barray: *mut GBSearchArray = ::core::ptr::null_mut::<GBSearchArray>();
    let mut size: guint = 0;
    if !bconfig.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_bsearch_array_create\0" as *const u8 as *const ::core::ffi::c_char,
            b"bconfig != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBSearchArray>();
    }
    size = (::core::mem::size_of::<GBSearchArray>() as usize)
        .wrapping_add((*bconfig).sizeof_node as usize) as guint;
    if (*bconfig).flags & G_BSEARCH_ARRAY_ALIGN_POWER2 as ::core::ffi::c_int as guint != 0 {
        size = (if size != 0 {
            (1 as ::core::ffi::c_int) << g_bit_storage_impl(size.wrapping_sub(1 as guint) as gulong)
        } else {
            0 as ::core::ffi::c_int
        }) as guint;
    }
    barray = g_malloc(size as gsize) as *mut GBSearchArray;
    memset(
        barray as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<GBSearchArray>() as size_t,
    );
    return barray;
}
#[inline]
unsafe extern "C" fn g_bsearch_array_lookup_fuzzy(
    mut barray: *mut GBSearchArray,
    mut bconfig: *const GBSearchConfig,
    mut key_node: gconstpointer,
    sibling_or_after: guint,
) -> gpointer {
    let mut cmp_nodes: GBSearchCompareFunc = (*bconfig).cmp_nodes;
    let mut check: *mut guint8 = ::core::ptr::null_mut::<guint8>();
    let mut nodes: *mut guint8 =
        (barray as *mut guint8).offset(::core::mem::size_of::<GBSearchArray>() as usize as isize);
    let mut n_nodes: guint = (*barray).n_nodes;
    let mut offs: guint = 0 as guint;
    let mut sizeof_node: guint = (*bconfig).sizeof_node;
    let mut cmp: gint = 0 as gint;
    while offs < n_nodes {
        let mut i: guint = offs.wrapping_add(n_nodes) >> 1 as ::core::ffi::c_int;
        check = nodes.offset(i.wrapping_mul(sizeof_node) as isize);
        cmp = cmp_nodes.expect("non-null function pointer")(key_node, check as gconstpointer);
        if cmp == 0 as ::core::ffi::c_int {
            return (if sibling_or_after > 1 as guint {
                ::core::ptr::null_mut::<guint8>()
            } else {
                check
            }) as gpointer;
        } else if cmp < 0 as ::core::ffi::c_int {
            n_nodes = i;
        } else {
            offs = i.wrapping_add(1 as guint);
        }
    }
    return (if sibling_or_after == 0 {
        ::core::ptr::null_mut::<guint8>()
    } else if sibling_or_after > 1 as guint && cmp > 0 as ::core::ffi::c_int {
        check.offset(sizeof_node as isize)
    } else {
        check
    }) as gpointer;
}
#[inline]
unsafe extern "C" fn g_bsearch_array_get_nth(
    mut barray: *mut GBSearchArray,
    mut bconfig: *const GBSearchConfig,
    mut nth: guint,
) -> gpointer {
    return (if nth < (*barray).n_nodes {
        (barray as *mut guint8)
            .offset(::core::mem::size_of::<GBSearchArray>() as usize as isize)
            .offset(nth.wrapping_mul((*bconfig).sizeof_node) as isize)
    } else {
        ::core::ptr::null_mut::<guint8>()
    }) as gpointer;
}
#[inline]
unsafe extern "C" fn g_bsearch_array_get_index(
    mut barray: *mut GBSearchArray,
    mut bconfig: *const GBSearchConfig,
    mut node_in_array: gconstpointer,
) -> guint {
    let mut distance: guint = (node_in_array as *mut guint8).offset_from(
        (barray as *mut guint8).offset(::core::mem::size_of::<GBSearchArray>() as usize as isize),
    ) as ::core::ffi::c_long as guint;
    if !node_in_array.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_bsearch_array_get_index\0" as *const u8 as *const ::core::ffi::c_char,
            b"node_in_array != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return (*barray).n_nodes;
    }
    distance = distance.wrapping_div((*bconfig).sizeof_node);
    return if distance < (*barray).n_nodes.wrapping_add(1 as guint) {
        distance
    } else {
        (*barray).n_nodes.wrapping_add(1 as guint)
    };
}
#[inline]
unsafe extern "C" fn g_bsearch_array_grow(
    mut barray: *mut GBSearchArray,
    mut bconfig: *const GBSearchConfig,
    mut index_: guint,
) -> *mut GBSearchArray {
    let mut old_size: guint = (*barray).n_nodes.wrapping_mul((*bconfig).sizeof_node);
    let mut new_size: guint = old_size.wrapping_add((*bconfig).sizeof_node);
    let mut node: *mut guint8 = ::core::ptr::null_mut::<guint8>();
    if index_ <= (*barray).n_nodes {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_bsearch_array_grow\0" as *const u8 as *const ::core::ffi::c_char,
            b"index_ <= barray->n_nodes\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBSearchArray>();
    }
    if (*bconfig).flags & G_BSEARCH_ARRAY_ALIGN_POWER2 as ::core::ffi::c_int as guint != 0 {
        new_size = (if (::core::mem::size_of::<GBSearchArray>() as usize)
            .wrapping_add(new_size as usize)
            != 0
        {
            (1 as ::core::ffi::c_int)
                << g_bit_storage_impl(
                    (::core::mem::size_of::<GBSearchArray>() as gulong)
                        .wrapping_add(new_size as gulong)
                        .wrapping_sub(1 as gulong),
                )
        } else {
            0 as ::core::ffi::c_int
        }) as guint;
        old_size = (if (::core::mem::size_of::<GBSearchArray>() as usize)
            .wrapping_add(old_size as usize)
            != 0
        {
            (1 as ::core::ffi::c_int)
                << g_bit_storage_impl(
                    (::core::mem::size_of::<GBSearchArray>() as gulong)
                        .wrapping_add(old_size as gulong)
                        .wrapping_sub(1 as gulong),
                )
        } else {
            0 as ::core::ffi::c_int
        }) as guint;
        if old_size != new_size {
            barray = g_realloc(barray as gpointer, new_size as gsize) as *mut GBSearchArray;
        }
    } else {
        barray = g_realloc(
            barray as gpointer,
            (::core::mem::size_of::<GBSearchArray>() as gsize).wrapping_add(new_size as gsize),
        ) as *mut GBSearchArray;
    }
    node = (barray as *mut guint8)
        .offset(::core::mem::size_of::<GBSearchArray>() as usize as isize)
        .offset(index_.wrapping_mul((*bconfig).sizeof_node) as isize);
    memmove(
        node.offset((*bconfig).sizeof_node as isize) as *mut ::core::ffi::c_void,
        node as *const ::core::ffi::c_void,
        (*barray)
            .n_nodes
            .wrapping_sub(index_)
            .wrapping_mul((*bconfig).sizeof_node) as size_t,
    );
    (*barray).n_nodes = (*barray).n_nodes.wrapping_add(1 as guint);
    return barray;
}
#[inline]
unsafe extern "C" fn g_bsearch_array_insert(
    mut barray: *mut GBSearchArray,
    mut bconfig: *const GBSearchConfig,
    mut key_node: gconstpointer,
) -> *mut GBSearchArray {
    let mut node: *mut guint8 = ::core::ptr::null_mut::<guint8>();
    if (*barray).n_nodes == 0 {
        barray = g_bsearch_array_grow(barray, bconfig, 0 as guint);
        node = (barray as *mut guint8)
            .offset(::core::mem::size_of::<GBSearchArray>() as usize as isize);
    } else {
        node = g_bsearch_array_lookup_fuzzy(barray, bconfig, key_node, 2 as guint) as *mut guint8;
        if !node.is_null() {
            let mut index_: guint =
                g_bsearch_array_get_index(barray, bconfig, node as gconstpointer);
            barray = g_bsearch_array_grow(barray, bconfig, index_);
            node = (barray as *mut guint8)
                .offset(::core::mem::size_of::<GBSearchArray>() as usize as isize)
                .offset(index_.wrapping_mul((*bconfig).sizeof_node) as isize);
        } else {
            return barray;
        }
    }
    memcpy(
        node as *mut ::core::ffi::c_void,
        key_node as *const ::core::ffi::c_void,
        (*bconfig).sizeof_node as size_t,
    );
    return barray;
}
#[inline]
unsafe extern "C" fn g_bsearch_array_free(
    mut barray: *mut GBSearchArray,
    mut bconfig: *const GBSearchConfig,
) {
    if !barray.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_bsearch_array_free\0" as *const u8 as *const ::core::ffi::c_char,
            b"barray != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_free(barray as gpointer);
}
static mut g_signal_key_bsa: *mut GBSearchArray =
    ::core::ptr::null::<GBSearchArray>() as *mut GBSearchArray;
static mut g_signal_key_bconfig: GBSearchConfig = unsafe {
    GBSearchConfig {
        sizeof_node: ::core::mem::size_of::<SignalKey>() as guint,
        cmp_nodes: Some(
            signal_key_cmp as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint,
        ),
        flags: G_BSEARCH_ARRAY_ALIGN_POWER2 as ::core::ffi::c_int as guint,
    }
};
static mut g_signal_hlbsa_bconfig: GBSearchConfig = unsafe {
    GBSearchConfig {
        sizeof_node: ::core::mem::size_of::<HandlerList>() as guint,
        cmp_nodes: Some(
            handler_lists_cmp as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint,
        ),
        flags: 0 as guint,
    }
};
static mut g_class_closure_bconfig: GBSearchConfig = unsafe {
    GBSearchConfig {
        sizeof_node: ::core::mem::size_of::<ClassClosure>() as guint,
        cmp_nodes: Some(
            class_closures_cmp as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint,
        ),
        flags: 0 as guint,
    }
};
static mut g_handler_list_bsa_ht: *mut GHashTable =
    ::core::ptr::null::<GHashTable>() as *mut GHashTable;
static mut g_emissions: *mut Emission = ::core::ptr::null::<Emission>() as *mut Emission;
static mut g_handler_sequential_number: gulong = 1 as gulong;
static mut g_handlers: *mut GHashTable = ::core::ptr::null::<GHashTable>() as *mut GHashTable;
static mut g__g_signal_mutex_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
static mut g_n_signal_nodes: guint = 0 as guint;
static mut g_signal_nodes: *mut *mut SignalNode =
    ::core::ptr::null::<*mut SignalNode>() as *mut *mut SignalNode;
#[inline]
unsafe extern "C" fn LOOKUP_SIGNAL_NODE(mut signal_id: guint) -> *mut SignalNode {
    if signal_id < g_n_signal_nodes {
        return *g_signal_nodes.offset(signal_id as isize);
    } else {
        return ::core::ptr::null_mut::<SignalNode>();
    };
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
pub unsafe extern "C" fn g_signal_is_valid_name(mut name: *const gchar) -> gboolean {
    if strcmp(
        name as *const ::core::ffi::c_char,
        b"-gtk-private-changed\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    return g_param_spec_is_valid_name(name);
}
#[inline]
unsafe extern "C" fn signal_id_lookup(mut name: *const gchar, mut itype: GType) -> guint {
    let mut quark: GQuark = 0;
    let mut ifaces: *mut GType = ::core::ptr::null_mut::<GType>();
    let mut type_0: GType = itype;
    let mut key: SignalKey = _SignalKey {
        itype: 0,
        quark: 0,
        signal_id: 0,
    };
    let mut n_ifaces: guint = 0;
    quark = g_quark_try_string(name);
    key.quark = quark;
    loop {
        let mut signal_key: *mut SignalKey = ::core::ptr::null_mut::<SignalKey>();
        key.itype = type_0;
        signal_key = g_bsearch_array_lookup_fuzzy(
            g_signal_key_bsa,
            &raw const g_signal_key_bconfig,
            &raw mut key as gconstpointer,
            0 as guint,
        ) as *mut SignalKey;
        if !signal_key.is_null() {
            return (*signal_key).signal_id;
        }
        type_0 = g_type_parent(type_0);
        if !(type_0 != 0) {
            break;
        }
    }
    ifaces = g_type_interfaces(itype, &raw mut n_ifaces);
    loop {
        let fresh2 = n_ifaces;
        n_ifaces = n_ifaces.wrapping_sub(1);
        if !(fresh2 != 0) {
            break;
        }
        let mut signal_key_0: *mut SignalKey = ::core::ptr::null_mut::<SignalKey>();
        key.itype = *ifaces.offset(n_ifaces as isize);
        signal_key_0 = g_bsearch_array_lookup_fuzzy(
            g_signal_key_bsa,
            &raw const g_signal_key_bconfig,
            &raw mut key as gconstpointer,
            0 as guint,
        ) as *mut SignalKey;
        if !signal_key_0.is_null() {
            g_free(ifaces as gpointer);
            return (*signal_key_0).signal_id;
        }
    }
    g_free(ifaces as gpointer);
    if is_canonical(name) == 0 {
        let mut signal_id: guint = 0;
        let mut name_copy: *mut gchar =
            g_strdup_inline(name as *const ::core::ffi::c_char) as *mut gchar;
        canonicalize_key(name_copy);
        signal_id = signal_id_lookup(name_copy, itype);
        g_free(name_copy as gpointer);
        return signal_id;
    }
    return 0 as guint;
}
unsafe extern "C" fn class_closures_cmp(
    mut node1: gconstpointer,
    mut node2: gconstpointer,
) -> gint {
    let mut c1: *const ClassClosure = node1 as *const ClassClosure;
    let mut c2: *const ClassClosure = node2 as *const ClassClosure;
    return if (*c1).instance_type > (*c2).instance_type {
        1 as gint
    } else if (*c1).instance_type == (*c2).instance_type {
        0 as gint
    } else {
        -(1 as gint)
    };
}
unsafe extern "C" fn handler_lists_cmp(mut node1: gconstpointer, mut node2: gconstpointer) -> gint {
    let mut hlist1: *const HandlerList = node1 as *const HandlerList;
    let mut hlist2: *const HandlerList = node2 as *const HandlerList;
    return if (*hlist1).signal_id > (*hlist2).signal_id {
        1 as gint
    } else if (*hlist1).signal_id == (*hlist2).signal_id {
        0 as gint
    } else {
        -(1 as gint)
    };
}
#[inline]
unsafe extern "C" fn handler_list_ensure(
    mut signal_id: guint,
    mut instance: gpointer,
) -> *mut HandlerList {
    let mut hlbsa: *mut GBSearchArray =
        g_hash_table_lookup(g_handler_list_bsa_ht, instance as gconstpointer) as *mut GBSearchArray;
    let mut key: HandlerList = _HandlerList {
        signal_id: 0,
        handlers: ::core::ptr::null_mut::<Handler>(),
        tail_before: ::core::ptr::null_mut::<Handler>(),
        tail_after: ::core::ptr::null_mut::<Handler>(),
    };
    key.signal_id = signal_id;
    key.handlers = ::core::ptr::null_mut::<Handler>();
    key.tail_before = ::core::ptr::null_mut::<Handler>();
    key.tail_after = ::core::ptr::null_mut::<Handler>();
    if hlbsa.is_null() {
        hlbsa = g_bsearch_array_create(&raw mut g_signal_hlbsa_bconfig);
    }
    hlbsa = g_bsearch_array_insert(
        hlbsa,
        &raw mut g_signal_hlbsa_bconfig,
        &raw mut key as gconstpointer,
    );
    g_hash_table_insert(g_handler_list_bsa_ht, instance, hlbsa as gpointer);
    return g_bsearch_array_lookup_fuzzy(
        hlbsa,
        &raw mut g_signal_hlbsa_bconfig,
        &raw mut key as gconstpointer,
        0 as guint,
    ) as *mut HandlerList;
}
#[inline]
unsafe extern "C" fn handler_list_lookup(
    mut signal_id: guint,
    mut instance: gpointer,
) -> *mut HandlerList {
    let mut hlbsa: *mut GBSearchArray =
        g_hash_table_lookup(g_handler_list_bsa_ht, instance as gconstpointer) as *mut GBSearchArray;
    let mut key: HandlerList = _HandlerList {
        signal_id: 0,
        handlers: ::core::ptr::null_mut::<Handler>(),
        tail_before: ::core::ptr::null_mut::<Handler>(),
        tail_after: ::core::ptr::null_mut::<Handler>(),
    };
    key.signal_id = signal_id;
    return (if !hlbsa.is_null() {
        g_bsearch_array_lookup_fuzzy(
            hlbsa,
            &raw mut g_signal_hlbsa_bconfig,
            &raw mut key as gconstpointer,
            0 as guint,
        )
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_void>()
    }) as *mut HandlerList;
}
unsafe extern "C" fn handler_hash(mut key: gconstpointer) -> guint {
    return (*(key as *mut Handler)).sequential_number as guint;
}
unsafe extern "C" fn handler_equal(mut a: gconstpointer, mut b: gconstpointer) -> gboolean {
    let mut ha: *mut Handler = a as *mut Handler;
    let mut hb: *mut Handler = b as *mut Handler;
    return ((*ha).sequential_number == (*hb).sequential_number && (*ha).instance == (*hb).instance)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn handler_lookup(
    mut instance: gpointer,
    mut handler_id: gulong,
    mut closure: *mut GClosure,
    mut signal_id_p: *mut guint,
) -> *mut Handler {
    let mut hlbsa: *mut GBSearchArray = ::core::ptr::null_mut::<GBSearchArray>();
    if handler_id != 0 {
        let mut key: Handler = _Handler {
            sequential_number: 0,
            next: ::core::ptr::null_mut::<Handler>(),
            prev: ::core::ptr::null_mut::<Handler>(),
            detail: 0,
            signal_id: 0,
            ref_count: 0,
            block_count_after_has_invalid_closure_notify: [0; 3],
            c2rust_padding: [0; 1],
            closure: ::core::ptr::null_mut::<GClosure>(),
            instance: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        };
        key.sequential_number = handler_id;
        key.instance = instance;
        return g_hash_table_lookup(g_handlers, &raw mut key as gconstpointer) as *mut Handler;
    }
    hlbsa =
        g_hash_table_lookup(g_handler_list_bsa_ht, instance as gconstpointer) as *mut GBSearchArray;
    if !hlbsa.is_null() {
        let mut i: guint = 0;
        i = 0 as guint;
        while i < (*hlbsa).n_nodes {
            let mut hlist: *mut HandlerList =
                g_bsearch_array_get_nth(hlbsa, &raw mut g_signal_hlbsa_bconfig, i)
                    as *mut HandlerList;
            let mut handler: *mut Handler = ::core::ptr::null_mut::<Handler>();
            handler = (*hlist).handlers;
            while !handler.is_null() {
                if if !closure.is_null() {
                    ((*handler).closure == closure) as ::core::ffi::c_int
                } else {
                    ((*handler).sequential_number == handler_id) as ::core::ffi::c_int
                } != 0
                {
                    if !signal_id_p.is_null() {
                        *signal_id_p = (*hlist).signal_id;
                    }
                    return handler;
                }
                handler = (*handler).next;
            }
            i = i.wrapping_add(1);
        }
    }
    return ::core::ptr::null_mut::<Handler>();
}
#[inline]
unsafe extern "C" fn handler_match_prepend(
    mut list: *mut HandlerMatch,
    mut handler: *mut Handler,
    mut signal_id: guint,
) -> *mut HandlerMatch {
    let mut node: *mut HandlerMatch = ::core::ptr::null_mut::<HandlerMatch>();
    node = g_slice_alloc(::core::mem::size_of::<HandlerMatch>() as gsize) as *mut HandlerMatch;
    (*node).handler = handler;
    (*node).next = list;
    (*node).signal_id = signal_id;
    handler_ref(handler);
    return node;
}
#[inline]
unsafe extern "C" fn handler_match_free1_R(
    mut node: *mut HandlerMatch,
    mut instance: gpointer,
) -> *mut HandlerMatch {
    let mut next: *mut HandlerMatch = (*node).next;
    handler_unref_R((*node).signal_id, instance, (*node).handler);
    g_slice_free1(
        ::core::mem::size_of::<HandlerMatch>() as gsize,
        node as gpointer,
    );
    return next;
}
unsafe extern "C" fn handlers_find(
    mut instance: gpointer,
    mut mask: GSignalMatchType,
    mut signal_id: guint,
    mut detail: GQuark,
    mut closure: *mut GClosure,
    mut func: gpointer,
    mut data: gpointer,
    mut one_and_only: gboolean,
) -> *mut HandlerMatch {
    let mut mlist: *mut HandlerMatch = ::core::ptr::null_mut::<HandlerMatch>();
    if mask as ::core::ffi::c_uint & G_SIGNAL_MATCH_ID as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        let mut hlist: *mut HandlerList = handler_list_lookup(signal_id, instance);
        let mut handler: *mut Handler = ::core::ptr::null_mut::<Handler>();
        let mut node: *mut SignalNode = ::core::ptr::null_mut::<SignalNode>();
        if mask as ::core::ffi::c_uint
            & G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            node = LOOKUP_SIGNAL_NODE(signal_id);
            if node.is_null() || (*node).c_marshaller.is_none() {
                return ::core::ptr::null_mut::<HandlerMatch>();
            }
        }
        mask = !(mask as ::core::ffi::c_uint) as GSignalMatchType;
        handler = if !hlist.is_null() {
            (*hlist).handlers
        } else {
            ::core::ptr::null_mut::<Handler>()
        };
        while !handler.is_null() {
            if (*handler).sequential_number != 0
                && (mask as ::core::ffi::c_uint
                    & G_SIGNAL_MATCH_DETAIL as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0
                    || (*handler).detail == detail)
                && (mask as ::core::ffi::c_uint
                    & G_SIGNAL_MATCH_CLOSURE as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0
                    || (*handler).closure == closure)
                && (mask as ::core::ffi::c_uint
                    & G_SIGNAL_MATCH_DATA as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0
                    || (*(*handler).closure).data == data)
                && (mask as ::core::ffi::c_uint
                    & G_SIGNAL_MATCH_UNBLOCKED as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0
                    || (*handler).block_count() as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
                && (mask as ::core::ffi::c_uint
                    & G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0
                    || (*(*handler).closure).marshal == (*node).c_marshaller
                        && (*(((*handler).closure as *mut guint8)
                            .offset(-(32 as ::core::ffi::c_ulong as glong) as isize)
                            as gpointer as *mut GRealClosure))
                            .meta_marshal
                            .is_none()
                        && (*((*handler).closure as *mut GCClosure)).callback == func)
            {
                mlist = handler_match_prepend(mlist, handler, signal_id);
                if one_and_only != 0 {
                    return mlist;
                }
            }
            handler = (*handler).next;
        }
    } else {
        let mut hlbsa: *mut GBSearchArray =
            g_hash_table_lookup(g_handler_list_bsa_ht, instance as gconstpointer)
                as *mut GBSearchArray;
        mask = !(mask as ::core::ffi::c_uint) as GSignalMatchType;
        if !hlbsa.is_null() {
            let mut i: guint = 0;
            let mut current_block_23: u64;
            i = 0 as guint;
            while i < (*hlbsa).n_nodes {
                let mut hlist_0: *mut HandlerList =
                    g_bsearch_array_get_nth(hlbsa, &raw mut g_signal_hlbsa_bconfig, i)
                        as *mut HandlerList;
                let mut node_0: *mut SignalNode = ::core::ptr::null_mut::<SignalNode>();
                let mut handler_0: *mut Handler = ::core::ptr::null_mut::<Handler>();
                if mask as ::core::ffi::c_uint
                    & G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int as ::core::ffi::c_uint
                    == 0
                {
                    node_0 = LOOKUP_SIGNAL_NODE((*hlist_0).signal_id);
                    if (*node_0).c_marshaller.is_none() {
                        current_block_23 = 8831408221741692167;
                    } else {
                        current_block_23 = 4495394744059808450;
                    }
                } else {
                    current_block_23 = 4495394744059808450;
                }
                match current_block_23 {
                    4495394744059808450 => {
                        handler_0 = (*hlist_0).handlers;
                        while !handler_0.is_null() {
                            if (*handler_0).sequential_number != 0
                                && (mask as ::core::ffi::c_uint
                                    & G_SIGNAL_MATCH_DETAIL as ::core::ffi::c_int
                                        as ::core::ffi::c_uint
                                    != 0
                                    || (*handler_0).detail == detail)
                                && (mask as ::core::ffi::c_uint
                                    & G_SIGNAL_MATCH_CLOSURE as ::core::ffi::c_int
                                        as ::core::ffi::c_uint
                                    != 0
                                    || (*handler_0).closure == closure)
                                && (mask as ::core::ffi::c_uint
                                    & G_SIGNAL_MATCH_DATA as ::core::ffi::c_int
                                        as ::core::ffi::c_uint
                                    != 0
                                    || (*(*handler_0).closure).data == data)
                                && (mask as ::core::ffi::c_uint
                                    & G_SIGNAL_MATCH_UNBLOCKED as ::core::ffi::c_int
                                        as ::core::ffi::c_uint
                                    != 0
                                    || (*handler_0).block_count() as ::core::ffi::c_int
                                        == 0 as ::core::ffi::c_int)
                                && (mask as ::core::ffi::c_uint
                                    & G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int
                                        as ::core::ffi::c_uint
                                    != 0
                                    || (*(*handler_0).closure).marshal == (*node_0).c_marshaller
                                        && (*(((*handler_0).closure as *mut guint8)
                                            .offset(-(32 as ::core::ffi::c_ulong as glong) as isize)
                                            as gpointer
                                            as *mut GRealClosure))
                                            .meta_marshal
                                            .is_none()
                                        && (*((*handler_0).closure as *mut GCClosure)).callback
                                            == func)
                            {
                                mlist =
                                    handler_match_prepend(mlist, handler_0, (*hlist_0).signal_id);
                                if one_and_only != 0 {
                                    return mlist;
                                }
                            }
                            handler_0 = (*handler_0).next;
                        }
                    }
                    _ => {}
                }
                i = i.wrapping_add(1);
            }
        }
    }
    return mlist;
}
#[inline]
unsafe extern "C" fn handler_new(
    mut signal_id: guint,
    mut instance: gpointer,
    mut after: gboolean,
) -> *mut Handler {
    let mut handler: *mut Handler =
        g_slice_alloc(::core::mem::size_of::<Handler>() as gsize) as *mut Handler;
    if g_handler_sequential_number < 1 as gulong {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:590: handler id overflow, %s\0"
                as *const u8 as *const gchar,
            b"please report occurrence circumstances to https://gitlab.gnome.org/GNOME/glib/issues/new\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        loop {}
    }
    let fresh14 = g_handler_sequential_number;
    g_handler_sequential_number = g_handler_sequential_number.wrapping_add(1);
    (*handler).sequential_number = fresh14;
    (*handler).prev = ::core::ptr::null_mut::<Handler>();
    (*handler).next = ::core::ptr::null_mut::<Handler>();
    (*handler).detail = 0 as GQuark;
    (*handler).signal_id = signal_id;
    (*handler).instance = instance;
    (*handler).ref_count = 1 as guint;
    (*handler).set_block_count(0 as guint as guint);
    (*handler)
        .set_after((after != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as guint as guint);
    (*handler).closure = ::core::ptr::null_mut::<GClosure>();
    (*handler).set_has_invalid_closure_notify(0 as guint as guint);
    g_hash_table_add(g_handlers, handler as gpointer);
    return handler;
}
#[inline]
unsafe extern "C" fn handler_ref(mut handler: *mut Handler) {
    if (*handler).ref_count > 0 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"handler_ref\0" as *const u8 as *const ::core::ffi::c_char,
            b"handler->ref_count > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*handler).ref_count = (*handler).ref_count.wrapping_add(1);
}
#[inline]
unsafe extern "C" fn handler_unref_R(
    mut signal_id: guint,
    mut instance: gpointer,
    mut handler: *mut Handler,
) {
    if (*handler).ref_count > 0 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"handler_unref_R\0" as *const u8 as *const ::core::ffi::c_char,
            b"handler->ref_count > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*handler).ref_count = (*handler).ref_count.wrapping_sub(1);
    if (*handler).ref_count == 0 as guint {
        let mut hlist: *mut HandlerList = ::core::ptr::null_mut::<HandlerList>();
        if !(*handler).next.is_null() {
            (*(*handler).next).prev = (*handler).prev;
        }
        if !(*handler).prev.is_null() {
            (*(*handler).prev).next = (*handler).next;
        } else {
            hlist = handler_list_lookup(signal_id, instance);
            if !hlist.is_null() {
            } else {
                g_assertion_message_expr(
                    b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    638 as ::core::ffi::c_int,
                    b"handler_unref_R\0" as *const u8 as *const ::core::ffi::c_char,
                    b"hlist != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            (*hlist).handlers = (*handler).next;
        }
        if !instance.is_null() {
            if (*handler).after() == 0
                && ((*handler).next.is_null()
                    || (*(*handler).next).after() as ::core::ffi::c_int != 0)
            {
                if hlist.is_null() {
                    hlist = handler_list_lookup(signal_id, instance);
                }
                if !hlist.is_null() {
                    if (*hlist).tail_before == handler {
                    } else {
                        g_assertion_message_expr(
                            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            651 as ::core::ffi::c_int,
                            b"handler_unref_R\0" as *const u8
                                as *const ::core::ffi::c_char,
                            b"hlist->tail_before == handler\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                    (*hlist).tail_before = (*handler).prev;
                }
            }
            if (*handler).next.is_null() {
                if hlist.is_null() {
                    hlist = handler_list_lookup(signal_id, instance);
                }
                if !hlist.is_null() {
                    if (*hlist).tail_after == handler {
                    } else {
                        g_assertion_message_expr(
                            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            663 as ::core::ffi::c_int,
                            b"handler_unref_R\0" as *const u8
                                as *const ::core::ffi::c_char,
                            b"hlist->tail_after == handler\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                    (*hlist).tail_after = (*handler).prev;
                }
            }
        }
        g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
        g_closure_unref((*handler).closure);
        g_mutex_lock(&raw mut g__g_signal_mutex_lock);
        g_slice_free1(
            ::core::mem::size_of::<Handler>() as gsize,
            handler as gpointer,
        );
    }
}
unsafe extern "C" fn handler_insert(
    mut signal_id: guint,
    mut instance: gpointer,
    mut handler: *mut Handler,
) {
    let mut hlist: *mut HandlerList = ::core::ptr::null_mut::<HandlerList>();
    if (*handler).prev.is_null() && (*handler).next.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c\0" as *const u8
                as *const ::core::ffi::c_char,
            683 as ::core::ffi::c_int,
            b"handler_insert\0" as *const u8 as *const ::core::ffi::c_char,
            b"handler->prev == NULL && handler->next == NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    hlist = handler_list_ensure(signal_id, instance);
    if (*hlist).handlers.is_null() {
        (*hlist).handlers = handler;
        if (*handler).after() == 0 {
            (*hlist).tail_before = handler;
        }
    } else if (*handler).after() != 0 {
        (*handler).prev = (*hlist).tail_after;
        (*(*hlist).tail_after).next = handler;
    } else {
        if !(*hlist).tail_before.is_null() {
            (*handler).next = (*(*hlist).tail_before).next;
            if !(*handler).next.is_null() {
                (*(*handler).next).prev = handler;
            }
            (*handler).prev = (*hlist).tail_before;
            (*(*hlist).tail_before).next = handler;
        } else {
            (*handler).next = (*hlist).handlers;
            if !(*handler).next.is_null() {
                (*(*handler).next).prev = handler;
            }
            (*hlist).handlers = handler;
        }
        (*hlist).tail_before = handler;
    }
    if (*handler).next.is_null() {
        (*hlist).tail_after = handler;
    }
}
unsafe extern "C" fn node_update_single_va_closure(mut node: *mut SignalNode) {
    let mut closure: *mut GClosure = ::core::ptr::null_mut::<GClosure>();
    let mut is_after: gboolean = 0 as gboolean;
    if g_type_fundamental((*node).itype)
        == ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
        && (*node).flags() as ::core::ffi::c_int & G_SIGNAL_MUST_COLLECT as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        && ((*node).emission_hooks.is_null() || (*(*node).emission_hooks).hooks.is_null())
    {
        let mut run_type: GSignalFlags = 0 as GSignalFlags;
        let mut cc: *mut ClassClosure = ::core::ptr::null_mut::<ClassClosure>();
        let mut bsa: *mut GBSearchArray = (*node).class_closure_bsa;
        if bsa.is_null() || (*bsa).n_nodes == 0 as guint {
            closure = 1 as ::core::ffi::c_int as glong as gpointer as *mut GClosure;
        } else if (*bsa).n_nodes == 1 as guint {
            cc = g_bsearch_array_get_nth(bsa, &raw mut g_class_closure_bconfig, 0 as guint)
                as *mut ClassClosure;
            if (*cc).instance_type == 0 as GType {
                run_type = ((*node).flags() as ::core::ffi::c_int
                    & (G_SIGNAL_RUN_FIRST as ::core::ffi::c_int
                        | G_SIGNAL_RUN_LAST as ::core::ffi::c_int
                        | G_SIGNAL_RUN_CLEANUP as ::core::ffi::c_int))
                    as GSignalFlags;
                if run_type as ::core::ffi::c_uint
                    == G_SIGNAL_RUN_FIRST as ::core::ffi::c_int as ::core::ffi::c_uint
                    || run_type as ::core::ffi::c_uint
                        == G_SIGNAL_RUN_LAST as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    closure = (*cc).closure;
                    is_after = (run_type as ::core::ffi::c_uint
                        == G_SIGNAL_RUN_LAST as ::core::ffi::c_int as ::core::ffi::c_uint)
                        as ::core::ffi::c_int as gboolean;
                }
            }
        }
    }
    (*node).set_single_va_closure_is_valid(
        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as guint as guint,
    );
    (*node).single_va_closure = closure;
    (*node).set_single_va_closure_is_after(is_after as guint as guint);
}
#[inline]
unsafe extern "C" fn emission_push(mut emission: *mut Emission) {
    (*emission).next = g_emissions;
    g_emissions = emission;
}
#[inline]
unsafe extern "C" fn emission_pop(mut emission: *mut Emission) {
    let mut node: *mut Emission = ::core::ptr::null_mut::<Emission>();
    let mut last: *mut Emission = ::core::ptr::null_mut::<Emission>();
    node = g_emissions;
    while !node.is_null() {
        if node == emission {
            if !last.is_null() {
                (*last).next = (*node).next;
            } else {
                g_emissions = (*node).next;
            }
            return;
        }
        last = node;
        node = (*last).next;
    }
    g_assertion_message_expr(
        b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
        b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c\0" as *const u8
            as *const ::core::ffi::c_char,
        783 as ::core::ffi::c_int,
        b"emission_pop\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
#[inline]
unsafe extern "C" fn emission_find(
    mut signal_id: guint,
    mut detail: GQuark,
    mut instance: gpointer,
) -> *mut Emission {
    let mut emission: *mut Emission = ::core::ptr::null_mut::<Emission>();
    emission = g_emissions;
    while !emission.is_null() {
        if (*emission).instance == instance
            && (*emission).ihint.signal_id == signal_id
            && (*emission).ihint.detail == detail
        {
            return emission;
        }
        emission = (*emission).next;
    }
    return ::core::ptr::null_mut::<Emission>();
}
#[inline]
unsafe extern "C" fn emission_find_innermost(mut instance: gpointer) -> *mut Emission {
    let mut emission: *mut Emission = ::core::ptr::null_mut::<Emission>();
    emission = g_emissions;
    while !emission.is_null() {
        if (*emission).instance == instance {
            return emission;
        }
        emission = (*emission).next;
    }
    return ::core::ptr::null_mut::<Emission>();
}
unsafe extern "C" fn signal_key_cmp(mut node1: gconstpointer, mut node2: gconstpointer) -> gint {
    let mut key1: *const SignalKey = node1 as *const SignalKey;
    let mut key2: *const SignalKey = node2 as *const SignalKey;
    if (*key1).itype == (*key2).itype {
        return if (*key1).quark > (*key2).quark {
            1 as gint
        } else if (*key1).quark == (*key2).quark {
            0 as gint
        } else {
            -(1 as gint)
        };
    } else {
        return if (*key1).itype > (*key2).itype {
            1 as gint
        } else if (*key1).itype == (*key2).itype {
            0 as gint
        } else {
            -(1 as gint)
        };
    };
}
#[no_mangle]
pub unsafe extern "C" fn _g_signal_init() {
    g_mutex_lock(&raw mut g__g_signal_mutex_lock);
    if g_n_signal_nodes == 0 {
        g_handler_list_bsa_ht = g_hash_table_new(
            Some(g_direct_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            None,
        );
        g_signal_key_bsa = g_bsearch_array_create(&raw const g_signal_key_bconfig);
        g_n_signal_nodes = 1 as guint;
        g_signal_nodes = g_realloc_n(
            g_signal_nodes as gpointer,
            g_n_signal_nodes as gsize,
            ::core::mem::size_of::<*mut SignalNode>() as gsize,
        ) as *mut *mut SignalNode;
        let ref mut fresh20 = *g_signal_nodes.offset(0 as ::core::ffi::c_int as isize);
        *fresh20 = ::core::ptr::null_mut::<SignalNode>();
        g_handlers = g_hash_table_new(
            Some(handler_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(handler_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        );
    }
    g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
}
#[no_mangle]
pub unsafe extern "C" fn _g_signals_destroy(mut itype: GType) {
    let mut i: guint = 0;
    g_mutex_lock(&raw mut g__g_signal_mutex_lock);
    i = 1 as guint;
    while i < g_n_signal_nodes {
        let mut node: *mut SignalNode = *g_signal_nodes.offset(i as isize);
        if (*node).itype == itype {
            if (*node).destroyed() != 0 {
                g_log(
                    b"GLib-GObject\0" as *const u8 as *const gchar,
                    G_LOG_LEVEL_CRITICAL,
                    b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:857: signal \"%s\" of type '%s' already destroyed\0"
                        as *const u8 as *const gchar,
                    (*node).name,
                    type_debug_name((*node).itype),
                );
            } else {
                signal_destroy_R(node);
            }
        }
        i = i.wrapping_add(1);
    }
    g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_stop_emission(
    mut instance: gpointer,
    mut signal_id: guint,
    mut detail: GQuark,
) {
    let mut node: *mut SignalNode = ::core::ptr::null_mut::<SignalNode>();
    if g_type_check_instance(instance as *mut GTypeInstance) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_stop_emission\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_CHECK_INSTANCE (instance)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if signal_id > 0 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_stop_emission\0" as *const u8 as *const ::core::ffi::c_char,
            b"signal_id > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut g__g_signal_mutex_lock);
    node = LOOKUP_SIGNAL_NODE(signal_id);
    if !node.is_null()
        && detail != 0
        && (*node).flags() as ::core::ffi::c_int & G_SIGNAL_DETAILED as ::core::ffi::c_int == 0
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: signal id '%u' does not support detail (%u)\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:895\0"
                as *const u8 as *const ::core::ffi::c_char,
            signal_id,
            detail,
        );
        g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
        return;
    }
    if !node.is_null()
        && ((*(*(instance as *mut GTypeInstance)).g_class).g_type == (*node).itype
            || g_type_is_a(
                (*(*(instance as *mut GTypeInstance)).g_class).g_type,
                (*node).itype,
            ) != 0)
    {
        let mut emission: *mut Emission = emission_find(signal_id, detail, instance);
        if !emission.is_null() {
            if (*emission).state as ::core::ffi::c_uint
                == EMISSION_HOOK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                g_log(
                    b"GLib-GObject\0" as *const u8 as *const gchar,
                    G_LOG_LEVEL_CRITICAL,
                    b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:906: emission of signal \"%s\" for instance '%p' cannot be stopped from emission hook\0"
                        as *const u8 as *const gchar,
                    (*node).name,
                    instance,
                );
            } else if (*emission).state as ::core::ffi::c_uint
                == EMISSION_RUN as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*emission).state = EMISSION_STOP;
            }
        } else {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:912: no emission of signal \"%s\" to stop for instance '%p'\0"
                    as *const u8 as *const gchar,
                (*node).name,
                instance,
            );
        }
    } else {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: signal id '%u' is invalid for instance '%p'\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:916\0"
                as *const u8 as *const ::core::ffi::c_char,
            signal_id,
            instance,
        );
    }
    g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
}
unsafe extern "C" fn signal_finalize_hook(mut hook_list: *mut GHookList, mut hook: *mut GHook) {
    let mut destroy: GDestroyNotify = (*hook).destroy;
    if destroy.is_some() {
        (*hook).destroy = None;
        g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
        destroy.expect("non-null function pointer")((*hook).data);
        g_mutex_lock(&raw mut g__g_signal_mutex_lock);
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_add_emission_hook(
    mut signal_id: guint,
    mut detail: GQuark,
    mut hook_func: GSignalEmissionHook,
    mut hook_data: gpointer,
    mut data_destroy: GDestroyNotify,
) -> gulong {
    static mut seq_hook_id: gulong = 1 as gulong;
    let mut node: *mut SignalNode = ::core::ptr::null_mut::<SignalNode>();
    let mut hook: *mut GHook = ::core::ptr::null_mut::<GHook>();
    let mut signal_hook: *mut SignalHook = ::core::ptr::null_mut::<SignalHook>();
    if signal_id > 0 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_add_emission_hook\0" as *const u8 as *const ::core::ffi::c_char,
            b"signal_id > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gulong;
    }
    if hook_func.is_some() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_add_emission_hook\0" as *const u8 as *const ::core::ffi::c_char,
            b"hook_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gulong;
    }
    g_mutex_lock(&raw mut g__g_signal_mutex_lock);
    node = LOOKUP_SIGNAL_NODE(signal_id);
    if node.is_null() || (*node).destroyed() as ::core::ffi::c_int != 0 {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: invalid signal id '%u'\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:968\0"
                as *const u8 as *const ::core::ffi::c_char,
            signal_id,
        );
        g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
        return 0 as gulong;
    }
    if (*node).flags() as ::core::ffi::c_int & G_SIGNAL_NO_HOOKS as ::core::ffi::c_int != 0 {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: signal id '%u' does not support emission hooks (G_SIGNAL_NO_HOOKS flag set)\0"
                as *const u8 as *const gchar,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:974\0"
                as *const u8 as *const ::core::ffi::c_char,
            signal_id,
        );
        g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
        return 0 as gulong;
    }
    if detail != 0
        && (*node).flags() as ::core::ffi::c_int & G_SIGNAL_DETAILED as ::core::ffi::c_int == 0
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: signal id '%u' does not support detail (%u)\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:980\0"
                as *const u8 as *const ::core::ffi::c_char,
            signal_id,
            detail,
        );
        g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
        return 0 as gulong;
    }
    (*node).set_single_va_closure_is_valid(0 as guint as guint);
    if (*node).emission_hooks.is_null() {
        (*node).emission_hooks =
            g_malloc_n(1 as gsize, ::core::mem::size_of::<GHookList>() as gsize) as *mut GHookList;
        g_hook_list_init(
            (*node).emission_hooks,
            ::core::mem::size_of::<SignalHook>() as guint,
        );
        (*(*node).emission_hooks).finalize_hook =
            Some(signal_finalize_hook as unsafe extern "C" fn(*mut GHookList, *mut GHook) -> ())
                as GHookFinalizeFunc;
    }
    node_check_deprecated(node);
    hook = g_hook_alloc((*node).emission_hooks);
    (*hook).data = hook_data;
    (*hook).func = ::core::mem::transmute::<GSignalEmissionHook, gpointer>(hook_func);
    (*hook).destroy = data_destroy;
    signal_hook = hook as *mut SignalHook;
    (*signal_hook).detail = detail;
    (*(*node).emission_hooks).seq_id = seq_hook_id;
    g_hook_insert_before(
        (*node).emission_hooks,
        ::core::ptr::null_mut::<GHook>(),
        hook,
    );
    seq_hook_id = (*(*node).emission_hooks).seq_id;
    g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
    return (*hook).hook_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_remove_emission_hook(mut signal_id: guint, mut hook_id: gulong) {
    let mut node: *mut SignalNode = ::core::ptr::null_mut::<SignalNode>();
    if signal_id > 0 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_remove_emission_hook\0" as *const u8 as *const ::core::ffi::c_char,
            b"signal_id > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if hook_id > 0 as gulong {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_remove_emission_hook\0" as *const u8 as *const ::core::ffi::c_char,
            b"hook_id > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut g__g_signal_mutex_lock);
    node = LOOKUP_SIGNAL_NODE(signal_id);
    if node.is_null() || (*node).destroyed() as ::core::ffi::c_int != 0 {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: invalid signal id '%u'\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:1030\0"
                as *const u8 as *const ::core::ffi::c_char,
            signal_id,
        );
    } else {
        if (*node).emission_hooks.is_null() || g_hook_destroy((*node).emission_hooks, hook_id) == 0
        {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"%s: signal \"%s\" had no hook (%lu) to remove\0" as *const u8 as *const gchar,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:1034\0"
                    as *const u8 as *const ::core::ffi::c_char,
                (*node).name,
                hook_id,
            );
        }
        (*node).set_single_va_closure_is_valid(0 as guint as guint);
    }
    g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
}
#[inline]
unsafe extern "C" fn signal_parse_name(
    mut name: *const gchar,
    mut itype: GType,
    mut detail_p: *mut GQuark,
    mut force_quark: gboolean,
) -> guint {
    let mut colon: *const gchar = strchr(name as *const ::core::ffi::c_char, ':' as i32);
    let mut signal_id: guint = 0;
    if colon.is_null() {
        signal_id = signal_id_lookup(name, itype);
        if signal_id != 0 && !detail_p.is_null() {
            *detail_p = 0 as GQuark;
        }
    } else if *colon.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ':' as i32 {
        let mut buffer: [gchar; 32] = [0; 32];
        let mut l: guint = colon.offset_from(name) as ::core::ffi::c_long as guint;
        if *colon.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32 {
            return 0 as guint;
        }
        if l < 32 as guint {
            memcpy(
                &raw mut buffer as *mut gchar as *mut ::core::ffi::c_void,
                name as *const ::core::ffi::c_void,
                l as size_t,
            );
            buffer[l as usize] = 0 as gchar;
            signal_id = signal_id_lookup(&raw mut buffer as *mut gchar, itype);
        } else {
            let mut signal: *mut gchar = g_malloc_n(
                l.wrapping_add(1 as guint) as gsize,
                ::core::mem::size_of::<gchar>() as gsize,
            ) as *mut gchar;
            memcpy(
                signal as *mut ::core::ffi::c_void,
                name as *const ::core::ffi::c_void,
                l as size_t,
            );
            *signal.offset(l as isize) = 0 as gchar;
            signal_id = signal_id_lookup(signal, itype);
            g_free(signal as gpointer);
        }
        if signal_id != 0 && !detail_p.is_null() {
            *detail_p = if force_quark != 0 {
                Some(g_quark_from_string as unsafe extern "C" fn(*const gchar) -> GQuark)
            } else {
                Some(g_quark_try_string as unsafe extern "C" fn(*const gchar) -> GQuark)
            }
            .expect("non-null function pointer")(
                colon.offset(2 as ::core::ffi::c_int as isize)
            );
        }
    } else {
        signal_id = 0 as guint;
    }
    return signal_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_parse_name(
    mut detailed_signal: *const gchar,
    mut itype: GType,
    mut signal_id_p: *mut guint,
    mut detail_p: *mut GQuark,
    mut force_detail_quark: gboolean,
) -> gboolean {
    let mut node: *mut SignalNode = ::core::ptr::null_mut::<SignalNode>();
    let mut detail: GQuark = 0 as GQuark;
    let mut signal_id: guint = 0;
    if !detailed_signal.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_parse_name\0" as *const u8 as *const ::core::ffi::c_char,
            b"detailed_signal != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if g_type_test_flags(
        itype,
        G_TYPE_FLAG_INSTANTIATABLE as ::core::ffi::c_int as guint,
    ) != 0
        || g_type_fundamental(itype)
            == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_parse_name\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_IS_INSTANTIATABLE (itype) || G_TYPE_IS_INTERFACE (itype)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    g_mutex_lock(&raw mut g__g_signal_mutex_lock);
    signal_id = signal_parse_name(detailed_signal, itype, &raw mut detail, force_detail_quark);
    node = if signal_id != 0 {
        LOOKUP_SIGNAL_NODE(signal_id)
    } else {
        ::core::ptr::null_mut::<SignalNode>()
    };
    if node.is_null()
        || (*node).destroyed() as ::core::ffi::c_int != 0
        || detail != 0
            && (*node).flags() as ::core::ffi::c_int & G_SIGNAL_DETAILED as ::core::ffi::c_int == 0
    {
        g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
        return 0 as gboolean;
    }
    g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
    if !signal_id_p.is_null() {
        *signal_id_p = signal_id;
    }
    if !detail_p.is_null() {
        *detail_p = detail;
    }
    return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_stop_emission_by_name(
    mut instance: gpointer,
    mut detailed_signal: *const gchar,
) {
    let mut signal_id: guint = 0;
    let mut detail: GQuark = 0 as GQuark;
    let mut itype: GType = 0;
    if g_type_check_instance(instance as *mut GTypeInstance) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_stop_emission_by_name\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_CHECK_INSTANCE (instance)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !detailed_signal.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_stop_emission_by_name\0" as *const u8 as *const ::core::ffi::c_char,
            b"detailed_signal != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut g__g_signal_mutex_lock);
    itype = (*(*(instance as *mut GTypeInstance)).g_class).g_type;
    signal_id = signal_parse_name(
        detailed_signal,
        itype,
        &raw mut detail,
        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
    );
    if signal_id != 0 {
        let mut node: *mut SignalNode = LOOKUP_SIGNAL_NODE(signal_id);
        if detail != 0
            && (*node).flags() as ::core::ffi::c_int & G_SIGNAL_DETAILED as ::core::ffi::c_int == 0
        {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"%s: signal '%s' does not support details\0" as *const u8 as *const gchar,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:1167\0"
                    as *const u8 as *const ::core::ffi::c_char,
                detailed_signal,
            );
        } else if !(itype == (*node).itype || g_type_is_a(itype, (*node).itype) != 0) {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"%s: signal '%s' is invalid for instance '%p' of type '%s'\0" as *const u8
                    as *const gchar,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:1170\0"
                    as *const u8 as *const ::core::ffi::c_char,
                detailed_signal,
                instance,
                g_type_name(itype),
            );
        } else {
            let mut emission: *mut Emission = emission_find(signal_id, detail, instance);
            if !emission.is_null() {
                if (*emission).state as ::core::ffi::c_uint
                    == EMISSION_HOOK as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    g_log(
                        b"GLib-GObject\0" as *const u8 as *const gchar,
                        G_LOG_LEVEL_CRITICAL,
                        b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:1178: emission of signal \"%s\" for instance '%p' cannot be stopped from emission hook\0"
                            as *const u8 as *const gchar,
                        (*node).name,
                        instance,
                    );
                } else if (*emission).state as ::core::ffi::c_uint
                    == EMISSION_RUN as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    (*emission).state = EMISSION_STOP;
                }
            } else {
                g_log(
                    b"GLib-GObject\0" as *const u8 as *const gchar,
                    G_LOG_LEVEL_CRITICAL,
                    b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:1184: no emission of signal \"%s\" to stop for instance '%p'\0"
                        as *const u8 as *const gchar,
                    (*node).name,
                    instance,
                );
            }
        }
    } else {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: signal '%s' is invalid for instance '%p' of type '%s'\0" as *const u8
                as *const gchar,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:1190\0"
                as *const u8 as *const ::core::ffi::c_char,
            detailed_signal,
            instance,
            g_type_name(itype),
        );
    }
    g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_lookup(mut name: *const gchar, mut itype: GType) -> guint {
    let mut signal_id: guint = 0;
    if !name.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_lookup\0" as *const u8 as *const ::core::ffi::c_char,
            b"name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if g_type_test_flags(
        itype,
        G_TYPE_FLAG_INSTANTIATABLE as ::core::ffi::c_int as guint,
    ) != 0
        || g_type_fundamental(itype)
            == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_lookup\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_IS_INSTANTIATABLE (itype) || G_TYPE_IS_INTERFACE (itype)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    g_mutex_lock(&raw mut g__g_signal_mutex_lock);
    signal_id = signal_id_lookup(name, itype);
    g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
    if signal_id == 0 {
        if g_type_name(itype).is_null() {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:1228: unable to look up signal \"%s\" for invalid type id '%lu'\0"
                    as *const u8 as *const gchar,
                name,
                itype,
            );
        } else if g_signal_is_valid_name(name) == 0 {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:1231: unable to look up invalid signal name \"%s\" on type '%s'\0"
                    as *const u8 as *const gchar,
                name,
                g_type_name(itype),
            );
        }
    }
    return signal_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_list_ids(mut itype: GType, mut n_ids: *mut guint) -> *mut guint {
    let mut keys: *mut SignalKey = ::core::ptr::null_mut::<SignalKey>();
    let mut result: *mut GArray = ::core::ptr::null_mut::<GArray>();
    let mut n_nodes: guint = 0;
    let mut i: guint = 0;
    if g_type_test_flags(
        itype,
        G_TYPE_FLAG_INSTANTIATABLE as ::core::ffi::c_int as guint,
    ) != 0
        || g_type_fundamental(itype)
            == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_list_ids\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_IS_INSTANTIATABLE (itype) || G_TYPE_IS_INTERFACE (itype)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<guint>();
    }
    if !n_ids.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_list_ids\0" as *const u8 as *const ::core::ffi::c_char,
            b"n_ids != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<guint>();
    }
    g_mutex_lock(&raw mut g__g_signal_mutex_lock);
    keys = g_bsearch_array_get_nth(
        g_signal_key_bsa,
        &raw const g_signal_key_bconfig,
        0 as guint,
    ) as *mut SignalKey;
    n_nodes = (*g_signal_key_bsa).n_nodes;
    result = g_array_new(
        0 as gboolean,
        0 as gboolean,
        ::core::mem::size_of::<guint>() as guint,
    );
    i = 0 as guint;
    while i < n_nodes {
        if (*keys.offset(i as isize)).itype == itype {
            g_array_append_vals(
                result,
                &raw mut (*keys.offset(i as isize)).signal_id as gconstpointer,
                1 as guint,
            );
        }
        i = i.wrapping_add(1);
    }
    *n_ids = (*result).len;
    g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
    if n_nodes == 0 {
        if g_type_name(itype).is_null() {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:1277: unable to list signals for invalid type id '%lu'\0"
                    as *const u8 as *const gchar,
                itype,
            );
        } else if g_type_test_flags(
            itype,
            G_TYPE_FLAG_INSTANTIATABLE as ::core::ffi::c_int as guint,
        ) == 0
            && !(g_type_fundamental(itype)
                == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType)
        {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:1280: unable to list signals of non instantiatable type '%s'\0"
                    as *const u8 as *const gchar,
                g_type_name(itype),
            );
        } else if g_type_class_peek(itype).is_null()
            && !(g_type_fundamental(itype)
                == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType)
        {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:1283: unable to list signals of unloaded type '%s'\0"
                    as *const u8 as *const gchar,
                g_type_name(itype),
            );
        }
    }
    return g_array_free(result, 0 as gboolean) as *mut guint;
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_name(mut signal_id: guint) -> *const gchar {
    let mut node: *mut SignalNode = ::core::ptr::null_mut::<SignalNode>();
    let mut name: *const gchar = ::core::ptr::null::<gchar>();
    g_mutex_lock(&raw mut g__g_signal_mutex_lock);
    node = LOOKUP_SIGNAL_NODE(signal_id);
    name = if !node.is_null() {
        (*node).name
    } else {
        ::core::ptr::null::<gchar>()
    };
    g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
    return name as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_query(mut signal_id: guint, mut query: *mut GSignalQuery) {
    let mut node: *mut SignalNode = ::core::ptr::null_mut::<SignalNode>();
    if !query.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_query\0" as *const u8 as *const ::core::ffi::c_char,
            b"query != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut g__g_signal_mutex_lock);
    node = LOOKUP_SIGNAL_NODE(signal_id);
    if node.is_null() || (*node).destroyed() as ::core::ffi::c_int != 0 {
        (*query).signal_id = 0 as guint;
    } else {
        (*query).signal_id = (*node).signal_id;
        (*query).signal_name = (*node).name;
        (*query).itype = (*node).itype;
        (*query).signal_flags = (*node).flags() as GSignalFlags;
        (*query).return_type = (*node).return_type;
        (*query).n_params = (*node).n_params();
        (*query).param_types = (*node).param_types;
    }
    g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_new(
    mut signal_name: *const gchar,
    mut itype: GType,
    mut signal_flags: GSignalFlags,
    mut class_offset: guint,
    mut accumulator: GSignalAccumulator,
    mut accu_data: gpointer,
    mut c_marshaller: GSignalCMarshaller,
    mut return_type: GType,
    mut n_params: guint,
    mut args: ...
) -> guint {
    let mut args_0: ::core::ffi::VaList<'_>;
    let mut signal_id: guint = 0;
    if !signal_name.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_new\0" as *const u8 as *const ::core::ffi::c_char,
            b"signal_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    args_0 = args.clone();
    signal_id = g_signal_new_valist(
        signal_name,
        itype,
        signal_flags,
        if class_offset != 0 {
            g_signal_type_cclosure_new(itype, class_offset)
        } else {
            ::core::ptr::null_mut::<GClosure>()
        },
        accumulator,
        accu_data,
        c_marshaller,
        return_type,
        n_params,
        args_0,
    );
    return signal_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_new_class_handler(
    mut signal_name: *const gchar,
    mut itype: GType,
    mut signal_flags: GSignalFlags,
    mut class_handler: GCallback,
    mut accumulator: GSignalAccumulator,
    mut accu_data: gpointer,
    mut c_marshaller: GSignalCMarshaller,
    mut return_type: GType,
    mut n_params: guint,
    mut args: ...
) -> guint {
    let mut args_0: ::core::ffi::VaList<'_>;
    let mut signal_id: guint = 0;
    if !signal_name.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_new_class_handler\0" as *const u8 as *const ::core::ffi::c_char,
            b"signal_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    args_0 = args.clone();
    signal_id = g_signal_new_valist(
        signal_name,
        itype,
        signal_flags,
        if class_handler.is_some() {
            g_cclosure_new(
                class_handler,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                None,
            )
        } else {
            ::core::ptr::null_mut::<GClosure>()
        },
        accumulator,
        accu_data,
        c_marshaller,
        return_type,
        n_params,
        args_0,
    );
    return signal_id;
}
#[inline]
unsafe extern "C" fn signal_find_class_closure(
    mut node: *mut SignalNode,
    mut itype: GType,
) -> *mut ClassClosure {
    let mut bsa: *mut GBSearchArray = (*node).class_closure_bsa;
    let mut cc: *mut ClassClosure = ::core::ptr::null_mut::<ClassClosure>();
    if !bsa.is_null() {
        let mut key: ClassClosure = ClassClosure {
            instance_type: 0,
            closure: ::core::ptr::null_mut::<GClosure>(),
        };
        if (*bsa).n_nodes == 1 as guint {
            cc = g_bsearch_array_get_nth(bsa, &raw mut g_class_closure_bconfig, 0 as guint)
                as *mut ClassClosure;
            if !cc.is_null() && (*cc).instance_type == 0 as GType {
                return cc;
            }
        }
        key.instance_type = itype;
        cc = g_bsearch_array_lookup_fuzzy(
            bsa,
            &raw mut g_class_closure_bconfig,
            &raw mut key as gconstpointer,
            0 as guint,
        ) as *mut ClassClosure;
        while cc.is_null() && key.instance_type != 0 {
            key.instance_type = g_type_parent(key.instance_type);
            cc = g_bsearch_array_lookup_fuzzy(
                bsa,
                &raw mut g_class_closure_bconfig,
                &raw mut key as gconstpointer,
                0 as guint,
            ) as *mut ClassClosure;
        }
    } else {
        cc = ::core::ptr::null_mut::<ClassClosure>();
    }
    return cc;
}
#[inline]
unsafe extern "C" fn signal_lookup_closure(
    mut node: *mut SignalNode,
    mut instance: *mut GTypeInstance,
) -> *mut GClosure {
    let mut cc: *mut ClassClosure = ::core::ptr::null_mut::<ClassClosure>();
    cc = signal_find_class_closure(node, (*(*instance).g_class).g_type);
    return if !cc.is_null() {
        (*cc).closure
    } else {
        ::core::ptr::null_mut::<GClosure>()
    };
}
unsafe extern "C" fn signal_add_class_closure(
    mut node: *mut SignalNode,
    mut itype: GType,
    mut closure: *mut GClosure,
) {
    let mut key: ClassClosure = ClassClosure {
        instance_type: 0,
        closure: ::core::ptr::null_mut::<GClosure>(),
    };
    (*node).set_single_va_closure_is_valid(0 as guint as guint);
    if (*node).class_closure_bsa.is_null() {
        (*node).class_closure_bsa = g_bsearch_array_create(&raw mut g_class_closure_bconfig);
    }
    key.instance_type = itype;
    key.closure = g_closure_ref(closure);
    (*node).class_closure_bsa = g_bsearch_array_insert(
        (*node).class_closure_bsa,
        &raw mut g_class_closure_bconfig,
        &raw mut key as gconstpointer,
    );
    g_closure_sink(closure);
    if (*node).c_marshaller.is_some() && !closure.is_null() && (*closure).marshal.is_none() {
        g_closure_set_marshal(closure, (*node).c_marshaller as GClosureMarshal);
        if (*node).va_marshaller.is_some() {
            _g_closure_set_va_marshal(closure, (*node).va_marshaller as GVaClosureMarshal);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_newv(
    mut signal_name: *const gchar,
    mut itype: GType,
    mut signal_flags: GSignalFlags,
    mut class_closure: *mut GClosure,
    mut accumulator: GSignalAccumulator,
    mut accu_data: gpointer,
    mut c_marshaller: GSignalCMarshaller,
    mut return_type: GType,
    mut n_params: guint,
    mut param_types: *mut GType,
) -> guint {
    let mut name: *const gchar = ::core::ptr::null::<gchar>();
    let mut signal_name_copy: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut signal_id: guint = 0;
    let mut i: guint = 0;
    let mut node: *mut SignalNode = ::core::ptr::null_mut::<SignalNode>();
    let mut builtin_c_marshaller: GSignalCMarshaller = None;
    let mut builtin_va_marshaller: GSignalCVaMarshaller = None;
    let mut va_marshaller: GSignalCVaMarshaller = None;
    if !signal_name.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_newv\0" as *const u8 as *const ::core::ffi::c_char,
            b"signal_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if g_signal_is_valid_name(signal_name) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_newv\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_is_valid_name (signal_name)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if g_type_test_flags(
        itype,
        G_TYPE_FLAG_INSTANTIATABLE as ::core::ffi::c_int as guint,
    ) != 0
        || g_type_fundamental(itype)
            == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_newv\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_IS_INSTANTIATABLE (itype) || G_TYPE_IS_INTERFACE (itype)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if n_params != 0 {
        if !param_types.is_null() {
        } else {
            g_return_if_fail_warning(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"g_signal_newv\0" as *const u8 as *const ::core::ffi::c_char,
                b"param_types != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return 0 as guint;
        }
    }
    if return_type & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType == 0 as GType {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_newv\0" as *const u8 as *const ::core::ffi::c_char,
            b"(return_type & G_SIGNAL_TYPE_STATIC_SCOPE) == 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if return_type
        == ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
            & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType)
    {
        if accumulator.is_none() {
        } else {
            g_return_if_fail_warning(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"g_signal_newv\0" as *const u8 as *const ::core::ffi::c_char,
                b"accumulator == NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return 0 as guint;
        }
    }
    if accumulator.is_none() {
        if accu_data.is_null() {
        } else {
            g_return_if_fail_warning(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"g_signal_newv\0" as *const u8 as *const ::core::ffi::c_char,
                b"accu_data == NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return 0 as guint;
        }
    }
    if signal_flags as ::core::ffi::c_uint
        & G_SIGNAL_ACCUMULATOR_FIRST_RUN as ::core::ffi::c_int as ::core::ffi::c_uint
        == 0 as ::core::ffi::c_uint
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_newv\0" as *const u8 as *const ::core::ffi::c_char,
            b"(signal_flags & G_SIGNAL_ACCUMULATOR_FIRST_RUN) == 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if is_canonical(signal_name) == 0 {
        signal_name_copy = g_strdup_inline(signal_name as *const ::core::ffi::c_char) as *mut gchar;
        canonicalize_key(signal_name_copy);
        name = signal_name_copy;
    } else {
        name = signal_name;
    }
    g_mutex_lock(&raw mut g__g_signal_mutex_lock);
    signal_id = signal_id_lookup(name, itype);
    node = LOOKUP_SIGNAL_NODE(signal_id);
    if !node.is_null() && (*node).destroyed() == 0 {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:1645: signal \"%s\" already exists in the '%s' %s\0"
                as *const u8 as *const gchar,
            name,
            type_debug_name((*node).itype),
            if g_type_fundamental((*node).itype)
                == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
            {
                b"interface\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"class ancestry\0" as *const u8 as *const ::core::ffi::c_char
            },
        );
        g_free(signal_name_copy as gpointer);
        g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
        return 0 as guint;
    }
    if !node.is_null() && (*node).itype != itype {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:1655: signal \"%s\" for type '%s' was previously created for type '%s'\0"
                as *const u8 as *const gchar,
            name,
            type_debug_name(itype),
            type_debug_name((*node).itype),
        );
        g_free(signal_name_copy as gpointer);
        g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
        return 0 as guint;
    }
    i = 0 as guint;
    while i < n_params {
        if g_type_check_is_value_type(
            *param_types.offset(i as isize)
                & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType),
        ) == 0
        {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:1666: parameter %d of type '%s' for signal \"%s::%s\" is not a value type\0"
                    as *const u8 as *const gchar,
                i.wrapping_add(1 as guint),
                type_debug_name(*param_types.offset(i as isize)),
                type_debug_name(itype),
                name,
            );
            g_free(signal_name_copy as gpointer);
            g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
            return 0 as guint;
        }
        i = i.wrapping_add(1);
    }
    if return_type != ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
        && g_type_check_is_value_type(
            return_type & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType),
        ) == 0
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:1674: return value of type '%s' for signal \"%s::%s\" is not a value type\0"
                as *const u8 as *const gchar,
            type_debug_name(return_type),
            type_debug_name(itype),
            name,
        );
        g_free(signal_name_copy as gpointer);
        g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
        return 0 as guint;
    }
    if node.is_null() {
        let mut key: SignalKey = _SignalKey {
            itype: 0,
            quark: 0,
            signal_id: 0,
        };
        let fresh0 = g_n_signal_nodes;
        g_n_signal_nodes = g_n_signal_nodes.wrapping_add(1);
        signal_id = fresh0;
        node = g_malloc_n(1 as gsize, ::core::mem::size_of::<SignalNode>() as gsize)
            as *mut SignalNode;
        (*node).signal_id = signal_id;
        g_signal_nodes = g_realloc_n(
            g_signal_nodes as gpointer,
            g_n_signal_nodes as gsize,
            ::core::mem::size_of::<*mut SignalNode>() as gsize,
        ) as *mut *mut SignalNode;
        let ref mut fresh1 = *g_signal_nodes.offset(signal_id as isize);
        *fresh1 = node;
        (*node).itype = itype;
        key.itype = itype;
        key.signal_id = signal_id;
        (*node).name = g_intern_string(name);
        key.quark = g_quark_from_string(name);
        g_signal_key_bsa = g_bsearch_array_insert(
            g_signal_key_bsa,
            &raw const g_signal_key_bconfig,
            &raw mut key as gconstpointer,
        );
    }
    (*node).set_destroyed(0 as guint as guint);
    (*node).set_single_va_closure_is_valid(0 as guint as guint);
    (*node).set_flags(
        (signal_flags as ::core::ffi::c_uint & 0x1ff as ::core::ffi::c_uint) as guint as guint,
    );
    (*node).set_n_params(n_params as guint);
    (*node).param_types = g_memdup2(
        param_types as gconstpointer,
        (::core::mem::size_of::<GType>() as gsize).wrapping_mul(n_params as gsize),
    ) as *mut GType;
    (*node).return_type = return_type;
    (*node).class_closure_bsa = ::core::ptr::null_mut::<GBSearchArray>();
    if accumulator.is_some() {
        (*node).accumulator = g_malloc_n(
            1 as gsize,
            ::core::mem::size_of::<SignalAccumulator>() as gsize,
        ) as *mut SignalAccumulator;
        (*(*node).accumulator).func = accumulator;
        (*(*node).accumulator).data = accu_data;
    } else {
        (*node).accumulator = ::core::ptr::null_mut::<SignalAccumulator>();
    }
    builtin_c_marshaller = None;
    builtin_va_marshaller = None;
    if n_params == 0 as guint
        && return_type == ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
        builtin_c_marshaller = Some(
            g_cclosure_marshal_VOID__VOID
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    guint,
                    *const GValue,
                    gpointer,
                    gpointer,
                ) -> (),
        ) as GSignalCMarshaller;
        builtin_va_marshaller = Some(
            g_cclosure_marshal_VOID__VOIDv
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    gpointer,
                    ::core::ffi::VaList,
                    gpointer,
                    ::core::ffi::c_int,
                    *mut GType,
                ) -> (),
        ) as GSignalCVaMarshaller;
    } else if n_params == 1 as guint
        && return_type == ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
        if *param_types.offset(0 as ::core::ffi::c_int as isize)
            & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType)
            == ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
            || g_type_is_a(
                *param_types.offset(0 as ::core::ffi::c_int as isize)
                    & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType),
                ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            ) != 0
        {
            builtin_c_marshaller = Some(
                g_cclosure_marshal_VOID__BOOLEAN
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        guint,
                        *const GValue,
                        gpointer,
                        gpointer,
                    ) -> (),
            ) as GSignalCMarshaller;
            builtin_va_marshaller = Some(
                g_cclosure_marshal_VOID__BOOLEANv
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        gpointer,
                        ::core::ffi::VaList,
                        gpointer,
                        ::core::ffi::c_int,
                        *mut GType,
                    ) -> (),
            ) as GSignalCVaMarshaller;
        } else if *param_types.offset(0 as ::core::ffi::c_int as isize)
            & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType)
            == ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
            || g_type_is_a(
                *param_types.offset(0 as ::core::ffi::c_int as isize)
                    & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType),
                ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            ) != 0
        {
            builtin_c_marshaller = Some(
                g_cclosure_marshal_VOID__CHAR
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        guint,
                        *const GValue,
                        gpointer,
                        gpointer,
                    ) -> (),
            ) as GSignalCMarshaller;
            builtin_va_marshaller = Some(
                g_cclosure_marshal_VOID__CHARv
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        gpointer,
                        ::core::ffi::VaList,
                        gpointer,
                        ::core::ffi::c_int,
                        *mut GType,
                    ) -> (),
            ) as GSignalCVaMarshaller;
        } else if *param_types.offset(0 as ::core::ffi::c_int as isize)
            & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType)
            == ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
            || g_type_is_a(
                *param_types.offset(0 as ::core::ffi::c_int as isize)
                    & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType),
                ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            ) != 0
        {
            builtin_c_marshaller = Some(
                g_cclosure_marshal_VOID__UCHAR
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        guint,
                        *const GValue,
                        gpointer,
                        gpointer,
                    ) -> (),
            ) as GSignalCMarshaller;
            builtin_va_marshaller = Some(
                g_cclosure_marshal_VOID__UCHARv
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        gpointer,
                        ::core::ffi::VaList,
                        gpointer,
                        ::core::ffi::c_int,
                        *mut GType,
                    ) -> (),
            ) as GSignalCVaMarshaller;
        } else if *param_types.offset(0 as ::core::ffi::c_int as isize)
            & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType)
            == ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
            || g_type_is_a(
                *param_types.offset(0 as ::core::ffi::c_int as isize)
                    & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType),
                ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            ) != 0
        {
            builtin_c_marshaller = Some(
                g_cclosure_marshal_VOID__INT
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        guint,
                        *const GValue,
                        gpointer,
                        gpointer,
                    ) -> (),
            ) as GSignalCMarshaller;
            builtin_va_marshaller = Some(
                g_cclosure_marshal_VOID__INTv
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        gpointer,
                        ::core::ffi::VaList,
                        gpointer,
                        ::core::ffi::c_int,
                        *mut GType,
                    ) -> (),
            ) as GSignalCVaMarshaller;
        } else if *param_types.offset(0 as ::core::ffi::c_int as isize)
            & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType)
            == ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
            || g_type_is_a(
                *param_types.offset(0 as ::core::ffi::c_int as isize)
                    & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType),
                ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            ) != 0
        {
            builtin_c_marshaller = Some(
                g_cclosure_marshal_VOID__UINT
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        guint,
                        *const GValue,
                        gpointer,
                        gpointer,
                    ) -> (),
            ) as GSignalCMarshaller;
            builtin_va_marshaller = Some(
                g_cclosure_marshal_VOID__UINTv
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        gpointer,
                        ::core::ffi::VaList,
                        gpointer,
                        ::core::ffi::c_int,
                        *mut GType,
                    ) -> (),
            ) as GSignalCVaMarshaller;
        } else if *param_types.offset(0 as ::core::ffi::c_int as isize)
            & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType)
            == ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
            || g_type_is_a(
                *param_types.offset(0 as ::core::ffi::c_int as isize)
                    & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType),
                ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            ) != 0
        {
            builtin_c_marshaller = Some(
                g_cclosure_marshal_VOID__LONG
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        guint,
                        *const GValue,
                        gpointer,
                        gpointer,
                    ) -> (),
            ) as GSignalCMarshaller;
            builtin_va_marshaller = Some(
                g_cclosure_marshal_VOID__LONGv
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        gpointer,
                        ::core::ffi::VaList,
                        gpointer,
                        ::core::ffi::c_int,
                        *mut GType,
                    ) -> (),
            ) as GSignalCVaMarshaller;
        } else if *param_types.offset(0 as ::core::ffi::c_int as isize)
            & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType)
            == ((9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
            || g_type_is_a(
                *param_types.offset(0 as ::core::ffi::c_int as isize)
                    & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType),
                ((9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            ) != 0
        {
            builtin_c_marshaller = Some(
                g_cclosure_marshal_VOID__ULONG
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        guint,
                        *const GValue,
                        gpointer,
                        gpointer,
                    ) -> (),
            ) as GSignalCMarshaller;
            builtin_va_marshaller = Some(
                g_cclosure_marshal_VOID__ULONGv
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        gpointer,
                        ::core::ffi::VaList,
                        gpointer,
                        ::core::ffi::c_int,
                        *mut GType,
                    ) -> (),
            ) as GSignalCVaMarshaller;
        } else if *param_types.offset(0 as ::core::ffi::c_int as isize)
            & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType)
            == ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
            || g_type_is_a(
                *param_types.offset(0 as ::core::ffi::c_int as isize)
                    & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType),
                ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            ) != 0
        {
            builtin_c_marshaller = Some(
                g_cclosure_marshal_VOID__ENUM
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        guint,
                        *const GValue,
                        gpointer,
                        gpointer,
                    ) -> (),
            ) as GSignalCMarshaller;
            builtin_va_marshaller = Some(
                g_cclosure_marshal_VOID__ENUMv
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        gpointer,
                        ::core::ffi::VaList,
                        gpointer,
                        ::core::ffi::c_int,
                        *mut GType,
                    ) -> (),
            ) as GSignalCVaMarshaller;
        } else if *param_types.offset(0 as ::core::ffi::c_int as isize)
            & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType)
            == ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
            || g_type_is_a(
                *param_types.offset(0 as ::core::ffi::c_int as isize)
                    & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType),
                ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            ) != 0
        {
            builtin_c_marshaller = Some(
                g_cclosure_marshal_VOID__FLAGS
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        guint,
                        *const GValue,
                        gpointer,
                        gpointer,
                    ) -> (),
            ) as GSignalCMarshaller;
            builtin_va_marshaller = Some(
                g_cclosure_marshal_VOID__FLAGSv
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        gpointer,
                        ::core::ffi::VaList,
                        gpointer,
                        ::core::ffi::c_int,
                        *mut GType,
                    ) -> (),
            ) as GSignalCVaMarshaller;
        } else if *param_types.offset(0 as ::core::ffi::c_int as isize)
            & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType)
            == ((14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
            || g_type_is_a(
                *param_types.offset(0 as ::core::ffi::c_int as isize)
                    & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType),
                ((14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            ) != 0
        {
            builtin_c_marshaller = Some(
                g_cclosure_marshal_VOID__FLOAT
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        guint,
                        *const GValue,
                        gpointer,
                        gpointer,
                    ) -> (),
            ) as GSignalCMarshaller;
            builtin_va_marshaller = Some(
                g_cclosure_marshal_VOID__FLOATv
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        gpointer,
                        ::core::ffi::VaList,
                        gpointer,
                        ::core::ffi::c_int,
                        *mut GType,
                    ) -> (),
            ) as GSignalCVaMarshaller;
        } else if *param_types.offset(0 as ::core::ffi::c_int as isize)
            & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType)
            == ((15 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
            || g_type_is_a(
                *param_types.offset(0 as ::core::ffi::c_int as isize)
                    & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType),
                ((15 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            ) != 0
        {
            builtin_c_marshaller = Some(
                g_cclosure_marshal_VOID__DOUBLE
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        guint,
                        *const GValue,
                        gpointer,
                        gpointer,
                    ) -> (),
            ) as GSignalCMarshaller;
            builtin_va_marshaller = Some(
                g_cclosure_marshal_VOID__DOUBLEv
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        gpointer,
                        ::core::ffi::VaList,
                        gpointer,
                        ::core::ffi::c_int,
                        *mut GType,
                    ) -> (),
            ) as GSignalCVaMarshaller;
        } else if *param_types.offset(0 as ::core::ffi::c_int as isize)
            & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType)
            == ((16 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
            || g_type_is_a(
                *param_types.offset(0 as ::core::ffi::c_int as isize)
                    & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType),
                ((16 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            ) != 0
        {
            builtin_c_marshaller = Some(
                g_cclosure_marshal_VOID__STRING
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        guint,
                        *const GValue,
                        gpointer,
                        gpointer,
                    ) -> (),
            ) as GSignalCMarshaller;
            builtin_va_marshaller = Some(
                g_cclosure_marshal_VOID__STRINGv
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        gpointer,
                        ::core::ffi::VaList,
                        gpointer,
                        ::core::ffi::c_int,
                        *mut GType,
                    ) -> (),
            ) as GSignalCVaMarshaller;
        } else if *param_types.offset(0 as ::core::ffi::c_int as isize)
            & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType)
            == ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
            || g_type_is_a(
                *param_types.offset(0 as ::core::ffi::c_int as isize)
                    & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType),
                ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            ) != 0
        {
            builtin_c_marshaller = Some(
                g_cclosure_marshal_VOID__PARAM
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        guint,
                        *const GValue,
                        gpointer,
                        gpointer,
                    ) -> (),
            ) as GSignalCMarshaller;
            builtin_va_marshaller = Some(
                g_cclosure_marshal_VOID__PARAMv
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        gpointer,
                        ::core::ffi::VaList,
                        gpointer,
                        ::core::ffi::c_int,
                        *mut GType,
                    ) -> (),
            ) as GSignalCVaMarshaller;
        } else if *param_types.offset(0 as ::core::ffi::c_int as isize)
            & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType)
            == ((18 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
            || g_type_is_a(
                *param_types.offset(0 as ::core::ffi::c_int as isize)
                    & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType),
                ((18 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            ) != 0
        {
            builtin_c_marshaller = Some(
                g_cclosure_marshal_VOID__BOXED
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        guint,
                        *const GValue,
                        gpointer,
                        gpointer,
                    ) -> (),
            ) as GSignalCMarshaller;
            builtin_va_marshaller = Some(
                g_cclosure_marshal_VOID__BOXEDv
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        gpointer,
                        ::core::ffi::VaList,
                        gpointer,
                        ::core::ffi::c_int,
                        *mut GType,
                    ) -> (),
            ) as GSignalCVaMarshaller;
        } else if *param_types.offset(0 as ::core::ffi::c_int as isize)
            & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType)
            == ((17 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
            || g_type_is_a(
                *param_types.offset(0 as ::core::ffi::c_int as isize)
                    & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType),
                ((17 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            ) != 0
        {
            builtin_c_marshaller = Some(
                g_cclosure_marshal_VOID__POINTER
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        guint,
                        *const GValue,
                        gpointer,
                        gpointer,
                    ) -> (),
            ) as GSignalCMarshaller;
            builtin_va_marshaller = Some(
                g_cclosure_marshal_VOID__POINTERv
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        gpointer,
                        ::core::ffi::VaList,
                        gpointer,
                        ::core::ffi::c_int,
                        *mut GType,
                    ) -> (),
            ) as GSignalCVaMarshaller;
        } else if *param_types.offset(0 as ::core::ffi::c_int as isize)
            & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType)
            == ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
            || g_type_is_a(
                *param_types.offset(0 as ::core::ffi::c_int as isize)
                    & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType),
                ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            ) != 0
        {
            builtin_c_marshaller = Some(
                g_cclosure_marshal_VOID__OBJECT
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        guint,
                        *const GValue,
                        gpointer,
                        gpointer,
                    ) -> (),
            ) as GSignalCMarshaller;
            builtin_va_marshaller = Some(
                g_cclosure_marshal_VOID__OBJECTv
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        gpointer,
                        ::core::ffi::VaList,
                        gpointer,
                        ::core::ffi::c_int,
                        *mut GType,
                    ) -> (),
            ) as GSignalCVaMarshaller;
        } else if *param_types.offset(0 as ::core::ffi::c_int as isize)
            & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType)
            == ((21 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
            || g_type_is_a(
                *param_types.offset(0 as ::core::ffi::c_int as isize)
                    & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType),
                ((21 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            ) != 0
        {
            builtin_c_marshaller = Some(
                g_cclosure_marshal_VOID__VARIANT
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        guint,
                        *const GValue,
                        gpointer,
                        gpointer,
                    ) -> (),
            ) as GSignalCMarshaller;
            builtin_va_marshaller = Some(
                g_cclosure_marshal_VOID__VARIANTv
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        gpointer,
                        ::core::ffi::VaList,
                        gpointer,
                        ::core::ffi::c_int,
                        *mut GType,
                    ) -> (),
            ) as GSignalCVaMarshaller;
        }
    }
    if c_marshaller.is_none() {
        if builtin_c_marshaller.is_some() {
            c_marshaller = builtin_c_marshaller;
            va_marshaller = builtin_va_marshaller;
        } else {
            c_marshaller = Some(
                g_cclosure_marshal_generic
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        guint,
                        *const GValue,
                        gpointer,
                        gpointer,
                    ) -> (),
            ) as GSignalCMarshaller;
            va_marshaller = Some(
                g_cclosure_marshal_generic_va
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        gpointer,
                        ::core::ffi::VaList,
                        gpointer,
                        ::core::ffi::c_int,
                        *mut GType,
                    ) -> (),
            ) as GSignalCVaMarshaller;
        }
    } else {
        va_marshaller = None;
    }
    (*node).c_marshaller = c_marshaller;
    (*node).va_marshaller = va_marshaller;
    (*node).emission_hooks = ::core::ptr::null_mut::<GHookList>();
    if !class_closure.is_null() {
        signal_add_class_closure(node, 0 as GType, class_closure);
    }
    g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
    g_free(signal_name_copy as gpointer);
    return signal_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_set_va_marshaller(
    mut signal_id: guint,
    mut instance_type: GType,
    mut va_marshaller: GSignalCVaMarshaller,
) {
    let mut node: *mut SignalNode = ::core::ptr::null_mut::<SignalNode>();
    if signal_id > 0 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_set_va_marshaller\0" as *const u8 as *const ::core::ffi::c_char,
            b"signal_id > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if va_marshaller.is_some() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_set_va_marshaller\0" as *const u8 as *const ::core::ffi::c_char,
            b"va_marshaller != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut g__g_signal_mutex_lock);
    node = LOOKUP_SIGNAL_NODE(signal_id);
    if !node.is_null() {
        (*node).va_marshaller = va_marshaller;
        if !(*node).class_closure_bsa.is_null() {
            let mut cc: *mut ClassClosure = g_bsearch_array_get_nth(
                (*node).class_closure_bsa,
                &raw mut g_class_closure_bconfig,
                0 as guint,
            ) as *mut ClassClosure;
            if (*(*cc).closure).marshal == (*node).c_marshaller {
                _g_closure_set_va_marshal((*cc).closure, va_marshaller as GVaClosureMarshal);
            }
        }
        (*node).set_single_va_closure_is_valid(0 as guint as guint);
    }
    g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_new_valist(
    mut signal_name: *const gchar,
    mut itype: GType,
    mut signal_flags: GSignalFlags,
    mut class_closure: *mut GClosure,
    mut accumulator: GSignalAccumulator,
    mut accu_data: gpointer,
    mut c_marshaller: GSignalCMarshaller,
    mut return_type: GType,
    mut n_params: guint,
    mut args: ::core::ffi::VaList,
) -> guint {
    let mut param_types_stack: [GType; 25] = [0; 25];
    let mut param_types_heap: *mut GType = ::core::ptr::null_mut::<GType>();
    let mut param_types: *mut GType = ::core::ptr::null_mut::<GType>();
    let mut i: guint = 0;
    let mut signal_id: guint = 0;
    param_types = &raw mut param_types_stack as *mut GType;
    if n_params > 0 as guint {
        if n_params as usize
            > (::core::mem::size_of::<[GType; 25]>() as usize)
                .wrapping_div(::core::mem::size_of::<GType>() as usize)
        {
            param_types_heap =
                g_malloc_n(n_params as gsize, ::core::mem::size_of::<GType>() as gsize)
                    as *mut GType;
            param_types = param_types_heap;
        }
        i = 0 as guint;
        while i < n_params {
            *param_types.offset(i as isize) = args.arg::<GType>();
            i = i.wrapping_add(1);
        }
    }
    signal_id = g_signal_newv(
        signal_name,
        itype,
        signal_flags,
        class_closure,
        accumulator,
        accu_data,
        c_marshaller,
        return_type,
        n_params,
        param_types,
    );
    g_free(param_types_heap as gpointer);
    return signal_id;
}
unsafe extern "C" fn signal_destroy_R(mut signal_node: *mut SignalNode) {
    let mut node: SignalNode = *signal_node;
    (*signal_node)
        .set_destroyed((0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as guint as guint);
    (*signal_node).set_single_va_closure_is_valid(0 as guint as guint);
    (*signal_node).set_n_params(0 as guint as guint);
    (*signal_node).param_types = ::core::ptr::null_mut::<GType>();
    (*signal_node).return_type = 0 as GType;
    (*signal_node).class_closure_bsa = ::core::ptr::null_mut::<GBSearchArray>();
    (*signal_node).accumulator = ::core::ptr::null_mut::<SignalAccumulator>();
    (*signal_node).c_marshaller = None;
    (*signal_node).va_marshaller = None;
    (*signal_node).emission_hooks = ::core::ptr::null_mut::<GHookList>();
    g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
    g_free(node.param_types as gpointer);
    if !node.class_closure_bsa.is_null() {
        let mut i: guint = 0;
        i = 0 as guint;
        while i < (*node.class_closure_bsa).n_nodes {
            let mut cc: *mut ClassClosure = g_bsearch_array_get_nth(
                node.class_closure_bsa,
                &raw mut g_class_closure_bconfig,
                i,
            ) as *mut ClassClosure;
            g_closure_unref((*cc).closure);
            i = i.wrapping_add(1);
        }
        g_bsearch_array_free(node.class_closure_bsa, &raw mut g_class_closure_bconfig);
    }
    g_free(node.accumulator as gpointer);
    if !node.emission_hooks.is_null() {
        g_hook_list_clear(node.emission_hooks);
        g_free(node.emission_hooks as gpointer);
    }
    g_mutex_lock(&raw mut g__g_signal_mutex_lock);
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_override_class_closure(
    mut signal_id: guint,
    mut instance_type: GType,
    mut class_closure: *mut GClosure,
) {
    let mut node: *mut SignalNode = ::core::ptr::null_mut::<SignalNode>();
    if signal_id > 0 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_override_class_closure\0" as *const u8 as *const ::core::ffi::c_char,
            b"signal_id > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !class_closure.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_override_class_closure\0" as *const u8 as *const ::core::ffi::c_char,
            b"class_closure != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut g__g_signal_mutex_lock);
    node = LOOKUP_SIGNAL_NODE(signal_id);
    node_check_deprecated(node);
    if !(instance_type == (*node).itype || g_type_is_a(instance_type, (*node).itype) != 0) {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: type '%s' cannot be overridden for signal id '%u'\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:1981\0"
                as *const u8 as *const ::core::ffi::c_char,
            type_debug_name(instance_type),
            signal_id,
        );
    } else {
        let mut cc: *mut ClassClosure = signal_find_class_closure(node, instance_type);
        if !cc.is_null() && (*cc).instance_type == instance_type {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"%s: type '%s' is already overridden for signal id '%u'\0" as *const u8
                    as *const gchar,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:1987\0"
                    as *const u8 as *const ::core::ffi::c_char,
                type_debug_name(instance_type),
                signal_id,
            );
        } else {
            signal_add_class_closure(node, instance_type, class_closure);
        }
    }
    g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_override_class_handler(
    mut signal_name: *const gchar,
    mut instance_type: GType,
    mut class_handler: GCallback,
) {
    let mut signal_id: guint = 0;
    if !signal_name.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_override_class_handler\0" as *const u8 as *const ::core::ffi::c_char,
            b"signal_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if instance_type != ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_override_class_handler\0" as *const u8 as *const ::core::ffi::c_char,
            b"instance_type != G_TYPE_NONE\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if class_handler.is_some() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_override_class_handler\0" as *const u8 as *const ::core::ffi::c_char,
            b"class_handler != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    signal_id = g_signal_lookup(signal_name, instance_type);
    if signal_id != 0 {
        g_signal_override_class_closure(
            signal_id,
            instance_type,
            g_cclosure_new(
                class_handler,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                None,
            ),
        );
    } else {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: signal name '%s' is invalid for type id '%lu'\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:2030\0"
                as *const u8 as *const ::core::ffi::c_char,
            signal_name,
            instance_type,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_chain_from_overridden(
    mut instance_and_params: *const GValue,
    mut return_value: *mut GValue,
) {
    let mut chain_type: GType = 0 as GType;
    let mut restore_type: GType = 0 as GType;
    let mut emission: *mut Emission = ::core::ptr::null_mut::<Emission>();
    let mut closure: *mut GClosure = ::core::ptr::null_mut::<GClosure>();
    let mut n_params: guint = 0 as guint;
    let mut instance: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if !instance_and_params.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_chain_from_overridden\0" as *const u8 as *const ::core::ffi::c_char,
            b"instance_and_params != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    instance = g_value_peek_pointer(instance_and_params);
    if g_type_check_instance(instance as *mut GTypeInstance) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_chain_from_overridden\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_CHECK_INSTANCE (instance)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut g__g_signal_mutex_lock);
    emission = emission_find_innermost(instance);
    if !emission.is_null() {
        let mut node: *mut SignalNode = LOOKUP_SIGNAL_NODE((*emission).ihint.signal_id);
        if !node.is_null() {
        } else {
            g_assertion_message_expr(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                2066 as ::core::ffi::c_int,
                b"g_signal_chain_from_overridden\0" as *const u8 as *const ::core::ffi::c_char,
                b"node != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if (*emission).chain_type != ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
        {
            let mut cc: *mut ClassClosure = signal_find_class_closure(node, (*emission).chain_type);
            if !cc.is_null() {
            } else {
                g_assertion_message_expr(
                    b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    2074 as ::core::ffi::c_int,
                    b"g_signal_chain_from_overridden\0" as *const u8 as *const ::core::ffi::c_char,
                    b"cc != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            n_params = (*node).n_params();
            restore_type = (*cc).instance_type;
            cc = signal_find_class_closure(node, g_type_parent((*cc).instance_type));
            if !cc.is_null() && (*cc).instance_type != restore_type {
                closure = (*cc).closure;
                chain_type = (*cc).instance_type;
            }
        } else {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"%s: signal id '%u' cannot be chained from current emission stage for instance '%p'\0"
                    as *const u8 as *const gchar,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:2086\0"
                    as *const u8 as *const ::core::ffi::c_char,
                (*node).signal_id,
                instance,
            );
        }
    } else {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: no signal is currently being emitted for instance '%p'\0" as *const u8
                as *const gchar,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:2089\0"
                as *const u8 as *const ::core::ffi::c_char,
            instance,
        );
    }
    if !closure.is_null() {
        (*emission).chain_type = chain_type;
        g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
        g_closure_invoke(
            closure,
            return_value,
            n_params.wrapping_add(1 as guint),
            instance_and_params,
            &raw mut (*emission).ihint as gpointer,
        );
        g_mutex_lock(&raw mut g__g_signal_mutex_lock);
        (*emission).chain_type = restore_type;
    }
    g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_chain_from_overridden_handler(
    mut instance: gpointer,
    mut args: ...
) {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut chain_type: GType = 0 as GType;
    let mut restore_type: GType = 0 as GType;
    let mut emission: *mut Emission = ::core::ptr::null_mut::<Emission>();
    let mut closure: *mut GClosure = ::core::ptr::null_mut::<GClosure>();
    let mut node: *mut SignalNode = ::core::ptr::null_mut::<SignalNode>();
    let mut n_params: guint = 0 as guint;
    if g_type_check_instance(instance as *mut GTypeInstance) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_chain_from_overridden_handler\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_CHECK_INSTANCE (instance)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut g__g_signal_mutex_lock);
    emission = emission_find_innermost(instance);
    if !emission.is_null() {
        node = LOOKUP_SIGNAL_NODE((*emission).ihint.signal_id);
        if !node.is_null() {
        } else {
            g_assertion_message_expr(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                2139 as ::core::ffi::c_int,
                b"g_signal_chain_from_overridden_handler\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"node != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if (*emission).chain_type != ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
        {
            let mut cc: *mut ClassClosure = signal_find_class_closure(node, (*emission).chain_type);
            if !cc.is_null() {
            } else {
                g_assertion_message_expr(
                    b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    2147 as ::core::ffi::c_int,
                    b"g_signal_chain_from_overridden_handler\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"cc != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            n_params = (*node).n_params();
            restore_type = (*cc).instance_type;
            cc = signal_find_class_closure(node, g_type_parent((*cc).instance_type));
            if !cc.is_null() && (*cc).instance_type != restore_type {
                closure = (*cc).closure;
                chain_type = (*cc).instance_type;
            }
        } else {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"%s: signal id '%u' cannot be chained from current emission stage for instance '%p'\0"
                    as *const u8 as *const gchar,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:2159\0"
                    as *const u8 as *const ::core::ffi::c_char,
                (*node).signal_id,
                instance,
            );
        }
    } else {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: no signal is currently being emitted for instance '%p'\0" as *const u8
                as *const gchar,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:2162\0"
                as *const u8 as *const ::core::ffi::c_char,
            instance,
        );
    }
    if !closure.is_null() {
        let mut instance_and_params: *mut GValue = ::core::ptr::null_mut::<GValue>();
        let mut signal_return_type: GType = 0;
        let mut param_values: *mut GValue = ::core::ptr::null_mut::<GValue>();
        let mut var_args: ::core::ffi::VaList<'_>;
        let mut i: guint = 0;
        var_args = args.clone();
        signal_return_type = (*node).return_type;
        instance_and_params = (if (::core::mem::size_of::<GValue>() as usize)
            .wrapping_mul(n_params.wrapping_add(1 as guint) as usize)
            == 0 as usize
        {
            ::core::ptr::null_mut::<::core::ffi::c_void>()
        } else {
            alloca_allocations.push(::std::vec::from_elem(
                0,
                (::core::mem::size_of::<GValue>() as usize)
                    .wrapping_mul(n_params.wrapping_add(1 as guint) as usize)
                    as usize,
            ));
            memset(
                alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (::core::mem::size_of::<GValue>() as size_t)
                    .wrapping_mul(n_params.wrapping_add(1 as guint) as size_t),
            )
        }) as *mut GValue;
        param_values = instance_and_params.offset(1 as ::core::ffi::c_int as isize);
        i = 0 as guint;
        while i < (*node).n_params() {
            let mut error: *mut gchar = ::core::ptr::null_mut::<gchar>();
            let mut ptype: GType = *(*node).param_types.offset(i as isize)
                & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType);
            let mut static_scope: gboolean = (*(*node).param_types.offset(i as isize)
                & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType)
                as gboolean;
            g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
            let mut g_vci_vtab: *mut GTypeValueTable = ::core::ptr::null_mut::<GTypeValueTable>();
            let mut g_vci_val: *mut GValue = param_values.offset(i as isize);
            let mut g_vci_flags: guint = (if static_scope != 0 {
                (1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as guint;
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
            g_vci_vtab = g_type_value_table_peek(ptype);
            g_vci_collect_format = (*g_vci_vtab).collect_format;
            (*g_vci_val).g_type = ptype;
            while *g_vci_collect_format != 0 {
                let fresh15 = g_vci_n_values;
                g_vci_n_values = g_vci_n_values.wrapping_add(1);
                let mut g_vci_cvalue: *mut GTypeCValue =
                    (&raw mut g_vci_cvalues as *mut GTypeCValue).offset(fresh15 as isize);
                let fresh16 = g_vci_collect_format;
                g_vci_collect_format = g_vci_collect_format.offset(1);
                match *fresh16 as ::core::ffi::c_int {
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
                            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            2188 as ::core::ffi::c_int,
                            b"g_signal_chain_from_overridden_handler\0" as *const u8
                                as *const ::core::ffi::c_char,
                            ::core::ptr::null::<::core::ffi::c_char>(),
                        );
                    }
                }
            }
            error = (*g_vci_vtab)
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
                    b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:2191\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    error,
                );
                g_free(error as gpointer);
                loop {
                    let fresh17 = i;
                    i = i.wrapping_sub(1);
                    if !(fresh17 != 0) {
                        break;
                    }
                    g_value_unset(param_values.offset(i as isize));
                }
                return;
            }
            g_mutex_lock(&raw mut g__g_signal_mutex_lock);
            i = i.wrapping_add(1);
        }
        g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
        g_value_init_from_instance(instance_and_params, instance);
        g_mutex_lock(&raw mut g__g_signal_mutex_lock);
        (*emission).chain_type = chain_type;
        g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
        if signal_return_type == ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType {
            g_closure_invoke(
                closure,
                ::core::ptr::null_mut::<GValue>(),
                n_params.wrapping_add(1 as guint),
                instance_and_params,
                &raw mut (*emission).ihint as gpointer,
            );
        } else {
            let mut return_value: GValue = _GValue {
                g_type: 0 as GType,
                data: [
                    C2RustUnnamed_0 {
                        v_int: 0 as ::core::ffi::c_int,
                    },
                    C2RustUnnamed_0 { v_int: 0 },
                ],
            };
            let mut error_0: *mut gchar = ::core::ptr::null_mut::<gchar>();
            let mut rtype: GType = signal_return_type
                & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType);
            let mut static_scope_0: gboolean = (signal_return_type
                & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType)
                as gboolean;
            g_value_init(&raw mut return_value, rtype);
            g_closure_invoke(
                closure,
                &raw mut return_value,
                n_params.wrapping_add(1 as guint),
                instance_and_params,
                &raw mut (*emission).ihint as gpointer,
            );
            let mut g_vl_value: *const GValue = &raw mut return_value;
            let mut g_vl_flags: guint = (if static_scope_0 != 0 {
                (1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as guint;
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
                let fresh18 = g_vl_n_values;
                g_vl_n_values = g_vl_n_values.wrapping_add(1);
                let mut g_vl_cvalue: *mut GTypeCValue =
                    (&raw mut g_vl_cvalues as *mut GTypeCValue).offset(fresh18 as isize);
                let fresh19 = g_vl_lcopy_format;
                g_vl_lcopy_format = g_vl_lcopy_format.offset(1);
                match *fresh19 as ::core::ffi::c_int {
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
                            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            2239 as ::core::ffi::c_int,
                            b"g_signal_chain_from_overridden_handler\0" as *const u8
                                as *const ::core::ffi::c_char,
                            ::core::ptr::null::<::core::ffi::c_char>(),
                        );
                    }
                }
            }
            error_0 = (*g_vl_vtable)
                .lcopy_value
                .expect("non-null function pointer")(
                g_vl_value,
                g_vl_n_values,
                &raw mut g_vl_cvalues as *mut GTypeCValue,
                g_vl_flags,
            );
            if error_0.is_null() {
                g_value_unset(&raw mut return_value);
            } else {
                g_log(
                    b"GLib-GObject\0" as *const u8 as *const gchar,
                    G_LOG_LEVEL_CRITICAL,
                    b"%s: %s\0" as *const u8 as *const gchar,
                    b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:2246\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    error_0,
                );
                g_free(error_0 as gpointer);
            }
        }
        i = 0 as guint;
        while i < n_params {
            g_value_unset(param_values.offset(i as isize));
            i = i.wrapping_add(1);
        }
        g_value_unset(instance_and_params);
        g_mutex_lock(&raw mut g__g_signal_mutex_lock);
        (*emission).chain_type = restore_type;
    }
    g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_get_invocation_hint(
    mut instance: gpointer,
) -> *mut GSignalInvocationHint {
    let mut emission: *mut Emission = ::core::ptr::null_mut::<Emission>();
    if g_type_check_instance(instance as *mut GTypeInstance) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_get_invocation_hint\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_CHECK_INSTANCE (instance)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSignalInvocationHint>();
    }
    g_mutex_lock(&raw mut g__g_signal_mutex_lock);
    emission = emission_find_innermost(instance);
    g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
    return if !emission.is_null() {
        &raw mut (*emission).ihint
    } else {
        ::core::ptr::null_mut::<GSignalInvocationHint>()
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_connect_closure_by_id(
    mut instance: gpointer,
    mut signal_id: guint,
    mut detail: GQuark,
    mut closure: *mut GClosure,
    mut after: gboolean,
) -> gulong {
    let mut node: *mut SignalNode = ::core::ptr::null_mut::<SignalNode>();
    let mut handler_seq_no: gulong = 0 as gulong;
    if g_type_check_instance(instance as *mut GTypeInstance) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_connect_closure_by_id\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_CHECK_INSTANCE (instance)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gulong;
    }
    if signal_id > 0 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_connect_closure_by_id\0" as *const u8 as *const ::core::ffi::c_char,
            b"signal_id > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gulong;
    }
    if !closure.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_connect_closure_by_id\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gulong;
    }
    g_mutex_lock(&raw mut g__g_signal_mutex_lock);
    node = LOOKUP_SIGNAL_NODE(signal_id);
    if !node.is_null() {
        if detail != 0
            && (*node).flags() as ::core::ffi::c_int & G_SIGNAL_DETAILED as ::core::ffi::c_int == 0
        {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"%s: signal id '%u' does not support detail (%u)\0" as *const u8 as *const gchar,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:2328\0"
                    as *const u8 as *const ::core::ffi::c_char,
                signal_id,
                detail,
            );
        } else if !((*(*(instance as *mut GTypeInstance)).g_class).g_type == (*node).itype
            || g_type_is_a(
                (*(*(instance as *mut GTypeInstance)).g_class).g_type,
                (*node).itype,
            ) != 0)
        {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"%s: signal id '%u' is invalid for instance '%p'\0" as *const u8 as *const gchar,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:2330\0"
                    as *const u8 as *const ::core::ffi::c_char,
                signal_id,
                instance,
            );
        } else {
            let mut handler: *mut Handler = handler_new(signal_id, instance, after);
            if g_type_fundamental((*node).itype)
                == ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
            {
                _g_object_set_has_signal_handler(instance as *mut GObject, signal_id);
            }
            handler_seq_no = (*handler).sequential_number;
            (*handler).detail = detail;
            (*handler).closure = g_closure_ref(closure);
            g_closure_sink(closure);
            add_invalid_closure_notify(handler, instance);
            handler_insert(signal_id, instance, handler);
            if (*node).c_marshaller.is_some() && (*closure).marshal.is_none() {
                g_closure_set_marshal(closure, (*node).c_marshaller as GClosureMarshal);
                if (*node).va_marshaller.is_some() {
                    _g_closure_set_va_marshal(closure, (*node).va_marshaller as GVaClosureMarshal);
                }
            }
        }
    } else {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: signal id '%u' is invalid for instance '%p'\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:2353\0"
                as *const u8 as *const ::core::ffi::c_char,
            signal_id,
            instance,
        );
    }
    g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
    return handler_seq_no;
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_connect_closure(
    mut instance: gpointer,
    mut detailed_signal: *const gchar,
    mut closure: *mut GClosure,
    mut after: gboolean,
) -> gulong {
    let mut signal_id: guint = 0;
    let mut handler_seq_no: gulong = 0 as gulong;
    let mut detail: GQuark = 0 as GQuark;
    let mut itype: GType = 0;
    if g_type_check_instance(instance as *mut GTypeInstance) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_connect_closure\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_CHECK_INSTANCE (instance)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gulong;
    }
    if !detailed_signal.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_connect_closure\0" as *const u8 as *const ::core::ffi::c_char,
            b"detailed_signal != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gulong;
    }
    if !closure.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_connect_closure\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gulong;
    }
    g_mutex_lock(&raw mut g__g_signal_mutex_lock);
    itype = (*(*(instance as *mut GTypeInstance)).g_class).g_type;
    signal_id = signal_parse_name(
        detailed_signal,
        itype,
        &raw mut detail,
        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
    );
    if signal_id != 0 {
        let mut node: *mut SignalNode = LOOKUP_SIGNAL_NODE(signal_id);
        if detail != 0
            && (*node).flags() as ::core::ffi::c_int & G_SIGNAL_DETAILED as ::core::ffi::c_int == 0
        {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"%s: signal '%s' does not support details\0" as *const u8 as *const gchar,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:2400\0"
                    as *const u8 as *const ::core::ffi::c_char,
                detailed_signal,
            );
        } else if !(itype == (*node).itype || g_type_is_a(itype, (*node).itype) != 0) {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"%s: signal '%s' is invalid for instance '%p' of type '%s'\0" as *const u8
                    as *const gchar,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:2403\0"
                    as *const u8 as *const ::core::ffi::c_char,
                detailed_signal,
                instance,
                g_type_name(itype),
            );
        } else {
            let mut handler: *mut Handler = handler_new(signal_id, instance, after);
            if g_type_fundamental((*node).itype)
                == ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
            {
                _g_object_set_has_signal_handler(instance as *mut GObject, signal_id);
            }
            handler_seq_no = (*handler).sequential_number;
            (*handler).detail = detail;
            (*handler).closure = g_closure_ref(closure);
            g_closure_sink(closure);
            add_invalid_closure_notify(handler, instance);
            handler_insert(signal_id, instance, handler);
            if (*node).c_marshaller.is_some() && (*(*handler).closure).marshal.is_none() {
                g_closure_set_marshal((*handler).closure, (*node).c_marshaller as GClosureMarshal);
                if (*node).va_marshaller.is_some() {
                    _g_closure_set_va_marshal(
                        (*handler).closure,
                        (*node).va_marshaller as GVaClosureMarshal,
                    );
                }
            }
        }
    } else {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: signal '%s' is invalid for instance '%p' of type '%s'\0" as *const u8
                as *const gchar,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:2427\0"
                as *const u8 as *const ::core::ffi::c_char,
            detailed_signal,
            instance,
            g_type_name(itype),
        );
    }
    g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
    return handler_seq_no;
}
unsafe extern "C" fn node_check_deprecated(mut node: *const SignalNode) {
    static mut g_enable_diagnostic: *const gchar = ::core::ptr::null::<gchar>();
    if g_enable_diagnostic.is_null() {
        g_enable_diagnostic = g_getenv(b"G_ENABLE_DIAGNOSTIC\0" as *const u8 as *const gchar);
        if g_enable_diagnostic.is_null() {
            g_enable_diagnostic = b"0\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
        }
    }
    if *g_enable_diagnostic.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == '1' as i32
    {
        if (*node).flags() as ::core::ffi::c_int & G_SIGNAL_DEPRECATED as ::core::ffi::c_int != 0 {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"The signal %s::%s is deprecated and shouldn't be used anymore. It will be removed in a future version.\0"
                    as *const u8 as *const gchar,
                type_debug_name((*node).itype),
                (*node).name,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_connect_data(
    mut instance: gpointer,
    mut detailed_signal: *const gchar,
    mut c_handler: GCallback,
    mut data: gpointer,
    mut destroy_data: GClosureNotify,
    mut connect_flags: GConnectFlags,
) -> gulong {
    let mut signal_id: guint = 0;
    let mut handler_seq_no: gulong = 0 as gulong;
    let mut detail: GQuark = 0 as GQuark;
    let mut itype: GType = 0;
    let mut swapped: gboolean = 0;
    let mut after: gboolean = 0;
    if g_type_check_instance(instance as *mut GTypeInstance) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_connect_data\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_CHECK_INSTANCE (instance)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gulong;
    }
    if !detailed_signal.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_connect_data\0" as *const u8 as *const ::core::ffi::c_char,
            b"detailed_signal != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gulong;
    }
    if c_handler.is_some() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_connect_data\0" as *const u8 as *const ::core::ffi::c_char,
            b"c_handler != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gulong;
    }
    swapped = (connect_flags as ::core::ffi::c_uint
        & G_CONNECT_SWAPPED as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int as gboolean;
    after = (connect_flags as ::core::ffi::c_uint
        & G_CONNECT_AFTER as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int as gboolean;
    g_mutex_lock(&raw mut g__g_signal_mutex_lock);
    itype = (*(*(instance as *mut GTypeInstance)).g_class).g_type;
    signal_id = signal_parse_name(
        detailed_signal,
        itype,
        &raw mut detail,
        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
    );
    if signal_id != 0 {
        let mut node: *mut SignalNode = LOOKUP_SIGNAL_NODE(signal_id);
        node_check_deprecated(node);
        if detail != 0
            && (*node).flags() as ::core::ffi::c_int & G_SIGNAL_DETAILED as ::core::ffi::c_int == 0
        {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"%s: signal '%s' does not support details\0" as *const u8 as *const gchar,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:2507\0"
                    as *const u8 as *const ::core::ffi::c_char,
                detailed_signal,
            );
        } else if !(itype == (*node).itype || g_type_is_a(itype, (*node).itype) != 0) {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"%s: signal '%s' is invalid for instance '%p' of type '%s'\0" as *const u8
                    as *const gchar,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:2510\0"
                    as *const u8 as *const ::core::ffi::c_char,
                detailed_signal,
                instance,
                g_type_name(itype),
            );
        } else {
            let mut handler: *mut Handler = handler_new(signal_id, instance, after);
            if g_type_fundamental((*node).itype)
                == ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
            {
                _g_object_set_has_signal_handler(instance as *mut GObject, signal_id);
            }
            handler_seq_no = (*handler).sequential_number;
            (*handler).detail = detail;
            (*handler).closure = g_closure_ref(if swapped != 0 {
                Some(
                    g_cclosure_new_swap
                        as unsafe extern "C" fn(
                            GCallback,
                            gpointer,
                            GClosureNotify,
                        ) -> *mut GClosure,
                )
            } else {
                Some(
                    g_cclosure_new
                        as unsafe extern "C" fn(
                            GCallback,
                            gpointer,
                            GClosureNotify,
                        ) -> *mut GClosure,
                )
            }
            .expect("non-null function pointer")(
                c_handler, data, destroy_data
            ));
            g_closure_sink((*handler).closure);
            handler_insert(signal_id, instance, handler);
            if (*node).c_marshaller.is_some() && (*(*handler).closure).marshal.is_none() {
                g_closure_set_marshal((*handler).closure, (*node).c_marshaller as GClosureMarshal);
                if (*node).va_marshaller.is_some() {
                    _g_closure_set_va_marshal(
                        (*handler).closure,
                        (*node).va_marshaller as GVaClosureMarshal,
                    );
                }
            }
        }
    } else {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: signal '%s' is invalid for instance '%p' of type '%s'\0" as *const u8
                as *const gchar,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:2533\0"
                as *const u8 as *const ::core::ffi::c_char,
            detailed_signal,
            instance,
            g_type_name(itype),
        );
    }
    g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
    return handler_seq_no;
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_handler_block(mut instance: gpointer, mut handler_id: gulong) {
    if g_type_check_instance(instance as *mut GTypeInstance) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_handler_block\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_CHECK_INSTANCE (instance)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if handler_id > 0 as gulong {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_handler_block\0" as *const u8 as *const ::core::ffi::c_char,
            b"handler_id > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut g__g_signal_mutex_lock);
    signal_handler_block_unlocked(instance, handler_id);
    g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
}
unsafe extern "C" fn signal_handler_block_unlocked(mut instance: gpointer, mut handler_id: gulong) {
    let mut handler: *mut Handler = ::core::ptr::null_mut::<Handler>();
    handler = handler_lookup(
        instance,
        handler_id,
        ::core::ptr::null_mut::<GClosure>(),
        ::core::ptr::null_mut::<guint>(),
    );
    if !handler.is_null() {
        if (*handler).block_count() as ::core::ffi::c_int
            >= ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int
        {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_ERROR,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:2580: handler block_count overflow, %s\0"
                    as *const u8 as *const gchar,
                b"please report occurrence circumstances to https://gitlab.gnome.org/GNOME/glib/issues/new\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            loop {}
        }
        (*handler).set_block_count((*handler).block_count() + 1 as ::core::ffi::c_int as guint);
    } else {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: instance '%p' has no handler with id '%lu'\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:2585\0"
                as *const u8 as *const ::core::ffi::c_char,
            instance,
            handler_id,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_handler_unblock(mut instance: gpointer, mut handler_id: gulong) {
    if g_type_check_instance(instance as *mut GTypeInstance) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_handler_unblock\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_CHECK_INSTANCE (instance)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if handler_id > 0 as gulong {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_handler_unblock\0" as *const u8 as *const ::core::ffi::c_char,
            b"handler_id > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut g__g_signal_mutex_lock);
    signal_handler_unblock_unlocked(instance, handler_id);
    g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
}
unsafe extern "C" fn signal_handler_unblock_unlocked(
    mut instance: gpointer,
    mut handler_id: gulong,
) {
    let mut handler: *mut Handler = ::core::ptr::null_mut::<Handler>();
    handler = handler_lookup(
        instance,
        handler_id,
        ::core::ptr::null_mut::<GClosure>(),
        ::core::ptr::null_mut::<guint>(),
    );
    if !handler.is_null() {
        if (*handler).block_count() != 0 {
            (*handler).set_block_count((*handler).block_count() - 1 as ::core::ffi::c_int as guint);
        } else {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:2635: handler '%lu' of instance '%p' is not blocked\0"
                    as *const u8 as *const gchar,
                handler_id,
                instance,
            );
        }
    } else {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: instance '%p' has no handler with id '%lu'\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:2638\0"
                as *const u8 as *const ::core::ffi::c_char,
            instance,
            handler_id,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_handler_disconnect(
    mut instance: gpointer,
    mut handler_id: gulong,
) {
    if g_type_check_instance(instance as *mut GTypeInstance) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_handler_disconnect\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_CHECK_INSTANCE (instance)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if handler_id > 0 as gulong {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_handler_disconnect\0" as *const u8 as *const ::core::ffi::c_char,
            b"handler_id > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut g__g_signal_mutex_lock);
    signal_handler_disconnect_unlocked(instance, handler_id);
    g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
}
unsafe extern "C" fn signal_handler_disconnect_unlocked(
    mut instance: gpointer,
    mut handler_id: gulong,
) {
    let mut handler: *mut Handler = ::core::ptr::null_mut::<Handler>();
    handler = handler_lookup(
        instance,
        handler_id,
        ::core::ptr::null_mut::<GClosure>(),
        ::core::ptr::null_mut::<guint>(),
    );
    if !handler.is_null() {
        g_hash_table_remove(g_handlers, handler as gconstpointer);
        (*handler).sequential_number = 0 as gulong;
        (*handler).set_block_count(1 as guint as guint);
        remove_invalid_closure_notify(handler, instance);
        handler_unref_R((*handler).signal_id, instance, handler);
    } else {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: instance '%p' has no handler with id '%lu'\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:2685\0"
                as *const u8 as *const ::core::ffi::c_char,
            instance,
            handler_id,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_handler_is_connected(
    mut instance: gpointer,
    mut handler_id: gulong,
) -> gboolean {
    let mut handler: *mut Handler = ::core::ptr::null_mut::<Handler>();
    let mut connected: gboolean = 0;
    if g_type_check_instance(instance as *mut GTypeInstance) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_handler_is_connected\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_CHECK_INSTANCE (instance)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    g_mutex_lock(&raw mut g__g_signal_mutex_lock);
    handler = handler_lookup(
        instance,
        handler_id,
        ::core::ptr::null_mut::<GClosure>(),
        ::core::ptr::null_mut::<guint>(),
    );
    connected = (handler != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Handler)
        as ::core::ffi::c_int as gboolean;
    g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
    return connected;
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_handlers_destroy(mut instance: gpointer) {
    let mut hlbsa: *mut GBSearchArray = ::core::ptr::null_mut::<GBSearchArray>();
    if g_type_check_instance(instance as *mut GTypeInstance) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_handlers_destroy\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_CHECK_INSTANCE (instance)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut g__g_signal_mutex_lock);
    hlbsa =
        g_hash_table_lookup(g_handler_list_bsa_ht, instance as gconstpointer) as *mut GBSearchArray;
    if !hlbsa.is_null() {
        let mut i: guint = 0;
        g_hash_table_remove(g_handler_list_bsa_ht, instance as gconstpointer);
        i = 0 as guint;
        while i < (*hlbsa).n_nodes {
            let mut hlist: *mut HandlerList =
                g_bsearch_array_get_nth(hlbsa, &raw mut g_signal_hlbsa_bconfig, i)
                    as *mut HandlerList;
            let mut handler: *mut Handler = (*hlist).handlers;
            while !handler.is_null() {
                let mut tmp: *mut Handler = handler;
                handler = (*tmp).next;
                (*tmp).set_block_count(1 as guint as guint);
                (*tmp).next = ::core::ptr::null_mut::<Handler>();
                (*tmp).prev = tmp;
                if (*tmp).sequential_number != 0 {
                    g_hash_table_remove(g_handlers, tmp as gconstpointer);
                    remove_invalid_closure_notify(tmp, instance);
                    (*tmp).sequential_number = 0 as gulong;
                    handler_unref_R(
                        0 as guint,
                        ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        tmp,
                    );
                }
            }
            i = i.wrapping_add(1);
        }
        g_bsearch_array_free(hlbsa, &raw mut g_signal_hlbsa_bconfig);
    }
    g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_handler_find(
    mut instance: gpointer,
    mut mask: GSignalMatchType,
    mut signal_id: guint,
    mut detail: GQuark,
    mut closure: *mut GClosure,
    mut func: gpointer,
    mut data: gpointer,
) -> gulong {
    let mut handler_seq_no: gulong = 0 as gulong;
    if g_type_check_instance(instance as *mut GTypeInstance) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_handler_find\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_CHECK_INSTANCE (instance)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gulong;
    }
    if mask as ::core::ffi::c_uint & !(0x3f as ::core::ffi::c_int) as ::core::ffi::c_uint
        == 0 as ::core::ffi::c_uint
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_handler_find\0" as *const u8 as *const ::core::ffi::c_char,
            b"(mask & ~G_SIGNAL_MATCH_MASK) == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gulong;
    }
    if mask as ::core::ffi::c_uint & 0x3f as ::core::ffi::c_uint != 0 {
        let mut mlist: *mut HandlerMatch = ::core::ptr::null_mut::<HandlerMatch>();
        g_mutex_lock(&raw mut g__g_signal_mutex_lock);
        mlist = handlers_find(
            instance,
            mask,
            signal_id,
            detail,
            closure,
            func,
            data,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
        );
        if !mlist.is_null() {
            handler_seq_no = (*(*mlist).handler).sequential_number;
            handler_match_free1_R(mlist, instance);
        }
        g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
    }
    return handler_seq_no;
}
unsafe extern "C" fn signal_handlers_foreach_matched_unlocked_R(
    mut instance: gpointer,
    mut mask: GSignalMatchType,
    mut signal_id: guint,
    mut detail: GQuark,
    mut closure: *mut GClosure,
    mut func: gpointer,
    mut data: gpointer,
    mut callback: CallbackHandlerFunc,
) -> guint {
    let mut mlist: *mut HandlerMatch = ::core::ptr::null_mut::<HandlerMatch>();
    let mut n_handlers: guint = 0 as guint;
    mlist = handlers_find(
        instance,
        mask,
        signal_id,
        detail,
        closure,
        func,
        data,
        0 as gboolean,
    );
    while !mlist.is_null() {
        n_handlers = n_handlers.wrapping_add(1);
        if (*(*mlist).handler).sequential_number != 0 {
            callback.expect("non-null function pointer")(
                instance,
                (*(*mlist).handler).sequential_number,
            );
        }
        mlist = handler_match_free1_R(mlist, instance);
    }
    return n_handlers;
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_handlers_block_matched(
    mut instance: gpointer,
    mut mask: GSignalMatchType,
    mut signal_id: guint,
    mut detail: GQuark,
    mut closure: *mut GClosure,
    mut func: gpointer,
    mut data: gpointer,
) -> guint {
    let mut n_handlers: guint = 0 as guint;
    if g_type_check_instance(instance as *mut GTypeInstance) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_handlers_block_matched\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_CHECK_INSTANCE (instance)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if mask as ::core::ffi::c_uint & !(0x3f as ::core::ffi::c_int) as ::core::ffi::c_uint
        == 0 as ::core::ffi::c_uint
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_handlers_block_matched\0" as *const u8 as *const ::core::ffi::c_char,
            b"(mask & ~G_SIGNAL_MATCH_MASK) == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if mask as ::core::ffi::c_uint
        & (G_SIGNAL_MATCH_ID as ::core::ffi::c_int
            | G_SIGNAL_MATCH_CLOSURE as ::core::ffi::c_int
            | G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int
            | G_SIGNAL_MATCH_DATA as ::core::ffi::c_int) as ::core::ffi::c_uint
        != 0
    {
        g_mutex_lock(&raw mut g__g_signal_mutex_lock);
        n_handlers = signal_handlers_foreach_matched_unlocked_R(
            instance,
            mask,
            signal_id,
            detail,
            closure,
            func,
            data,
            Some(signal_handler_block_unlocked as unsafe extern "C" fn(gpointer, gulong) -> ()),
        );
        g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
    }
    return n_handlers;
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_handlers_unblock_matched(
    mut instance: gpointer,
    mut mask: GSignalMatchType,
    mut signal_id: guint,
    mut detail: GQuark,
    mut closure: *mut GClosure,
    mut func: gpointer,
    mut data: gpointer,
) -> guint {
    let mut n_handlers: guint = 0 as guint;
    if g_type_check_instance(instance as *mut GTypeInstance) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_handlers_unblock_matched\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_CHECK_INSTANCE (instance)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if mask as ::core::ffi::c_uint & !(0x3f as ::core::ffi::c_int) as ::core::ffi::c_uint
        == 0 as ::core::ffi::c_uint
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_handlers_unblock_matched\0" as *const u8 as *const ::core::ffi::c_char,
            b"(mask & ~G_SIGNAL_MATCH_MASK) == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if mask as ::core::ffi::c_uint
        & (G_SIGNAL_MATCH_ID as ::core::ffi::c_int
            | G_SIGNAL_MATCH_CLOSURE as ::core::ffi::c_int
            | G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int
            | G_SIGNAL_MATCH_DATA as ::core::ffi::c_int) as ::core::ffi::c_uint
        != 0
    {
        g_mutex_lock(&raw mut g__g_signal_mutex_lock);
        n_handlers = signal_handlers_foreach_matched_unlocked_R(
            instance,
            mask,
            signal_id,
            detail,
            closure,
            func,
            data,
            Some(signal_handler_unblock_unlocked as unsafe extern "C" fn(gpointer, gulong) -> ()),
        );
        g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
    }
    return n_handlers;
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_handlers_disconnect_matched(
    mut instance: gpointer,
    mut mask: GSignalMatchType,
    mut signal_id: guint,
    mut detail: GQuark,
    mut closure: *mut GClosure,
    mut func: gpointer,
    mut data: gpointer,
) -> guint {
    let mut n_handlers: guint = 0 as guint;
    if g_type_check_instance(instance as *mut GTypeInstance) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_handlers_disconnect_matched\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_CHECK_INSTANCE (instance)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if mask as ::core::ffi::c_uint & !(0x3f as ::core::ffi::c_int) as ::core::ffi::c_uint
        == 0 as ::core::ffi::c_uint
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_handlers_disconnect_matched\0" as *const u8 as *const ::core::ffi::c_char,
            b"(mask & ~G_SIGNAL_MATCH_MASK) == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if mask as ::core::ffi::c_uint
        & (G_SIGNAL_MATCH_ID as ::core::ffi::c_int
            | G_SIGNAL_MATCH_CLOSURE as ::core::ffi::c_int
            | G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int
            | G_SIGNAL_MATCH_DATA as ::core::ffi::c_int) as ::core::ffi::c_uint
        != 0
    {
        g_mutex_lock(&raw mut g__g_signal_mutex_lock);
        n_handlers = signal_handlers_foreach_matched_unlocked_R(
            instance,
            mask,
            signal_id,
            detail,
            closure,
            func,
            data,
            Some(
                signal_handler_disconnect_unlocked as unsafe extern "C" fn(gpointer, gulong) -> (),
            ),
        );
        g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
    }
    return n_handlers;
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_has_handler_pending(
    mut instance: gpointer,
    mut signal_id: guint,
    mut detail: GQuark,
    mut may_be_blocked: gboolean,
) -> gboolean {
    let mut mlist: *mut HandlerMatch = ::core::ptr::null_mut::<HandlerMatch>();
    let mut has_pending: gboolean = 0;
    let mut node: *mut SignalNode = ::core::ptr::null_mut::<SignalNode>();
    if g_type_check_instance(instance as *mut GTypeInstance) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_has_handler_pending\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_CHECK_INSTANCE (instance)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if signal_id > 0 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_has_handler_pending\0" as *const u8 as *const ::core::ffi::c_char,
            b"signal_id > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    g_mutex_lock(&raw mut g__g_signal_mutex_lock);
    node = LOOKUP_SIGNAL_NODE(signal_id);
    if detail != 0 {
        if (*node).flags() as ::core::ffi::c_int & G_SIGNAL_DETAILED as ::core::ffi::c_int == 0 {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"%s: signal id '%u' does not support detail (%u)\0" as *const u8 as *const gchar,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:3056\0"
                    as *const u8 as *const ::core::ffi::c_char,
                signal_id,
                detail,
            );
            g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
            return 0 as gboolean;
        }
    }
    mlist = handlers_find(
        instance,
        (G_SIGNAL_MATCH_ID as ::core::ffi::c_int
            | G_SIGNAL_MATCH_DETAIL as ::core::ffi::c_int
            | (if may_be_blocked != 0 {
                0 as ::core::ffi::c_int
            } else {
                G_SIGNAL_MATCH_UNBLOCKED as ::core::ffi::c_int
            })) as GSignalMatchType,
        signal_id,
        detail,
        ::core::ptr::null_mut::<GClosure>(),
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
    );
    if !mlist.is_null() {
        has_pending = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        handler_match_free1_R(mlist, instance);
    } else {
        let mut class_closure: *mut ClassClosure =
            signal_find_class_closure(node, (*(*(instance as *mut GTypeInstance)).g_class).g_type);
        if !class_closure.is_null() && (*class_closure).instance_type != 0 as GType {
            has_pending = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            has_pending = 0 as ::core::ffi::c_int as gboolean;
        }
    }
    g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
    return has_pending;
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_emitv(
    mut instance_and_params: *const GValue,
    mut signal_id: guint,
    mut detail: GQuark,
    mut return_value: *mut GValue,
) {
    g_mutex_lock(&raw mut g__g_signal_mutex_lock);
    signal_emitv_unlocked(instance_and_params, signal_id, detail, return_value);
    g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
}
unsafe extern "C" fn signal_emitv_unlocked(
    mut instance_and_params: *const GValue,
    mut signal_id: guint,
    mut detail: GQuark,
    mut return_value: *mut GValue,
) {
    let mut instance: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut node: *mut SignalNode = ::core::ptr::null_mut::<SignalNode>();
    if !instance_and_params.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"signal_emitv_unlocked\0" as *const u8 as *const ::core::ffi::c_char,
            b"instance_and_params != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    instance = g_value_peek_pointer(instance_and_params);
    if g_type_check_instance(instance as *mut GTypeInstance) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"signal_emitv_unlocked\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_CHECK_INSTANCE (instance)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if signal_id > 0 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"signal_emitv_unlocked\0" as *const u8 as *const ::core::ffi::c_char,
            b"signal_id > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    node = LOOKUP_SIGNAL_NODE(signal_id);
    if node.is_null()
        || !((*(*(instance as *mut GTypeInstance)).g_class).g_type == (*node).itype
            || g_type_is_a(
                (*(*(instance as *mut GTypeInstance)).g_class).g_type,
                (*node).itype,
            ) != 0)
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: signal id '%u' is invalid for instance '%p'\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:3141\0"
                as *const u8 as *const ::core::ffi::c_char,
            signal_id,
            instance,
        );
        return;
    }
    if (*node).single_va_closure_is_valid() == 0 {
        node_update_single_va_closure(node);
    }
    if !(*node).single_va_closure.is_null()
        && ((*node).single_va_closure
            == 1 as ::core::ffi::c_int as glong as gpointer as *mut GClosure
            || _g_closure_is_void((*node).single_va_closure, instance) != 0)
    {
        let mut hlist: *mut HandlerList = ::core::ptr::null_mut::<HandlerList>();
        if _g_object_has_signal_handler(instance as *mut GObject) != 0 {
            hlist = handler_list_lookup((*node).signal_id, instance);
        } else {
            hlist = ::core::ptr::null_mut::<HandlerList>();
        }
        if hlist.is_null() || (*hlist).handlers.is_null() {
            return;
        }
    }
    let mut node_copy: SignalNode = *node;
    signal_emit_unlocked_R(
        &raw mut node_copy,
        detail,
        instance,
        return_value,
        instance_and_params,
    );
}
#[inline]
unsafe extern "C" fn accumulate(
    mut ihint: *mut GSignalInvocationHint,
    mut return_accu: *mut GValue,
    mut handler_return: *mut GValue,
    mut accumulator: *mut SignalAccumulator,
) -> gboolean {
    let mut continue_emission: gboolean = 0;
    if accumulator.is_null() {
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    continue_emission = (*accumulator).func.expect("non-null function pointer")(
        ihint,
        return_accu,
        handler_return,
        (*accumulator).data,
    );
    g_value_reset(handler_return);
    (*ihint).run_type = ::core::mem::transmute::<::core::ffi::c_uint, GSignalFlags>(
        (*ihint).run_type as ::core::ffi::c_uint
            & !(G_SIGNAL_ACCUMULATOR_FIRST_RUN as ::core::ffi::c_int) as ::core::ffi::c_uint,
    );
    return continue_emission;
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_emit_valist(
    mut instance: gpointer,
    mut signal_id: guint,
    mut detail: GQuark,
    mut var_args: ::core::ffi::VaList,
) {
    g_mutex_lock(&raw mut g__g_signal_mutex_lock);
    if signal_emit_valist_unlocked(instance, signal_id, detail, var_args) != 0 {
        g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
    }
}
unsafe extern "C" fn signal_emit_valist_unlocked(
    mut instance: gpointer,
    mut signal_id: guint,
    mut detail: GQuark,
    mut var_args: ::core::ffi::VaList,
) -> gboolean {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut instance_and_params: *mut GValue = ::core::ptr::null_mut::<GValue>();
    let mut param_values: *mut GValue = ::core::ptr::null_mut::<GValue>();
    let mut node: *mut SignalNode = ::core::ptr::null_mut::<SignalNode>();
    let mut i: guint = 0;
    if g_type_check_instance(instance as *mut GTypeInstance) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"signal_emit_valist_unlocked\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_CHECK_INSTANCE (instance)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    if signal_id > 0 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"signal_emit_valist_unlocked\0" as *const u8 as *const ::core::ffi::c_char,
            b"signal_id > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    node = LOOKUP_SIGNAL_NODE(signal_id);
    if node.is_null()
        || !((*(*(instance as *mut GTypeInstance)).g_class).g_type == (*node).itype
            || g_type_is_a(
                (*(*(instance as *mut GTypeInstance)).g_class).g_type,
                (*node).itype,
            ) != 0)
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: signal id '%u' is invalid for instance '%p'\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:3293\0"
                as *const u8 as *const ::core::ffi::c_char,
            signal_id,
            instance,
        );
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    if detail != 0
        && (*node).flags() as ::core::ffi::c_int & G_SIGNAL_DETAILED as ::core::ffi::c_int == 0
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: signal id '%u' does not support detail (%u)\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:3299\0"
                as *const u8 as *const ::core::ffi::c_char,
            signal_id,
            detail,
        );
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    if (*node).single_va_closure_is_valid() == 0 {
        node_update_single_va_closure(node);
    }
    let mut node_copy: SignalNode = *node;
    if !(*node).single_va_closure.is_null() {
        let mut hlist: *mut HandlerList = ::core::ptr::null_mut::<HandlerList>();
        let mut fastpath_handler: *mut Handler = ::core::ptr::null_mut::<Handler>();
        let mut l: *mut Handler = ::core::ptr::null_mut::<Handler>();
        let mut closure: *mut GClosure = ::core::ptr::null_mut::<GClosure>();
        let mut fastpath: gboolean = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
        let mut run_type: GSignalFlags = G_SIGNAL_RUN_FIRST;
        if (*node).single_va_closure
            != 1 as ::core::ffi::c_int as glong as gpointer as *mut GClosure
            && _g_closure_is_void((*node).single_va_closure, instance) == 0
        {
            if _g_closure_supports_invoke_va((*node).single_va_closure) != 0 {
                closure = (*node).single_va_closure;
                if (*node).single_va_closure_is_after() != 0 {
                    run_type = G_SIGNAL_RUN_LAST;
                } else {
                    run_type = G_SIGNAL_RUN_FIRST;
                }
            } else {
                fastpath = 0 as ::core::ffi::c_int as gboolean;
            }
        }
        if _g_object_has_signal_handler(instance as *mut GObject) != 0 {
            hlist = handler_list_lookup((*node).signal_id, instance);
        } else {
            hlist = ::core::ptr::null_mut::<HandlerList>();
        }
        l = if !hlist.is_null() {
            (*hlist).handlers
        } else {
            ::core::ptr::null_mut::<Handler>()
        };
        while fastpath != 0 && !l.is_null() {
            if (*l).block_count() == 0 && ((*l).detail == 0 || (*l).detail == detail) {
                if !closure.is_null() || _g_closure_supports_invoke_va((*l).closure) == 0 {
                    fastpath = 0 as ::core::ffi::c_int as gboolean;
                    break;
                } else {
                    fastpath_handler = l;
                    closure = (*l).closure;
                    if (*l).after() != 0 {
                        run_type = G_SIGNAL_RUN_LAST;
                    } else {
                        run_type = G_SIGNAL_RUN_FIRST;
                    }
                }
            }
            l = (*l).next;
        }
        if fastpath != 0
            && closure.is_null()
            && node_copy.return_type
                == ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
        {
            return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
        }
        if !closure.is_null()
            && node_copy.flags() as ::core::ffi::c_int & G_SIGNAL_NO_RECURSE as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
        {
            fastpath = 0 as ::core::ffi::c_int as gboolean;
        }
        if fastpath != 0 {
            let mut emission: Emission = _Emission {
                next: ::core::ptr::null_mut::<Emission>(),
                instance: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                ihint: _GSignalInvocationHint {
                    signal_id: 0,
                    detail: 0,
                    run_type: 0 as GSignalFlags,
                },
                state: EMISSION_STOP,
                chain_type: 0,
            };
            let mut return_accu: *mut GValue = ::core::ptr::null_mut::<GValue>();
            let mut accu: GValue = _GValue {
                g_type: 0 as GType,
                data: [
                    C2RustUnnamed_0 {
                        v_int: 0 as ::core::ffi::c_int,
                    },
                    C2RustUnnamed_0 { v_int: 0 },
                ],
            };
            let mut instance_type: GType = (*(*(instance as *mut GTypeInstance)).g_class).g_type;
            let mut emission_return: GValue = _GValue {
                g_type: 0 as GType,
                data: [
                    C2RustUnnamed_0 {
                        v_int: 0 as ::core::ffi::c_int,
                    },
                    C2RustUnnamed_0 { v_int: 0 },
                ],
            };
            let mut rtype: GType = node_copy.return_type
                & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType);
            let mut static_scope: gboolean = (node_copy.return_type
                & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType)
                as gboolean;
            if rtype == ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType {
                return_accu = ::core::ptr::null_mut::<GValue>();
            } else if !node_copy.accumulator.is_null() {
                return_accu = &raw mut accu;
            } else {
                return_accu = &raw mut emission_return;
            }
            emission.instance = instance;
            emission.ihint.signal_id = signal_id;
            emission.ihint.detail = detail;
            emission.ihint.run_type = (run_type as ::core::ffi::c_uint
                | G_SIGNAL_ACCUMULATOR_FIRST_RUN as ::core::ffi::c_int as ::core::ffi::c_uint)
                as GSignalFlags;
            emission.state = EMISSION_RUN;
            emission.chain_type = instance_type;
            emission_push(&raw mut emission);
            if !fastpath_handler.is_null() {
                handler_ref(fastpath_handler);
            }
            if !closure.is_null() {
                g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
                if rtype != ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType {
                    g_value_init(&raw mut emission_return, rtype);
                }
                if !node_copy.accumulator.is_null() {
                    g_value_init(&raw mut accu, rtype);
                }
                g_object_ref(instance);
                _g_closure_invoke_va(
                    closure,
                    return_accu,
                    instance,
                    var_args.clone(),
                    node_copy.n_params() as ::core::ffi::c_int,
                    node_copy.param_types,
                );
                accumulate(
                    &raw mut emission.ihint,
                    &raw mut emission_return,
                    &raw mut accu,
                    node_copy.accumulator,
                );
                if !node_copy.accumulator.is_null() {
                    g_value_unset(&raw mut accu);
                }
                g_mutex_lock(&raw mut g__g_signal_mutex_lock);
            }
            emission.chain_type = ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
            emission_pop(&raw mut emission);
            if !fastpath_handler.is_null() {
                handler_unref_R(signal_id, instance, fastpath_handler);
            }
            g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
            if rtype != ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType {
                let mut error: *mut gchar = ::core::ptr::null_mut::<gchar>();
                i = 0 as guint;
                while i < node_copy.n_params() {
                    let mut ptype: GType = *node_copy.param_types.offset(i as isize)
                        & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType);
                    let mut g_vcs_vtable: *mut GTypeValueTable = g_type_value_table_peek(ptype);
                    let mut g_vcs_collect_format: *const gchar = (*g_vcs_vtable).collect_format;
                    while *g_vcs_collect_format != 0 {
                        let fresh6 = g_vcs_collect_format;
                        g_vcs_collect_format = g_vcs_collect_format.offset(1);
                        match *fresh6 as ::core::ffi::c_int {
                            105 => {
                                var_args.arg::<gint>();
                            }
                            108 => {
                                var_args.arg::<glong>();
                            }
                            113 => {
                                var_args.arg::<gint64>();
                            }
                            100 => {
                                var_args.arg::<gdouble>();
                            }
                            112 => {
                                var_args.arg::<gpointer>();
                            }
                            _ => {
                                g_assertion_message_expr(
                                    b"GLib-GObject\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    3452 as ::core::ffi::c_int,
                                    b"signal_emit_valist_unlocked\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                );
                            }
                        }
                    }
                    i = i.wrapping_add(1);
                }
                if closure.is_null() {
                    g_value_init(&raw mut emission_return, rtype);
                }
                let mut g_vl_value: *const GValue = &raw mut emission_return;
                let mut g_vl_flags: guint = (if static_scope != 0 {
                    (1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as guint;
                let mut g_vl_value_type: GType = (*(g_vl_value as *mut GValue)).g_type;
                let mut g_vl_vtable: *mut GTypeValueTable =
                    g_type_value_table_peek(g_vl_value_type);
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
                    let fresh7 = g_vl_n_values;
                    g_vl_n_values = g_vl_n_values.wrapping_add(1);
                    let mut g_vl_cvalue: *mut GTypeCValue =
                        (&raw mut g_vl_cvalues as *mut GTypeCValue).offset(fresh7 as isize);
                    let fresh8 = g_vl_lcopy_format;
                    g_vl_lcopy_format = g_vl_lcopy_format.offset(1);
                    match *fresh8 as ::core::ffi::c_int {
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
                                b"GLib-GObject\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c\0"
                                    as *const u8 as *const ::core::ffi::c_char,
                                3461 as ::core::ffi::c_int,
                                b"signal_emit_valist_unlocked\0" as *const u8
                                    as *const ::core::ffi::c_char,
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
                if error.is_null() {
                    g_value_unset(&raw mut emission_return);
                } else {
                    g_log(
                        b"GLib-GObject\0" as *const u8 as *const gchar,
                        G_LOG_LEVEL_CRITICAL,
                        b"%s: %s\0" as *const u8 as *const gchar,
                        b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:3466\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        error,
                    );
                    g_free(error as gpointer);
                }
            }
            if !closure.is_null() {
                g_object_unref(instance);
            }
            return 0 as gboolean;
        }
    }
    g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
    instance_and_params = (if (::core::mem::size_of::<GValue>() as usize).wrapping_mul(
        (node_copy.n_params() as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize,
    ) == 0 as usize
    {
        ::core::ptr::null_mut::<::core::ffi::c_void>()
    } else {
        alloca_allocations.push(::std::vec::from_elem(
            0,
            (::core::mem::size_of::<GValue>() as usize).wrapping_mul(
                (node_copy.n_params() as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize,
            ) as usize,
        ));
        memset(
            alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<GValue>() as size_t).wrapping_mul(
                (node_copy.n_params() as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t,
            ),
        )
    }) as *mut GValue;
    param_values = instance_and_params.offset(1 as ::core::ffi::c_int as isize);
    i = 0 as guint;
    while i < node_copy.n_params() {
        let mut error_0: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut ptype_0: GType = *node_copy.param_types.offset(i as isize)
            & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType);
        let mut static_scope_0: gboolean = (*node_copy.param_types.offset(i as isize)
            & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType)
            as gboolean;
        let mut g_vci_vtab: *mut GTypeValueTable = ::core::ptr::null_mut::<GTypeValueTable>();
        let mut g_vci_val: *mut GValue = param_values.offset(i as isize);
        let mut g_vci_flags: guint = (if static_scope_0 != 0 {
            (1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as guint;
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
        g_vci_vtab = g_type_value_table_peek(ptype_0);
        g_vci_collect_format = (*g_vci_vtab).collect_format;
        (*g_vci_val).g_type = ptype_0;
        while *g_vci_collect_format != 0 {
            let fresh9 = g_vci_n_values;
            g_vci_n_values = g_vci_n_values.wrapping_add(1);
            let mut g_vci_cvalue: *mut GTypeCValue =
                (&raw mut g_vci_cvalues as *mut GTypeCValue).offset(fresh9 as isize);
            let fresh10 = g_vci_collect_format;
            g_vci_collect_format = g_vci_collect_format.offset(1);
            match *fresh10 as ::core::ffi::c_int {
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
                        b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        3500 as ::core::ffi::c_int,
                        b"signal_emit_valist_unlocked\0" as *const u8 as *const ::core::ffi::c_char,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                    );
                }
            }
        }
        error_0 = (*g_vci_vtab)
            .collect_value
            .expect("non-null function pointer")(
            g_vci_val,
            g_vci_n_values,
            &raw mut g_vci_cvalues as *mut GTypeCValue,
            g_vci_flags,
        );
        if !error_0.is_null() {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"%s: %s\0" as *const u8 as *const gchar,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:3503\0"
                    as *const u8 as *const ::core::ffi::c_char,
                error_0,
            );
            g_free(error_0 as gpointer);
            loop {
                let fresh11 = i;
                i = i.wrapping_sub(1);
                if !(fresh11 != 0) {
                    break;
                }
                g_value_unset(param_values.offset(i as isize));
            }
            return 0 as gboolean;
        }
        i = i.wrapping_add(1);
    }
    g_value_init_from_instance(instance_and_params, instance);
    if node_copy.return_type == ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType {
        g_mutex_lock(&raw mut g__g_signal_mutex_lock);
        signal_emit_unlocked_R(
            &raw mut node_copy,
            detail,
            instance,
            ::core::ptr::null_mut::<GValue>(),
            instance_and_params,
        );
        g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
    } else {
        let mut return_value: GValue = _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed_0 {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed_0 { v_int: 0 },
            ],
        };
        let mut error_1: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut rtype_0: GType = node_copy.return_type
            & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType);
        let mut static_scope_1: gboolean = (node_copy.return_type
            & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType)
            as gboolean;
        g_value_init(&raw mut return_value, rtype_0);
        g_mutex_lock(&raw mut g__g_signal_mutex_lock);
        signal_emit_unlocked_R(
            &raw mut node_copy,
            detail,
            instance,
            &raw mut return_value,
            instance_and_params,
        );
        g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
        let mut g_vl_value_0: *const GValue = &raw mut return_value;
        let mut g_vl_flags_0: guint = (if static_scope_1 != 0 {
            (1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as guint;
        let mut g_vl_value_type_0: GType = (*(g_vl_value_0 as *mut GValue)).g_type;
        let mut g_vl_vtable_0: *mut GTypeValueTable = g_type_value_table_peek(g_vl_value_type_0);
        let mut g_vl_lcopy_format_0: *const gchar = (*g_vl_vtable_0).lcopy_format;
        let mut g_vl_cvalues_0: [GTypeCValue; 8] = [
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
        let mut g_vl_n_values_0: guint = 0 as guint;
        while *g_vl_lcopy_format_0 != 0 {
            let fresh12 = g_vl_n_values_0;
            g_vl_n_values_0 = g_vl_n_values_0.wrapping_add(1);
            let mut g_vl_cvalue_0: *mut GTypeCValue =
                (&raw mut g_vl_cvalues_0 as *mut GTypeCValue).offset(fresh12 as isize);
            let fresh13 = g_vl_lcopy_format_0;
            g_vl_lcopy_format_0 = g_vl_lcopy_format_0.offset(1);
            match *fresh13 as ::core::ffi::c_int {
                105 => {
                    (*g_vl_cvalue_0).v_int = var_args.arg::<gint>();
                }
                108 => {
                    (*g_vl_cvalue_0).v_long = var_args.arg::<glong>();
                }
                113 => {
                    (*g_vl_cvalue_0).v_int64 = var_args.arg::<gint64>();
                }
                100 => {
                    (*g_vl_cvalue_0).v_double = var_args.arg::<gdouble>();
                }
                112 => {
                    (*g_vl_cvalue_0).v_pointer = var_args.arg::<gpointer>();
                }
                _ => {
                    g_assertion_message_expr(
                        b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                        b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        3539 as ::core::ffi::c_int,
                        b"signal_emit_valist_unlocked\0" as *const u8 as *const ::core::ffi::c_char,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                    );
                }
            }
        }
        error_1 = (*g_vl_vtable_0)
            .lcopy_value
            .expect("non-null function pointer")(
            g_vl_value_0,
            g_vl_n_values_0,
            &raw mut g_vl_cvalues_0 as *mut GTypeCValue,
            g_vl_flags_0,
        );
        if error_1.is_null() {
            g_value_unset(&raw mut return_value);
        } else {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"%s: %s\0" as *const u8 as *const gchar,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:3544\0"
                    as *const u8 as *const ::core::ffi::c_char,
                error_1,
            );
            g_free(error_1 as gpointer);
        }
    }
    i = 0 as guint;
    while i < node_copy.n_params() {
        g_value_unset(param_values.offset(i as isize));
        i = i.wrapping_add(1);
    }
    g_value_unset(instance_and_params);
    return 0 as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_emit(
    mut instance: gpointer,
    mut signal_id: guint,
    mut detail: GQuark,
    mut args: ...
) {
    let mut var_args: ::core::ffi::VaList<'_>;
    var_args = args.clone();
    g_signal_emit_valist(instance, signal_id, detail, var_args);
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_emit_by_name(
    mut instance: gpointer,
    mut detailed_signal: *const gchar,
    mut args: ...
) {
    let mut detail: GQuark = 0 as GQuark;
    let mut signal_id: guint = 0;
    let mut itype: GType = 0;
    if g_type_check_instance(instance as *mut GTypeInstance) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_emit_by_name\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_CHECK_INSTANCE (instance)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !detailed_signal.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_emit_by_name\0" as *const u8 as *const ::core::ffi::c_char,
            b"detailed_signal != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    itype = (*(*(instance as *mut GTypeInstance)).g_class).g_type;
    g_mutex_lock(&raw mut g__g_signal_mutex_lock);
    signal_id = signal_parse_name(
        detailed_signal,
        itype,
        &raw mut detail,
        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
    );
    if signal_id != 0 {
        let mut var_args: ::core::ffi::VaList<'_>;
        var_args = args.clone();
        if signal_emit_valist_unlocked(instance, signal_id, detail, var_args) != 0 {
            g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
        }
    } else {
        g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: signal name '%s' is invalid for instance '%p' of type '%s'\0" as *const u8
                as *const gchar,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c:3633\0"
                as *const u8 as *const ::core::ffi::c_char,
            detailed_signal,
            instance,
            g_type_name(itype),
        );
    };
}
#[inline(always)]
unsafe extern "C" fn maybe_init_accumulator_unlocked(
    mut node: *mut SignalNode,
    mut emission_return: *mut GValue,
    mut accumulator_value: *mut GValue,
) -> *mut GValue {
    if !(*node).accumulator.is_null() {
        if (*accumulator_value).g_type != 0 {
            return accumulator_value;
        }
        g_value_init(
            accumulator_value,
            (*node).return_type
                & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType),
        );
        return accumulator_value;
    }
    return emission_return;
}
unsafe extern "C" fn signal_emit_unlocked_R(
    mut node: *mut SignalNode,
    mut detail: GQuark,
    mut instance: gpointer,
    mut emission_return: *mut GValue,
    mut instance_and_params: *const GValue,
) -> gboolean {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut current_block: u64;
    let mut accumulator: *mut SignalAccumulator = ::core::ptr::null_mut::<SignalAccumulator>();
    let mut emission: Emission = _Emission {
        next: ::core::ptr::null_mut::<Emission>(),
        instance: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ihint: _GSignalInvocationHint {
            signal_id: 0,
            detail: 0,
            run_type: 0 as GSignalFlags,
        },
        state: EMISSION_STOP,
        chain_type: 0,
    };
    let mut class_closure: *mut GClosure = ::core::ptr::null_mut::<GClosure>();
    let mut hlist: *mut HandlerList = ::core::ptr::null_mut::<HandlerList>();
    let mut handler_list: *mut Handler = ::core::ptr::null_mut::<Handler>();
    let mut return_accu: *mut GValue = ::core::ptr::null_mut::<GValue>();
    let mut accu: GValue = _GValue {
        g_type: 0 as GType,
        data: [
            C2RustUnnamed_0 {
                v_int: 0 as ::core::ffi::c_int,
            },
            C2RustUnnamed_0 { v_int: 0 },
        ],
    };
    let mut signal_id: guint = 0;
    let mut max_sequential_handler_number: gulong = 0;
    let mut return_value_altered: gboolean = 0 as gboolean;
    let mut n_params: guint = 0;
    signal_id = (*node).signal_id;
    n_params = ((*node).n_params() as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as guint;
    if (*node).flags() as ::core::ffi::c_int & G_SIGNAL_NO_RECURSE as ::core::ffi::c_int != 0 {
        let mut emission_node: *mut Emission = emission_find(signal_id, detail, instance);
        if !emission_node.is_null() {
            (*emission_node).state = EMISSION_RESTART;
            return return_value_altered;
        }
    }
    accumulator = (*node).accumulator;
    emission.instance = instance;
    emission.ihint.signal_id = (*node).signal_id;
    emission.ihint.detail = detail;
    emission.ihint.run_type = 0 as GSignalFlags;
    emission.state = EMISSION_STOP;
    emission.chain_type = ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
    emission_push(&raw mut emission);
    class_closure = signal_lookup_closure(node, instance as *mut GTypeInstance);
    loop {
        if !handler_list.is_null() {
            handler_unref_R(signal_id, instance, handler_list);
        }
        max_sequential_handler_number = g_handler_sequential_number;
        hlist = handler_list_lookup(signal_id, instance);
        handler_list = if !hlist.is_null() {
            (*hlist).handlers
        } else {
            ::core::ptr::null_mut::<Handler>()
        };
        if !handler_list.is_null() {
            handler_ref(handler_list);
        }
        emission.ihint.run_type = (G_SIGNAL_RUN_FIRST as ::core::ffi::c_int
            | G_SIGNAL_ACCUMULATOR_FIRST_RUN as ::core::ffi::c_int)
            as GSignalFlags;
        if (*node).flags() as ::core::ffi::c_int & G_SIGNAL_RUN_FIRST as ::core::ffi::c_int != 0
            && !class_closure.is_null()
        {
            emission.state = EMISSION_RUN;
            emission.chain_type = (*(*(instance as *mut GTypeInstance)).g_class).g_type;
            g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
            return_accu = maybe_init_accumulator_unlocked(node, emission_return, &raw mut accu);
            g_closure_invoke(
                class_closure,
                return_accu,
                n_params,
                instance_and_params,
                &raw mut emission.ihint as gpointer,
            );
            if accumulate(
                &raw mut emission.ihint,
                emission_return,
                &raw mut accu,
                accumulator,
            ) == 0
                && emission.state as ::core::ffi::c_uint
                    == EMISSION_RUN as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                emission.state = EMISSION_STOP;
            }
            g_mutex_lock(&raw mut g__g_signal_mutex_lock);
            emission.chain_type = ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
            return_value_altered = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            if emission.state as ::core::ffi::c_uint
                == EMISSION_STOP as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                current_block = 8488926859410606517;
            } else {
                if emission.state as ::core::ffi::c_uint
                    == EMISSION_RESTART as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    continue;
                }
                current_block = 5494826135382683477;
            }
        } else {
            current_block = 5494826135382683477;
        }
        match current_block {
            5494826135382683477 => {
                if !(*node).emission_hooks.is_null() {
                    let mut hook: *mut GHook = ::core::ptr::null_mut::<GHook>();
                    let mut static_emission_hooks: [*mut GHook; 3] =
                        [::core::ptr::null_mut::<GHook>(); 3];
                    let mut n_emission_hooks: size_t = 0 as size_t;
                    let may_recurse: gboolean =
                        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
                    let mut i: guint = 0;
                    emission.state = EMISSION_HOOK;
                    hook = g_hook_first_valid((*node).emission_hooks, may_recurse);
                    while !hook.is_null() {
                        let mut signal_hook: *mut SignalHook = hook as *mut SignalHook;
                        if (*signal_hook).detail == 0 || (*signal_hook).detail == detail {
                            if n_emission_hooks
                                < (::core::mem::size_of::<[*mut GHook; 3]>() as usize)
                                    .wrapping_div(::core::mem::size_of::<*mut GHook>() as usize)
                            {
                                static_emission_hooks[n_emission_hooks as usize] =
                                    g_hook_ref((*node).emission_hooks, hook);
                            }
                            n_emission_hooks = n_emission_hooks.wrapping_add(1 as size_t);
                        }
                        hook = g_hook_next_valid((*node).emission_hooks, hook, may_recurse);
                    }
                    if n_emission_hooks > 0 as size_t {
                        let mut static_hook_returns: [guint8; 3] = [0; 3];
                        let mut emission_hooks: *mut *mut GHook =
                            ::core::ptr::null_mut::<*mut GHook>();
                        let mut hook_returns: *mut guint8 = ::core::ptr::null_mut::<guint8>();
                        if n_emission_hooks
                            <= (::core::mem::size_of::<[*mut GHook; 3]>() as usize)
                                .wrapping_div(::core::mem::size_of::<*mut GHook>() as usize)
                        {
                            emission_hooks = &raw mut static_emission_hooks as *mut *mut GHook;
                            hook_returns = &raw mut static_hook_returns as *mut guint8;
                        } else {
                            alloca_allocations.push(::std::vec::from_elem(
                                0,
                                (::core::mem::size_of::<*mut GHook>() as usize)
                                    .wrapping_mul(n_emission_hooks)
                                    as usize,
                            ));
                            emission_hooks = alloca_allocations.last_mut().unwrap().as_mut_ptr()
                                as *mut *mut GHook;
                            alloca_allocations.push(::std::vec::from_elem(
                                0,
                                (::core::mem::size_of::<guint8>() as usize)
                                    .wrapping_mul(n_emission_hooks)
                                    as usize,
                            ));
                            hook_returns =
                                alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut guint8;
                            i = 0 as guint;
                            hook = g_hook_first_valid((*node).emission_hooks, may_recurse);
                            while !hook.is_null() {
                                let mut signal_hook_0: *mut SignalHook = hook as *mut SignalHook;
                                if (*signal_hook_0).detail == 0 || (*signal_hook_0).detail == detail
                                {
                                    if (i as usize)
                                        < (::core::mem::size_of::<[*mut GHook; 3]>() as usize)
                                            .wrapping_div(
                                                ::core::mem::size_of::<*mut GHook>() as usize
                                            )
                                    {
                                        let ref mut fresh3 = *emission_hooks.offset(i as isize);
                                        *fresh3 = g_steal_pointer(
                                            (&raw mut static_emission_hooks as *mut *mut GHook)
                                                .offset(i as isize)
                                                as *mut *mut GHook
                                                as gpointer,
                                        )
                                            as *mut GHook
                                            as *mut GHook;
                                        if *emission_hooks.offset(i as isize) == hook {
                                        } else {
                                            g_assertion_message_expr(
                                                b"GLib-GObject\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c\0"
                                                    as *const u8 as *const ::core::ffi::c_char,
                                                3811 as ::core::ffi::c_int,
                                                b"signal_emit_unlocked_R\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                                b"emission_hooks[i] == hook\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                            );
                                        }
                                    } else {
                                        let ref mut fresh4 = *emission_hooks.offset(i as isize);
                                        *fresh4 = g_hook_ref((*node).emission_hooks, hook);
                                    }
                                    i = i.wrapping_add(1 as guint);
                                }
                                hook = g_hook_next_valid((*node).emission_hooks, hook, may_recurse);
                            }
                            if i as size_t == n_emission_hooks {
                            } else {
                                g_assertion_message_expr(
                                    b"GLib-GObject\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    3822 as ::core::ffi::c_int,
                                    b"signal_emit_unlocked_R\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    b"i == n_emission_hooks\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                            }
                        }
                        g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
                        i = 0 as guint;
                        while (i as size_t) < n_emission_hooks {
                            let mut hook_func: GSignalEmissionHook = None;
                            let mut need_destroy: gboolean = 0;
                            let mut old_flags: guint = 0;
                            hook = *emission_hooks.offset(i as isize);
                            hook_func = ::core::mem::transmute::<gpointer, GSignalEmissionHook>(
                                (*hook).func,
                            );
                            old_flags = ({
                                if 0 as ::core::ffi::c_int != 0 {
                                    (*hook).flags;
                                } else {
                                };
                                crate::translated::compat::atomic_or_seqcst(
                                    &raw mut (*hook).flags,
                                    G_HOOK_FLAG_IN_CALL as ::core::ffi::c_int as guint,
                                )
                            });
                            need_destroy = (hook_func.expect("non-null function pointer")(
                                &raw mut emission.ihint,
                                n_params,
                                instance_and_params,
                                (*hook).data,
                            ) == 0) as ::core::ffi::c_int
                                as gboolean;
                            if old_flags & G_HOOK_FLAG_IN_CALL as ::core::ffi::c_int as guint == 0 {
                                let mut gaicae_oldval: gint = (old_flags
                                    | G_HOOK_FLAG_IN_CALL as ::core::ffi::c_int as guint)
                                    as gint;
                                if 0 as ::core::ffi::c_int != 0 {
                                    (*hook).flags;
                                } else {
                                };
                                let fresh5 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                                    &raw mut (*hook).flags,
                                    *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void
                                        as *mut guint),
                                    old_flags,
                                );
                                *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void
                                    as *mut guint) = fresh5.0;
                                if fresh5.1 as ::core::ffi::c_int != 0 {
                                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
                                } else {
                                };
                            }
                            *hook_returns.offset(i as isize) =
                                (need_destroy != 0) as ::core::ffi::c_int as guint8;
                            i = i.wrapping_add(1);
                        }
                        g_mutex_lock(&raw mut g__g_signal_mutex_lock);
                        i = 0 as guint;
                        while (i as size_t) < n_emission_hooks {
                            hook = *emission_hooks.offset(i as isize);
                            g_hook_unref((*node).emission_hooks, hook);
                            if *hook_returns.offset(i as isize) != 0 {
                                g_hook_destroy_link((*node).emission_hooks, hook);
                            }
                            i = i.wrapping_add(1);
                        }
                    }
                    if emission.state as ::core::ffi::c_uint
                        == EMISSION_RESTART as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        continue;
                    }
                }
                if !handler_list.is_null() {
                    let mut handler: *mut Handler = handler_list;
                    emission.state = EMISSION_RUN;
                    handler_ref(handler);
                    loop {
                        let mut tmp: *mut Handler = ::core::ptr::null_mut::<Handler>();
                        if (*handler).after() != 0 {
                            handler_unref_R(signal_id, instance, handler_list);
                            handler_list = handler;
                            break;
                        } else {
                            if (*handler).block_count() == 0
                                && ((*handler).detail == 0 || (*handler).detail == detail)
                                && (*handler).sequential_number < max_sequential_handler_number
                            {
                                g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
                                return_accu = maybe_init_accumulator_unlocked(
                                    node,
                                    emission_return,
                                    &raw mut accu,
                                );
                                g_closure_invoke(
                                    (*handler).closure,
                                    return_accu,
                                    n_params,
                                    instance_and_params,
                                    &raw mut emission.ihint as gpointer,
                                );
                                if accumulate(
                                    &raw mut emission.ihint,
                                    emission_return,
                                    &raw mut accu,
                                    accumulator,
                                ) == 0
                                    && emission.state as ::core::ffi::c_uint
                                        == EMISSION_RUN as ::core::ffi::c_int as ::core::ffi::c_uint
                                {
                                    emission.state = EMISSION_STOP;
                                }
                                g_mutex_lock(&raw mut g__g_signal_mutex_lock);
                                return_value_altered = (0 as ::core::ffi::c_int == 0)
                                    as ::core::ffi::c_int
                                    as gboolean;
                                tmp = if emission.state as ::core::ffi::c_uint
                                    == EMISSION_RUN as ::core::ffi::c_int as ::core::ffi::c_uint
                                {
                                    (*handler).next
                                } else {
                                    ::core::ptr::null_mut::<Handler>()
                                };
                            } else {
                                tmp = (*handler).next;
                            }
                            if !tmp.is_null() {
                                handler_ref(tmp);
                            }
                            handler_unref_R(signal_id, instance, handler_list);
                            handler_list = handler;
                            handler = tmp;
                            if handler.is_null() {
                                break;
                            }
                        }
                    }
                    if emission.state as ::core::ffi::c_uint
                        == EMISSION_STOP as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        current_block = 8488926859410606517;
                    } else {
                        if emission.state as ::core::ffi::c_uint
                            == EMISSION_RESTART as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            continue;
                        }
                        current_block = 2522825242109451841;
                    }
                } else {
                    current_block = 2522825242109451841;
                }
                match current_block {
                    8488926859410606517 => {}
                    _ => {
                        emission.ihint.run_type =
                            ::core::mem::transmute::<::core::ffi::c_uint, GSignalFlags>(
                                emission.ihint.run_type as ::core::ffi::c_uint
                                    & !(G_SIGNAL_RUN_FIRST as ::core::ffi::c_int)
                                        as ::core::ffi::c_uint,
                            );
                        emission.ihint.run_type =
                            ::core::mem::transmute::<::core::ffi::c_uint, GSignalFlags>(
                                emission.ihint.run_type as ::core::ffi::c_uint
                                    | G_SIGNAL_RUN_LAST as ::core::ffi::c_int
                                        as ::core::ffi::c_uint,
                            );
                        if (*node).flags() as ::core::ffi::c_int
                            & G_SIGNAL_RUN_LAST as ::core::ffi::c_int
                            != 0
                            && !class_closure.is_null()
                        {
                            emission.state = EMISSION_RUN;
                            emission.chain_type =
                                (*(*(instance as *mut GTypeInstance)).g_class).g_type;
                            g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
                            return_accu = maybe_init_accumulator_unlocked(
                                node,
                                emission_return,
                                &raw mut accu,
                            );
                            g_closure_invoke(
                                class_closure,
                                return_accu,
                                n_params,
                                instance_and_params,
                                &raw mut emission.ihint as gpointer,
                            );
                            if accumulate(
                                &raw mut emission.ihint,
                                emission_return,
                                &raw mut accu,
                                accumulator,
                            ) == 0
                                && emission.state as ::core::ffi::c_uint
                                    == EMISSION_RUN as ::core::ffi::c_int as ::core::ffi::c_uint
                            {
                                emission.state = EMISSION_STOP;
                            }
                            g_mutex_lock(&raw mut g__g_signal_mutex_lock);
                            emission.chain_type =
                                ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
                            return_value_altered =
                                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
                            if emission.state as ::core::ffi::c_uint
                                == EMISSION_STOP as ::core::ffi::c_int as ::core::ffi::c_uint
                            {
                                current_block = 8488926859410606517;
                            } else {
                                if emission.state as ::core::ffi::c_uint
                                    == EMISSION_RESTART as ::core::ffi::c_int as ::core::ffi::c_uint
                                {
                                    continue;
                                }
                                current_block = 7728257318064351663;
                            }
                        } else {
                            current_block = 7728257318064351663;
                        }
                        match current_block {
                            8488926859410606517 => {}
                            _ => {
                                if !handler_list.is_null() {
                                    let mut handler_0: *mut Handler = handler_list;
                                    emission.state = EMISSION_RUN;
                                    handler_ref(handler_0);
                                    loop {
                                        let mut tmp_0: *mut Handler =
                                            ::core::ptr::null_mut::<Handler>();
                                        if (*handler_0).after() as ::core::ffi::c_int != 0
                                            && (*handler_0).block_count() == 0
                                            && ((*handler_0).detail == 0
                                                || (*handler_0).detail == detail)
                                            && (*handler_0).sequential_number
                                                < max_sequential_handler_number
                                        {
                                            g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
                                            return_accu = maybe_init_accumulator_unlocked(
                                                node,
                                                emission_return,
                                                &raw mut accu,
                                            );
                                            g_closure_invoke(
                                                (*handler_0).closure,
                                                return_accu,
                                                n_params,
                                                instance_and_params,
                                                &raw mut emission.ihint as gpointer,
                                            );
                                            if accumulate(
                                                &raw mut emission.ihint,
                                                emission_return,
                                                &raw mut accu,
                                                accumulator,
                                            ) == 0
                                                && emission.state as ::core::ffi::c_uint
                                                    == EMISSION_RUN as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint
                                            {
                                                emission.state = EMISSION_STOP;
                                            }
                                            g_mutex_lock(&raw mut g__g_signal_mutex_lock);
                                            return_value_altered = (0 as ::core::ffi::c_int == 0)
                                                as ::core::ffi::c_int
                                                as gboolean;
                                            tmp_0 = if emission.state as ::core::ffi::c_uint
                                                == EMISSION_RUN as ::core::ffi::c_int
                                                    as ::core::ffi::c_uint
                                            {
                                                (*handler_0).next
                                            } else {
                                                ::core::ptr::null_mut::<Handler>()
                                            };
                                        } else {
                                            tmp_0 = (*handler_0).next;
                                        }
                                        if !tmp_0.is_null() {
                                            handler_ref(tmp_0);
                                        }
                                        handler_unref_R(signal_id, instance, handler_0);
                                        handler_0 = tmp_0;
                                        if handler_0.is_null() {
                                            break;
                                        }
                                    }
                                    if !(emission.state as ::core::ffi::c_uint
                                        == EMISSION_STOP as ::core::ffi::c_int
                                            as ::core::ffi::c_uint)
                                    {
                                        if emission.state as ::core::ffi::c_uint
                                            == EMISSION_RESTART as ::core::ffi::c_int
                                                as ::core::ffi::c_uint
                                        {
                                            continue;
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
        emission.ihint.run_type = ::core::mem::transmute::<::core::ffi::c_uint, GSignalFlags>(
            emission.ihint.run_type as ::core::ffi::c_uint
                & !(G_SIGNAL_RUN_LAST as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
        emission.ihint.run_type = ::core::mem::transmute::<::core::ffi::c_uint, GSignalFlags>(
            emission.ihint.run_type as ::core::ffi::c_uint
                | G_SIGNAL_RUN_CLEANUP as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        if !((*node).flags() as ::core::ffi::c_int & G_SIGNAL_RUN_CLEANUP as ::core::ffi::c_int
            != 0
            && !class_closure.is_null())
        {
            break;
        }
        let mut need_unset: gboolean = 0 as gboolean;
        emission.state = EMISSION_STOP;
        emission.chain_type = (*(*(instance as *mut GTypeInstance)).g_class).g_type;
        g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
        if (*node).return_type != ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
            && accumulator.is_null()
        {
            g_value_init(
                &raw mut accu,
                (*node).return_type
                    & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType),
            );
            need_unset = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        }
        g_closure_invoke(
            class_closure,
            if (*node).return_type
                != ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
            {
                &raw mut accu
            } else {
                ::core::ptr::null_mut::<GValue>()
            },
            n_params,
            instance_and_params,
            &raw mut emission.ihint as gpointer,
        );
        if accumulate(
            &raw mut emission.ihint,
            emission_return,
            &raw mut accu,
            accumulator,
        ) == 0
            && emission.state as ::core::ffi::c_uint
                == EMISSION_RUN as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            emission.state = EMISSION_STOP;
        }
        if need_unset != 0 {
            g_value_unset(&raw mut accu);
        }
        g_mutex_lock(&raw mut g__g_signal_mutex_lock);
        return_value_altered = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        emission.chain_type = ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        if !(emission.state as ::core::ffi::c_uint
            == EMISSION_RESTART as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            break;
        }
    }
    if !handler_list.is_null() {
        handler_unref_R(signal_id, instance, handler_list);
    }
    emission_pop(&raw mut emission);
    if !accumulator.is_null() {
        g_value_unset(&raw mut accu);
    }
    return return_value_altered;
}
unsafe extern "C" fn add_invalid_closure_notify(mut handler: *mut Handler, mut instance: gpointer) {
    g_closure_add_invalidate_notifier(
        (*handler).closure,
        instance,
        Some(invalid_closure_notify as unsafe extern "C" fn(gpointer, *mut GClosure) -> ()),
    );
    (*handler).set_has_invalid_closure_notify(1 as guint as guint);
}
unsafe extern "C" fn remove_invalid_closure_notify(
    mut handler: *mut Handler,
    mut instance: gpointer,
) {
    if (*handler).has_invalid_closure_notify() != 0 {
        g_closure_remove_invalidate_notifier(
            (*handler).closure,
            instance,
            Some(invalid_closure_notify as unsafe extern "C" fn(gpointer, *mut GClosure) -> ()),
        );
        (*handler).set_has_invalid_closure_notify(0 as guint as guint);
    }
}
unsafe extern "C" fn invalid_closure_notify(mut instance: gpointer, mut closure: *mut GClosure) {
    let mut handler: *mut Handler = ::core::ptr::null_mut::<Handler>();
    let mut signal_id: guint = 0;
    g_mutex_lock(&raw mut g__g_signal_mutex_lock);
    handler = handler_lookup(instance, 0 as gulong, closure, &raw mut signal_id);
    if !handler.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c\0" as *const u8
                as *const ::core::ffi::c_char,
            4069 as ::core::ffi::c_int,
            b"invalid_closure_notify\0" as *const u8 as *const ::core::ffi::c_char,
            b"handler != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*handler).closure == closure {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignal.c\0" as *const u8
                as *const ::core::ffi::c_char,
            4070 as ::core::ffi::c_int,
            b"invalid_closure_notify\0" as *const u8 as *const ::core::ffi::c_char,
            b"handler->closure == closure\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_hash_table_remove(g_handlers, handler as gconstpointer);
    (*handler).sequential_number = 0 as gulong;
    (*handler).set_block_count(1 as guint as guint);
    handler_unref_R(signal_id, instance, handler);
    g_mutex_unlock(&raw mut g__g_signal_mutex_lock);
}
unsafe extern "C" fn type_debug_name(mut type_0: GType) -> *const gchar {
    if type_0 != 0 {
        let mut name: *const ::core::ffi::c_char = g_type_name(
            type_0 & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType),
        ) as *const ::core::ffi::c_char;
        return if !name.is_null() {
            name as *const gchar
        } else {
            b"<unknown>\0" as *const u8 as *const gchar
        };
    } else {
        return b"<invalid>\0" as *const u8 as *const gchar;
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_accumulator_true_handled(
    mut ihint: *mut GSignalInvocationHint,
    mut return_accu: *mut GValue,
    mut handler_return: *const GValue,
    mut dummy: gpointer,
) -> gboolean {
    let mut continue_emission: gboolean = 0;
    let mut signal_handled: gboolean = 0;
    signal_handled = g_value_get_boolean(handler_return);
    g_value_set_boolean(return_accu, signal_handled);
    continue_emission = (signal_handled == 0) as ::core::ffi::c_int as gboolean;
    return continue_emission;
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_accumulator_first_wins(
    mut ihint: *mut GSignalInvocationHint,
    mut return_accu: *mut GValue,
    mut handler_return: *const GValue,
    mut dummy: gpointer,
) -> gboolean {
    g_value_copy(handler_return, return_accu);
    return 0 as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn g_clear_signal_handler(
    mut handler_id_ptr: *mut gulong,
    mut instance: gpointer,
) {
    if !handler_id_ptr.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_clear_signal_handler\0" as *const u8 as *const ::core::ffi::c_char,
            b"handler_id_ptr != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    let _instance: gpointer = instance;
    let _handler_id_ptr: *mut gulong = handler_id_ptr;
    let _handler_id: gulong = *_handler_id_ptr;
    if _handler_id > 0 as gulong {
        *_handler_id_ptr = 0 as gulong;
        g_signal_handler_disconnect(_instance, _handler_id);
    }
}
