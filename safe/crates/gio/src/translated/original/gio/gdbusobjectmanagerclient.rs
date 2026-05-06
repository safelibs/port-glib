use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GHashTable;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GAsyncResult;
    pub type _GAsyncInitable;
    pub type _GCancellablePrivate;
    pub type _GInitable;
    pub type _GDBusConnection;
    pub type _GDBusProxyPrivate;
    pub type _GDBusInterface;
    pub type _GDBusObject;
    pub type _GDBusObjectProxyPrivate;
    pub type _GDBusObjectManager;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_ptr_array_unref(array: *mut GPtrArray);
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_error_matches(error: *const GError, domain: GQuark, code: gint) -> gboolean;
    fn g_clear_error(err: *mut *mut GError);
    fn g_mutex_init(mutex: *mut GMutex);
    fn g_mutex_clear(mutex: *mut GMutex);
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_list_free(list: *mut GList);
    fn g_list_free_full(list: *mut GList, free_func: GDestroyNotify);
    fn g_list_append(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_length(list: *mut GList) -> guint;
    fn g_list_foreach(list: *mut GList, func: GFunc, user_data: gpointer);
    fn g_hash_table_new_full(
        hash_func: GHashFunc,
        key_equal_func: GEqualFunc,
        key_destroy_func: GDestroyNotify,
        value_destroy_func: GDestroyNotify,
    ) -> *mut GHashTable;
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_remove(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_steal_all_values(hash_table: *mut GHashTable) -> *mut GPtrArray;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_hash_table_get_values(hash_table: *mut GHashTable) -> *mut GList;
    fn g_hash_table_unref(hash_table: *mut GHashTable);
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_strv_length(str_array: *mut *mut gchar) -> guint;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_is_object_path(string: *const gchar) -> gboolean;
    fn g_variant_get_child_value(value: *mut GVariant, index_: gsize) -> *mut GVariant;
    fn g_variant_iter_init(iter: *mut GVariantIter, value: *mut GVariant) -> gsize;
    fn g_variant_iter_next(iter: *mut GVariantIter, format_string: *const gchar, ...) -> gboolean;
    fn g_variant_new(format_string: *const gchar, ...) -> *mut GVariant;
    fn g_variant_get(value: *mut GVariant, format_string: *const gchar, ...);
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
    fn g_type_name(type_0: GType) -> *const gchar;
    fn g_type_is_a(type_0: GType, is_a_type: GType) -> gboolean;
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
    fn g_type_add_interface_static(
        instance_type: GType,
        interface_type: GType,
        info: *const GInterfaceInfo,
    );
    fn g_type_add_instance_private(class_type: GType, private_size: gsize) -> gint;
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
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
    fn g_signal_set_va_marshaller(
        signal_id: guint,
        instance_type: GType,
        va_marshaller: GSignalCVaMarshaller,
    );
    fn g_signal_emit(instance: gpointer, signal_id: guint, detail: GQuark, ...);
    fn g_signal_emit_by_name(instance: gpointer, detailed_signal: *const gchar, ...);
    fn g_signal_connect_data(
        instance: gpointer,
        detailed_signal: *const gchar,
        c_handler: GCallback,
        data: gpointer,
        destroy_data: GClosureNotify,
        connect_flags: GConnectFlags,
    ) -> gulong;
    fn g_signal_handler_disconnect(instance: gpointer, handler_id: gulong);
    fn g_strv_get_type() -> GType;
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_value_set_object(value: *mut GValue, v_object: gpointer);
    fn g_value_get_object(value: *const GValue) -> gpointer;
    fn g_value_dup_object(value: *const GValue) -> gpointer;
    fn g_weak_ref_init(weak_ref: *mut GWeakRef, object: gpointer);
    fn g_weak_ref_clear(weak_ref: *mut GWeakRef);
    fn g_weak_ref_get(weak_ref: *mut GWeakRef) -> gpointer;
    fn g_value_get_enum(value: *const GValue) -> gint;
    fn g_value_set_flags(value: *mut GValue, v_flags: guint);
    fn g_value_get_flags(value: *const GValue) -> guint;
    fn g_param_spec_enum(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        enum_type: GType,
        default_value: gint,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_flags(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        flags_type: GType,
        default_value: guint,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_string(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: *const gchar,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_pointer(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_object(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        object_type: GType,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_value_set_string(value: *mut GValue, v_string: *const gchar);
    fn g_value_get_string(value: *const GValue) -> *const gchar;
    fn g_value_dup_string(value: *const GValue) -> *mut gchar;
    fn g_value_get_pointer(value: *const GValue) -> gpointer;
    fn g_value_take_string(value: *mut GValue, v_string: *mut gchar);
    fn g_cancellable_new() -> *mut GCancellable;
    fn g_cancellable_cancel(cancellable: *mut GCancellable);
    fn g_dbus_object_manager_get_type() -> GType;
    fn g_dbus_object_manager_get_object_path(manager: *mut GDBusObjectManager) -> *const gchar;
    fn g_dbus_object_manager_get_object(
        manager: *mut GDBusObjectManager,
        object_path: *const gchar,
    ) -> *mut GDBusObject;
    fn g_dbus_object_get_interfaces(object: *mut GDBusObject) -> *mut GList;
    fn g_dbus_object_get_interface(
        object: *mut GDBusObject,
        interface_name: *const gchar,
    ) -> *mut GDBusInterface;
    fn _g_dbus_object_proxy_add_interface(
        proxy: *mut GDBusObjectProxy,
        interface_proxy: *mut GDBusProxy,
    );
    fn _g_dbus_object_proxy_remove_interface(
        proxy: *mut GDBusObjectProxy,
        interface_name: *const gchar,
    );
    fn g_bus_type_get_type() -> GType;
    fn g_dbus_object_manager_client_flags_get_type() -> GType;
    fn g_io_error_quark() -> GQuark;
    fn g_initable_get_type() -> GType;
    fn g_initable_new(
        object_type: GType,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
        first_property_name: *const gchar,
        ...
    ) -> gpointer;
    fn g_async_result_get_source_object(res: *mut GAsyncResult) -> *mut GObject;
    fn g_async_initable_get_type() -> GType;
    fn g_async_initable_new_async(
        object_type: GType,
        io_priority: ::core::ffi::c_int,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
        first_property_name: *const gchar,
        ...
    );
    fn g_async_initable_new_finish(
        initable: *mut GAsyncInitable,
        res: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> *mut GObject;
    fn g_dbus_connection_get_type() -> GType;
    fn g_bus_get_sync(
        bus_type: GBusType,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GDBusConnection;
    fn g_dbus_connection_get_unique_name(connection: *mut GDBusConnection) -> *const gchar;
    fn g_dbus_connection_call(
        connection: *mut GDBusConnection,
        bus_name: *const gchar,
        object_path: *const gchar,
        interface_name: *const gchar,
        method_name: *const gchar,
        parameters: *mut GVariant,
        reply_type: *const GVariantType,
        flags: GDBusCallFlags,
        timeout_msec: gint,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_dbus_connection_call_sync(
        connection: *mut GDBusConnection,
        bus_name: *const gchar,
        object_path: *const gchar,
        interface_name: *const gchar,
        method_name: *const gchar,
        parameters: *mut GVariant,
        reply_type: *const GVariantType,
        flags: GDBusCallFlags,
        timeout_msec: gint,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GVariant;
    fn g_dbus_connection_signal_subscribe(
        connection: *mut GDBusConnection,
        sender: *const gchar,
        interface_name: *const gchar,
        member: *const gchar,
        object_path: *const gchar,
        arg0: *const gchar,
        flags: GDBusSignalFlags,
        callback: GDBusSignalCallback,
        user_data: gpointer,
        user_data_free_func: GDestroyNotify,
    ) -> guint;
    fn g_dbus_connection_signal_unsubscribe(
        connection: *mut GDBusConnection,
        subscription_id: guint,
    );
    fn g_dbus_is_name(string: *const gchar) -> gboolean;
    fn g_dbus_is_unique_name(string: *const gchar) -> gboolean;
    fn g_dbus_object_proxy_get_type() -> GType;
    fn g_dbus_proxy_get_type() -> GType;
    fn g_dbus_proxy_new_sync(
        connection: *mut GDBusConnection,
        flags: GDBusProxyFlags,
        info: *mut GDBusInterfaceInfo,
        name: *const gchar,
        object_path: *const gchar,
        interface_name: *const gchar,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GDBusProxy;
    fn g_dbus_proxy_get_name_owner(proxy: *mut GDBusProxy) -> *mut gchar;
    fn g_dbus_proxy_set_cached_property(
        proxy: *mut GDBusProxy,
        property_name: *const gchar,
        value: *mut GVariant,
    );
    fn g_dbus_proxy_call(
        proxy: *mut GDBusProxy,
        method_name: *const gchar,
        parameters: *mut GVariant,
        flags: GDBusCallFlags,
        timeout_msec: gint,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_dbus_proxy_call_finish(
        proxy: *mut GDBusProxy,
        res: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> *mut GVariant;
    fn g_dbus_proxy_call_sync(
        proxy: *mut GDBusProxy,
        method_name: *const gchar,
        parameters: *mut GVariant,
        flags: GDBusCallFlags,
        timeout_msec: gint,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GVariant;
    fn g_dbus_interface_set_object(interface_: *mut GDBusInterface, object: *mut GDBusObject);
    fn _g_cclosure_marshal_VOID__OBJECT_OBJECT_STRING_STRING_VARIANT(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn _g_cclosure_marshal_VOID__OBJECT_OBJECT_STRING_STRING_VARIANTv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
    fn _g_cclosure_marshal_VOID__OBJECT_OBJECT_VARIANT_BOXED(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn _g_cclosure_marshal_VOID__OBJECT_OBJECT_VARIANT_BOXEDv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
}
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
pub type GVariantType = _GVariantType;
pub type GVariant = _GVariant;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVariantIter {
    pub x: [guintptr; 16],
}
pub type GVariantIter = _GVariantIter;
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
pub type GInstanceInitFunc = Option<unsafe extern "C" fn(*mut GTypeInstance, gpointer) -> ()>;
pub type GClassInitFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInterfaceInfo {
    pub interface_init: GInterfaceInitFunc,
    pub interface_finalize: GInterfaceFinalizeFunc,
    pub interface_data: gpointer,
}
pub type GInterfaceFinalizeFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
pub type GInterfaceInitFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
pub type GInterfaceInfo = _GInterfaceInfo;
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
pub type GSignalCVaMarshaller = GVaClosureMarshal;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GWeakRef {
    pub priv_0: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub p: gpointer,
}
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const G_IO_ERROR_DESTINATION_UNSET: C2RustUnnamed_1 = 48;
pub const G_IO_ERROR_NO_SUCH_DEVICE: C2RustUnnamed_1 = 47;
pub const G_IO_ERROR_MESSAGE_TOO_LARGE: C2RustUnnamed_1 = 46;
pub const G_IO_ERROR_NOT_CONNECTED: C2RustUnnamed_1 = 45;
pub const G_IO_ERROR_CONNECTION_CLOSED: C2RustUnnamed_1 = 44;
pub const G_IO_ERROR_BROKEN_PIPE: C2RustUnnamed_1 = 44;
pub const G_IO_ERROR_PROXY_NOT_ALLOWED: C2RustUnnamed_1 = 43;
pub const G_IO_ERROR_PROXY_NEED_AUTH: C2RustUnnamed_1 = 42;
pub const G_IO_ERROR_PROXY_AUTH_FAILED: C2RustUnnamed_1 = 41;
pub const G_IO_ERROR_PROXY_FAILED: C2RustUnnamed_1 = 40;
pub const G_IO_ERROR_CONNECTION_REFUSED: C2RustUnnamed_1 = 39;
pub const G_IO_ERROR_NETWORK_UNREACHABLE: C2RustUnnamed_1 = 38;
pub const G_IO_ERROR_HOST_UNREACHABLE: C2RustUnnamed_1 = 37;
pub const G_IO_ERROR_DBUS_ERROR: C2RustUnnamed_1 = 36;
pub const G_IO_ERROR_INVALID_DATA: C2RustUnnamed_1 = 35;
pub const G_IO_ERROR_PARTIAL_INPUT: C2RustUnnamed_1 = 34;
pub const G_IO_ERROR_ADDRESS_IN_USE: C2RustUnnamed_1 = 33;
pub const G_IO_ERROR_NOT_INITIALIZED: C2RustUnnamed_1 = 32;
pub const G_IO_ERROR_TOO_MANY_OPEN_FILES: C2RustUnnamed_1 = 31;
pub const G_IO_ERROR_FAILED_HANDLED: C2RustUnnamed_1 = 30;
pub const G_IO_ERROR_WOULD_MERGE: C2RustUnnamed_1 = 29;
pub const G_IO_ERROR_HOST_NOT_FOUND: C2RustUnnamed_1 = 28;
pub const G_IO_ERROR_WOULD_BLOCK: C2RustUnnamed_1 = 27;
pub const G_IO_ERROR_BUSY: C2RustUnnamed_1 = 26;
pub const G_IO_ERROR_WOULD_RECURSE: C2RustUnnamed_1 = 25;
pub const G_IO_ERROR_TIMED_OUT: C2RustUnnamed_1 = 24;
pub const G_IO_ERROR_WRONG_ETAG: C2RustUnnamed_1 = 23;
pub const G_IO_ERROR_CANT_CREATE_BACKUP: C2RustUnnamed_1 = 22;
pub const G_IO_ERROR_READ_ONLY: C2RustUnnamed_1 = 21;
pub const G_IO_ERROR_PENDING: C2RustUnnamed_1 = 20;
pub const G_IO_ERROR_CANCELLED: C2RustUnnamed_1 = 19;
pub const G_IO_ERROR_CLOSED: C2RustUnnamed_1 = 18;
pub const G_IO_ERROR_ALREADY_MOUNTED: C2RustUnnamed_1 = 17;
pub const G_IO_ERROR_NOT_MOUNTED: C2RustUnnamed_1 = 16;
pub const G_IO_ERROR_NOT_SUPPORTED: C2RustUnnamed_1 = 15;
pub const G_IO_ERROR_PERMISSION_DENIED: C2RustUnnamed_1 = 14;
pub const G_IO_ERROR_INVALID_ARGUMENT: C2RustUnnamed_1 = 13;
pub const G_IO_ERROR_NO_SPACE: C2RustUnnamed_1 = 12;
pub const G_IO_ERROR_TOO_MANY_LINKS: C2RustUnnamed_1 = 11;
pub const G_IO_ERROR_INVALID_FILENAME: C2RustUnnamed_1 = 10;
pub const G_IO_ERROR_FILENAME_TOO_LONG: C2RustUnnamed_1 = 9;
pub const G_IO_ERROR_NOT_MOUNTABLE_FILE: C2RustUnnamed_1 = 8;
pub const G_IO_ERROR_NOT_SYMBOLIC_LINK: C2RustUnnamed_1 = 7;
pub const G_IO_ERROR_NOT_REGULAR_FILE: C2RustUnnamed_1 = 6;
pub const G_IO_ERROR_NOT_EMPTY: C2RustUnnamed_1 = 5;
pub const G_IO_ERROR_NOT_DIRECTORY: C2RustUnnamed_1 = 4;
pub const G_IO_ERROR_IS_DIRECTORY: C2RustUnnamed_1 = 3;
pub const G_IO_ERROR_EXISTS: C2RustUnnamed_1 = 2;
pub const G_IO_ERROR_NOT_FOUND: C2RustUnnamed_1 = 1;
pub const G_IO_ERROR_FAILED: C2RustUnnamed_1 = 0;
pub type GBusType = ::core::ffi::c_int;
pub const G_BUS_TYPE_SESSION: GBusType = 2;
pub const G_BUS_TYPE_SYSTEM: GBusType = 1;
pub const G_BUS_TYPE_NONE: GBusType = 0;
pub const G_BUS_TYPE_STARTER: GBusType = -1;
pub type GDBusProxyFlags = ::core::ffi::c_uint;
pub const G_DBUS_PROXY_FLAGS_NO_MATCH_RULE: GDBusProxyFlags = 32;
pub const G_DBUS_PROXY_FLAGS_DO_NOT_AUTO_START_AT_CONSTRUCTION: GDBusProxyFlags = 16;
pub const G_DBUS_PROXY_FLAGS_GET_INVALIDATED_PROPERTIES: GDBusProxyFlags = 8;
pub const G_DBUS_PROXY_FLAGS_DO_NOT_AUTO_START: GDBusProxyFlags = 4;
pub const G_DBUS_PROXY_FLAGS_DO_NOT_CONNECT_SIGNALS: GDBusProxyFlags = 2;
pub const G_DBUS_PROXY_FLAGS_DO_NOT_LOAD_PROPERTIES: GDBusProxyFlags = 1;
pub const G_DBUS_PROXY_FLAGS_NONE: GDBusProxyFlags = 0;
pub type GDBusCallFlags = ::core::ffi::c_uint;
pub const G_DBUS_CALL_FLAGS_ALLOW_INTERACTIVE_AUTHORIZATION: GDBusCallFlags = 2;
pub const G_DBUS_CALL_FLAGS_NO_AUTO_START: GDBusCallFlags = 1;
pub const G_DBUS_CALL_FLAGS_NONE: GDBusCallFlags = 0;
pub type GDBusPropertyInfoFlags = ::core::ffi::c_uint;
pub const G_DBUS_PROPERTY_INFO_FLAGS_WRITABLE: GDBusPropertyInfoFlags = 2;
pub const G_DBUS_PROPERTY_INFO_FLAGS_READABLE: GDBusPropertyInfoFlags = 1;
pub const G_DBUS_PROPERTY_INFO_FLAGS_NONE: GDBusPropertyInfoFlags = 0;
pub type GDBusSignalFlags = ::core::ffi::c_uint;
pub const G_DBUS_SIGNAL_FLAGS_MATCH_ARG0_PATH: GDBusSignalFlags = 4;
pub const G_DBUS_SIGNAL_FLAGS_MATCH_ARG0_NAMESPACE: GDBusSignalFlags = 2;
pub const G_DBUS_SIGNAL_FLAGS_NO_MATCH_RULE: GDBusSignalFlags = 1;
pub const G_DBUS_SIGNAL_FLAGS_NONE: GDBusSignalFlags = 0;
pub type GDBusObjectManagerClientFlags = ::core::ffi::c_uint;
pub const G_DBUS_OBJECT_MANAGER_CLIENT_FLAGS_DO_NOT_AUTO_START: GDBusObjectManagerClientFlags = 1;
pub const G_DBUS_OBJECT_MANAGER_CLIENT_FLAGS_NONE: GDBusObjectManagerClientFlags = 0;
pub type GAsyncResult = _GAsyncResult;
pub type GAsyncInitable = _GAsyncInitable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
pub type GInitable = _GInitable;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
pub type GDBusConnection = _GDBusConnection;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusProxy {
    pub parent_instance: GObject,
    pub priv_0: *mut GDBusProxyPrivate,
}
pub type GDBusProxyPrivate = _GDBusProxyPrivate;
pub type GDBusProxy = _GDBusProxy;
pub type GDBusInterfaceInfo = _GDBusInterfaceInfo;
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
pub type GDBusAnnotationInfo = _GDBusAnnotationInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusAnnotationInfo {
    pub ref_count: gint,
    pub key: *mut gchar,
    pub value: *mut gchar,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusPropertyInfo = _GDBusPropertyInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusPropertyInfo {
    pub ref_count: gint,
    pub name: *mut gchar,
    pub signature: *mut gchar,
    pub flags: GDBusPropertyInfoFlags,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusSignalInfo = _GDBusSignalInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusSignalInfo {
    pub ref_count: gint,
    pub name: *mut gchar,
    pub args: *mut *mut GDBusArgInfo,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusArgInfo = _GDBusArgInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusArgInfo {
    pub ref_count: gint,
    pub name: *mut gchar,
    pub signature: *mut gchar,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusMethodInfo = _GDBusMethodInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusMethodInfo {
    pub ref_count: gint,
    pub name: *mut gchar,
    pub in_args: *mut *mut GDBusArgInfo,
    pub out_args: *mut *mut GDBusArgInfo,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusInterface = _GDBusInterface;
pub type GDBusObject = _GDBusObject;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusObjectProxy {
    pub parent_instance: GObject,
    pub priv_0: *mut GDBusObjectProxyPrivate,
}
pub type GDBusObjectProxyPrivate = _GDBusObjectProxyPrivate;
pub type GDBusObjectProxy = _GDBusObjectProxy;
pub type GDBusObjectManager = _GDBusObjectManager;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusObjectManagerClient {
    pub parent_instance: GObject,
    pub priv_0: *mut GDBusObjectManagerClientPrivate,
}
pub type GDBusObjectManagerClientPrivate = _GDBusObjectManagerClientPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusObjectManagerClientPrivate {
    pub lock: GMutex,
    pub bus_type: GBusType,
    pub connection: *mut GDBusConnection,
    pub object_path: *mut gchar,
    pub name: *mut gchar,
    pub name_owner: *mut gchar,
    pub flags: GDBusObjectManagerClientFlags,
    pub control_proxy: *mut GDBusProxy,
    pub map_object_path_to_object_proxy: *mut GHashTable,
    pub signal_subscription_id: guint,
    pub match_rule: *mut gchar,
    pub get_proxy_type_func: GDBusProxyTypeFunc,
    pub get_proxy_type_user_data: gpointer,
    pub get_proxy_type_destroy_notify: GDestroyNotify,
    pub name_owner_signal_id: gulong,
    pub signal_signal_id: gulong,
    pub cancel: *mut GCancellable,
}
pub type GDBusProxyTypeFunc = Option<
    unsafe extern "C" fn(
        *mut GDBusObjectManagerClient,
        *const gchar,
        *const gchar,
        gpointer,
    ) -> GType,
>;
pub type GDBusObjectManagerClient = _GDBusObjectManagerClient;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusObjectManagerIface {
    pub parent_iface: GTypeInterface,
    pub get_object_path: Option<unsafe extern "C" fn(*mut GDBusObjectManager) -> *const gchar>,
    pub get_objects: Option<unsafe extern "C" fn(*mut GDBusObjectManager) -> *mut GList>,
    pub get_object:
        Option<unsafe extern "C" fn(*mut GDBusObjectManager, *const gchar) -> *mut GDBusObject>,
    pub get_interface: Option<
        unsafe extern "C" fn(
            *mut GDBusObjectManager,
            *const gchar,
            *const gchar,
        ) -> *mut GDBusInterface,
    >,
    pub object_added: Option<unsafe extern "C" fn(*mut GDBusObjectManager, *mut GDBusObject) -> ()>,
    pub object_removed:
        Option<unsafe extern "C" fn(*mut GDBusObjectManager, *mut GDBusObject) -> ()>,
    pub interface_added: Option<
        unsafe extern "C" fn(*mut GDBusObjectManager, *mut GDBusObject, *mut GDBusInterface) -> (),
    >,
    pub interface_removed: Option<
        unsafe extern "C" fn(*mut GDBusObjectManager, *mut GDBusObject, *mut GDBusInterface) -> (),
    >,
}
pub type GDBusObjectManagerIface = _GDBusObjectManagerIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusObjectManagerClientClass {
    pub parent_class: GObjectClass,
    pub interface_proxy_signal: Option<
        unsafe extern "C" fn(
            *mut GDBusObjectManagerClient,
            *mut GDBusObjectProxy,
            *mut GDBusProxy,
            *const gchar,
            *const gchar,
            *mut GVariant,
        ) -> (),
    >,
    pub interface_proxy_properties_changed: Option<
        unsafe extern "C" fn(
            *mut GDBusObjectManagerClient,
            *mut GDBusObjectProxy,
            *mut GDBusProxy,
            *mut GVariant,
            *const *const gchar,
        ) -> (),
    >,
    pub padding: [gpointer; 8],
}
pub type GDBusObjectManagerClientClass = _GDBusObjectManagerClientClass;
pub const INTERFACE_PROXY_PROPERTIES_CHANGED_SIGNAL: C2RustUnnamed_3 = 1;
pub const INTERFACE_PROXY_SIGNAL_SIGNAL: C2RustUnnamed_3 = 0;
pub const PROP_GET_PROXY_TYPE_DESTROY_NOTIFY: C2RustUnnamed_2 = 9;
pub const PROP_GET_PROXY_TYPE_USER_DATA: C2RustUnnamed_2 = 8;
pub const PROP_GET_PROXY_TYPE_FUNC: C2RustUnnamed_2 = 7;
pub const PROP_NAME_OWNER: C2RustUnnamed_2 = 6;
pub const PROP_NAME: C2RustUnnamed_2 = 5;
pub const PROP_OBJECT_PATH: C2RustUnnamed_2 = 4;
pub const PROP_FLAGS: C2RustUnnamed_2 = 3;
pub const PROP_BUS_TYPE: C2RustUnnamed_2 = 1;
pub const PROP_CONNECTION: C2RustUnnamed_2 = 2;
pub type GAsyncInitableIface = _GAsyncInitableIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GAsyncInitableIface {
    pub g_iface: GTypeInterface,
    pub init_async: Option<
        unsafe extern "C" fn(
            *mut GAsyncInitable,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub init_finish: Option<
        unsafe extern "C" fn(*mut GAsyncInitable, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    >,
}
pub type GInitableIface = _GInitableIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInitableIface {
    pub g_iface: GTypeInterface,
    pub init: Option<
        unsafe extern "C" fn(*mut GInitable, *mut GCancellable, *mut *mut GError) -> gboolean,
    >,
}
pub type GDBusSignalCallback = Option<
    unsafe extern "C" fn(
        *mut GDBusConnection,
        *const gchar,
        *const gchar,
        *const gchar,
        *const gchar,
        *mut GVariant,
        gpointer,
    ) -> (),
>;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const LAST_SIGNAL: C2RustUnnamed_3 = 2;
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
pub const G_PRIORITY_DEFAULT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_NONE: GType = ((1 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_STRING: GType = ((16 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_VARIANT: GType = ((21 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
static mut safe_c2rust_signals: [guint; 2] = [0 as ::core::ffi::c_int as guint, 0];
unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_client_class_intern_init(
    mut klass: gpointer,
) {
    safe_c2rust_g_dbus_object_manager_client_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GDBusObjectManagerClient_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GDBusObjectManagerClient_private_offset,
        );
    }
    safe_c2rust_g_dbus_object_manager_client_class_init(
        klass as *mut GDBusObjectManagerClientClass,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_client_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_dbus_object_manager_client_get_type_once();
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
static mut safe_c2rust_GDBusObjectManagerClient_private_offset: gint = 0;
#[inline]
unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_client_get_instance_private(
    mut self_0: *mut GDBusObjectManagerClient,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GDBusObjectManagerClient_private_offset as glong as isize)
        as gpointer;
}
static mut safe_c2rust_g_dbus_object_manager_client_parent_class: gpointer = NULL_0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_client_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GDBusObjectManagerClient\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GDBusObjectManagerClientClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dbus_object_manager_client_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GDBusObjectManagerClient>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusObjectManagerClient) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dbus_object_manager_client_init
                    as unsafe extern "C" fn(*mut GDBusObjectManagerClient) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GDBusObjectManagerClient_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GDBusObjectManagerClientPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GInitableIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_initable_iface_init as unsafe extern "C" fn(*mut GInitableIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_initable_get_type(),
        &raw const g_implement_interface_info,
    );
    let g_implement_interface_info_0: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GAsyncInitableIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_async_initable_iface_init
                as unsafe extern "C" fn(*mut GAsyncInitableIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_async_initable_get_type(),
        &raw const g_implement_interface_info_0,
    );
    let g_implement_interface_info_1: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GDBusObjectManagerIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_dbus_object_manager_interface_init
                as unsafe extern "C" fn(*mut GDBusObjectManagerIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_dbus_object_manager_get_type(),
        &raw const g_implement_interface_info_1,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_client_dispose(mut object: *mut GObject) {
    let mut manager: *mut GDBusObjectManagerClient =
        object as *mut ::core::ffi::c_void as *mut GDBusObjectManagerClient;
    if !(*(*manager).priv_0).cancel.is_null() {
        g_cancellable_cancel((*(*manager).priv_0).cancel);
        let mut _pp: *mut *mut GCancellable = &raw mut (*(*manager).priv_0).cancel;
        let mut _ptr: *mut GCancellable = *_pp;
        *_pp = ::core::ptr::null_mut::<GCancellable>();
        if !_ptr.is_null() {
            g_object_unref(_ptr as gpointer);
        }
    }
    (*(safe_c2rust_g_dbus_object_manager_client_parent_class as *mut GObjectClass))
        .dispose
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_client_finalize(mut object: *mut GObject) {
    let mut manager: *mut GDBusObjectManagerClient =
        object as *mut ::core::ffi::c_void as *mut GDBusObjectManagerClient;
    safe_c2rust_maybe_unsubscribe_signals(manager);
    g_hash_table_unref((*(*manager).priv_0).map_object_path_to_object_proxy);
    if !(*(*manager).priv_0).control_proxy.is_null()
        && (*(*manager).priv_0).signal_signal_id != 0 as gulong
    {
        g_signal_handler_disconnect(
            (*(*manager).priv_0).control_proxy as gpointer,
            (*(*manager).priv_0).signal_signal_id,
        );
    }
    (*(*manager).priv_0).signal_signal_id = 0 as gulong;
    if !(*(*manager).priv_0).control_proxy.is_null()
        && (*(*manager).priv_0).name_owner_signal_id != 0 as gulong
    {
        g_signal_handler_disconnect(
            (*(*manager).priv_0).control_proxy as gpointer,
            (*(*manager).priv_0).name_owner_signal_id,
        );
    }
    (*(*manager).priv_0).name_owner_signal_id = 0 as gulong;
    let mut _pp: *mut *mut GDBusProxy = &raw mut (*(*manager).priv_0).control_proxy;
    let mut _ptr: *mut GDBusProxy = *_pp;
    *_pp = ::core::ptr::null_mut::<GDBusProxy>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    if !(*(*manager).priv_0).connection.is_null() {
        g_object_unref((*(*manager).priv_0).connection as gpointer);
    }
    g_free((*(*manager).priv_0).object_path as gpointer);
    g_free((*(*manager).priv_0).name as gpointer);
    g_free((*(*manager).priv_0).name_owner as gpointer);
    if (*(*manager).priv_0).get_proxy_type_destroy_notify.is_some() {
        (*(*manager).priv_0)
            .get_proxy_type_destroy_notify
            .expect("non-null function pointer")(
            (*(*manager).priv_0).get_proxy_type_user_data
        );
    }
    g_mutex_clear(&raw mut (*(*manager).priv_0).lock);
    if (*(safe_c2rust_g_dbus_object_manager_client_parent_class as *mut GObjectClass))
        .finalize
        .is_some()
    {
        (*(safe_c2rust_g_dbus_object_manager_client_parent_class as *mut GObjectClass))
            .finalize
            .expect("non-null function pointer")(object);
    }
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_client_get_property(
    mut _object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut manager: *mut GDBusObjectManagerClient =
        _object as *mut ::core::ffi::c_void as *mut GDBusObjectManagerClient;
    match prop_id {
        2 => {
            g_value_set_object(
                value,
                safe_c2rust_g_dbus_object_manager_client_get_connection(manager) as gpointer,
            );
        }
        4 => {
            g_value_set_string(
                value,
                g_dbus_object_manager_get_object_path(
                    manager as *mut ::core::ffi::c_void as *mut GDBusObjectManager,
                ),
            );
        }
        5 => {
            g_value_set_string(
                value,
                safe_c2rust_g_dbus_object_manager_client_get_name(manager),
            );
        }
        3 => {
            g_value_set_flags(
                value,
                safe_c2rust_g_dbus_object_manager_client_get_flags(manager) as guint,
            );
        }
        6 => {
            g_value_take_string(
                value,
                safe_c2rust_g_dbus_object_manager_client_get_name_owner(manager),
            );
        }
        _ => {
            let mut _glib__object: *mut GObject = manager as *mut GObject;
            let mut _glib__pspec: *mut GParamSpec = pspec;
            let mut _glib__property_id: guint = prop_id;
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8
                    as *const gchar,
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectmanagerclient.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                283 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_client_set_property(
    mut _object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut manager: *mut GDBusObjectManagerClient =
        _object as *mut ::core::ffi::c_void as *mut GDBusObjectManagerClient;
    let mut name: *const gchar = ::core::ptr::null::<gchar>();
    match prop_id {
        1 => {
            (*(*manager).priv_0).bus_type = g_value_get_enum(value) as GBusType;
        }
        2 => {
            if !g_value_get_object(value).is_null() {
                if ({
                    let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
                    if (*(*manager).priv_0).connection.is_null() {
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
                        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectmanagerclient.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        306 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"manager->priv->connection == NULL\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                if ({
                    let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
                    if ({
                        let mut __inst: *mut GTypeInstance =
                            g_value_get_object(value) as *mut GTypeInstance;
                        let mut __t: GType = g_dbus_connection_get_type();
                        let mut __r: gboolean = 0;
                        if __inst.is_null() {
                            __r = 0 as ::core::ffi::c_int as gboolean;
                        } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t
                        {
                            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
                        } else {
                            __r = g_type_check_instance_is_a(__inst, __t);
                        }
                        __r
                    }) != 0
                    {
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
                        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectmanagerclient.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        307 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"G_IS_DBUS_CONNECTION (g_value_get_object (value))\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                (*(*manager).priv_0).connection = g_value_dup_object(value) as *mut GDBusConnection;
            }
        }
        4 => {
            if ({
                let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
                if (*(*manager).priv_0).object_path.is_null() {
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
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectmanagerclient.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    313 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"manager->priv->object_path == NULL\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            if ({
                let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
                if g_variant_is_object_path(g_value_get_string(value)) != 0 {
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
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectmanagerclient.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    314 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"g_variant_is_object_path (g_value_get_string (value))\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
            (*(*manager).priv_0).object_path = g_value_dup_string(value);
        }
        5 => {
            if ({
                let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
                if (*(*manager).priv_0).name.is_null() {
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
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectmanagerclient.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    319 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"manager->priv->name == NULL\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            name = g_value_get_string(value);
            if ({
                let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
                if name.is_null() || g_dbus_is_name(name) != 0 {
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
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectmanagerclient.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    321 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"name == NULL || g_dbus_is_name (name)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            (*(*manager).priv_0).name =
                safe_c2rust_g_strdup_inline(name as *const ::core::ffi::c_char) as *mut gchar;
        }
        3 => {
            (*(*manager).priv_0).flags = g_value_get_flags(value) as GDBusObjectManagerClientFlags;
        }
        7 => {
            (*(*manager).priv_0).get_proxy_type_func =
                ::core::mem::transmute::<gpointer, GDBusProxyTypeFunc>(g_value_get_pointer(value));
        }
        8 => {
            (*(*manager).priv_0).get_proxy_type_user_data = g_value_get_pointer(value);
        }
        9 => {
            (*(*manager).priv_0).get_proxy_type_destroy_notify =
                ::core::mem::transmute::<gpointer, GDestroyNotify>(g_value_get_pointer(value));
        }
        _ => {
            let mut _glib__object: *mut GObject = manager as *mut GObject;
            let mut _glib__pspec: *mut GParamSpec = pspec;
            let mut _glib__property_id: guint = prop_id;
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8
                    as *const gchar,
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectmanagerclient.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                342 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_client_class_init(
    mut klass: *mut GDBusObjectManagerClientClass,
) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).dispose = Some(
        safe_c2rust_g_dbus_object_manager_client_dispose
            as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).finalize = Some(
        safe_c2rust_g_dbus_object_manager_client_finalize
            as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_dbus_object_manager_client_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_dbus_object_manager_client_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    g_object_class_install_property(
        gobject_class,
        PROP_CONNECTION as ::core::ffi::c_int as guint,
        g_param_spec_object(
            b"connection\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_dbus_connection_get_type(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_BUS_TYPE as ::core::ffi::c_int as guint,
        g_param_spec_enum(
            b"bus-type\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_bus_type_get_type(),
            G_BUS_TYPE_NONE as ::core::ffi::c_int as gint,
            (G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_FLAGS as ::core::ffi::c_int as guint,
        g_param_spec_flags(
            b"flags\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_dbus_object_manager_client_flags_get_type(),
            G_DBUS_OBJECT_MANAGER_CLIENT_FLAGS_NONE as ::core::ffi::c_int as guint,
            (G_PARAM_READABLE as ::core::ffi::c_int
                | G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_OBJECT_PATH as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"object-path\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_NAME as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"name\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_NAME_OWNER as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"name-owner\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_GET_PROXY_TYPE_FUNC as ::core::ffi::c_int as guint,
        g_param_spec_pointer(
            b"get-proxy-type-func\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_GET_PROXY_TYPE_USER_DATA as ::core::ffi::c_int as guint,
        g_param_spec_pointer(
            b"get-proxy-type-user-data\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_GET_PROXY_TYPE_DESTROY_NOTIFY as ::core::ffi::c_int as guint,
        g_param_spec_pointer(
            b"get-proxy-type-destroy-notify\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    safe_c2rust_signals[INTERFACE_PROXY_SIGNAL_SIGNAL as ::core::ffi::c_int as usize] =
        g_signal_new(
            g_intern_static_string(b"interface-proxy-signal\0" as *const u8 as *const gchar),
            safe_c2rust_g_dbus_object_manager_client_get_type(),
            G_SIGNAL_RUN_LAST,
            136 as ::core::ffi::c_ulong as glong as guint,
            None,
            NULL_0,
            Some(
                _g_cclosure_marshal_VOID__OBJECT_OBJECT_STRING_STRING_VARIANT
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        guint,
                        *const GValue,
                        gpointer,
                        gpointer,
                    ) -> (),
            ),
            G_TYPE_NONE,
            5 as guint,
            g_dbus_object_proxy_get_type(),
            g_dbus_proxy_get_type(),
            G_TYPE_STRING,
            G_TYPE_STRING,
            G_TYPE_VARIANT,
        );
    g_signal_set_va_marshaller(
        safe_c2rust_signals[INTERFACE_PROXY_SIGNAL_SIGNAL as ::core::ffi::c_int as usize],
        (*(klass as *mut GTypeClass)).g_type,
        Some(
            _g_cclosure_marshal_VOID__OBJECT_OBJECT_STRING_STRING_VARIANTv
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    gpointer,
                    ::core::ffi::VaList,
                    gpointer,
                    ::core::ffi::c_int,
                    *mut GType,
                ) -> (),
        ),
    );
    safe_c2rust_signals[INTERFACE_PROXY_PROPERTIES_CHANGED_SIGNAL as ::core::ffi::c_int as usize] =
        g_signal_new(
            g_intern_static_string(
                b"interface-proxy-properties-changed\0" as *const u8 as *const gchar,
            ),
            safe_c2rust_g_dbus_object_manager_client_get_type(),
            G_SIGNAL_RUN_LAST,
            144 as ::core::ffi::c_ulong as glong as guint,
            None,
            NULL_0,
            Some(
                _g_cclosure_marshal_VOID__OBJECT_OBJECT_VARIANT_BOXED
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        guint,
                        *const GValue,
                        gpointer,
                        gpointer,
                    ) -> (),
            ),
            G_TYPE_NONE,
            4 as guint,
            g_dbus_object_proxy_get_type(),
            g_dbus_proxy_get_type(),
            G_TYPE_VARIANT,
            g_strv_get_type(),
        );
    g_signal_set_va_marshaller(
        safe_c2rust_signals
            [INTERFACE_PROXY_PROPERTIES_CHANGED_SIGNAL as ::core::ffi::c_int as usize],
        (*(klass as *mut GTypeClass)).g_type,
        Some(
            _g_cclosure_marshal_VOID__OBJECT_OBJECT_VARIANT_BOXEDv
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    gpointer,
                    ::core::ffi::VaList,
                    gpointer,
                    ::core::ffi::c_int,
                    *mut GType,
                ) -> (),
        ),
    );
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_client_init(
    mut manager: *mut GDBusObjectManagerClient,
) {
    (*manager).priv_0 = safe_c2rust_g_dbus_object_manager_client_get_instance_private(manager)
        as *mut GDBusObjectManagerClientPrivate;
    g_mutex_init(&raw mut (*(*manager).priv_0).lock);
    (*(*manager).priv_0).map_object_path_to_object_proxy = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        ::core::mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> ()>, GDestroyNotify>(
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        ),
    );
    (*(*manager).priv_0).cancel = g_cancellable_new();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_client_new_sync(
    mut connection: *mut GDBusConnection,
    mut flags: GDBusObjectManagerClientFlags,
    mut name: *const gchar,
    mut object_path: *const gchar,
    mut get_proxy_type_func: GDBusProxyTypeFunc,
    mut get_proxy_type_user_data: gpointer,
    mut get_proxy_type_destroy_notify: GDestroyNotify,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GDBusObjectManager {
    let mut initable: *mut GInitable = ::core::ptr::null_mut::<GInitable>();
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = g_dbus_connection_get_type();
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
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusObjectManager>();
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if name.is_null() && g_dbus_connection_get_unique_name(connection).is_null()
            || g_dbus_is_name(name) != 0
        {
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
            b"(name == NULL && g_dbus_connection_get_unique_name (connection) == NULL) || g_dbus_is_name (name)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusObjectManager>();
    }
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if g_variant_is_object_path(object_path) != 0 {
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
            b"g_variant_is_object_path (object_path)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusObjectManager>();
    }
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusObjectManager>();
    }
    initable = g_initable_new(
        safe_c2rust_g_dbus_object_manager_client_get_type(),
        cancellable,
        error,
        b"connection\0" as *const u8 as *const gchar,
        connection,
        b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        flags as ::core::ffi::c_uint,
        b"name\0" as *const u8 as *const ::core::ffi::c_char,
        name,
        b"object-path\0" as *const u8 as *const ::core::ffi::c_char,
        object_path,
        b"get-proxy-type-func\0" as *const u8 as *const ::core::ffi::c_char,
        get_proxy_type_func,
        b"get-proxy-type-user-data\0" as *const u8 as *const ::core::ffi::c_char,
        get_proxy_type_user_data,
        b"get-proxy-type-destroy-notify\0" as *const u8 as *const ::core::ffi::c_char,
        get_proxy_type_destroy_notify,
        NULL_0,
    ) as *mut GInitable;
    if !initable.is_null() {
        return initable as *mut ::core::ffi::c_void as *mut GDBusObjectManager;
    } else {
        return ::core::ptr::null_mut::<GDBusObjectManager>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_client_new(
    mut connection: *mut GDBusConnection,
    mut flags: GDBusObjectManagerClientFlags,
    mut name: *const gchar,
    mut object_path: *const gchar,
    mut get_proxy_type_func: GDBusProxyTypeFunc,
    mut get_proxy_type_user_data: gpointer,
    mut get_proxy_type_destroy_notify: GDestroyNotify,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = g_dbus_connection_get_type();
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
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if name.is_null() && g_dbus_connection_get_unique_name(connection).is_null()
            || g_dbus_is_name(name) != 0
        {
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
            b"(name == NULL && g_dbus_connection_get_unique_name (connection) == NULL) || g_dbus_is_name (name)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if g_variant_is_object_path(object_path) != 0 {
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
            b"g_variant_is_object_path (object_path)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_async_initable_new_async(
        safe_c2rust_g_dbus_object_manager_client_get_type(),
        G_PRIORITY_DEFAULT,
        cancellable,
        callback,
        user_data,
        b"connection\0" as *const u8 as *const gchar,
        connection,
        b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        flags as ::core::ffi::c_uint,
        b"name\0" as *const u8 as *const ::core::ffi::c_char,
        name,
        b"object-path\0" as *const u8 as *const ::core::ffi::c_char,
        object_path,
        b"get-proxy-type-func\0" as *const u8 as *const ::core::ffi::c_char,
        get_proxy_type_func,
        b"get-proxy-type-user-data\0" as *const u8 as *const ::core::ffi::c_char,
        get_proxy_type_user_data,
        b"get-proxy-type-destroy-notify\0" as *const u8 as *const ::core::ffi::c_char,
        get_proxy_type_destroy_notify,
        NULL_0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_client_new_finish(
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GDBusObjectManager {
    let mut object: *mut GObject = ::core::ptr::null_mut::<GObject>();
    let mut source_object: *mut GObject = ::core::ptr::null_mut::<GObject>();
    source_object = g_async_result_get_source_object(res);
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if !source_object.is_null() {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectmanagerclient.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            739 as ::core::ffi::c_int,
            G_STRFUNC,
            b"source_object != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    object = g_async_initable_new_finish(
        source_object as *mut ::core::ffi::c_void as *mut GAsyncInitable,
        res,
        error,
    );
    g_object_unref(source_object as gpointer);
    if !object.is_null() {
        return object as *mut ::core::ffi::c_void as *mut GDBusObjectManager;
    } else {
        return ::core::ptr::null_mut::<GDBusObjectManager>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_client_new_for_bus_sync(
    mut bus_type: GBusType,
    mut flags: GDBusObjectManagerClientFlags,
    mut name: *const gchar,
    mut object_path: *const gchar,
    mut get_proxy_type_func: GDBusProxyTypeFunc,
    mut get_proxy_type_user_data: gpointer,
    mut get_proxy_type_destroy_notify: GDestroyNotify,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GDBusObjectManager {
    let mut initable: *mut GInitable = ::core::ptr::null_mut::<GInitable>();
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if bus_type as ::core::ffi::c_int != G_BUS_TYPE_NONE as ::core::ffi::c_int {
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
            b"bus_type != G_BUS_TYPE_NONE\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusObjectManager>();
    }
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if g_dbus_is_name(name) != 0 {
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
            b"g_dbus_is_name (name)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusObjectManager>();
    }
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if g_variant_is_object_path(object_path) != 0 {
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
            b"g_variant_is_object_path (object_path)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusObjectManager>();
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
        return ::core::ptr::null_mut::<GDBusObjectManager>();
    }
    initable = g_initable_new(
        safe_c2rust_g_dbus_object_manager_client_get_type(),
        cancellable,
        error,
        b"bus-type\0" as *const u8 as *const gchar,
        bus_type as ::core::ffi::c_int,
        b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        flags as ::core::ffi::c_uint,
        b"name\0" as *const u8 as *const ::core::ffi::c_char,
        name,
        b"object-path\0" as *const u8 as *const ::core::ffi::c_char,
        object_path,
        b"get-proxy-type-func\0" as *const u8 as *const ::core::ffi::c_char,
        get_proxy_type_func,
        b"get-proxy-type-user-data\0" as *const u8 as *const ::core::ffi::c_char,
        get_proxy_type_user_data,
        b"get-proxy-type-destroy-notify\0" as *const u8 as *const ::core::ffi::c_char,
        get_proxy_type_destroy_notify,
        NULL_0,
    ) as *mut GInitable;
    if !initable.is_null() {
        return initable as *mut ::core::ffi::c_void as *mut GDBusObjectManager;
    } else {
        return ::core::ptr::null_mut::<GDBusObjectManager>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_client_new_for_bus(
    mut bus_type: GBusType,
    mut flags: GDBusObjectManagerClientFlags,
    mut name: *const gchar,
    mut object_path: *const gchar,
    mut get_proxy_type_func: GDBusProxyTypeFunc,
    mut get_proxy_type_user_data: gpointer,
    mut get_proxy_type_destroy_notify: GDestroyNotify,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if bus_type as ::core::ffi::c_int != G_BUS_TYPE_NONE as ::core::ffi::c_int {
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
            b"bus_type != G_BUS_TYPE_NONE\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if g_dbus_is_name(name) != 0 {
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
            b"g_dbus_is_name (name)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if g_variant_is_object_path(object_path) != 0 {
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
            b"g_variant_is_object_path (object_path)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_async_initable_new_async(
        safe_c2rust_g_dbus_object_manager_client_get_type(),
        G_PRIORITY_DEFAULT,
        cancellable,
        callback,
        user_data,
        b"bus-type\0" as *const u8 as *const gchar,
        bus_type as ::core::ffi::c_int,
        b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        flags as ::core::ffi::c_uint,
        b"name\0" as *const u8 as *const ::core::ffi::c_char,
        name,
        b"object-path\0" as *const u8 as *const ::core::ffi::c_char,
        object_path,
        b"get-proxy-type-func\0" as *const u8 as *const ::core::ffi::c_char,
        get_proxy_type_func,
        b"get-proxy-type-user-data\0" as *const u8 as *const ::core::ffi::c_char,
        get_proxy_type_user_data,
        b"get-proxy-type-destroy-notify\0" as *const u8 as *const ::core::ffi::c_char,
        get_proxy_type_destroy_notify,
        NULL_0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_client_new_for_bus_finish(
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GDBusObjectManager {
    let mut object: *mut GObject = ::core::ptr::null_mut::<GObject>();
    let mut source_object: *mut GObject = ::core::ptr::null_mut::<GObject>();
    source_object = g_async_result_get_source_object(res);
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if !source_object.is_null() {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectmanagerclient.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            891 as ::core::ffi::c_int,
            G_STRFUNC,
            b"source_object != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    object = g_async_initable_new_finish(
        source_object as *mut ::core::ffi::c_void as *mut GAsyncInitable,
        res,
        error,
    );
    g_object_unref(source_object as gpointer);
    if !object.is_null() {
        return object as *mut ::core::ffi::c_void as *mut GDBusObjectManager;
    } else {
        return ::core::ptr::null_mut::<GDBusObjectManager>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_client_get_connection(
    mut manager: *mut GDBusObjectManagerClient,
) -> *mut GDBusConnection {
    let mut ret: *mut GDBusConnection = ::core::ptr::null_mut::<GDBusConnection>();
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = manager as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_object_manager_client_get_type();
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
            b"G_IS_DBUS_OBJECT_MANAGER_CLIENT (manager)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusConnection>();
    }
    g_mutex_lock(&raw mut (*(*manager).priv_0).lock);
    ret = (*(*manager).priv_0).connection;
    g_mutex_unlock(&raw mut (*(*manager).priv_0).lock);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_client_get_name(
    mut manager: *mut GDBusObjectManagerClient,
) -> *const gchar {
    let mut ret: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = manager as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_object_manager_client_get_type();
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
            b"G_IS_DBUS_OBJECT_MANAGER_CLIENT (manager)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    g_mutex_lock(&raw mut (*(*manager).priv_0).lock);
    ret = (*(*manager).priv_0).name;
    g_mutex_unlock(&raw mut (*(*manager).priv_0).lock);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_client_get_flags(
    mut manager: *mut GDBusObjectManagerClient,
) -> GDBusObjectManagerClientFlags {
    let mut ret: GDBusObjectManagerClientFlags = G_DBUS_OBJECT_MANAGER_CLIENT_FLAGS_NONE;
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = manager as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_object_manager_client_get_type();
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
            b"G_IS_DBUS_OBJECT_MANAGER_CLIENT (manager)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return G_DBUS_OBJECT_MANAGER_CLIENT_FLAGS_NONE;
    }
    g_mutex_lock(&raw mut (*(*manager).priv_0).lock);
    ret = (*(*manager).priv_0).flags;
    g_mutex_unlock(&raw mut (*(*manager).priv_0).lock);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_client_get_name_owner(
    mut manager: *mut GDBusObjectManagerClient,
) -> *mut gchar {
    let mut ret: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = manager as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_object_manager_client_get_type();
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
            b"G_IS_DBUS_OBJECT_MANAGER_CLIENT (manager)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    g_mutex_lock(&raw mut (*(*manager).priv_0).lock);
    ret = safe_c2rust_g_strdup_inline((*(*manager).priv_0).name_owner) as *mut gchar;
    g_mutex_unlock(&raw mut (*(*manager).priv_0).lock);
    return ret;
}
unsafe extern "C" fn safe_c2rust_signal_cb(
    mut connection: *mut GDBusConnection,
    mut sender_name: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut signal_name: *const gchar,
    mut parameters: *mut GVariant,
    mut user_data: gpointer,
) {
    let mut manager: *mut GDBusObjectManagerClient = user_data as *mut GDBusObjectManagerClient;
    let mut object_proxy: *mut GDBusObjectProxy = ::core::ptr::null_mut::<GDBusObjectProxy>();
    let mut interface: *mut GDBusInterface = ::core::ptr::null_mut::<GDBusInterface>();
    g_mutex_lock(&raw mut (*(*manager).priv_0).lock);
    object_proxy = g_hash_table_lookup(
        (*(*manager).priv_0).map_object_path_to_object_proxy,
        object_path as gconstpointer,
    ) as *mut GDBusObjectProxy;
    if object_proxy.is_null() {
        g_mutex_unlock(&raw mut (*(*manager).priv_0).lock);
    } else {
        g_object_ref(object_proxy as gpointer);
        g_mutex_unlock(&raw mut (*(*manager).priv_0).lock);
        g_object_ref(manager as gpointer);
        if g_strcmp0(
            interface_name as *const ::core::ffi::c_char,
            b"org.freedesktop.DBus.Properties\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            if g_strcmp0(
                signal_name as *const ::core::ffi::c_char,
                b"PropertiesChanged\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                let mut properties_interface_name: *const gchar = ::core::ptr::null::<gchar>();
                let mut changed_properties: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
                let mut invalidated_properties: *mut *const gchar =
                    ::core::ptr::null_mut::<*const gchar>();
                g_variant_get(
                    parameters,
                    b"(&s@a{sv}^a&s)\0" as *const u8 as *const gchar,
                    &raw mut properties_interface_name,
                    &raw mut changed_properties,
                    &raw mut invalidated_properties,
                );
                interface = g_dbus_object_get_interface(
                    object_proxy as *mut ::core::ffi::c_void as *mut GDBusObject,
                    properties_interface_name,
                );
                if !interface.is_null() {
                    let mut property_iter: GVariantIter = _GVariantIter { x: [0; 16] };
                    let mut property_name: *const gchar = ::core::ptr::null::<gchar>();
                    let mut property_value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
                    let mut n: guint = 0;
                    g_variant_iter_init(&raw mut property_iter, changed_properties);
                    while g_variant_iter_next(
                        &raw mut property_iter,
                        b"{&sv}\0" as *const u8 as *const gchar,
                        &raw mut property_name,
                        &raw mut property_value,
                    ) != 0
                    {
                        g_dbus_proxy_set_cached_property(
                            interface as *mut ::core::ffi::c_void as *mut GDBusProxy,
                            property_name,
                            property_value,
                        );
                        g_variant_unref(property_value);
                    }
                    n = 0 as guint;
                    while !(*invalidated_properties.offset(n as isize)).is_null() {
                        g_dbus_proxy_set_cached_property(
                            interface as *mut ::core::ffi::c_void as *mut GDBusProxy,
                            *invalidated_properties.offset(n as isize),
                            ::core::ptr::null_mut::<GVariant>(),
                        );
                        n = n.wrapping_add(1);
                    }
                    g_signal_emit_by_name(
                        interface as gpointer,
                        b"g-properties-changed\0" as *const u8 as *const gchar,
                        changed_properties,
                        invalidated_properties,
                    );
                    g_signal_emit(
                        manager as gpointer,
                        safe_c2rust_signals[INTERFACE_PROXY_PROPERTIES_CHANGED_SIGNAL
                            as ::core::ffi::c_int
                            as usize],
                        0 as GQuark,
                        object_proxy,
                        interface,
                        changed_properties,
                        invalidated_properties,
                    );
                    g_object_unref(interface as gpointer);
                }
                g_variant_unref(changed_properties);
                g_free(invalidated_properties as gpointer);
            }
        } else {
            interface = g_dbus_object_get_interface(
                object_proxy as *mut ::core::ffi::c_void as *mut GDBusObject,
                interface_name,
            );
            if !interface.is_null() {
                g_signal_emit_by_name(
                    interface as gpointer,
                    b"g-signal\0" as *const u8 as *const gchar,
                    sender_name,
                    signal_name,
                    parameters,
                );
                g_signal_emit(
                    manager as gpointer,
                    safe_c2rust_signals
                        [INTERFACE_PROXY_SIGNAL_SIGNAL as ::core::ffi::c_int as usize],
                    0 as GQuark,
                    object_proxy,
                    interface,
                    sender_name,
                    signal_name,
                    parameters,
                );
                g_object_unref(interface as gpointer);
            }
        }
        g_object_unref(manager as gpointer);
    }
    let mut _pp: *mut *mut GDBusObjectProxy = &raw mut object_proxy;
    let mut _ptr: *mut GDBusObjectProxy = *_pp;
    *_pp = ::core::ptr::null_mut::<GDBusObjectProxy>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_subscribe_signals(
    mut manager: *mut GDBusObjectManagerClient,
    mut name_owner: *const gchar,
) {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = manager as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_object_manager_client_get_type();
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
            b"G_IS_DBUS_OBJECT_MANAGER_CLIENT (manager)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if (*(*manager).priv_0).signal_subscription_id == 0 as guint {
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
            b"manager->priv->signal_subscription_id == 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if name_owner.is_null() || g_dbus_is_unique_name(name_owner) != 0 {
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
            b"name_owner == NULL || g_dbus_is_unique_name (name_owner)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if !name_owner.is_null() {
        if strcmp(
            (*(*manager).priv_0).object_path as *const ::core::ffi::c_char,
            b"/\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*(*manager).priv_0).match_rule = g_strdup_printf(
                b"type='signal',sender='%s'\0" as *const u8 as *const gchar,
                name_owner,
            );
        } else {
            (*(*manager).priv_0).match_rule = g_strdup_printf(
                b"type='signal',sender='%s',path_namespace='%s'\0" as *const u8 as *const gchar,
                name_owner,
                (*(*manager).priv_0).object_path,
            );
        }
        ret = g_dbus_connection_call_sync(
            (*(*manager).priv_0).connection,
            b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
            b"/org/freedesktop/DBus\0" as *const u8 as *const gchar,
            b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
            b"AddMatch\0" as *const u8 as *const gchar,
            g_variant_new(
                b"(s)\0" as *const u8 as *const gchar,
                (*(*manager).priv_0).match_rule,
            ),
            ::core::ptr::null::<GVariantType>(),
            G_DBUS_CALL_FLAGS_NONE,
            -(1 as gint),
            ::core::ptr::null_mut::<GCancellable>(),
            &raw mut error,
        );
        if !ret.is_null() {
            g_variant_unref(ret);
        }
    }
    if error.is_null() {
        (*(*manager).priv_0).signal_subscription_id = g_dbus_connection_signal_subscribe(
            (*(*manager).priv_0).connection,
            name_owner,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_DBUS_SIGNAL_FLAGS_NONE as ::core::ffi::c_int
                | G_DBUS_SIGNAL_FLAGS_NO_MATCH_RULE as ::core::ffi::c_int)
                as GDBusSignalFlags,
            Some(
                safe_c2rust_signal_cb
                    as unsafe extern "C" fn(
                        *mut GDBusConnection,
                        *const gchar,
                        *const gchar,
                        *const gchar,
                        *const gchar,
                        *mut GVariant,
                        gpointer,
                    ) -> (),
            ),
            manager as gpointer,
            None,
        );
    } else {
        g_error_free(error);
        g_free((*(*manager).priv_0).match_rule as gpointer);
        (*(*manager).priv_0).match_rule = ::core::ptr::null_mut::<gchar>();
        (*(*manager).priv_0).signal_subscription_id = g_dbus_connection_signal_subscribe(
            (*(*manager).priv_0).connection,
            name_owner,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            G_DBUS_SIGNAL_FLAGS_NONE,
            Some(
                safe_c2rust_signal_cb
                    as unsafe extern "C" fn(
                        *mut GDBusConnection,
                        *const gchar,
                        *const gchar,
                        *const gchar,
                        *const gchar,
                        *mut GVariant,
                        gpointer,
                    ) -> (),
            ),
            manager as gpointer,
            None,
        );
    };
}
unsafe extern "C" fn safe_c2rust_maybe_unsubscribe_signals(
    mut manager: *mut GDBusObjectManagerClient,
) {
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = manager as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_object_manager_client_get_type();
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
            b"G_IS_DBUS_OBJECT_MANAGER_CLIENT (manager)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*(*manager).priv_0).signal_subscription_id > 0 as guint {
        g_dbus_connection_signal_unsubscribe(
            (*(*manager).priv_0).connection,
            (*(*manager).priv_0).signal_subscription_id,
        );
        (*(*manager).priv_0).signal_subscription_id = 0 as guint;
    }
    if !(*(*manager).priv_0).match_rule.is_null() {
        g_dbus_connection_call(
            (*(*manager).priv_0).connection,
            b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
            b"/org/freedesktop/DBus\0" as *const u8 as *const gchar,
            b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
            b"RemoveMatch\0" as *const u8 as *const gchar,
            g_variant_new(
                b"(s)\0" as *const u8 as *const gchar,
                (*(*manager).priv_0).match_rule,
            ),
            ::core::ptr::null::<GVariantType>(),
            G_DBUS_CALL_FLAGS_NONE,
            -(1 as gint),
            ::core::ptr::null_mut::<GCancellable>(),
            None,
            NULL_0,
        );
        g_free((*(*manager).priv_0).match_rule as gpointer);
        (*(*manager).priv_0).match_rule = ::core::ptr::null_mut::<gchar>();
    }
}
unsafe extern "C" fn safe_c2rust_weak_ref_new(mut object: *mut GObject) -> *mut GWeakRef {
    let mut weak_ref: *mut GWeakRef = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GWeakRef>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GWeakRef;
    g_weak_ref_init(weak_ref, object as gpointer);
    return safe_c2rust_g_steal_pointer(&raw mut weak_ref as gpointer) as *mut GWeakRef;
}
unsafe extern "C" fn safe_c2rust_weak_ref_free(mut weak_ref: *mut GWeakRef) {
    g_weak_ref_clear(weak_ref);
    g_free(weak_ref as gpointer);
}
unsafe extern "C" fn safe_c2rust_on_get_managed_objects_finish(
    mut source: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut proxy: *mut GDBusProxy = source as *mut ::core::ffi::c_void as *mut GDBusProxy;
    let mut manager_weak: *mut GWeakRef = user_data as *mut GWeakRef;
    let mut manager: *mut GDBusObjectManagerClient =
        ::core::ptr::null_mut::<GDBusObjectManagerClient>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut new_name_owner: *mut gchar = ::core::ptr::null_mut::<gchar>();
    value = g_dbus_proxy_call_finish(proxy, result, &raw mut error);
    manager = g_weak_ref_get(manager_weak) as *mut GDBusObjectManagerClient;
    if !manager.is_null() {
        new_name_owner = g_dbus_proxy_get_name_owner((*(*manager).priv_0).control_proxy);
        if value.is_null() {
            safe_c2rust_maybe_unsubscribe_signals(manager);
            if g_error_matches(
                error,
                g_io_error_quark(),
                G_IO_ERROR_CANCELLED as ::core::ffi::c_int as gint,
            ) == 0
            {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_WARNING,
                    b"Error calling GetManagedObjects() when name owner %s for name %s came back: %s\0"
                        as *const u8 as *const gchar,
                    new_name_owner,
                    (*(*manager).priv_0).name,
                    (*error).message,
                );
            }
        } else {
            safe_c2rust_process_get_all_result(manager, value, new_name_owner);
        }
        g_mutex_lock(&raw mut (*(*manager).priv_0).lock);
        (*(*manager).priv_0).name_owner =
            safe_c2rust_g_steal_pointer(&raw mut new_name_owner as gpointer) as *mut gchar
                as *mut gchar;
        g_mutex_unlock(&raw mut (*(*manager).priv_0).lock);
        g_object_notify(
            manager as *mut ::core::ffi::c_void as *mut GObject,
            b"name-owner\0" as *const u8 as *const gchar,
        );
        g_object_unref(manager as gpointer);
    }
    g_clear_error(&raw mut error);
    let mut _pp: *mut *mut GVariant = &raw mut value;
    let mut _ptr: *mut GVariant = *_pp;
    *_pp = ::core::ptr::null_mut::<GVariant>();
    if !_ptr.is_null() {
        g_variant_unref(_ptr as *mut GVariant);
    }
    safe_c2rust_weak_ref_free(manager_weak);
}
unsafe extern "C" fn safe_c2rust_on_notify_g_name_owner(
    mut object: *mut GObject,
    mut pspec: *mut GParamSpec,
    mut user_data: gpointer,
) {
    let mut manager_weak: *mut GWeakRef = user_data as *mut GWeakRef;
    let mut manager: *mut GDBusObjectManagerClient =
        ::core::ptr::null_mut::<GDBusObjectManagerClient>();
    let mut old_name_owner: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut new_name_owner: *mut gchar = ::core::ptr::null_mut::<gchar>();
    manager = g_weak_ref_get(manager_weak) as *mut GDBusObjectManagerClient;
    if manager.is_null() {
        return;
    }
    g_mutex_lock(&raw mut (*(*manager).priv_0).lock);
    old_name_owner = (*(*manager).priv_0).name_owner;
    new_name_owner = g_dbus_proxy_get_name_owner((*(*manager).priv_0).control_proxy);
    (*(*manager).priv_0).name_owner = ::core::ptr::null_mut::<gchar>();
    if g_strcmp0(old_name_owner, new_name_owner) != 0 as ::core::ffi::c_int {
        let mut proxies: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
        proxies =
            g_hash_table_steal_all_values((*(*manager).priv_0).map_object_path_to_object_proxy);
        g_mutex_unlock(&raw mut (*(*manager).priv_0).lock);
        g_object_notify(
            manager as *mut ::core::ffi::c_void as *mut GObject,
            b"name-owner\0" as *const u8 as *const gchar,
        );
        let mut i: guint = 0 as guint;
        while i < (*proxies).len {
            let mut object_proxy: *mut GDBusObjectProxy =
                *(*proxies).pdata.offset(i as isize) as *mut GDBusObjectProxy;
            g_signal_emit_by_name(
                manager as gpointer,
                b"object-removed\0" as *const u8 as *const gchar,
                object_proxy,
            );
            i = i.wrapping_add(1);
        }
        let mut _pp: *mut *mut GPtrArray = &raw mut proxies;
        let mut _ptr: *mut GPtrArray = *_pp;
        *_pp = ::core::ptr::null_mut::<GPtrArray>();
        if !_ptr.is_null() {
            g_ptr_array_unref(_ptr as *mut GPtrArray);
        }
        safe_c2rust_maybe_unsubscribe_signals(manager);
    } else {
        g_mutex_unlock(&raw mut (*(*manager).priv_0).lock);
    }
    if !new_name_owner.is_null() {
        safe_c2rust_subscribe_signals(manager, new_name_owner);
        g_dbus_proxy_call(
            (*(*manager).priv_0).control_proxy,
            b"GetManagedObjects\0" as *const u8 as *const gchar,
            ::core::ptr::null_mut::<GVariant>(),
            G_DBUS_CALL_FLAGS_NONE,
            -(1 as gint),
            (*(*manager).priv_0).cancel,
            Some(
                safe_c2rust_on_get_managed_objects_finish
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            safe_c2rust_weak_ref_new(manager as *mut ::core::ffi::c_void as *mut GObject)
                as gpointer,
        );
    }
    g_free(new_name_owner as gpointer);
    g_free(old_name_owner as gpointer);
    g_object_unref(manager as gpointer);
}
unsafe extern "C" fn safe_c2rust_initable_init(
    mut initable: *mut GInitable,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut current_block: u64;
    let mut manager: *mut GDBusObjectManagerClient =
        initable as *mut ::core::ffi::c_void as *mut GDBusObjectManagerClient;
    let mut ret: gboolean = 0;
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut proxy_flags: GDBusProxyFlags = G_DBUS_PROXY_FLAGS_NONE;
    ret = FALSE as gboolean;
    if (*(*manager).priv_0).bus_type as ::core::ffi::c_int != G_BUS_TYPE_NONE as ::core::ffi::c_int
    {
        if ({
            let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
            if (*(*manager).priv_0).connection.is_null() {
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectmanagerclient.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1407 as ::core::ffi::c_int,
                G_STRFUNC,
                b"manager->priv->connection == NULL\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        (*(*manager).priv_0).connection =
            g_bus_get_sync((*(*manager).priv_0).bus_type, cancellable, error);
        if (*(*manager).priv_0).connection.is_null() {
            current_block = 12912888199596355060;
        } else {
            current_block = 3276175668257526147;
        }
    } else {
        current_block = 3276175668257526147;
    }
    match current_block {
        3276175668257526147 => {
            proxy_flags = G_DBUS_PROXY_FLAGS_DO_NOT_LOAD_PROPERTIES;
            if (*(*manager).priv_0).flags as ::core::ffi::c_uint
                & G_DBUS_OBJECT_MANAGER_CLIENT_FLAGS_DO_NOT_AUTO_START as ::core::ffi::c_int
                    as ::core::ffi::c_uint
                != 0
            {
                proxy_flags = ::core::mem::transmute::<::core::ffi::c_uint, GDBusProxyFlags>(
                    proxy_flags as ::core::ffi::c_uint
                        | G_DBUS_PROXY_FLAGS_DO_NOT_AUTO_START as ::core::ffi::c_int
                            as ::core::ffi::c_uint,
                );
            }
            (*(*manager).priv_0).control_proxy = g_dbus_proxy_new_sync(
                (*(*manager).priv_0).connection,
                proxy_flags,
                ::core::ptr::null_mut::<GDBusInterfaceInfo>(),
                (*(*manager).priv_0).name,
                (*(*manager).priv_0).object_path,
                b"org.freedesktop.DBus.ObjectManager\0" as *const u8 as *const gchar,
                cancellable,
                error,
            );
            if !(*(*manager).priv_0).control_proxy.is_null() {
                (*(*manager).priv_0).name_owner_signal_id = g_signal_connect_data(
                    (*(*manager).priv_0).control_proxy as *mut ::core::ffi::c_void as *mut GObject
                        as gpointer,
                    b"notify::g-name-owner\0" as *const u8 as *const gchar,
                    ::core::mem::transmute::<
                        Option<unsafe extern "C" fn(*mut GObject, *mut GParamSpec, gpointer) -> ()>,
                        GCallback,
                    >(Some(
                        safe_c2rust_on_notify_g_name_owner
                            as unsafe extern "C" fn(*mut GObject, *mut GParamSpec, gpointer) -> (),
                    )),
                    safe_c2rust_weak_ref_new(manager as *mut ::core::ffi::c_void as *mut GObject)
                        as gpointer,
                    ::core::mem::transmute::<
                        Option<unsafe extern "C" fn(*mut GWeakRef) -> ()>,
                        GClosureNotify,
                    >(Some(
                        safe_c2rust_weak_ref_free as unsafe extern "C" fn(*mut GWeakRef) -> (),
                    )),
                    G_CONNECT_DEFAULT,
                );
                (*(*manager).priv_0).signal_signal_id = g_signal_connect_data(
                    (*(*manager).priv_0).control_proxy as gpointer,
                    b"g-signal\0" as *const u8 as *const gchar,
                    ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut GDBusProxy,
                                *const gchar,
                                *const gchar,
                                *mut GVariant,
                                gpointer,
                            ) -> (),
                        >,
                        GCallback,
                    >(Some(
                        safe_c2rust_on_control_proxy_g_signal
                            as unsafe extern "C" fn(
                                *mut GDBusProxy,
                                *const gchar,
                                *const gchar,
                                *mut GVariant,
                                gpointer,
                            ) -> (),
                    )),
                    safe_c2rust_weak_ref_new(manager as *mut ::core::ffi::c_void as *mut GObject)
                        as gpointer,
                    ::core::mem::transmute::<
                        Option<unsafe extern "C" fn(*mut GWeakRef) -> ()>,
                        GClosureNotify,
                    >(Some(
                        safe_c2rust_weak_ref_free as unsafe extern "C" fn(*mut GWeakRef) -> (),
                    )),
                    G_CONNECT_DEFAULT,
                );
                (*(*manager).priv_0).name_owner =
                    g_dbus_proxy_get_name_owner((*(*manager).priv_0).control_proxy);
                if (*(*manager).priv_0).name_owner.is_null() && !(*(*manager).priv_0).name.is_null()
                {
                    current_block = 14434620278749266018;
                } else {
                    safe_c2rust_subscribe_signals(manager, (*(*manager).priv_0).name_owner);
                    value = g_dbus_proxy_call_sync(
                        (*(*manager).priv_0).control_proxy,
                        b"GetManagedObjects\0" as *const u8 as *const gchar,
                        ::core::ptr::null_mut::<GVariant>(),
                        G_DBUS_CALL_FLAGS_NONE,
                        -(1 as gint),
                        cancellable,
                        error,
                    );
                    if value.is_null() {
                        safe_c2rust_maybe_unsubscribe_signals(manager);
                        if !(({
                            let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
                            if (*(*manager).priv_0).signal_signal_id != 0 as gulong {
                                _g_boolean_var_41 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_41 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_41
                        }) as ::core::ffi::c_long
                            != 0)
                        {
                            g_warn_message(
                                G_LOG_DOMAIN.as_ptr(),
                                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectmanagerclient.c\0"
                                    as *const u8 as *const ::core::ffi::c_char,
                                1476 as ::core::ffi::c_int,
                                G_STRFUNC,
                                b"manager->priv->signal_signal_id != 0\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                        }
                        g_signal_handler_disconnect(
                            (*(*manager).priv_0).control_proxy as gpointer,
                            (*(*manager).priv_0).signal_signal_id,
                        );
                        (*(*manager).priv_0).signal_signal_id = 0 as gulong;
                        if !(({
                            let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
                            if (*(*manager).priv_0).name_owner_signal_id != 0 as gulong {
                                _g_boolean_var_42 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_42 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_42
                        }) as ::core::ffi::c_long
                            != 0)
                        {
                            g_warn_message(
                                G_LOG_DOMAIN.as_ptr(),
                                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectmanagerclient.c\0"
                                    as *const u8 as *const ::core::ffi::c_char,
                                1481 as ::core::ffi::c_int,
                                G_STRFUNC,
                                b"manager->priv->name_owner_signal_id != 0\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                        }
                        g_signal_handler_disconnect(
                            (*(*manager).priv_0).control_proxy as gpointer,
                            (*(*manager).priv_0).name_owner_signal_id,
                        );
                        (*(*manager).priv_0).name_owner_signal_id = 0 as gulong;
                        g_object_unref((*(*manager).priv_0).control_proxy as gpointer);
                        (*(*manager).priv_0).control_proxy = ::core::ptr::null_mut::<GDBusProxy>();
                        current_block = 12912888199596355060;
                    } else {
                        safe_c2rust_process_get_all_result(
                            manager,
                            value,
                            (*(*manager).priv_0).name_owner,
                        );
                        g_variant_unref(value);
                        current_block = 14434620278749266018;
                    }
                }
                match current_block {
                    12912888199596355060 => {}
                    _ => {
                        ret = TRUE as gboolean;
                    }
                }
            }
        }
        _ => {}
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust_initable_iface_init(mut initable_iface: *mut GInitableIface) {
    (*initable_iface).init = Some(
        safe_c2rust_initable_init
            as unsafe extern "C" fn(
                *mut GInitable,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut GInitable, *mut GCancellable, *mut *mut GError) -> gboolean,
        >;
}
unsafe extern "C" fn safe_c2rust_async_initable_iface_init(
    mut async_initable_iface: *mut GAsyncInitableIface,
) {
}
unsafe extern "C" fn safe_c2rust_add_interfaces(
    mut manager: *mut GDBusObjectManagerClient,
    mut object_path: *const gchar,
    mut ifaces_and_properties: *mut GVariant,
    mut name_owner: *const gchar,
) {
    let mut op: *mut GDBusObjectProxy = ::core::ptr::null_mut::<GDBusObjectProxy>();
    let mut added: gboolean = 0;
    let mut iter: GVariantIter = _GVariantIter { x: [0; 16] };
    let mut interface_name: *const gchar = ::core::ptr::null::<gchar>();
    let mut properties: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut interface_added_signals: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut interface_proxy: *mut GDBusProxy = ::core::ptr::null_mut::<GDBusProxy>();
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if name_owner.is_null() || g_dbus_is_unique_name(name_owner) != 0 {
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
            b"name_owner == NULL || g_dbus_is_unique_name (name_owner)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut (*(*manager).priv_0).lock);
    interface_added_signals = ::core::ptr::null_mut::<GList>();
    added = FALSE as gboolean;
    op = g_hash_table_lookup(
        (*(*manager).priv_0).map_object_path_to_object_proxy,
        object_path as gconstpointer,
    ) as *mut GDBusObjectProxy;
    if op.is_null() {
        let mut object_proxy_type: GType = 0;
        if (*(*manager).priv_0).get_proxy_type_func.is_some() {
            object_proxy_type = (*(*manager).priv_0)
                .get_proxy_type_func
                .expect("non-null function pointer")(
                manager,
                object_path,
                ::core::ptr::null::<gchar>(),
                (*(*manager).priv_0).get_proxy_type_user_data,
            );
            if !(({
                let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
                if object_proxy_type == g_dbus_object_proxy_get_type()
                    || g_type_is_a(object_proxy_type, g_dbus_object_proxy_get_type()) != 0
                {
                    _g_boolean_var_44 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_44 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_44
            }) as ::core::ffi::c_long
                != 0)
            {
                g_warn_message(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectmanagerclient.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    1547 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"g_type_is_a (object_proxy_type, G_TYPE_DBUS_OBJECT_PROXY)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        } else {
            object_proxy_type = g_dbus_object_proxy_get_type();
        }
        op = g_object_new(
            object_proxy_type,
            b"g-connection\0" as *const u8 as *const gchar,
            (*(*manager).priv_0).connection,
            b"g-object-path\0" as *const u8 as *const ::core::ffi::c_char,
            object_path,
            NULL_0,
        ) as *mut GDBusObjectProxy;
        added = TRUE as gboolean;
    }
    g_object_ref(op as gpointer);
    g_variant_iter_init(&raw mut iter, ifaces_and_properties);
    while g_variant_iter_next(
        &raw mut iter,
        b"{&s@a{sv}}\0" as *const u8 as *const gchar,
        &raw mut interface_name,
        &raw mut properties,
    ) != 0
    {
        let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
        let mut interface_proxy_type: GType = 0;
        if (*(*manager).priv_0).get_proxy_type_func.is_some() {
            interface_proxy_type = (*(*manager).priv_0)
                .get_proxy_type_func
                .expect("non-null function pointer")(
                manager,
                object_path,
                interface_name,
                (*(*manager).priv_0).get_proxy_type_user_data,
            );
            if !(({
                let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
                if interface_proxy_type == g_dbus_proxy_get_type()
                    || g_type_is_a(interface_proxy_type, g_dbus_proxy_get_type()) != 0
                {
                    _g_boolean_var_45 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_45 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_45
            }) as ::core::ffi::c_long
                != 0)
            {
                g_warn_message(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectmanagerclient.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    1576 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"g_type_is_a (interface_proxy_type, G_TYPE_DBUS_PROXY)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        } else {
            interface_proxy_type = g_dbus_proxy_get_type();
        }
        error = ::core::ptr::null_mut::<GError>();
        interface_proxy = g_initable_new(
            interface_proxy_type,
            ::core::ptr::null_mut::<GCancellable>(),
            &raw mut error,
            b"g-connection\0" as *const u8 as *const gchar,
            (*(*manager).priv_0).connection,
            b"g-flags\0" as *const u8 as *const ::core::ffi::c_char,
            G_DBUS_PROXY_FLAGS_DO_NOT_LOAD_PROPERTIES as ::core::ffi::c_int
                | G_DBUS_PROXY_FLAGS_DO_NOT_CONNECT_SIGNALS as ::core::ffi::c_int,
            b"g-name\0" as *const u8 as *const ::core::ffi::c_char,
            name_owner,
            b"g-object-path\0" as *const u8 as *const ::core::ffi::c_char,
            object_path,
            b"g-interface-name\0" as *const u8 as *const ::core::ffi::c_char,
            interface_name,
            NULL_0,
        ) as *mut GDBusProxy;
        if interface_proxy.is_null() {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"%s: Error constructing proxy for path %s and interface %s: %s\0"
                    as *const u8 as *const gchar,
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectmanagerclient.c:1600\0"
                    as *const u8 as *const ::core::ffi::c_char,
                object_path,
                interface_name,
                (*error).message,
            );
            g_error_free(error);
        } else {
            let mut property_iter: GVariantIter = _GVariantIter { x: [0; 16] };
            let mut property_name: *const gchar = ::core::ptr::null::<gchar>();
            let mut property_value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
            g_dbus_interface_set_object(
                interface_proxy as *mut ::core::ffi::c_void as *mut GDBusInterface,
                op as *mut ::core::ffi::c_void as *mut GDBusObject,
            );
            g_variant_iter_init(&raw mut property_iter, properties);
            while g_variant_iter_next(
                &raw mut property_iter,
                b"{&sv}\0" as *const u8 as *const gchar,
                &raw mut property_name,
                &raw mut property_value,
            ) != 0
            {
                g_dbus_proxy_set_cached_property(interface_proxy, property_name, property_value);
                g_variant_unref(property_value);
            }
            _g_dbus_object_proxy_add_interface(op, interface_proxy);
            if added == 0 {
                interface_added_signals = g_list_append(
                    interface_added_signals,
                    g_object_ref(interface_proxy as gpointer) as *mut GDBusProxy as gpointer,
                );
            }
            g_object_unref(interface_proxy as gpointer);
        }
        g_variant_unref(properties);
    }
    if added != 0 {
        g_hash_table_insert(
            (*(*manager).priv_0).map_object_path_to_object_proxy,
            safe_c2rust_g_strdup_inline(object_path as *const ::core::ffi::c_char) as gpointer,
            op as gpointer,
        );
    }
    g_mutex_unlock(&raw mut (*(*manager).priv_0).lock);
    g_object_ref(manager as gpointer);
    l = interface_added_signals;
    while !l.is_null() {
        interface_proxy = (*l).data as *mut GDBusProxy;
        g_signal_emit_by_name(
            manager as gpointer,
            b"interface-added\0" as *const u8 as *const gchar,
            op,
            interface_proxy,
        );
        g_object_unref(interface_proxy as gpointer);
        l = (*l).next;
    }
    g_list_free(interface_added_signals);
    if added != 0 {
        g_signal_emit_by_name(
            manager as gpointer,
            b"object-added\0" as *const u8 as *const gchar,
            op,
        );
    }
    g_object_unref(manager as gpointer);
    g_object_unref(op as gpointer);
}
unsafe extern "C" fn safe_c2rust_remove_interfaces(
    mut manager: *mut GDBusObjectManagerClient,
    mut object_path: *const gchar,
    mut interface_names: *const *const gchar,
) {
    let mut op: *mut GDBusObjectProxy = ::core::ptr::null_mut::<GDBusObjectProxy>();
    let mut interfaces: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut n: guint = 0;
    let mut num_interfaces: guint = 0;
    let mut num_interfaces_to_remove: guint = 0;
    g_mutex_lock(&raw mut (*(*manager).priv_0).lock);
    op = g_hash_table_lookup(
        (*(*manager).priv_0).map_object_path_to_object_proxy,
        object_path as gconstpointer,
    ) as *mut GDBusObjectProxy;
    if op.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_DEBUG,
            b"%s: Processing InterfaceRemoved signal for path %s but no object proxy exists\0"
                as *const u8 as *const gchar,
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectmanagerclient.c:1679\0"
                as *const u8 as *const ::core::ffi::c_char,
            object_path,
        );
        g_mutex_unlock(&raw mut (*(*manager).priv_0).lock);
        return;
    }
    interfaces = g_dbus_object_get_interfaces(op as *mut ::core::ffi::c_void as *mut GDBusObject);
    num_interfaces = g_list_length(interfaces);
    g_list_free_full(
        interfaces,
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
    num_interfaces_to_remove = g_strv_length(interface_names as *mut *mut gchar);
    g_object_ref(manager as gpointer);
    if num_interfaces_to_remove == num_interfaces {
        g_object_ref(op as gpointer);
        if !(({
            let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
            if g_hash_table_remove(
                (*(*manager).priv_0).map_object_path_to_object_proxy,
                object_path as gconstpointer,
            ) != 0
            {
                _g_boolean_var_46 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_46 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_46
        }) as ::core::ffi::c_long
            != 0)
        {
            g_warn_message(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectmanagerclient.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1696 as ::core::ffi::c_int,
                G_STRFUNC,
                b"g_hash_table_remove (manager->priv->map_object_path_to_object_proxy, object_path)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        g_mutex_unlock(&raw mut (*(*manager).priv_0).lock);
        g_signal_emit_by_name(
            manager as gpointer,
            b"object-removed\0" as *const u8 as *const gchar,
            op,
        );
        g_object_unref(op as gpointer);
    } else {
        g_object_ref(op as gpointer);
        g_mutex_unlock(&raw mut (*(*manager).priv_0).lock);
        n = 0 as guint;
        while !interface_names.is_null() && !(*interface_names.offset(n as isize)).is_null() {
            let mut interface: *mut GDBusInterface = ::core::ptr::null_mut::<GDBusInterface>();
            interface = g_dbus_object_get_interface(
                op as *mut ::core::ffi::c_void as *mut GDBusObject,
                *interface_names.offset(n as isize),
            );
            _g_dbus_object_proxy_remove_interface(op, *interface_names.offset(n as isize));
            if !interface.is_null() {
                g_signal_emit_by_name(
                    manager as gpointer,
                    b"interface-removed\0" as *const u8 as *const gchar,
                    op,
                    interface,
                );
                g_object_unref(interface as gpointer);
            }
            n = n.wrapping_add(1);
        }
        g_object_unref(op as gpointer);
    }
    g_object_unref(manager as gpointer);
}
unsafe extern "C" fn safe_c2rust_process_get_all_result(
    mut manager: *mut GDBusObjectManagerClient,
    mut value: *mut GVariant,
    mut name_owner: *const gchar,
) {
    let mut arg0: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut object_path: *const gchar = ::core::ptr::null::<gchar>();
    let mut ifaces_and_properties: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut iter: GVariantIter = _GVariantIter { x: [0; 16] };
    if ({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
        if name_owner.is_null() || g_dbus_is_unique_name(name_owner) != 0 {
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
            b"name_owner == NULL || g_dbus_is_unique_name (name_owner)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    arg0 = g_variant_get_child_value(value, 0 as gsize);
    g_variant_iter_init(&raw mut iter, arg0);
    while g_variant_iter_next(
        &raw mut iter,
        b"{&o@a{sa{sv}}}\0" as *const u8 as *const gchar,
        &raw mut object_path,
        &raw mut ifaces_and_properties,
    ) != 0
    {
        safe_c2rust_add_interfaces(manager, object_path, ifaces_and_properties, name_owner);
        g_variant_unref(ifaces_and_properties);
    }
    g_variant_unref(arg0);
}
unsafe extern "C" fn safe_c2rust_on_control_proxy_g_signal(
    mut proxy: *mut GDBusProxy,
    mut sender_name: *const gchar,
    mut signal_name: *const gchar,
    mut parameters: *mut GVariant,
    mut user_data: gpointer,
) {
    let mut manager_weak: *mut GWeakRef = user_data as *mut GWeakRef;
    let mut manager: *mut GDBusObjectManagerClient =
        ::core::ptr::null_mut::<GDBusObjectManagerClient>();
    let mut object_path: *const gchar = ::core::ptr::null::<gchar>();
    manager = g_weak_ref_get(manager_weak) as *mut GDBusObjectManagerClient;
    if manager.is_null() {
        return;
    }
    if g_strcmp0(
        signal_name as *const ::core::ffi::c_char,
        b"InterfacesAdded\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut ifaces_and_properties: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        g_variant_get(
            parameters,
            b"(&o@a{sa{sv}})\0" as *const u8 as *const gchar,
            &raw mut object_path,
            &raw mut ifaces_and_properties,
        );
        safe_c2rust_add_interfaces(
            manager,
            object_path,
            ifaces_and_properties,
            (*(*manager).priv_0).name_owner,
        );
        g_variant_unref(ifaces_and_properties);
    } else if g_strcmp0(
        signal_name as *const ::core::ffi::c_char,
        b"InterfacesRemoved\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut ifaces: *mut *const gchar = ::core::ptr::null_mut::<*const gchar>();
        g_variant_get(
            parameters,
            b"(&o^a&s)\0" as *const u8 as *const gchar,
            &raw mut object_path,
            &raw mut ifaces,
        );
        safe_c2rust_remove_interfaces(manager, object_path, ifaces);
        g_free(ifaces as gpointer);
    }
    g_object_unref(manager as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_client_get_object_path(
    mut _manager: *mut GDBusObjectManager,
) -> *const gchar {
    let mut manager: *mut GDBusObjectManagerClient =
        _manager as *mut ::core::ffi::c_void as *mut GDBusObjectManagerClient;
    return (*(*manager).priv_0).object_path;
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_client_get_object(
    mut _manager: *mut GDBusObjectManager,
    mut object_path: *const gchar,
) -> *mut GDBusObject {
    let mut manager: *mut GDBusObjectManagerClient =
        _manager as *mut ::core::ffi::c_void as *mut GDBusObjectManagerClient;
    let mut ret: *mut GDBusObject = ::core::ptr::null_mut::<GDBusObject>();
    g_mutex_lock(&raw mut (*(*manager).priv_0).lock);
    ret = g_hash_table_lookup(
        (*(*manager).priv_0).map_object_path_to_object_proxy,
        object_path as gconstpointer,
    ) as *mut GDBusObject;
    if !ret.is_null() {
        g_object_ref(ret as gpointer);
    }
    g_mutex_unlock(&raw mut (*(*manager).priv_0).lock);
    return ret;
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_client_get_interface(
    mut _manager: *mut GDBusObjectManager,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
) -> *mut GDBusInterface {
    let mut ret: *mut GDBusInterface = ::core::ptr::null_mut::<GDBusInterface>();
    let mut object: *mut GDBusObject = ::core::ptr::null_mut::<GDBusObject>();
    ret = ::core::ptr::null_mut::<GDBusInterface>();
    object = g_dbus_object_manager_get_object(_manager, object_path);
    if !object.is_null() {
        ret = g_dbus_object_get_interface(object, interface_name);
        g_object_unref(object as gpointer);
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_client_get_objects(
    mut _manager: *mut GDBusObjectManager,
) -> *mut GList {
    let mut manager: *mut GDBusObjectManagerClient =
        _manager as *mut ::core::ffi::c_void as *mut GDBusObjectManagerClient;
    let mut ret: *mut GList = ::core::ptr::null_mut::<GList>();
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = manager as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_object_manager_client_get_type();
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
            b"G_IS_DBUS_OBJECT_MANAGER_CLIENT (manager)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    g_mutex_lock(&raw mut (*(*manager).priv_0).lock);
    ret = g_hash_table_get_values((*(*manager).priv_0).map_object_path_to_object_proxy);
    g_list_foreach(
        ret,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> gpointer>, GFunc>(Some(
            g_object_ref as unsafe extern "C" fn(gpointer) -> gpointer,
        )),
        NULL_0,
    );
    g_mutex_unlock(&raw mut (*(*manager).priv_0).lock);
    return ret;
}
unsafe extern "C" fn safe_c2rust_dbus_object_manager_interface_init(
    mut iface: *mut GDBusObjectManagerIface,
) {
    (*iface).get_object_path = Some(
        safe_c2rust_g_dbus_object_manager_client_get_object_path
            as unsafe extern "C" fn(*mut GDBusObjectManager) -> *const gchar,
    )
        as Option<unsafe extern "C" fn(*mut GDBusObjectManager) -> *const gchar>;
    (*iface).get_objects = Some(
        safe_c2rust_g_dbus_object_manager_client_get_objects
            as unsafe extern "C" fn(*mut GDBusObjectManager) -> *mut GList,
    )
        as Option<unsafe extern "C" fn(*mut GDBusObjectManager) -> *mut GList>;
    (*iface).get_object = Some(
        safe_c2rust_g_dbus_object_manager_client_get_object
            as unsafe extern "C" fn(*mut GDBusObjectManager, *const gchar) -> *mut GDBusObject,
    )
        as Option<unsafe extern "C" fn(*mut GDBusObjectManager, *const gchar) -> *mut GDBusObject>;
    (*iface).get_interface = Some(
        safe_c2rust_g_dbus_object_manager_client_get_interface
            as unsafe extern "C" fn(
                *mut GDBusObjectManager,
                *const gchar,
                *const gchar,
            ) -> *mut GDBusInterface,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GDBusObjectManager,
                *const gchar,
                *const gchar,
            ) -> *mut GDBusInterface,
        >;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
