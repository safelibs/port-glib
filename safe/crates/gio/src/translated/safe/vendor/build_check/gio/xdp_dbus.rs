use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GAsyncResult;
    pub type _GAsyncInitable;
    pub type _GCancellablePrivate;
    pub type _GInitable;
    pub type _GUnixFDListPrivate;
    pub type _GDBusMessage;
    pub type _GDBusConnection;
    pub type _GDBusProxyPrivate;
    pub type _GDBusMethodInvocation;
    pub type _GDBusInterfaceSkeletonPrivate;
    pub type _GXdpDocuments;
    pub type _GXdpOpenURI;
    pub type _GXdpProxyResolver;
    pub type _GXdpTrash;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn g_quark_try_string(string: *const gchar) -> GQuark;
    fn g_quark_to_string(quark: GQuark) -> *const gchar;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_mutex_init(mutex: *mut GMutex);
    fn g_mutex_clear(mutex: *mut GMutex);
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_datalist_clear(datalist: *mut *mut GData);
    fn g_datalist_id_set_data_full(
        datalist: *mut *mut GData,
        key_id: GQuark,
        data: gpointer,
        destroy_func: GDestroyNotify,
    );
    fn g_free(mem: gpointer);
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_list_free_full(list: *mut GList, free_func: GDestroyNotify);
    fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_main_context_unref(context: *mut GMainContext);
    fn g_main_context_ref_thread_default() -> *mut GMainContext;
    fn g_source_unref(source: *mut GSource);
    fn g_source_attach(source: *mut GSource, context: *mut GMainContext) -> guint;
    fn g_source_destroy(source: *mut GSource);
    fn g_source_set_priority(source: *mut GSource, priority: gint);
    fn g_source_set_callback(
        source: *mut GSource,
        func: GSourceFunc,
        data: gpointer,
        notify: GDestroyNotify,
    );
    fn g_source_set_name(source: *mut GSource, name: *const ::core::ffi::c_char);
    fn g_idle_source_new() -> *mut GSource;
    fn g_strv_length(str_array: *mut *mut gchar) -> guint;
    fn g_variant_type_checked_(type_string: *const gchar) -> *const GVariantType;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_ref_sink(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_take_ref(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_get_uint32(value: *mut GVariant) -> guint32;
    fn g_variant_n_children(value: *mut GVariant) -> gsize;
    fn g_variant_equal(one: gconstpointer, two: gconstpointer) -> gboolean;
    fn g_variant_iter_init(iter: *mut GVariantIter, value: *mut GVariant) -> gsize;
    fn g_variant_iter_free(iter: *mut GVariantIter);
    fn g_variant_iter_next_value(iter: *mut GVariantIter) -> *mut GVariant;
    fn g_variant_iter_next(iter: *mut GVariantIter, format_string: *const gchar, ...) -> gboolean;
    fn g_variant_builder_init(builder: *mut GVariantBuilder, type_0: *const GVariantType);
    fn g_variant_builder_end(builder: *mut GVariantBuilder) -> *mut GVariant;
    fn g_variant_builder_clear(builder: *mut GVariantBuilder);
    fn g_variant_builder_add(builder: *mut GVariantBuilder, format_string: *const gchar, ...);
    fn g_variant_new(format_string: *const gchar, ...) -> *mut GVariant;
    fn g_variant_get(value: *mut GVariant, format_string: *const gchar, ...);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
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
    fn g_type_class_peek_parent(g_class: gpointer) -> gpointer;
    fn g_type_interface_peek(instance_class: gpointer, iface_type: GType) -> gpointer;
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
    fn g_type_interface_add_prerequisite(interface_type: GType, prerequisite_type: GType);
    fn g_type_add_instance_private(class_type: GType, private_size: gsize) -> gint;
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_value_init(value: *mut GValue, g_type: GType) -> *mut GValue;
    fn g_value_copy(src_value: *const GValue, dest_value: *mut GValue);
    fn g_value_unset(value: *mut GValue);
    fn g_value_peek_pointer(value: *const GValue) -> gpointer;
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
    fn g_signal_emitv(
        instance_and_params: *const GValue,
        signal_id: guint,
        detail: GQuark,
        return_value: *mut GValue,
    );
    fn g_signal_lookup(name: *const gchar, itype: GType) -> guint;
    fn g_signal_accumulator_true_handled(
        ihint: *mut GSignalInvocationHint,
        return_accu: *mut GValue,
        handler_return: *const GValue,
        dummy: gpointer,
    ) -> gboolean;
    fn g_strv_get_type() -> GType;
    fn g_value_get_boxed(value: *const GValue) -> gpointer;
    fn g_object_class_find_property(
        oclass: *mut GObjectClass,
        property_name: *const gchar,
    ) -> *mut GParamSpec;
    fn g_object_class_override_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        name: *const gchar,
    );
    fn g_object_interface_install_property(g_iface: gpointer, pspec: *mut GParamSpec);
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_set(object: gpointer, first_property_name: *const gchar, ...);
    fn g_object_set_property(
        object: *mut GObject,
        property_name: *const gchar,
        value: *const GValue,
    );
    fn g_object_get_property(object: *mut GObject, property_name: *const gchar, value: *mut GValue);
    fn g_object_freeze_notify(object: *mut GObject);
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
    fn g_object_notify_by_pspec(object: *mut GObject, pspec: *mut GParamSpec);
    fn g_object_thaw_notify(object: *mut GObject);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_value_set_object(value: *mut GValue, v_object: gpointer);
    fn g_param_spec_uint(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        minimum: guint,
        maximum: guint,
        default_value: guint,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_value_get_uchar(value: *const GValue) -> guchar;
    fn g_value_set_boolean(value: *mut GValue, v_boolean: gboolean);
    fn g_value_get_boolean(value: *const GValue) -> gboolean;
    fn g_value_get_int(value: *const GValue) -> gint;
    fn g_value_get_uint(value: *const GValue) -> guint;
    fn g_value_get_int64(value: *const GValue) -> gint64;
    fn g_value_get_uint64(value: *const GValue) -> guint64;
    fn g_value_get_double(value: *const GValue) -> gdouble;
    fn g_value_get_string(value: *const GValue) -> *const gchar;
    fn g_value_set_variant(value: *mut GValue, variant: *mut GVariant);
    fn g_value_get_variant(value: *const GValue) -> *mut GVariant;
    fn g_initable_new(
        object_type: GType,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
        first_property_name: *const gchar,
        ...
    ) -> gpointer;
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
    fn g_async_result_get_source_object(res: *mut GAsyncResult) -> *mut GObject;
    fn g_dbus_connection_emit_signal(
        connection: *mut GDBusConnection,
        destination_bus_name: *const gchar,
        object_path: *const gchar,
        interface_name: *const gchar,
        signal_name: *const gchar,
        parameters: *mut GVariant,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_dbus_error_quark() -> GQuark;
    fn g_dbus_interface_skeleton_get_type() -> GType;
    fn g_dbus_interface_skeleton_get_connection(
        interface_: *mut GDBusInterfaceSkeleton,
    ) -> *mut GDBusConnection;
    fn g_dbus_interface_skeleton_get_connections(
        interface_: *mut GDBusInterfaceSkeleton,
    ) -> *mut GList;
    fn g_dbus_interface_skeleton_get_object_path(
        interface_: *mut GDBusInterfaceSkeleton,
    ) -> *const gchar;
    fn g_dbus_interface_info_lookup_signal(
        info: *mut GDBusInterfaceInfo,
        name: *const gchar,
    ) -> *mut GDBusSignalInfo;
    fn g_dbus_interface_info_lookup_property(
        info: *mut GDBusInterfaceInfo,
        name: *const gchar,
    ) -> *mut GDBusPropertyInfo;
    fn g_dbus_message_get_unix_fd_list(message: *mut GDBusMessage) -> *mut GUnixFDList;
    fn g_dbus_method_invocation_get_type() -> GType;
    fn g_dbus_method_invocation_get_method_info(
        invocation: *mut GDBusMethodInvocation,
    ) -> *const GDBusMethodInfo;
    fn g_dbus_method_invocation_get_message(
        invocation: *mut GDBusMethodInvocation,
    ) -> *mut GDBusMessage;
    fn g_dbus_method_invocation_return_value(
        invocation: *mut GDBusMethodInvocation,
        parameters: *mut GVariant,
    );
    fn g_dbus_method_invocation_return_value_with_unix_fd_list(
        invocation: *mut GDBusMethodInvocation,
        parameters: *mut GVariant,
        fd_list: *mut GUnixFDList,
    );
    fn g_dbus_method_invocation_return_error(
        invocation: *mut GDBusMethodInvocation,
        domain: GQuark,
        code: gint,
        format: *const gchar,
        ...
    );
    fn g_dbus_proxy_get_type() -> GType;
    fn g_dbus_proxy_set_interface_info(proxy: *mut GDBusProxy, info: *mut GDBusInterfaceInfo);
    fn g_dbus_proxy_get_cached_property(
        proxy: *mut GDBusProxy,
        property_name: *const gchar,
    ) -> *mut GVariant;
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
    fn g_dbus_proxy_call_with_unix_fd_list(
        proxy: *mut GDBusProxy,
        method_name: *const gchar,
        parameters: *mut GVariant,
        flags: GDBusCallFlags,
        timeout_msec: gint,
        fd_list: *mut GUnixFDList,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_dbus_proxy_call_with_unix_fd_list_finish(
        proxy: *mut GDBusProxy,
        out_fd_list: *mut *mut GUnixFDList,
        res: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> *mut GVariant;
    fn g_dbus_proxy_call_with_unix_fd_list_sync(
        proxy: *mut GDBusProxy,
        method_name: *const gchar,
        parameters: *mut GVariant,
        flags: GDBusCallFlags,
        timeout_msec: gint,
        fd_list: *mut GUnixFDList,
        out_fd_list: *mut *mut GUnixFDList,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GVariant;
    fn g_dbus_gvariant_to_gvalue(value: *mut GVariant, out_gvalue: *mut GValue);
    fn g_dbus_gvalue_to_gvariant(
        gvalue: *const GValue,
        type_0: *const GVariantType,
    ) -> *mut GVariant;
    fn g_unix_fd_list_get_type() -> GType;
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
pub type guchar = ::core::ffi::c_uchar;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gfloat = ::core::ffi::c_float;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type GMainContext = _GMainContext;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSource {
    pub callback_data: gpointer,
    pub callback_funcs: *mut GSourceCallbackFuncs,
    pub source_funcs: *const GSourceFuncs,
    pub ref_count: guint,
    pub context: *mut GMainContext,
    pub priority: gint,
    pub flags: guint,
    pub source_id: guint,
    pub poll_fds: *mut GSList,
    pub prev: *mut GSource,
    pub next: *mut GSource,
    pub name: *mut ::core::ffi::c_char,
    pub priv_0: *mut GSourcePrivate,
}
pub type GSourcePrivate = _GSourcePrivate;
pub type GSource = _GSource;
pub type GSourceFuncs = _GSourceFuncs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSourceFuncs {
    pub prepare: Option<unsafe extern "C" fn(*mut GSource, *mut gint) -> gboolean>,
    pub check: Option<unsafe extern "C" fn(*mut GSource) -> gboolean>,
    pub dispatch: Option<unsafe extern "C" fn(*mut GSource, GSourceFunc, gpointer) -> gboolean>,
    pub finalize: Option<unsafe extern "C" fn(*mut GSource) -> ()>,
    pub closure_callback: GSourceFunc,
    pub closure_marshal: GSourceDummyMarshal,
}
pub type GSourceDummyMarshal = Option<unsafe extern "C" fn() -> ()>;
pub type GSourceFunc = Option<unsafe extern "C" fn(gpointer) -> gboolean>;
pub type GSourceCallbackFuncs = _GSourceCallbackFuncs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSourceCallbackFuncs {
    pub ref_0: Option<unsafe extern "C" fn(gpointer) -> ()>,
    pub unref: Option<unsafe extern "C" fn(gpointer) -> ()>,
    pub get:
        Option<unsafe extern "C" fn(gpointer, *mut GSource, *mut GSourceFunc, *mut gpointer) -> ()>,
}
pub type GVariantType = _GVariantType;
pub type GVariant = _GVariant;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVariantIter {
    pub x: [guintptr; 16],
}
pub type GVariantIter = _GVariantIter;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVariantBuilder {
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub s: C2RustUnnamed_0,
    pub x: [guintptr; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub partial_magic: gsize,
    pub type_0: *const GVariantType,
    pub y: [guintptr; 14],
}
pub type GVariantBuilder = _GVariantBuilder;
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
    pub data: [C2RustUnnamed_1; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCClosure {
    pub closure: GClosure,
    pub callback: gpointer,
}
pub type GCClosure = _GCClosure;
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
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const G_DBUS_ERROR_PROPERTY_READ_ONLY: C2RustUnnamed_2 = 44;
pub const G_DBUS_ERROR_UNKNOWN_PROPERTY: C2RustUnnamed_2 = 43;
pub const G_DBUS_ERROR_UNKNOWN_INTERFACE: C2RustUnnamed_2 = 42;
pub const G_DBUS_ERROR_UNKNOWN_OBJECT: C2RustUnnamed_2 = 41;
pub const G_DBUS_ERROR_OBJECT_PATH_IN_USE: C2RustUnnamed_2 = 40;
pub const G_DBUS_ERROR_ADT_AUDIT_DATA_UNKNOWN: C2RustUnnamed_2 = 39;
pub const G_DBUS_ERROR_SELINUX_SECURITY_CONTEXT_UNKNOWN: C2RustUnnamed_2 = 38;
pub const G_DBUS_ERROR_INVALID_FILE_CONTENT: C2RustUnnamed_2 = 37;
pub const G_DBUS_ERROR_INVALID_SIGNATURE: C2RustUnnamed_2 = 36;
pub const G_DBUS_ERROR_UNIX_PROCESS_ID_UNKNOWN: C2RustUnnamed_2 = 35;
pub const G_DBUS_ERROR_SPAWN_NO_MEMORY: C2RustUnnamed_2 = 34;
pub const G_DBUS_ERROR_SPAWN_FILE_INVALID: C2RustUnnamed_2 = 33;
pub const G_DBUS_ERROR_SPAWN_PERMISSIONS_INVALID: C2RustUnnamed_2 = 32;
pub const G_DBUS_ERROR_SPAWN_SERVICE_NOT_FOUND: C2RustUnnamed_2 = 31;
pub const G_DBUS_ERROR_SPAWN_SERVICE_INVALID: C2RustUnnamed_2 = 30;
pub const G_DBUS_ERROR_SPAWN_CONFIG_INVALID: C2RustUnnamed_2 = 29;
pub const G_DBUS_ERROR_SPAWN_SETUP_FAILED: C2RustUnnamed_2 = 28;
pub const G_DBUS_ERROR_SPAWN_FAILED: C2RustUnnamed_2 = 27;
pub const G_DBUS_ERROR_SPAWN_CHILD_SIGNALED: C2RustUnnamed_2 = 26;
pub const G_DBUS_ERROR_SPAWN_CHILD_EXITED: C2RustUnnamed_2 = 25;
pub const G_DBUS_ERROR_SPAWN_FORK_FAILED: C2RustUnnamed_2 = 24;
pub const G_DBUS_ERROR_SPAWN_EXEC_FAILED: C2RustUnnamed_2 = 23;
pub const G_DBUS_ERROR_MATCH_RULE_INVALID: C2RustUnnamed_2 = 22;
pub const G_DBUS_ERROR_MATCH_RULE_NOT_FOUND: C2RustUnnamed_2 = 21;
pub const G_DBUS_ERROR_TIMED_OUT: C2RustUnnamed_2 = 20;
pub const G_DBUS_ERROR_UNKNOWN_METHOD: C2RustUnnamed_2 = 19;
pub const G_DBUS_ERROR_FILE_EXISTS: C2RustUnnamed_2 = 18;
pub const G_DBUS_ERROR_FILE_NOT_FOUND: C2RustUnnamed_2 = 17;
pub const G_DBUS_ERROR_INVALID_ARGS: C2RustUnnamed_2 = 16;
pub const G_DBUS_ERROR_DISCONNECTED: C2RustUnnamed_2 = 15;
pub const G_DBUS_ERROR_ADDRESS_IN_USE: C2RustUnnamed_2 = 14;
pub const G_DBUS_ERROR_NO_NETWORK: C2RustUnnamed_2 = 13;
pub const G_DBUS_ERROR_TIMEOUT: C2RustUnnamed_2 = 12;
pub const G_DBUS_ERROR_NO_SERVER: C2RustUnnamed_2 = 11;
pub const G_DBUS_ERROR_AUTH_FAILED: C2RustUnnamed_2 = 10;
pub const G_DBUS_ERROR_ACCESS_DENIED: C2RustUnnamed_2 = 9;
pub const G_DBUS_ERROR_LIMITS_EXCEEDED: C2RustUnnamed_2 = 8;
pub const G_DBUS_ERROR_NOT_SUPPORTED: C2RustUnnamed_2 = 7;
pub const G_DBUS_ERROR_BAD_ADDRESS: C2RustUnnamed_2 = 6;
pub const G_DBUS_ERROR_IO_ERROR: C2RustUnnamed_2 = 5;
pub const G_DBUS_ERROR_NO_REPLY: C2RustUnnamed_2 = 4;
pub const G_DBUS_ERROR_NAME_HAS_NO_OWNER: C2RustUnnamed_2 = 3;
pub const G_DBUS_ERROR_SERVICE_UNKNOWN: C2RustUnnamed_2 = 2;
pub const G_DBUS_ERROR_NO_MEMORY: C2RustUnnamed_2 = 1;
pub const G_DBUS_ERROR_FAILED: C2RustUnnamed_2 = 0;
pub type GDBusCallFlags = ::core::ffi::c_uint;
pub const G_DBUS_CALL_FLAGS_ALLOW_INTERACTIVE_AUTHORIZATION: GDBusCallFlags = 2;
pub const G_DBUS_CALL_FLAGS_NO_AUTO_START: GDBusCallFlags = 1;
pub const G_DBUS_CALL_FLAGS_NONE: GDBusCallFlags = 0;
pub type GDBusPropertyInfoFlags = ::core::ffi::c_uint;
pub const G_DBUS_PROPERTY_INFO_FLAGS_WRITABLE: GDBusPropertyInfoFlags = 2;
pub const G_DBUS_PROPERTY_INFO_FLAGS_READABLE: GDBusPropertyInfoFlags = 1;
pub const G_DBUS_PROPERTY_INFO_FLAGS_NONE: GDBusPropertyInfoFlags = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixFDList {
    pub parent_instance: GObject,
    pub priv_0: *mut GUnixFDListPrivate,
}
pub type GUnixFDListPrivate = _GUnixFDListPrivate;
pub type GUnixFDList = _GUnixFDList;
pub type GDBusMessage = _GDBusMessage;
pub type GDBusConnection = _GDBusConnection;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusProxy {
    pub parent_instance: GObject,
    pub priv_0: *mut GDBusProxyPrivate,
}
pub type GDBusProxyPrivate = _GDBusProxyPrivate;
pub type GDBusProxy = _GDBusProxy;
pub type GDBusMethodInvocation = _GDBusMethodInvocation;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusInterfaceVTable {
    pub method_call: GDBusInterfaceMethodCallFunc,
    pub get_property: GDBusInterfaceGetPropertyFunc,
    pub set_property: GDBusInterfaceSetPropertyFunc,
    pub padding: [gpointer; 8],
}
pub type GDBusInterfaceSetPropertyFunc = Option<
    unsafe extern "C" fn(
        *mut GDBusConnection,
        *const gchar,
        *const gchar,
        *const gchar,
        *const gchar,
        *mut GVariant,
        *mut *mut GError,
        gpointer,
    ) -> gboolean,
>;
pub type GDBusInterfaceGetPropertyFunc = Option<
    unsafe extern "C" fn(
        *mut GDBusConnection,
        *const gchar,
        *const gchar,
        *const gchar,
        *const gchar,
        *mut *mut GError,
        gpointer,
    ) -> *mut GVariant,
>;
pub type GDBusInterfaceMethodCallFunc = Option<
    unsafe extern "C" fn(
        *mut GDBusConnection,
        *const gchar,
        *const gchar,
        *const gchar,
        *const gchar,
        *mut GVariant,
        *mut GDBusMethodInvocation,
        gpointer,
    ) -> (),
>;
pub type GDBusInterfaceVTable = _GDBusInterfaceVTable;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusInterfaceSkeleton {
    pub parent_instance: GObject,
    pub priv_0: *mut GDBusInterfaceSkeletonPrivate,
}
pub type GDBusInterfaceSkeletonPrivate = _GDBusInterfaceSkeletonPrivate;
pub type GDBusInterfaceSkeleton = _GDBusInterfaceSkeleton;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusInterfaceSkeletonClass {
    pub parent_class: GObjectClass,
    pub get_info:
        Option<unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GDBusInterfaceInfo>,
    pub get_vtable:
        Option<unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GDBusInterfaceVTable>,
    pub get_properties: Option<unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GVariant>,
    pub flush: Option<unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> ()>,
    pub vfunc_padding: [gpointer; 8],
    pub g_authorize_method: Option<
        unsafe extern "C" fn(*mut GDBusInterfaceSkeleton, *mut GDBusMethodInvocation) -> gboolean,
    >,
    pub signal_padding: [gpointer; 8],
}
pub type GDBusInterfaceSkeletonClass = _GDBusInterfaceSkeletonClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusProxyClass {
    pub parent_class: GObjectClass,
    pub g_properties_changed:
        Option<unsafe extern "C" fn(*mut GDBusProxy, *mut GVariant, *const *const gchar) -> ()>,
    pub g_signal: Option<
        unsafe extern "C" fn(*mut GDBusProxy, *const gchar, *const gchar, *mut GVariant) -> (),
    >,
    pub padding: [gpointer; 32],
}
pub type GDBusProxyClass = _GDBusProxyClass;
pub type GXdpDocuments = _GXdpDocuments;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GXdpDocumentsIface {
    pub parent_iface: GTypeInterface,
    pub handle_add: Option<
        unsafe extern "C" fn(
            *mut GXdpDocuments,
            *mut GDBusMethodInvocation,
            *mut GUnixFDList,
            *mut GVariant,
            gboolean,
            gboolean,
        ) -> gboolean,
    >,
    pub handle_add_full: Option<
        unsafe extern "C" fn(
            *mut GXdpDocuments,
            *mut GDBusMethodInvocation,
            *mut GUnixFDList,
            *mut GVariant,
            guint,
            *const gchar,
            *const *const gchar,
        ) -> gboolean,
    >,
    pub handle_add_named: Option<
        unsafe extern "C" fn(
            *mut GXdpDocuments,
            *mut GDBusMethodInvocation,
            *mut GUnixFDList,
            *mut GVariant,
            *const gchar,
            gboolean,
            gboolean,
        ) -> gboolean,
    >,
    pub handle_add_named_full: Option<
        unsafe extern "C" fn(
            *mut GXdpDocuments,
            *mut GDBusMethodInvocation,
            *mut GUnixFDList,
            *mut GVariant,
            *const gchar,
            guint,
            *const gchar,
            *const *const gchar,
        ) -> gboolean,
    >,
    pub handle_delete: Option<
        unsafe extern "C" fn(
            *mut GXdpDocuments,
            *mut GDBusMethodInvocation,
            *const gchar,
        ) -> gboolean,
    >,
    pub handle_get_mount_point:
        Option<unsafe extern "C" fn(*mut GXdpDocuments, *mut GDBusMethodInvocation) -> gboolean>,
    pub handle_grant_permissions: Option<
        unsafe extern "C" fn(
            *mut GXdpDocuments,
            *mut GDBusMethodInvocation,
            *const gchar,
            *const gchar,
            *const *const gchar,
        ) -> gboolean,
    >,
    pub handle_info: Option<
        unsafe extern "C" fn(
            *mut GXdpDocuments,
            *mut GDBusMethodInvocation,
            *const gchar,
        ) -> gboolean,
    >,
    pub handle_list: Option<
        unsafe extern "C" fn(
            *mut GXdpDocuments,
            *mut GDBusMethodInvocation,
            *const gchar,
        ) -> gboolean,
    >,
    pub handle_lookup: Option<
        unsafe extern "C" fn(
            *mut GXdpDocuments,
            *mut GDBusMethodInvocation,
            *const gchar,
        ) -> gboolean,
    >,
    pub handle_revoke_permissions: Option<
        unsafe extern "C" fn(
            *mut GXdpDocuments,
            *mut GDBusMethodInvocation,
            *const gchar,
            *const gchar,
            *const *const gchar,
        ) -> gboolean,
    >,
    pub get_version: Option<unsafe extern "C" fn(*mut GXdpDocuments) -> guint>,
}
pub type GXdpDocumentsIface = _GXdpDocumentsIface;
pub type GXdpDocumentsInterface = GXdpDocumentsIface;
pub type _GDbusCodegenMarshalBoolean_ObjectStringFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *mut GDBusMethodInvocation,
        *const gchar,
        *mut ::core::ffi::c_void,
    ) -> gboolean,
>;
pub type _GDbusCodegenMarshalBoolean_ObjectStringStringBoxedFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *mut GDBusMethodInvocation,
        *const gchar,
        *const gchar,
        *const *const gchar,
        *mut ::core::ffi::c_void,
    ) -> gboolean,
>;
pub type _GDbusCodegenMarshalBoolean_ObjectObjectVariantStringUintStringBoxedFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *mut GUnixFDList,
        *mut GDBusMethodInvocation,
        *mut GVariant,
        *const gchar,
        guint,
        *const gchar,
        *const *const gchar,
        *mut ::core::ffi::c_void,
    ) -> gboolean,
>;
pub type _GDbusCodegenMarshalBoolean_ObjectObjectVariantUintStringBoxedFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *mut GUnixFDList,
        *mut GDBusMethodInvocation,
        *mut GVariant,
        guint,
        *const gchar,
        *const *const gchar,
        *mut ::core::ffi::c_void,
    ) -> gboolean,
>;
pub type _GDbusCodegenMarshalBoolean_ObjectObjectVariantStringBooleanBooleanFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *mut GUnixFDList,
        *mut GDBusMethodInvocation,
        *mut GVariant,
        *const gchar,
        gboolean,
        gboolean,
        *mut ::core::ffi::c_void,
    ) -> gboolean,
>;
pub type _GDbusCodegenMarshalBoolean_ObjectObjectVariantBooleanBooleanFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *mut GUnixFDList,
        *mut GDBusMethodInvocation,
        *mut GVariant,
        gboolean,
        gboolean,
        *mut ::core::ffi::c_void,
    ) -> gboolean,
>;
pub type _GDbusCodegenMarshalBoolean_ObjectFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *mut GDBusMethodInvocation,
        *mut ::core::ffi::c_void,
    ) -> gboolean,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ExtendedGDBusInterfaceInfo {
    pub parent_struct: GDBusInterfaceInfo,
    pub hyphen_name: *const gchar,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _ExtendedGDBusPropertyInfo {
    pub parent_struct: GDBusPropertyInfo,
    pub hyphen_name: *const gchar,
    #[bitfield(name = "use_gvariant", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "emits_changed_signal", ty = "guint", bits = "1..=1")]
    pub use_gvariant_emits_changed_signal: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ExtendedGDBusMethodInfo {
    pub parent_struct: GDBusMethodInfo,
    pub signal_name: *const gchar,
    pub pass_fdlist: gboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ExtendedGDBusArgInfo {
    pub parent_struct: GDBusArgInfo,
    pub use_gvariant: gboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GXdpDocumentsProxy {
    pub parent_instance: GDBusProxy,
    pub priv_0: *mut GXdpDocumentsProxyPrivate,
}
pub type GXdpDocumentsProxyPrivate = _GXdpDocumentsProxyPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GXdpDocumentsProxyPrivate {
    pub qdata: *mut GData,
}
pub type GXdpDocumentsProxy = _GXdpDocumentsProxy;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GXdpDocumentsProxyClass {
    pub parent_class: GDBusProxyClass,
}
pub type GXdpDocumentsProxyClass = _GXdpDocumentsProxyClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ExtendedGDBusSignalInfo {
    pub parent_struct: GDBusSignalInfo,
    pub signal_name: *const gchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GXdpDocumentsSkeleton {
    pub parent_instance: GDBusInterfaceSkeleton,
    pub priv_0: *mut GXdpDocumentsSkeletonPrivate,
}
pub type GXdpDocumentsSkeletonPrivate = _GXdpDocumentsSkeletonPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GXdpDocumentsSkeletonPrivate {
    pub properties: *mut GValue,
    pub changed_properties: *mut GList,
    pub changed_properties_idle_source: *mut GSource,
    pub context: *mut GMainContext,
    pub lock: GMutex,
}
pub type GXdpDocumentsSkeleton = _GXdpDocumentsSkeleton;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GXdpDocumentsSkeletonClass {
    pub parent_class: GDBusInterfaceSkeletonClass,
}
pub type GXdpDocumentsSkeletonClass = _GXdpDocumentsSkeletonClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ChangedProperty {
    pub info: *const _ExtendedGDBusPropertyInfo,
    pub prop_id: guint,
    pub orig_value: GValue,
}
pub type GXdpOpenURI = _GXdpOpenURI;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GXdpOpenURIIface {
    pub parent_iface: GTypeInterface,
    pub handle_open_directory: Option<
        unsafe extern "C" fn(
            *mut GXdpOpenURI,
            *mut GDBusMethodInvocation,
            *mut GUnixFDList,
            *const gchar,
            *mut GVariant,
            *mut GVariant,
        ) -> gboolean,
    >,
    pub handle_open_file: Option<
        unsafe extern "C" fn(
            *mut GXdpOpenURI,
            *mut GDBusMethodInvocation,
            *mut GUnixFDList,
            *const gchar,
            *mut GVariant,
            *mut GVariant,
        ) -> gboolean,
    >,
    pub handle_open_uri: Option<
        unsafe extern "C" fn(
            *mut GXdpOpenURI,
            *mut GDBusMethodInvocation,
            *const gchar,
            *const gchar,
            *mut GVariant,
        ) -> gboolean,
    >,
    pub get_version: Option<unsafe extern "C" fn(*mut GXdpOpenURI) -> guint>,
}
pub type GXdpOpenURIIface = _GXdpOpenURIIface;
pub type GXdpOpenURIInterface = GXdpOpenURIIface;
pub type _GDbusCodegenMarshalBoolean_ObjectObjectStringVariantVariantFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *mut GUnixFDList,
        *mut GDBusMethodInvocation,
        *const gchar,
        *mut GVariant,
        *mut GVariant,
        *mut ::core::ffi::c_void,
    ) -> gboolean,
>;
pub type _GDbusCodegenMarshalBoolean_ObjectStringStringVariantFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *mut GDBusMethodInvocation,
        *const gchar,
        *const gchar,
        *mut GVariant,
        *mut ::core::ffi::c_void,
    ) -> gboolean,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GXdpOpenURIProxy {
    pub parent_instance: GDBusProxy,
    pub priv_0: *mut GXdpOpenURIProxyPrivate,
}
pub type GXdpOpenURIProxyPrivate = _GXdpOpenURIProxyPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GXdpOpenURIProxyPrivate {
    pub qdata: *mut GData,
}
pub type GXdpOpenURIProxy = _GXdpOpenURIProxy;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GXdpOpenURIProxyClass {
    pub parent_class: GDBusProxyClass,
}
pub type GXdpOpenURIProxyClass = _GXdpOpenURIProxyClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GXdpOpenURISkeleton {
    pub parent_instance: GDBusInterfaceSkeleton,
    pub priv_0: *mut GXdpOpenURISkeletonPrivate,
}
pub type GXdpOpenURISkeletonPrivate = _GXdpOpenURISkeletonPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GXdpOpenURISkeletonPrivate {
    pub properties: *mut GValue,
    pub changed_properties: *mut GList,
    pub changed_properties_idle_source: *mut GSource,
    pub context: *mut GMainContext,
    pub lock: GMutex,
}
pub type GXdpOpenURISkeleton = _GXdpOpenURISkeleton;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GXdpOpenURISkeletonClass {
    pub parent_class: GDBusInterfaceSkeletonClass,
}
pub type GXdpOpenURISkeletonClass = _GXdpOpenURISkeletonClass;
pub type GXdpProxyResolver = _GXdpProxyResolver;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GXdpProxyResolverIface {
    pub parent_iface: GTypeInterface,
    pub handle_lookup: Option<
        unsafe extern "C" fn(
            *mut GXdpProxyResolver,
            *mut GDBusMethodInvocation,
            *const gchar,
        ) -> gboolean,
    >,
    pub get_version: Option<unsafe extern "C" fn(*mut GXdpProxyResolver) -> guint>,
}
pub type GXdpProxyResolverIface = _GXdpProxyResolverIface;
pub type GXdpProxyResolverInterface = GXdpProxyResolverIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GXdpProxyResolverProxy {
    pub parent_instance: GDBusProxy,
    pub priv_0: *mut GXdpProxyResolverProxyPrivate,
}
pub type GXdpProxyResolverProxyPrivate = _GXdpProxyResolverProxyPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GXdpProxyResolverProxyPrivate {
    pub qdata: *mut GData,
}
pub type GXdpProxyResolverProxy = _GXdpProxyResolverProxy;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GXdpProxyResolverProxyClass {
    pub parent_class: GDBusProxyClass,
}
pub type GXdpProxyResolverProxyClass = _GXdpProxyResolverProxyClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GXdpProxyResolverSkeleton {
    pub parent_instance: GDBusInterfaceSkeleton,
    pub priv_0: *mut GXdpProxyResolverSkeletonPrivate,
}
pub type GXdpProxyResolverSkeletonPrivate = _GXdpProxyResolverSkeletonPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GXdpProxyResolverSkeletonPrivate {
    pub properties: *mut GValue,
    pub changed_properties: *mut GList,
    pub changed_properties_idle_source: *mut GSource,
    pub context: *mut GMainContext,
    pub lock: GMutex,
}
pub type GXdpProxyResolverSkeleton = _GXdpProxyResolverSkeleton;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GXdpProxyResolverSkeletonClass {
    pub parent_class: GDBusInterfaceSkeletonClass,
}
pub type GXdpProxyResolverSkeletonClass = _GXdpProxyResolverSkeletonClass;
pub type GXdpTrash = _GXdpTrash;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GXdpTrashIface {
    pub parent_iface: GTypeInterface,
    pub handle_trash_file: Option<
        unsafe extern "C" fn(
            *mut GXdpTrash,
            *mut GDBusMethodInvocation,
            *mut GUnixFDList,
            *mut GVariant,
        ) -> gboolean,
    >,
    pub get_version: Option<unsafe extern "C" fn(*mut GXdpTrash) -> guint>,
}
pub type GXdpTrashIface = _GXdpTrashIface;
pub type GXdpTrashInterface = GXdpTrashIface;
pub type _GDbusCodegenMarshalBoolean_ObjectObjectVariantFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *mut GUnixFDList,
        *mut GDBusMethodInvocation,
        *mut GVariant,
        *mut ::core::ffi::c_void,
    ) -> gboolean,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GXdpTrashProxy {
    pub parent_instance: GDBusProxy,
    pub priv_0: *mut GXdpTrashProxyPrivate,
}
pub type GXdpTrashProxyPrivate = _GXdpTrashProxyPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GXdpTrashProxyPrivate {
    pub qdata: *mut GData,
}
pub type GXdpTrashProxy = _GXdpTrashProxy;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GXdpTrashProxyClass {
    pub parent_class: GDBusProxyClass,
}
pub type GXdpTrashProxyClass = _GXdpTrashProxyClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GXdpTrashSkeleton {
    pub parent_instance: GDBusInterfaceSkeleton,
    pub priv_0: *mut GXdpTrashSkeletonPrivate,
}
pub type GXdpTrashSkeletonPrivate = _GXdpTrashSkeletonPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GXdpTrashSkeletonPrivate {
    pub properties: *mut GValue,
    pub changed_properties: *mut GList,
    pub changed_properties_idle_source: *mut GSource,
    pub context: *mut GMainContext,
    pub lock: GMutex,
}
pub type GXdpTrashSkeleton = _GXdpTrashSkeleton;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GXdpTrashSkeletonClass {
    pub parent_class: GDBusInterfaceSkeletonClass,
}
pub type GXdpTrashSkeletonClass = _GXdpTrashSkeletonClass;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_MAXUINT32: guint32 = 0xffffffff as ::core::ffi::c_uint;
pub const G_PRIORITY_DEFAULT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_INVALID: GType = ((0 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_INTERFACE: GType =
    ((2 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_UCHAR: GType = 16;
pub const G_TYPE_BOOLEAN: GType = ((5 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_INT: GType = 24;
pub const G_TYPE_UINT: GType = ((7 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_INT64: GType = 40;
pub const G_TYPE_UINT64: GType = 44;
pub const G_TYPE_DOUBLE: GType = 60;
pub const G_TYPE_STRING: GType = ((16 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_VARIANT: GType = ((21 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
unsafe extern "C" fn safe_c2rust__changed_property_free(mut data: *mut ChangedProperty) {
    g_value_unset(&raw mut (*data).orig_value);
    g_free(data as gpointer);
}
unsafe extern "C" fn safe_c2rust__g_strv_equal0(
    mut a: *mut *mut gchar,
    mut b: *mut *mut gchar,
) -> gboolean {
    let mut current_block: u64;
    let mut ret: gboolean = FALSE;
    let mut n: guint = 0;
    if a.is_null() && b.is_null() {
        ret = TRUE as gboolean;
    } else if !(a.is_null() || b.is_null()) {
        if !(g_strv_length(a) != g_strv_length(b)) {
            n = 0 as guint;
            loop {
                if (*a.offset(n as isize)).is_null() {
                    current_block = 7815301370352969686;
                    break;
                }
                if g_strcmp0(*a.offset(n as isize), *b.offset(n as isize))
                    != 0 as ::core::ffi::c_int
                {
                    current_block = 9348506229402749297;
                    break;
                }
                n = n.wrapping_add(1);
            }
            match current_block {
                9348506229402749297 => {}
                _ => {
                    ret = TRUE as gboolean;
                }
            }
        }
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust__g_variant_equal0(
    mut a: *mut GVariant,
    mut b: *mut GVariant,
) -> gboolean {
    let mut ret: gboolean = FALSE;
    if a.is_null() && b.is_null() {
        ret = TRUE as gboolean;
    } else if !(a.is_null() || b.is_null()) {
        ret = g_variant_equal(a as gconstpointer, b as gconstpointer);
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust__g_value_equal(
    mut a: *const GValue,
    mut b: *const GValue,
) -> gboolean {
    let mut ret: gboolean = FALSE;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if (*(a as *mut GValue)).g_type == (*(b as *mut GValue)).g_type {
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
            b"gio/xdp-dbus.c\0" as *const u8 as *const ::core::ffi::c_char,
            155 as ::core::ffi::c_int,
            G_STRFUNC,
            b"G_VALUE_TYPE (a) == G_VALUE_TYPE (b)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    match (*(a as *mut GValue)).g_type {
        G_TYPE_BOOLEAN => {
            ret = (g_value_get_boolean(a) == g_value_get_boolean(b)) as ::core::ffi::c_int
                as gboolean;
        }
        G_TYPE_UCHAR => {
            ret = (g_value_get_uchar(a) as ::core::ffi::c_int
                == g_value_get_uchar(b) as ::core::ffi::c_int)
                as ::core::ffi::c_int as gboolean;
        }
        G_TYPE_INT => {
            ret = (g_value_get_int(a) == g_value_get_int(b)) as ::core::ffi::c_int as gboolean;
        }
        G_TYPE_UINT => {
            ret = (g_value_get_uint(a) == g_value_get_uint(b)) as ::core::ffi::c_int as gboolean;
        }
        G_TYPE_INT64 => {
            ret = (g_value_get_int64(a) == g_value_get_int64(b)) as ::core::ffi::c_int as gboolean;
        }
        G_TYPE_UINT64 => {
            ret =
                (g_value_get_uint64(a) == g_value_get_uint64(b)) as ::core::ffi::c_int as gboolean;
        }
        G_TYPE_DOUBLE => {
            let mut da: gdouble = g_value_get_double(a);
            let mut db: gdouble = g_value_get_double(b);
            ret = (memcmp(
                &raw mut da as *const ::core::ffi::c_void,
                &raw mut db as *const ::core::ffi::c_void,
                ::core::mem::size_of::<gdouble>() as size_t,
            ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int as gboolean;
        }
        G_TYPE_STRING => {
            ret = (g_strcmp0(
                g_value_get_string(a) as *const ::core::ffi::c_char,
                g_value_get_string(b) as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int as gboolean;
        }
        G_TYPE_VARIANT => {
            ret = safe_c2rust__g_variant_equal0(g_value_get_variant(a), g_value_get_variant(b));
        }
        _ => {
            if (*(a as *mut GValue)).g_type == g_strv_get_type() {
                ret = safe_c2rust__g_strv_equal0(
                    g_value_get_boxed(a) as *mut *mut gchar,
                    g_value_get_boxed(b) as *mut *mut gchar,
                );
            } else {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_CRITICAL,
                    b"_g_value_equal() does not handle type %s\0" as *const u8 as *const gchar,
                    g_type_name((*(a as *mut GValue)).g_type),
                );
            }
        }
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust__g_dbus_codegen_marshal_BOOLEAN__OBJECT(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: ::core::ffi::c_uint,
    mut param_values: *const GValue,
    mut invocation_hint: *mut ::core::ffi::c_void,
    mut marshal_data: *mut ::core::ffi::c_void,
) {
    let mut callback: _GDbusCodegenMarshalBoolean_ObjectFunc = None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut v_return: gboolean = 0;
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !return_value.is_null() {
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
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if n_param_values == 2 as ::core::ffi::c_uint {
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
            b"n_param_values == 2\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data as *mut ::core::ffi::c_void;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize))
            as *mut ::core::ffi::c_void;
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize))
            as *mut ::core::ffi::c_void;
        data2 = (*closure).data as *mut ::core::ffi::c_void;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        _GDbusCodegenMarshalBoolean_ObjectFunc,
    >(if !marshal_data.is_null() {
        marshal_data
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    v_return = callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *mut GDBusMethodInvocation,
        data2,
    );
    g_value_set_boolean(return_value, v_return);
}
unsafe extern "C" fn safe_c2rust__g_dbus_codegen_marshal_BOOLEAN__OBJECT_OBJECT_VARIANT_BOOLEAN_BOOLEAN(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: ::core::ffi::c_uint,
    mut param_values: *const GValue,
    mut invocation_hint: *mut ::core::ffi::c_void,
    mut marshal_data: *mut ::core::ffi::c_void,
) {
    let mut callback: _GDbusCodegenMarshalBoolean_ObjectObjectVariantBooleanBooleanFunc = None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut v_return: gboolean = 0;
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !return_value.is_null() {
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
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if n_param_values == 6 as ::core::ffi::c_uint {
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
            b"n_param_values == 6\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data as *mut ::core::ffi::c_void;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize))
            as *mut ::core::ffi::c_void;
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize))
            as *mut ::core::ffi::c_void;
        data2 = (*closure).data as *mut ::core::ffi::c_void;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        _GDbusCodegenMarshalBoolean_ObjectObjectVariantBooleanBooleanFunc,
    >(if !marshal_data.is_null() {
        marshal_data
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    v_return = callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *mut GUnixFDList,
        (*param_values.offset(2 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *mut GDBusMethodInvocation,
        (*param_values.offset(3 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *mut GVariant,
        (*param_values.offset(4 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_int as gboolean,
        (*param_values.offset(5 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_int as gboolean,
        data2,
    );
    g_value_set_boolean(return_value, v_return);
}
unsafe extern "C" fn safe_c2rust__g_dbus_codegen_marshal_BOOLEAN__OBJECT_OBJECT_VARIANT_STRING_BOOLEAN_BOOLEAN(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: ::core::ffi::c_uint,
    mut param_values: *const GValue,
    mut invocation_hint: *mut ::core::ffi::c_void,
    mut marshal_data: *mut ::core::ffi::c_void,
) {
    let mut callback: _GDbusCodegenMarshalBoolean_ObjectObjectVariantStringBooleanBooleanFunc =
        None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut v_return: gboolean = 0;
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !return_value.is_null() {
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
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if n_param_values == 7 as ::core::ffi::c_uint {
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
            b"n_param_values == 7\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data as *mut ::core::ffi::c_void;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize))
            as *mut ::core::ffi::c_void;
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize))
            as *mut ::core::ffi::c_void;
        data2 = (*closure).data as *mut ::core::ffi::c_void;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        _GDbusCodegenMarshalBoolean_ObjectObjectVariantStringBooleanBooleanFunc,
    >(if !marshal_data.is_null() {
        marshal_data
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    v_return = callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *mut GUnixFDList,
        (*param_values.offset(2 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *mut GDBusMethodInvocation,
        (*param_values.offset(3 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *mut GVariant,
        (*param_values.offset(4 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *const gchar,
        (*param_values.offset(5 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_int as gboolean,
        (*param_values.offset(6 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_int as gboolean,
        data2,
    );
    g_value_set_boolean(return_value, v_return);
}
unsafe extern "C" fn safe_c2rust__g_dbus_codegen_marshal_BOOLEAN__OBJECT_OBJECT_VARIANT_UINT_STRING_BOXED(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: ::core::ffi::c_uint,
    mut param_values: *const GValue,
    mut invocation_hint: *mut ::core::ffi::c_void,
    mut marshal_data: *mut ::core::ffi::c_void,
) {
    let mut callback: _GDbusCodegenMarshalBoolean_ObjectObjectVariantUintStringBoxedFunc = None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut v_return: gboolean = 0;
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !return_value.is_null() {
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
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if n_param_values == 7 as ::core::ffi::c_uint {
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
            b"n_param_values == 7\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data as *mut ::core::ffi::c_void;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize))
            as *mut ::core::ffi::c_void;
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize))
            as *mut ::core::ffi::c_void;
        data2 = (*closure).data as *mut ::core::ffi::c_void;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        _GDbusCodegenMarshalBoolean_ObjectObjectVariantUintStringBoxedFunc,
    >(if !marshal_data.is_null() {
        marshal_data
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    v_return = callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *mut GUnixFDList,
        (*param_values.offset(2 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *mut GDBusMethodInvocation,
        (*param_values.offset(3 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *mut GVariant,
        (*param_values.offset(4 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_uint,
        (*param_values.offset(5 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *const gchar,
        (*param_values.offset(6 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *const *const gchar,
        data2,
    );
    g_value_set_boolean(return_value, v_return);
}
unsafe extern "C" fn safe_c2rust__g_dbus_codegen_marshal_BOOLEAN__OBJECT_OBJECT_VARIANT_STRING_UINT_STRING_BOXED(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: ::core::ffi::c_uint,
    mut param_values: *const GValue,
    mut invocation_hint: *mut ::core::ffi::c_void,
    mut marshal_data: *mut ::core::ffi::c_void,
) {
    let mut callback: _GDbusCodegenMarshalBoolean_ObjectObjectVariantStringUintStringBoxedFunc =
        None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut v_return: gboolean = 0;
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !return_value.is_null() {
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
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if n_param_values == 8 as ::core::ffi::c_uint {
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
            b"n_param_values == 8\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data as *mut ::core::ffi::c_void;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize))
            as *mut ::core::ffi::c_void;
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize))
            as *mut ::core::ffi::c_void;
        data2 = (*closure).data as *mut ::core::ffi::c_void;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        _GDbusCodegenMarshalBoolean_ObjectObjectVariantStringUintStringBoxedFunc,
    >(if !marshal_data.is_null() {
        marshal_data
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    v_return = callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *mut GUnixFDList,
        (*param_values.offset(2 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *mut GDBusMethodInvocation,
        (*param_values.offset(3 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *mut GVariant,
        (*param_values.offset(4 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *const gchar,
        (*param_values.offset(5 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_uint,
        (*param_values.offset(6 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *const gchar,
        (*param_values.offset(7 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *const *const gchar,
        data2,
    );
    g_value_set_boolean(return_value, v_return);
}
unsafe extern "C" fn safe_c2rust__g_dbus_codegen_marshal_BOOLEAN__OBJECT_STRING_STRING_BOXED(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: ::core::ffi::c_uint,
    mut param_values: *const GValue,
    mut invocation_hint: *mut ::core::ffi::c_void,
    mut marshal_data: *mut ::core::ffi::c_void,
) {
    let mut callback: _GDbusCodegenMarshalBoolean_ObjectStringStringBoxedFunc = None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut v_return: gboolean = 0;
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !return_value.is_null() {
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
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if n_param_values == 5 as ::core::ffi::c_uint {
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
            b"n_param_values == 5\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data as *mut ::core::ffi::c_void;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize))
            as *mut ::core::ffi::c_void;
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize))
            as *mut ::core::ffi::c_void;
        data2 = (*closure).data as *mut ::core::ffi::c_void;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        _GDbusCodegenMarshalBoolean_ObjectStringStringBoxedFunc,
    >(if !marshal_data.is_null() {
        marshal_data
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    v_return = callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *mut GDBusMethodInvocation,
        (*param_values.offset(2 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *const gchar,
        (*param_values.offset(3 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *const gchar,
        (*param_values.offset(4 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *const *const gchar,
        data2,
    );
    g_value_set_boolean(return_value, v_return);
}
unsafe extern "C" fn safe_c2rust__g_dbus_codegen_marshal_BOOLEAN__OBJECT_STRING(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: ::core::ffi::c_uint,
    mut param_values: *const GValue,
    mut invocation_hint: *mut ::core::ffi::c_void,
    mut marshal_data: *mut ::core::ffi::c_void,
) {
    let mut callback: _GDbusCodegenMarshalBoolean_ObjectStringFunc = None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut v_return: gboolean = 0;
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if !return_value.is_null() {
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
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if n_param_values == 3 as ::core::ffi::c_uint {
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
            b"n_param_values == 3\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data as *mut ::core::ffi::c_void;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize))
            as *mut ::core::ffi::c_void;
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize))
            as *mut ::core::ffi::c_void;
        data2 = (*closure).data as *mut ::core::ffi::c_void;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        _GDbusCodegenMarshalBoolean_ObjectStringFunc,
    >(if !marshal_data.is_null() {
        marshal_data
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    v_return = callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *mut GDBusMethodInvocation,
        (*param_values.offset(2 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *const gchar,
        data2,
    );
    g_value_set_boolean(return_value, v_return);
}
unsafe extern "C" fn safe_c2rust__g_dbus_codegen_marshal_BOOLEAN__OBJECT_STRING_STRING_VARIANT(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: ::core::ffi::c_uint,
    mut param_values: *const GValue,
    mut invocation_hint: *mut ::core::ffi::c_void,
    mut marshal_data: *mut ::core::ffi::c_void,
) {
    let mut callback: _GDbusCodegenMarshalBoolean_ObjectStringStringVariantFunc = None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut v_return: gboolean = 0;
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if !return_value.is_null() {
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
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if n_param_values == 5 as ::core::ffi::c_uint {
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
            b"n_param_values == 5\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data as *mut ::core::ffi::c_void;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize))
            as *mut ::core::ffi::c_void;
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize))
            as *mut ::core::ffi::c_void;
        data2 = (*closure).data as *mut ::core::ffi::c_void;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        _GDbusCodegenMarshalBoolean_ObjectStringStringVariantFunc,
    >(if !marshal_data.is_null() {
        marshal_data
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    v_return = callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *mut GDBusMethodInvocation,
        (*param_values.offset(2 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *const gchar,
        (*param_values.offset(3 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *const gchar,
        (*param_values.offset(4 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *mut GVariant,
        data2,
    );
    g_value_set_boolean(return_value, v_return);
}
unsafe extern "C" fn safe_c2rust__g_dbus_codegen_marshal_BOOLEAN__OBJECT_OBJECT_STRING_VARIANT_VARIANT(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: ::core::ffi::c_uint,
    mut param_values: *const GValue,
    mut invocation_hint: *mut ::core::ffi::c_void,
    mut marshal_data: *mut ::core::ffi::c_void,
) {
    let mut callback: _GDbusCodegenMarshalBoolean_ObjectObjectStringVariantVariantFunc = None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut v_return: gboolean = 0;
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if !return_value.is_null() {
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
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if n_param_values == 6 as ::core::ffi::c_uint {
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
            b"n_param_values == 6\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data as *mut ::core::ffi::c_void;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize))
            as *mut ::core::ffi::c_void;
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize))
            as *mut ::core::ffi::c_void;
        data2 = (*closure).data as *mut ::core::ffi::c_void;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        _GDbusCodegenMarshalBoolean_ObjectObjectStringVariantVariantFunc,
    >(if !marshal_data.is_null() {
        marshal_data
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    v_return = callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *mut GUnixFDList,
        (*param_values.offset(2 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *mut GDBusMethodInvocation,
        (*param_values.offset(3 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *const gchar,
        (*param_values.offset(4 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *mut GVariant,
        (*param_values.offset(5 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *mut GVariant,
        data2,
    );
    g_value_set_boolean(return_value, v_return);
}
unsafe extern "C" fn safe_c2rust__g_dbus_codegen_marshal_BOOLEAN__OBJECT_OBJECT_VARIANT(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: ::core::ffi::c_uint,
    mut param_values: *const GValue,
    mut invocation_hint: *mut ::core::ffi::c_void,
    mut marshal_data: *mut ::core::ffi::c_void,
) {
    let mut callback: _GDbusCodegenMarshalBoolean_ObjectObjectVariantFunc = None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut v_return: gboolean = 0;
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if !return_value.is_null() {
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
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if n_param_values == 4 as ::core::ffi::c_uint {
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
            b"n_param_values == 4\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data as *mut ::core::ffi::c_void;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize))
            as *mut ::core::ffi::c_void;
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize))
            as *mut ::core::ffi::c_void;
        data2 = (*closure).data as *mut ::core::ffi::c_void;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        _GDbusCodegenMarshalBoolean_ObjectObjectVariantFunc,
    >(if !marshal_data.is_null() {
        marshal_data
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    v_return = callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *mut GUnixFDList,
        (*param_values.offset(2 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *mut GDBusMethodInvocation,
        (*param_values.offset(3 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *mut GVariant,
        data2,
    );
    g_value_set_boolean(return_value, v_return);
}
static mut safe_c2rust__gxdp_documents_method_info_get_mount_point_OUT_ARG_path:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"path\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"ay\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust__gxdp_documents_method_info_get_mount_point_OUT_ARG_pointers:
    [*const GDBusArgInfo; 2] = [::core::ptr::null::<GDBusArgInfo>(); 2];
static mut safe_c2rust__gxdp_documents_method_info_get_mount_point: _ExtendedGDBusMethodInfo = unsafe {
    _ExtendedGDBusMethodInfo {
        parent_struct: _GDBusMethodInfo {
            ref_count: -(1 as gint),
            name: b"GetMountPoint\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            in_args: ::core::ptr::null::<*mut GDBusArgInfo>() as *mut *mut GDBusArgInfo,
            out_args:
                &raw const safe_c2rust__gxdp_documents_method_info_get_mount_point_OUT_ARG_pointers
                    as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"handle-get-mount-point\0" as *const u8 as *const gchar,
        pass_fdlist: FALSE,
    }
};
static mut safe_c2rust__gxdp_documents_method_info_add_IN_ARG_o_path_fd: _ExtendedGDBusArgInfo =
    _ExtendedGDBusArgInfo {
        parent_struct: _GDBusArgInfo {
            ref_count: -(1 as gint),
            name: b"o_path_fd\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            signature: b"h\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        use_gvariant: FALSE,
    };
static mut safe_c2rust__gxdp_documents_method_info_add_IN_ARG_reuse_existing:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"reuse_existing\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"b\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust__gxdp_documents_method_info_add_IN_ARG_persistent: _ExtendedGDBusArgInfo =
    _ExtendedGDBusArgInfo {
        parent_struct: _GDBusArgInfo {
            ref_count: -(1 as gint),
            name: b"persistent\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            signature: b"b\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        use_gvariant: FALSE,
    };
static mut safe_c2rust__gxdp_documents_method_info_add_IN_ARG_pointers: [*const GDBusArgInfo; 4] =
    [::core::ptr::null::<GDBusArgInfo>(); 4];
static mut safe_c2rust__gxdp_documents_method_info_add_OUT_ARG_doc_id: _ExtendedGDBusArgInfo =
    _ExtendedGDBusArgInfo {
        parent_struct: _GDBusArgInfo {
            ref_count: -(1 as gint),
            name: b"doc_id\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        use_gvariant: FALSE,
    };
static mut safe_c2rust__gxdp_documents_method_info_add_OUT_ARG_pointers: [*const GDBusArgInfo; 2] =
    [::core::ptr::null::<GDBusArgInfo>(); 2];
static mut safe_c2rust__gxdp_documents_method_info_add: _ExtendedGDBusMethodInfo = unsafe {
    _ExtendedGDBusMethodInfo {
        parent_struct: _GDBusMethodInfo {
            ref_count: -(1 as gint),
            name: b"Add\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            in_args: &raw const safe_c2rust__gxdp_documents_method_info_add_IN_ARG_pointers
                as *mut *mut GDBusArgInfo,
            out_args: &raw const safe_c2rust__gxdp_documents_method_info_add_OUT_ARG_pointers
                as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"handle-add\0" as *const u8 as *const gchar,
        pass_fdlist: TRUE,
    }
};
static mut safe_c2rust__gxdp_documents_method_info_add_named_IN_ARG_o_path_parent_fd:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"o_path_parent_fd\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"h\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust__gxdp_documents_method_info_add_named_IN_ARG_filename:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"filename\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"ay\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust__gxdp_documents_method_info_add_named_IN_ARG_reuse_existing:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"reuse_existing\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"b\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust__gxdp_documents_method_info_add_named_IN_ARG_persistent:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"persistent\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"b\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust__gxdp_documents_method_info_add_named_IN_ARG_pointers:
    [*const GDBusArgInfo; 5] = [::core::ptr::null::<GDBusArgInfo>(); 5];
static mut safe_c2rust__gxdp_documents_method_info_add_named_OUT_ARG_doc_id: _ExtendedGDBusArgInfo =
    _ExtendedGDBusArgInfo {
        parent_struct: _GDBusArgInfo {
            ref_count: -(1 as gint),
            name: b"doc_id\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        use_gvariant: FALSE,
    };
static mut safe_c2rust__gxdp_documents_method_info_add_named_OUT_ARG_pointers:
    [*const GDBusArgInfo; 2] = [::core::ptr::null::<GDBusArgInfo>(); 2];
static mut safe_c2rust__gxdp_documents_method_info_add_named: _ExtendedGDBusMethodInfo = unsafe {
    _ExtendedGDBusMethodInfo {
        parent_struct: _GDBusMethodInfo {
            ref_count: -(1 as gint),
            name: b"AddNamed\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            in_args: &raw const safe_c2rust__gxdp_documents_method_info_add_named_IN_ARG_pointers
                as *mut *mut GDBusArgInfo,
            out_args: &raw const safe_c2rust__gxdp_documents_method_info_add_named_OUT_ARG_pointers
                as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"handle-add-named\0" as *const u8 as *const gchar,
        pass_fdlist: TRUE,
    }
};
static mut safe_c2rust__gxdp_documents_method_info_add_full_IN_ARG_o_path_fds:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"o_path_fds\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"ah\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust__gxdp_documents_method_info_add_full_IN_ARG_flags: _ExtendedGDBusArgInfo =
    _ExtendedGDBusArgInfo {
        parent_struct: _GDBusArgInfo {
            ref_count: -(1 as gint),
            name: b"flags\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            signature: b"u\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        use_gvariant: FALSE,
    };
static mut safe_c2rust__gxdp_documents_method_info_add_full_IN_ARG_app_id: _ExtendedGDBusArgInfo =
    _ExtendedGDBusArgInfo {
        parent_struct: _GDBusArgInfo {
            ref_count: -(1 as gint),
            name: b"app_id\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        use_gvariant: FALSE,
    };
static mut safe_c2rust__gxdp_documents_method_info_add_full_IN_ARG_permissions:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"permissions\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"as\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust__gxdp_documents_method_info_add_full_IN_ARG_pointers: [*const GDBusArgInfo;
    5] = [::core::ptr::null::<GDBusArgInfo>(); 5];
static mut safe_c2rust__gxdp_documents_method_info_add_full_OUT_ARG_doc_ids: _ExtendedGDBusArgInfo =
    _ExtendedGDBusArgInfo {
        parent_struct: _GDBusArgInfo {
            ref_count: -(1 as gint),
            name: b"doc_ids\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            signature: b"as\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        use_gvariant: FALSE,
    };
static mut safe_c2rust__gxdp_documents_method_info_add_full_OUT_ARG_extra_out:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"extra_out\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"a{sv}\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust__gxdp_documents_method_info_add_full_OUT_ARG_pointers:
    [*const GDBusArgInfo; 3] = [::core::ptr::null::<GDBusArgInfo>(); 3];
static mut safe_c2rust__gxdp_documents_method_info_add_full: _ExtendedGDBusMethodInfo = unsafe {
    _ExtendedGDBusMethodInfo {
        parent_struct: _GDBusMethodInfo {
            ref_count: -(1 as gint),
            name: b"AddFull\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            in_args: &raw const safe_c2rust__gxdp_documents_method_info_add_full_IN_ARG_pointers
                as *mut *mut GDBusArgInfo,
            out_args: &raw const safe_c2rust__gxdp_documents_method_info_add_full_OUT_ARG_pointers
                as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"handle-add-full\0" as *const u8 as *const gchar,
        pass_fdlist: TRUE,
    }
};
static mut safe_c2rust__gxdp_documents_method_info_add_named_full_IN_ARG_o_path_fd:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"o_path_fd\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"h\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust__gxdp_documents_method_info_add_named_full_IN_ARG_filename:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"filename\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"ay\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust__gxdp_documents_method_info_add_named_full_IN_ARG_flags:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"flags\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"u\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust__gxdp_documents_method_info_add_named_full_IN_ARG_app_id:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"app_id\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust__gxdp_documents_method_info_add_named_full_IN_ARG_permissions:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"permissions\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"as\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust__gxdp_documents_method_info_add_named_full_IN_ARG_pointers:
    [*const GDBusArgInfo; 6] = [::core::ptr::null::<GDBusArgInfo>(); 6];
static mut safe_c2rust__gxdp_documents_method_info_add_named_full_OUT_ARG_doc_id:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"doc_id\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust__gxdp_documents_method_info_add_named_full_OUT_ARG_extra_out:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"extra_out\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"a{sv}\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust__gxdp_documents_method_info_add_named_full_OUT_ARG_pointers:
    [*const GDBusArgInfo; 3] = [::core::ptr::null::<GDBusArgInfo>(); 3];
static mut safe_c2rust__gxdp_documents_method_info_add_named_full: _ExtendedGDBusMethodInfo = unsafe {
    _ExtendedGDBusMethodInfo {
        parent_struct: _GDBusMethodInfo {
            ref_count: -(1 as gint),
            name: b"AddNamedFull\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            in_args:
                &raw const safe_c2rust__gxdp_documents_method_info_add_named_full_IN_ARG_pointers
                    as *mut *mut GDBusArgInfo,
            out_args:
                &raw const safe_c2rust__gxdp_documents_method_info_add_named_full_OUT_ARG_pointers
                    as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"handle-add-named-full\0" as *const u8 as *const gchar,
        pass_fdlist: TRUE,
    }
};
static mut safe_c2rust__gxdp_documents_method_info_grant_permissions_IN_ARG_doc_id:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"doc_id\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust__gxdp_documents_method_info_grant_permissions_IN_ARG_app_id:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"app_id\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust__gxdp_documents_method_info_grant_permissions_IN_ARG_permissions:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"permissions\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"as\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust__gxdp_documents_method_info_grant_permissions_IN_ARG_pointers:
    [*const GDBusArgInfo; 4] = [::core::ptr::null::<GDBusArgInfo>(); 4];
static mut safe_c2rust__gxdp_documents_method_info_grant_permissions: _ExtendedGDBusMethodInfo = unsafe {
    _ExtendedGDBusMethodInfo {
        parent_struct: _GDBusMethodInfo {
            ref_count: -(1 as gint),
            name: b"GrantPermissions\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            in_args:
                &raw const safe_c2rust__gxdp_documents_method_info_grant_permissions_IN_ARG_pointers
                    as *mut *mut GDBusArgInfo,
            out_args: ::core::ptr::null::<*mut GDBusArgInfo>() as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"handle-grant-permissions\0" as *const u8 as *const gchar,
        pass_fdlist: FALSE,
    }
};
static mut safe_c2rust__gxdp_documents_method_info_revoke_permissions_IN_ARG_doc_id:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"doc_id\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust__gxdp_documents_method_info_revoke_permissions_IN_ARG_app_id:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"app_id\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust__gxdp_documents_method_info_revoke_permissions_IN_ARG_permissions:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"permissions\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"as\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust__gxdp_documents_method_info_revoke_permissions_IN_ARG_pointers:
    [*const GDBusArgInfo; 4] = [::core::ptr::null::<GDBusArgInfo>(); 4];
static mut safe_c2rust__gxdp_documents_method_info_revoke_permissions: _ExtendedGDBusMethodInfo = unsafe {
    _ExtendedGDBusMethodInfo {
        parent_struct: _GDBusMethodInfo {
            ref_count: -(1 as gint),
            name: b"RevokePermissions\0" as *const u8 as *const ::core::ffi::c_char
                as *mut gchar,
            in_args: &raw const safe_c2rust__gxdp_documents_method_info_revoke_permissions_IN_ARG_pointers
                as *mut *mut GDBusArgInfo,
            out_args: ::core::ptr::null::<*mut GDBusArgInfo>() as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"handle-revoke-permissions\0" as *const u8 as *const gchar,
        pass_fdlist: FALSE,
    }
};
static mut safe_c2rust__gxdp_documents_method_info_delete_IN_ARG_doc_id: _ExtendedGDBusArgInfo =
    _ExtendedGDBusArgInfo {
        parent_struct: _GDBusArgInfo {
            ref_count: -(1 as gint),
            name: b"doc_id\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        use_gvariant: FALSE,
    };
static mut safe_c2rust__gxdp_documents_method_info_delete_IN_ARG_pointers: [*const GDBusArgInfo;
    2] = [::core::ptr::null::<GDBusArgInfo>(); 2];
static mut safe_c2rust__gxdp_documents_method_info_delete: _ExtendedGDBusMethodInfo = unsafe {
    _ExtendedGDBusMethodInfo {
        parent_struct: _GDBusMethodInfo {
            ref_count: -(1 as gint),
            name: b"Delete\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            in_args: &raw const safe_c2rust__gxdp_documents_method_info_delete_IN_ARG_pointers
                as *mut *mut GDBusArgInfo,
            out_args: ::core::ptr::null::<*mut GDBusArgInfo>() as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"handle-delete\0" as *const u8 as *const gchar,
        pass_fdlist: FALSE,
    }
};
static mut safe_c2rust__gxdp_documents_method_info_lookup_IN_ARG_filename: _ExtendedGDBusArgInfo =
    _ExtendedGDBusArgInfo {
        parent_struct: _GDBusArgInfo {
            ref_count: -(1 as gint),
            name: b"filename\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            signature: b"ay\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        use_gvariant: FALSE,
    };
static mut safe_c2rust__gxdp_documents_method_info_lookup_IN_ARG_pointers: [*const GDBusArgInfo;
    2] = [::core::ptr::null::<GDBusArgInfo>(); 2];
static mut safe_c2rust__gxdp_documents_method_info_lookup_OUT_ARG_doc_id: _ExtendedGDBusArgInfo =
    _ExtendedGDBusArgInfo {
        parent_struct: _GDBusArgInfo {
            ref_count: -(1 as gint),
            name: b"doc_id\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        use_gvariant: FALSE,
    };
static mut safe_c2rust__gxdp_documents_method_info_lookup_OUT_ARG_pointers: [*const GDBusArgInfo;
    2] = [::core::ptr::null::<GDBusArgInfo>(); 2];
static mut safe_c2rust__gxdp_documents_method_info_lookup: _ExtendedGDBusMethodInfo = unsafe {
    _ExtendedGDBusMethodInfo {
        parent_struct: _GDBusMethodInfo {
            ref_count: -(1 as gint),
            name: b"Lookup\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            in_args: &raw const safe_c2rust__gxdp_documents_method_info_lookup_IN_ARG_pointers
                as *mut *mut GDBusArgInfo,
            out_args: &raw const safe_c2rust__gxdp_documents_method_info_lookup_OUT_ARG_pointers
                as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"handle-lookup\0" as *const u8 as *const gchar,
        pass_fdlist: FALSE,
    }
};
static mut safe_c2rust__gxdp_documents_method_info_info_IN_ARG_doc_id: _ExtendedGDBusArgInfo =
    _ExtendedGDBusArgInfo {
        parent_struct: _GDBusArgInfo {
            ref_count: -(1 as gint),
            name: b"doc_id\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        use_gvariant: FALSE,
    };
static mut safe_c2rust__gxdp_documents_method_info_info_IN_ARG_pointers: [*const GDBusArgInfo; 2] =
    [::core::ptr::null::<GDBusArgInfo>(); 2];
static mut safe_c2rust__gxdp_documents_method_info_info_OUT_ARG_path: _ExtendedGDBusArgInfo =
    _ExtendedGDBusArgInfo {
        parent_struct: _GDBusArgInfo {
            ref_count: -(1 as gint),
            name: b"path\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            signature: b"ay\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        use_gvariant: FALSE,
    };
static mut safe_c2rust__gxdp_documents_method_info_info_OUT_ARG_apps: _ExtendedGDBusArgInfo =
    _ExtendedGDBusArgInfo {
        parent_struct: _GDBusArgInfo {
            ref_count: -(1 as gint),
            name: b"apps\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            signature: b"a{sas}\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        use_gvariant: FALSE,
    };
static mut safe_c2rust__gxdp_documents_method_info_info_OUT_ARG_pointers: [*const GDBusArgInfo; 3] =
    [::core::ptr::null::<GDBusArgInfo>(); 3];
static mut safe_c2rust__gxdp_documents_method_info_info: _ExtendedGDBusMethodInfo = unsafe {
    _ExtendedGDBusMethodInfo {
        parent_struct: _GDBusMethodInfo {
            ref_count: -(1 as gint),
            name: b"Info\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            in_args: &raw const safe_c2rust__gxdp_documents_method_info_info_IN_ARG_pointers
                as *mut *mut GDBusArgInfo,
            out_args: &raw const safe_c2rust__gxdp_documents_method_info_info_OUT_ARG_pointers
                as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"handle-info\0" as *const u8 as *const gchar,
        pass_fdlist: FALSE,
    }
};
static mut safe_c2rust__gxdp_documents_method_info_list_IN_ARG_app_id: _ExtendedGDBusArgInfo =
    _ExtendedGDBusArgInfo {
        parent_struct: _GDBusArgInfo {
            ref_count: -(1 as gint),
            name: b"app_id\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        use_gvariant: FALSE,
    };
static mut safe_c2rust__gxdp_documents_method_info_list_IN_ARG_pointers: [*const GDBusArgInfo; 2] =
    [::core::ptr::null::<GDBusArgInfo>(); 2];
static mut safe_c2rust__gxdp_documents_method_info_list_OUT_ARG_docs: _ExtendedGDBusArgInfo =
    _ExtendedGDBusArgInfo {
        parent_struct: _GDBusArgInfo {
            ref_count: -(1 as gint),
            name: b"docs\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            signature: b"a{say}\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        use_gvariant: FALSE,
    };
static mut safe_c2rust__gxdp_documents_method_info_list_OUT_ARG_pointers: [*const GDBusArgInfo; 2] =
    [::core::ptr::null::<GDBusArgInfo>(); 2];
static mut safe_c2rust__gxdp_documents_method_info_list: _ExtendedGDBusMethodInfo = unsafe {
    _ExtendedGDBusMethodInfo {
        parent_struct: _GDBusMethodInfo {
            ref_count: -(1 as gint),
            name: b"List\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            in_args: &raw const safe_c2rust__gxdp_documents_method_info_list_IN_ARG_pointers
                as *mut *mut GDBusArgInfo,
            out_args: &raw const safe_c2rust__gxdp_documents_method_info_list_OUT_ARG_pointers
                as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"handle-list\0" as *const u8 as *const gchar,
        pass_fdlist: FALSE,
    }
};
static mut safe_c2rust__gxdp_documents_method_info_pointers: [*const GDBusMethodInfo; 12] =
    [::core::ptr::null::<GDBusMethodInfo>(); 12];
static mut safe_c2rust__gxdp_documents_property_info_version: _ExtendedGDBusPropertyInfo =
    _ExtendedGDBusPropertyInfo {
        parent_struct: _GDBusPropertyInfo {
            ref_count: 0,
            name: ::core::ptr::null::<gchar>() as *mut gchar,
            signature: ::core::ptr::null::<gchar>() as *mut gchar,
            flags: G_DBUS_PROPERTY_INFO_FLAGS_NONE,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        hyphen_name: ::core::ptr::null::<gchar>(),
        use_gvariant_emits_changed_signal: [0; 1],
        c2rust_padding: [0; 7],
    };
static mut safe_c2rust__gxdp_documents_property_info_pointers: [*const GDBusPropertyInfo; 2] =
    [::core::ptr::null::<GDBusPropertyInfo>(); 2];
static mut safe_c2rust__gxdp_documents_interface_info: _ExtendedGDBusInterfaceInfo = unsafe {
    _ExtendedGDBusInterfaceInfo {
        parent_struct: _GDBusInterfaceInfo {
            ref_count: -(1 as gint),
            name: b"org.freedesktop.portal.Documents\0" as *const u8 as *const ::core::ffi::c_char
                as *mut gchar,
            methods: &raw const safe_c2rust__gxdp_documents_method_info_pointers
                as *mut *mut GDBusMethodInfo,
            signals: ::core::ptr::null::<*mut GDBusSignalInfo>() as *mut *mut GDBusSignalInfo,
            properties: &raw const safe_c2rust__gxdp_documents_property_info_pointers
                as *mut *mut GDBusPropertyInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        hyphen_name: b"documents\0" as *const u8 as *const gchar,
    }
};
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_interface_info() -> *mut GDBusInterfaceInfo {
    return &raw const safe_c2rust__gxdp_documents_interface_info.parent_struct
        as *mut GDBusInterfaceInfo;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_override_properties(
    mut klass: *mut GObjectClass,
    mut property_id_begin: guint,
) -> guint {
    let fresh0 = property_id_begin;
    property_id_begin = property_id_begin.wrapping_add(1);
    g_object_class_override_property(klass, fresh0, b"version\0" as *const u8 as *const gchar);
    return property_id_begin.wrapping_sub(1 as guint);
}
#[inline]
unsafe extern "C" fn safe_c2rust_gxdp_documents_method_marshal_get_mount_point(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: ::core::ffi::c_uint,
    mut param_values: *const GValue,
    mut invocation_hint: *mut ::core::ffi::c_void,
    mut marshal_data: *mut ::core::ffi::c_void,
) {
    safe_c2rust__g_dbus_codegen_marshal_BOOLEAN__OBJECT(
        closure,
        return_value,
        n_param_values,
        param_values,
        invocation_hint,
        marshal_data,
    );
}
#[inline]
unsafe extern "C" fn safe_c2rust_gxdp_documents_method_marshal_add(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: ::core::ffi::c_uint,
    mut param_values: *const GValue,
    mut invocation_hint: *mut ::core::ffi::c_void,
    mut marshal_data: *mut ::core::ffi::c_void,
) {
    safe_c2rust__g_dbus_codegen_marshal_BOOLEAN__OBJECT_OBJECT_VARIANT_BOOLEAN_BOOLEAN(
        closure,
        return_value,
        n_param_values,
        param_values,
        invocation_hint,
        marshal_data,
    );
}
#[inline]
unsafe extern "C" fn safe_c2rust_gxdp_documents_method_marshal_add_named(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: ::core::ffi::c_uint,
    mut param_values: *const GValue,
    mut invocation_hint: *mut ::core::ffi::c_void,
    mut marshal_data: *mut ::core::ffi::c_void,
) {
    safe_c2rust__g_dbus_codegen_marshal_BOOLEAN__OBJECT_OBJECT_VARIANT_STRING_BOOLEAN_BOOLEAN(
        closure,
        return_value,
        n_param_values,
        param_values,
        invocation_hint,
        marshal_data,
    );
}
#[inline]
unsafe extern "C" fn safe_c2rust_gxdp_documents_method_marshal_add_full(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: ::core::ffi::c_uint,
    mut param_values: *const GValue,
    mut invocation_hint: *mut ::core::ffi::c_void,
    mut marshal_data: *mut ::core::ffi::c_void,
) {
    safe_c2rust__g_dbus_codegen_marshal_BOOLEAN__OBJECT_OBJECT_VARIANT_UINT_STRING_BOXED(
        closure,
        return_value,
        n_param_values,
        param_values,
        invocation_hint,
        marshal_data,
    );
}
#[inline]
unsafe extern "C" fn safe_c2rust_gxdp_documents_method_marshal_add_named_full(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: ::core::ffi::c_uint,
    mut param_values: *const GValue,
    mut invocation_hint: *mut ::core::ffi::c_void,
    mut marshal_data: *mut ::core::ffi::c_void,
) {
    safe_c2rust__g_dbus_codegen_marshal_BOOLEAN__OBJECT_OBJECT_VARIANT_STRING_UINT_STRING_BOXED(
        closure,
        return_value,
        n_param_values,
        param_values,
        invocation_hint,
        marshal_data,
    );
}
#[inline]
unsafe extern "C" fn safe_c2rust_gxdp_documents_method_marshal_grant_permissions(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: ::core::ffi::c_uint,
    mut param_values: *const GValue,
    mut invocation_hint: *mut ::core::ffi::c_void,
    mut marshal_data: *mut ::core::ffi::c_void,
) {
    safe_c2rust__g_dbus_codegen_marshal_BOOLEAN__OBJECT_STRING_STRING_BOXED(
        closure,
        return_value,
        n_param_values,
        param_values,
        invocation_hint,
        marshal_data,
    );
}
#[inline]
unsafe extern "C" fn safe_c2rust_gxdp_documents_method_marshal_revoke_permissions(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: ::core::ffi::c_uint,
    mut param_values: *const GValue,
    mut invocation_hint: *mut ::core::ffi::c_void,
    mut marshal_data: *mut ::core::ffi::c_void,
) {
    safe_c2rust__g_dbus_codegen_marshal_BOOLEAN__OBJECT_STRING_STRING_BOXED(
        closure,
        return_value,
        n_param_values,
        param_values,
        invocation_hint,
        marshal_data,
    );
}
#[inline]
unsafe extern "C" fn safe_c2rust_gxdp_documents_method_marshal_delete(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: ::core::ffi::c_uint,
    mut param_values: *const GValue,
    mut invocation_hint: *mut ::core::ffi::c_void,
    mut marshal_data: *mut ::core::ffi::c_void,
) {
    safe_c2rust__g_dbus_codegen_marshal_BOOLEAN__OBJECT_STRING(
        closure,
        return_value,
        n_param_values,
        param_values,
        invocation_hint,
        marshal_data,
    );
}
#[inline]
unsafe extern "C" fn safe_c2rust_gxdp_documents_method_marshal_lookup(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: ::core::ffi::c_uint,
    mut param_values: *const GValue,
    mut invocation_hint: *mut ::core::ffi::c_void,
    mut marshal_data: *mut ::core::ffi::c_void,
) {
    safe_c2rust__g_dbus_codegen_marshal_BOOLEAN__OBJECT_STRING(
        closure,
        return_value,
        n_param_values,
        param_values,
        invocation_hint,
        marshal_data,
    );
}
#[inline]
unsafe extern "C" fn safe_c2rust_gxdp_documents_method_marshal_info(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: ::core::ffi::c_uint,
    mut param_values: *const GValue,
    mut invocation_hint: *mut ::core::ffi::c_void,
    mut marshal_data: *mut ::core::ffi::c_void,
) {
    safe_c2rust__g_dbus_codegen_marshal_BOOLEAN__OBJECT_STRING(
        closure,
        return_value,
        n_param_values,
        param_values,
        invocation_hint,
        marshal_data,
    );
}
#[inline]
unsafe extern "C" fn safe_c2rust_gxdp_documents_method_marshal_list(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: ::core::ffi::c_uint,
    mut param_values: *const GValue,
    mut invocation_hint: *mut ::core::ffi::c_void,
    mut marshal_data: *mut ::core::ffi::c_void,
) {
    safe_c2rust__g_dbus_codegen_marshal_BOOLEAN__OBJECT_STRING(
        closure,
        return_value,
        n_param_values,
        param_values,
        invocation_hint,
        marshal_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_get_type() -> GType {
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
        let mut g_define_type_id: GType = g_type_register_static_simple(
            G_TYPE_INTERFACE,
            g_intern_static_string(b"GXdpDocuments\0" as *const u8 as *const gchar),
            ::core::mem::size_of::<GXdpDocumentsInterface>() as guint,
            ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GXdpDocumentsIface) -> ()>,
                    Option<unsafe extern "C" fn() -> ()>,
                >(Some(
                    safe_c2rust_gxdp_documents_default_init
                        as unsafe extern "C" fn(*mut GXdpDocumentsIface) -> (),
                )),
            ),
            0 as guint,
            ::core::mem::transmute::<*mut ::core::ffi::c_void, GInstanceInitFunc>(NULL),
            G_TYPE_FLAG_NONE,
        );
        if ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType != G_TYPE_INVALID {
            g_type_interface_add_prerequisite(
                g_define_type_id,
                ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            );
        }
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
unsafe extern "C" fn safe_c2rust_gxdp_documents_default_init(mut iface: *mut GXdpDocumentsIface) {
    g_signal_new(
        b"handle-get-mount-point\0" as *const u8 as *const gchar,
        (*(iface as *mut GTypeInterface)).g_type,
        G_SIGNAL_RUN_LAST,
        56 as ::core::ffi::c_ulong as glong as guint,
        Some(
            g_signal_accumulator_true_handled
                as unsafe extern "C" fn(
                    *mut GSignalInvocationHint,
                    *mut GValue,
                    *const GValue,
                    gpointer,
                ) -> gboolean,
        ),
        NULL,
        Some(
            safe_c2rust_gxdp_documents_method_marshal_get_mount_point
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    ::core::ffi::c_uint,
                    *const GValue,
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        G_TYPE_BOOLEAN,
        1 as guint,
        g_dbus_method_invocation_get_type(),
    );
    g_signal_new(
        b"handle-add\0" as *const u8 as *const gchar,
        (*(iface as *mut GTypeInterface)).g_type,
        G_SIGNAL_RUN_LAST,
        16 as ::core::ffi::c_ulong as glong as guint,
        Some(
            g_signal_accumulator_true_handled
                as unsafe extern "C" fn(
                    *mut GSignalInvocationHint,
                    *mut GValue,
                    *const GValue,
                    gpointer,
                ) -> gboolean,
        ),
        NULL,
        Some(
            safe_c2rust_gxdp_documents_method_marshal_add
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    ::core::ffi::c_uint,
                    *const GValue,
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        G_TYPE_BOOLEAN,
        5 as guint,
        g_dbus_method_invocation_get_type(),
        g_unix_fd_list_get_type(),
        G_TYPE_VARIANT,
        G_TYPE_BOOLEAN,
        G_TYPE_BOOLEAN,
    );
    g_signal_new(
        b"handle-add-named\0" as *const u8 as *const gchar,
        (*(iface as *mut GTypeInterface)).g_type,
        G_SIGNAL_RUN_LAST,
        32 as ::core::ffi::c_ulong as glong as guint,
        Some(
            g_signal_accumulator_true_handled
                as unsafe extern "C" fn(
                    *mut GSignalInvocationHint,
                    *mut GValue,
                    *const GValue,
                    gpointer,
                ) -> gboolean,
        ),
        NULL,
        Some(
            safe_c2rust_gxdp_documents_method_marshal_add_named
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    ::core::ffi::c_uint,
                    *const GValue,
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        G_TYPE_BOOLEAN,
        6 as guint,
        g_dbus_method_invocation_get_type(),
        g_unix_fd_list_get_type(),
        G_TYPE_VARIANT,
        G_TYPE_STRING,
        G_TYPE_BOOLEAN,
        G_TYPE_BOOLEAN,
    );
    g_signal_new(
        b"handle-add-full\0" as *const u8 as *const gchar,
        (*(iface as *mut GTypeInterface)).g_type,
        G_SIGNAL_RUN_LAST,
        24 as ::core::ffi::c_ulong as glong as guint,
        Some(
            g_signal_accumulator_true_handled
                as unsafe extern "C" fn(
                    *mut GSignalInvocationHint,
                    *mut GValue,
                    *const GValue,
                    gpointer,
                ) -> gboolean,
        ),
        NULL,
        Some(
            safe_c2rust_gxdp_documents_method_marshal_add_full
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    ::core::ffi::c_uint,
                    *const GValue,
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        G_TYPE_BOOLEAN,
        6 as guint,
        g_dbus_method_invocation_get_type(),
        g_unix_fd_list_get_type(),
        G_TYPE_VARIANT,
        G_TYPE_UINT,
        G_TYPE_STRING,
        g_strv_get_type(),
    );
    g_signal_new(
        b"handle-add-named-full\0" as *const u8 as *const gchar,
        (*(iface as *mut GTypeInterface)).g_type,
        G_SIGNAL_RUN_LAST,
        40 as ::core::ffi::c_ulong as glong as guint,
        Some(
            g_signal_accumulator_true_handled
                as unsafe extern "C" fn(
                    *mut GSignalInvocationHint,
                    *mut GValue,
                    *const GValue,
                    gpointer,
                ) -> gboolean,
        ),
        NULL,
        Some(
            safe_c2rust_gxdp_documents_method_marshal_add_named_full
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    ::core::ffi::c_uint,
                    *const GValue,
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        G_TYPE_BOOLEAN,
        7 as guint,
        g_dbus_method_invocation_get_type(),
        g_unix_fd_list_get_type(),
        G_TYPE_VARIANT,
        G_TYPE_STRING,
        G_TYPE_UINT,
        G_TYPE_STRING,
        g_strv_get_type(),
    );
    g_signal_new(
        b"handle-grant-permissions\0" as *const u8 as *const gchar,
        (*(iface as *mut GTypeInterface)).g_type,
        G_SIGNAL_RUN_LAST,
        64 as ::core::ffi::c_ulong as glong as guint,
        Some(
            g_signal_accumulator_true_handled
                as unsafe extern "C" fn(
                    *mut GSignalInvocationHint,
                    *mut GValue,
                    *const GValue,
                    gpointer,
                ) -> gboolean,
        ),
        NULL,
        Some(
            safe_c2rust_gxdp_documents_method_marshal_grant_permissions
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    ::core::ffi::c_uint,
                    *const GValue,
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        G_TYPE_BOOLEAN,
        4 as guint,
        g_dbus_method_invocation_get_type(),
        G_TYPE_STRING,
        G_TYPE_STRING,
        g_strv_get_type(),
    );
    g_signal_new(
        b"handle-revoke-permissions\0" as *const u8 as *const gchar,
        (*(iface as *mut GTypeInterface)).g_type,
        G_SIGNAL_RUN_LAST,
        96 as ::core::ffi::c_ulong as glong as guint,
        Some(
            g_signal_accumulator_true_handled
                as unsafe extern "C" fn(
                    *mut GSignalInvocationHint,
                    *mut GValue,
                    *const GValue,
                    gpointer,
                ) -> gboolean,
        ),
        NULL,
        Some(
            safe_c2rust_gxdp_documents_method_marshal_revoke_permissions
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    ::core::ffi::c_uint,
                    *const GValue,
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        G_TYPE_BOOLEAN,
        4 as guint,
        g_dbus_method_invocation_get_type(),
        G_TYPE_STRING,
        G_TYPE_STRING,
        g_strv_get_type(),
    );
    g_signal_new(
        b"handle-delete\0" as *const u8 as *const gchar,
        (*(iface as *mut GTypeInterface)).g_type,
        G_SIGNAL_RUN_LAST,
        48 as ::core::ffi::c_ulong as glong as guint,
        Some(
            g_signal_accumulator_true_handled
                as unsafe extern "C" fn(
                    *mut GSignalInvocationHint,
                    *mut GValue,
                    *const GValue,
                    gpointer,
                ) -> gboolean,
        ),
        NULL,
        Some(
            safe_c2rust_gxdp_documents_method_marshal_delete
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    ::core::ffi::c_uint,
                    *const GValue,
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        G_TYPE_BOOLEAN,
        2 as guint,
        g_dbus_method_invocation_get_type(),
        G_TYPE_STRING,
    );
    g_signal_new(
        b"handle-lookup\0" as *const u8 as *const gchar,
        (*(iface as *mut GTypeInterface)).g_type,
        G_SIGNAL_RUN_LAST,
        88 as ::core::ffi::c_ulong as glong as guint,
        Some(
            g_signal_accumulator_true_handled
                as unsafe extern "C" fn(
                    *mut GSignalInvocationHint,
                    *mut GValue,
                    *const GValue,
                    gpointer,
                ) -> gboolean,
        ),
        NULL,
        Some(
            safe_c2rust_gxdp_documents_method_marshal_lookup
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    ::core::ffi::c_uint,
                    *const GValue,
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        G_TYPE_BOOLEAN,
        2 as guint,
        g_dbus_method_invocation_get_type(),
        G_TYPE_STRING,
    );
    g_signal_new(
        b"handle-info\0" as *const u8 as *const gchar,
        (*(iface as *mut GTypeInterface)).g_type,
        G_SIGNAL_RUN_LAST,
        72 as ::core::ffi::c_ulong as glong as guint,
        Some(
            g_signal_accumulator_true_handled
                as unsafe extern "C" fn(
                    *mut GSignalInvocationHint,
                    *mut GValue,
                    *const GValue,
                    gpointer,
                ) -> gboolean,
        ),
        NULL,
        Some(
            safe_c2rust_gxdp_documents_method_marshal_info
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    ::core::ffi::c_uint,
                    *const GValue,
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        G_TYPE_BOOLEAN,
        2 as guint,
        g_dbus_method_invocation_get_type(),
        G_TYPE_STRING,
    );
    g_signal_new(
        b"handle-list\0" as *const u8 as *const gchar,
        (*(iface as *mut GTypeInterface)).g_type,
        G_SIGNAL_RUN_LAST,
        80 as ::core::ffi::c_ulong as glong as guint,
        Some(
            g_signal_accumulator_true_handled
                as unsafe extern "C" fn(
                    *mut GSignalInvocationHint,
                    *mut GValue,
                    *const GValue,
                    gpointer,
                ) -> gboolean,
        ),
        NULL,
        Some(
            safe_c2rust_gxdp_documents_method_marshal_list
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    ::core::ffi::c_uint,
                    *const GValue,
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        G_TYPE_BOOLEAN,
        2 as guint,
        g_dbus_method_invocation_get_type(),
        G_TYPE_STRING,
    );
    g_object_interface_install_property(
        iface as gpointer,
        g_param_spec_uint(
            b"version\0" as *const u8 as *const gchar,
            b"version\0" as *const u8 as *const gchar,
            b"version\0" as *const u8 as *const gchar,
            0 as guint,
            G_MAXUINT32,
            0 as guint,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_get_version(
    mut object: *mut GXdpDocuments,
) -> guint {
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = object as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_gxdp_documents_get_type();
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
            b"GXDP_IS_DOCUMENTS (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    return (*(g_type_interface_peek(
        (*(object as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_gxdp_documents_get_type(),
    ) as *mut GXdpDocumentsIface))
        .get_version
        .expect("non-null function pointer")(object);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_set_version(
    mut object: *mut GXdpDocuments,
    mut value: guint,
) {
    g_object_set(
        object as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"version\0" as *const u8 as *const gchar,
        value,
        NULL,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_call_get_mount_point(
    mut proxy: *mut GXdpDocuments,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_dbus_proxy_call(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"GetMountPoint\0" as *const u8 as *const gchar,
        g_variant_new(b"()\0" as *const u8 as *const gchar),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_call_get_mount_point_finish(
    mut proxy: *mut GXdpDocuments,
    mut out_path: *mut *mut gchar,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_finish(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        res,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(_ret, b"(^ay)\0" as *const u8 as *const gchar, out_path);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_call_get_mount_point_sync(
    mut proxy: *mut GXdpDocuments,
    mut out_path: *mut *mut gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_sync(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"GetMountPoint\0" as *const u8 as *const gchar,
        g_variant_new(b"()\0" as *const u8 as *const gchar),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(_ret, b"(^ay)\0" as *const u8 as *const gchar, out_path);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_call_add(
    mut proxy: *mut GXdpDocuments,
    mut arg_o_path_fd: *mut GVariant,
    mut arg_reuse_existing: gboolean,
    mut arg_persistent: gboolean,
    mut fd_list: *mut GUnixFDList,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_dbus_proxy_call_with_unix_fd_list(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"Add\0" as *const u8 as *const gchar,
        g_variant_new(
            b"(@hbb)\0" as *const u8 as *const gchar,
            arg_o_path_fd,
            arg_reuse_existing,
            arg_persistent,
        ),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        fd_list,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_call_add_finish(
    mut proxy: *mut GXdpDocuments,
    mut out_doc_id: *mut *mut gchar,
    mut out_fd_list: *mut *mut GUnixFDList,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_with_unix_fd_list_finish(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        out_fd_list,
        res,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(_ret, b"(s)\0" as *const u8 as *const gchar, out_doc_id);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_call_add_sync(
    mut proxy: *mut GXdpDocuments,
    mut arg_o_path_fd: *mut GVariant,
    mut arg_reuse_existing: gboolean,
    mut arg_persistent: gboolean,
    mut fd_list: *mut GUnixFDList,
    mut out_doc_id: *mut *mut gchar,
    mut out_fd_list: *mut *mut GUnixFDList,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_with_unix_fd_list_sync(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"Add\0" as *const u8 as *const gchar,
        g_variant_new(
            b"(@hbb)\0" as *const u8 as *const gchar,
            arg_o_path_fd,
            arg_reuse_existing,
            arg_persistent,
        ),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        fd_list,
        out_fd_list,
        cancellable,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(_ret, b"(s)\0" as *const u8 as *const gchar, out_doc_id);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_call_add_named(
    mut proxy: *mut GXdpDocuments,
    mut arg_o_path_parent_fd: *mut GVariant,
    mut arg_filename: *const gchar,
    mut arg_reuse_existing: gboolean,
    mut arg_persistent: gboolean,
    mut fd_list: *mut GUnixFDList,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_dbus_proxy_call_with_unix_fd_list(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"AddNamed\0" as *const u8 as *const gchar,
        g_variant_new(
            b"(@h^aybb)\0" as *const u8 as *const gchar,
            arg_o_path_parent_fd,
            arg_filename,
            arg_reuse_existing,
            arg_persistent,
        ),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        fd_list,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_call_add_named_finish(
    mut proxy: *mut GXdpDocuments,
    mut out_doc_id: *mut *mut gchar,
    mut out_fd_list: *mut *mut GUnixFDList,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_with_unix_fd_list_finish(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        out_fd_list,
        res,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(_ret, b"(s)\0" as *const u8 as *const gchar, out_doc_id);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_call_add_named_sync(
    mut proxy: *mut GXdpDocuments,
    mut arg_o_path_parent_fd: *mut GVariant,
    mut arg_filename: *const gchar,
    mut arg_reuse_existing: gboolean,
    mut arg_persistent: gboolean,
    mut fd_list: *mut GUnixFDList,
    mut out_doc_id: *mut *mut gchar,
    mut out_fd_list: *mut *mut GUnixFDList,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_with_unix_fd_list_sync(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"AddNamed\0" as *const u8 as *const gchar,
        g_variant_new(
            b"(@h^aybb)\0" as *const u8 as *const gchar,
            arg_o_path_parent_fd,
            arg_filename,
            arg_reuse_existing,
            arg_persistent,
        ),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        fd_list,
        out_fd_list,
        cancellable,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(_ret, b"(s)\0" as *const u8 as *const gchar, out_doc_id);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_call_add_full(
    mut proxy: *mut GXdpDocuments,
    mut arg_o_path_fds: *mut GVariant,
    mut arg_flags: guint,
    mut arg_app_id: *const gchar,
    mut arg_permissions: *const *const gchar,
    mut fd_list: *mut GUnixFDList,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_dbus_proxy_call_with_unix_fd_list(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"AddFull\0" as *const u8 as *const gchar,
        g_variant_new(
            b"(@ahus^as)\0" as *const u8 as *const gchar,
            arg_o_path_fds,
            arg_flags,
            arg_app_id,
            arg_permissions,
        ),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        fd_list,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_call_add_full_finish(
    mut proxy: *mut GXdpDocuments,
    mut out_doc_ids: *mut *mut *mut gchar,
    mut out_extra_out: *mut *mut GVariant,
    mut out_fd_list: *mut *mut GUnixFDList,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_with_unix_fd_list_finish(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        out_fd_list,
        res,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(
            _ret,
            b"(^as@a{sv})\0" as *const u8 as *const gchar,
            out_doc_ids,
            out_extra_out,
        );
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_call_add_full_sync(
    mut proxy: *mut GXdpDocuments,
    mut arg_o_path_fds: *mut GVariant,
    mut arg_flags: guint,
    mut arg_app_id: *const gchar,
    mut arg_permissions: *const *const gchar,
    mut fd_list: *mut GUnixFDList,
    mut out_doc_ids: *mut *mut *mut gchar,
    mut out_extra_out: *mut *mut GVariant,
    mut out_fd_list: *mut *mut GUnixFDList,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_with_unix_fd_list_sync(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"AddFull\0" as *const u8 as *const gchar,
        g_variant_new(
            b"(@ahus^as)\0" as *const u8 as *const gchar,
            arg_o_path_fds,
            arg_flags,
            arg_app_id,
            arg_permissions,
        ),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        fd_list,
        out_fd_list,
        cancellable,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(
            _ret,
            b"(^as@a{sv})\0" as *const u8 as *const gchar,
            out_doc_ids,
            out_extra_out,
        );
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_call_add_named_full(
    mut proxy: *mut GXdpDocuments,
    mut arg_o_path_fd: *mut GVariant,
    mut arg_filename: *const gchar,
    mut arg_flags: guint,
    mut arg_app_id: *const gchar,
    mut arg_permissions: *const *const gchar,
    mut fd_list: *mut GUnixFDList,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_dbus_proxy_call_with_unix_fd_list(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"AddNamedFull\0" as *const u8 as *const gchar,
        g_variant_new(
            b"(@h^ayus^as)\0" as *const u8 as *const gchar,
            arg_o_path_fd,
            arg_filename,
            arg_flags,
            arg_app_id,
            arg_permissions,
        ),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        fd_list,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_call_add_named_full_finish(
    mut proxy: *mut GXdpDocuments,
    mut out_doc_id: *mut *mut gchar,
    mut out_extra_out: *mut *mut GVariant,
    mut out_fd_list: *mut *mut GUnixFDList,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_with_unix_fd_list_finish(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        out_fd_list,
        res,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(
            _ret,
            b"(s@a{sv})\0" as *const u8 as *const gchar,
            out_doc_id,
            out_extra_out,
        );
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_call_add_named_full_sync(
    mut proxy: *mut GXdpDocuments,
    mut arg_o_path_fd: *mut GVariant,
    mut arg_filename: *const gchar,
    mut arg_flags: guint,
    mut arg_app_id: *const gchar,
    mut arg_permissions: *const *const gchar,
    mut fd_list: *mut GUnixFDList,
    mut out_doc_id: *mut *mut gchar,
    mut out_extra_out: *mut *mut GVariant,
    mut out_fd_list: *mut *mut GUnixFDList,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_with_unix_fd_list_sync(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"AddNamedFull\0" as *const u8 as *const gchar,
        g_variant_new(
            b"(@h^ayus^as)\0" as *const u8 as *const gchar,
            arg_o_path_fd,
            arg_filename,
            arg_flags,
            arg_app_id,
            arg_permissions,
        ),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        fd_list,
        out_fd_list,
        cancellable,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(
            _ret,
            b"(s@a{sv})\0" as *const u8 as *const gchar,
            out_doc_id,
            out_extra_out,
        );
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_call_grant_permissions(
    mut proxy: *mut GXdpDocuments,
    mut arg_doc_id: *const gchar,
    mut arg_app_id: *const gchar,
    mut arg_permissions: *const *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_dbus_proxy_call(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"GrantPermissions\0" as *const u8 as *const gchar,
        g_variant_new(
            b"(ss^as)\0" as *const u8 as *const gchar,
            arg_doc_id,
            arg_app_id,
            arg_permissions,
        ),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_call_grant_permissions_finish(
    mut proxy: *mut GXdpDocuments,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_finish(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        res,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(_ret, b"()\0" as *const u8 as *const gchar);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_call_grant_permissions_sync(
    mut proxy: *mut GXdpDocuments,
    mut arg_doc_id: *const gchar,
    mut arg_app_id: *const gchar,
    mut arg_permissions: *const *const gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_sync(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"GrantPermissions\0" as *const u8 as *const gchar,
        g_variant_new(
            b"(ss^as)\0" as *const u8 as *const gchar,
            arg_doc_id,
            arg_app_id,
            arg_permissions,
        ),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(_ret, b"()\0" as *const u8 as *const gchar);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_call_revoke_permissions(
    mut proxy: *mut GXdpDocuments,
    mut arg_doc_id: *const gchar,
    mut arg_app_id: *const gchar,
    mut arg_permissions: *const *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_dbus_proxy_call(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"RevokePermissions\0" as *const u8 as *const gchar,
        g_variant_new(
            b"(ss^as)\0" as *const u8 as *const gchar,
            arg_doc_id,
            arg_app_id,
            arg_permissions,
        ),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_call_revoke_permissions_finish(
    mut proxy: *mut GXdpDocuments,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_finish(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        res,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(_ret, b"()\0" as *const u8 as *const gchar);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_call_revoke_permissions_sync(
    mut proxy: *mut GXdpDocuments,
    mut arg_doc_id: *const gchar,
    mut arg_app_id: *const gchar,
    mut arg_permissions: *const *const gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_sync(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"RevokePermissions\0" as *const u8 as *const gchar,
        g_variant_new(
            b"(ss^as)\0" as *const u8 as *const gchar,
            arg_doc_id,
            arg_app_id,
            arg_permissions,
        ),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(_ret, b"()\0" as *const u8 as *const gchar);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_call_delete(
    mut proxy: *mut GXdpDocuments,
    mut arg_doc_id: *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_dbus_proxy_call(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"Delete\0" as *const u8 as *const gchar,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, arg_doc_id),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_call_delete_finish(
    mut proxy: *mut GXdpDocuments,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_finish(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        res,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(_ret, b"()\0" as *const u8 as *const gchar);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_call_delete_sync(
    mut proxy: *mut GXdpDocuments,
    mut arg_doc_id: *const gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_sync(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"Delete\0" as *const u8 as *const gchar,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, arg_doc_id),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(_ret, b"()\0" as *const u8 as *const gchar);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_call_lookup(
    mut proxy: *mut GXdpDocuments,
    mut arg_filename: *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_dbus_proxy_call(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"Lookup\0" as *const u8 as *const gchar,
        g_variant_new(b"(^ay)\0" as *const u8 as *const gchar, arg_filename),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_call_lookup_finish(
    mut proxy: *mut GXdpDocuments,
    mut out_doc_id: *mut *mut gchar,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_finish(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        res,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(_ret, b"(s)\0" as *const u8 as *const gchar, out_doc_id);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_call_lookup_sync(
    mut proxy: *mut GXdpDocuments,
    mut arg_filename: *const gchar,
    mut out_doc_id: *mut *mut gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_sync(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"Lookup\0" as *const u8 as *const gchar,
        g_variant_new(b"(^ay)\0" as *const u8 as *const gchar, arg_filename),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(_ret, b"(s)\0" as *const u8 as *const gchar, out_doc_id);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_call_info(
    mut proxy: *mut GXdpDocuments,
    mut arg_doc_id: *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_dbus_proxy_call(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"Info\0" as *const u8 as *const gchar,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, arg_doc_id),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_call_info_finish(
    mut proxy: *mut GXdpDocuments,
    mut out_path: *mut *mut gchar,
    mut out_apps: *mut *mut GVariant,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_finish(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        res,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(
            _ret,
            b"(^ay@a{sas})\0" as *const u8 as *const gchar,
            out_path,
            out_apps,
        );
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_call_info_sync(
    mut proxy: *mut GXdpDocuments,
    mut arg_doc_id: *const gchar,
    mut out_path: *mut *mut gchar,
    mut out_apps: *mut *mut GVariant,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_sync(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"Info\0" as *const u8 as *const gchar,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, arg_doc_id),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(
            _ret,
            b"(^ay@a{sas})\0" as *const u8 as *const gchar,
            out_path,
            out_apps,
        );
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_call_list(
    mut proxy: *mut GXdpDocuments,
    mut arg_app_id: *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_dbus_proxy_call(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"List\0" as *const u8 as *const gchar,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, arg_app_id),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_call_list_finish(
    mut proxy: *mut GXdpDocuments,
    mut out_docs: *mut *mut GVariant,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_finish(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        res,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(_ret, b"(@a{say})\0" as *const u8 as *const gchar, out_docs);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_call_list_sync(
    mut proxy: *mut GXdpDocuments,
    mut arg_app_id: *const gchar,
    mut out_docs: *mut *mut GVariant,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_sync(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"List\0" as *const u8 as *const gchar,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, arg_app_id),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(_ret, b"(@a{say})\0" as *const u8 as *const gchar, out_docs);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_complete_get_mount_point(
    mut object: *mut GXdpDocuments,
    mut invocation: *mut GDBusMethodInvocation,
    mut path: *const gchar,
) {
    g_dbus_method_invocation_return_value(
        invocation,
        g_variant_new(b"(^ay)\0" as *const u8 as *const gchar, path),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_complete_add(
    mut object: *mut GXdpDocuments,
    mut invocation: *mut GDBusMethodInvocation,
    mut fd_list: *mut GUnixFDList,
    mut doc_id: *const gchar,
) {
    g_dbus_method_invocation_return_value_with_unix_fd_list(
        invocation,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, doc_id),
        fd_list,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_complete_add_named(
    mut object: *mut GXdpDocuments,
    mut invocation: *mut GDBusMethodInvocation,
    mut fd_list: *mut GUnixFDList,
    mut doc_id: *const gchar,
) {
    g_dbus_method_invocation_return_value_with_unix_fd_list(
        invocation,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, doc_id),
        fd_list,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_complete_add_full(
    mut object: *mut GXdpDocuments,
    mut invocation: *mut GDBusMethodInvocation,
    mut fd_list: *mut GUnixFDList,
    mut doc_ids: *const *const gchar,
    mut extra_out: *mut GVariant,
) {
    g_dbus_method_invocation_return_value_with_unix_fd_list(
        invocation,
        g_variant_new(
            b"(^as@a{sv})\0" as *const u8 as *const gchar,
            doc_ids,
            extra_out,
        ),
        fd_list,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_complete_add_named_full(
    mut object: *mut GXdpDocuments,
    mut invocation: *mut GDBusMethodInvocation,
    mut fd_list: *mut GUnixFDList,
    mut doc_id: *const gchar,
    mut extra_out: *mut GVariant,
) {
    g_dbus_method_invocation_return_value_with_unix_fd_list(
        invocation,
        g_variant_new(
            b"(s@a{sv})\0" as *const u8 as *const gchar,
            doc_id,
            extra_out,
        ),
        fd_list,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_complete_grant_permissions(
    mut object: *mut GXdpDocuments,
    mut invocation: *mut GDBusMethodInvocation,
) {
    g_dbus_method_invocation_return_value(
        invocation,
        g_variant_new(b"()\0" as *const u8 as *const gchar),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_complete_revoke_permissions(
    mut object: *mut GXdpDocuments,
    mut invocation: *mut GDBusMethodInvocation,
) {
    g_dbus_method_invocation_return_value(
        invocation,
        g_variant_new(b"()\0" as *const u8 as *const gchar),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_complete_delete(
    mut object: *mut GXdpDocuments,
    mut invocation: *mut GDBusMethodInvocation,
) {
    g_dbus_method_invocation_return_value(
        invocation,
        g_variant_new(b"()\0" as *const u8 as *const gchar),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_complete_lookup(
    mut object: *mut GXdpDocuments,
    mut invocation: *mut GDBusMethodInvocation,
    mut doc_id: *const gchar,
) {
    g_dbus_method_invocation_return_value(
        invocation,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, doc_id),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_complete_info(
    mut object: *mut GXdpDocuments,
    mut invocation: *mut GDBusMethodInvocation,
    mut path: *const gchar,
    mut apps: *mut GVariant,
) {
    g_dbus_method_invocation_return_value(
        invocation,
        g_variant_new(b"(^ay@a{sas})\0" as *const u8 as *const gchar, path, apps),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_complete_list(
    mut object: *mut GXdpDocuments,
    mut invocation: *mut GDBusMethodInvocation,
    mut docs: *mut GVariant,
) {
    g_dbus_method_invocation_return_value(
        invocation,
        g_variant_new(b"(@a{say})\0" as *const u8 as *const gchar, docs),
    );
}
unsafe extern "C" fn safe_c2rust_gxdp_documents_proxy_class_intern_init(mut klass: gpointer) {
    safe_c2rust_gxdp_documents_proxy_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GXdpDocumentsProxy_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GXdpDocumentsProxy_private_offset,
        );
    }
    safe_c2rust_gxdp_documents_proxy_class_init(klass as *mut GXdpDocumentsProxyClass);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_proxy_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_gxdp_documents_proxy_get_type_once();
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
#[inline]
unsafe extern "C" fn safe_c2rust_gxdp_documents_proxy_get_instance_private(
    mut self_0: *mut GXdpDocumentsProxy,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GXdpDocumentsProxy_private_offset as glong as isize)
        as gpointer;
}
static mut safe_c2rust_GXdpDocumentsProxy_private_offset: gint = 0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_gxdp_documents_proxy_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_dbus_proxy_get_type(),
        g_intern_static_string(b"GXdpDocumentsProxy\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GXdpDocumentsProxyClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_gxdp_documents_proxy_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GXdpDocumentsProxy>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GXdpDocumentsProxy) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_gxdp_documents_proxy_init
                    as unsafe extern "C" fn(*mut GXdpDocumentsProxy) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GXdpDocumentsProxy_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GXdpDocumentsProxyPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GXdpDocumentsIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_gxdp_documents_proxy_iface_init
                as unsafe extern "C" fn(*mut GXdpDocumentsIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        safe_c2rust_gxdp_documents_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
static mut safe_c2rust_gxdp_documents_proxy_parent_class: gpointer = NULL;
unsafe extern "C" fn safe_c2rust_gxdp_documents_proxy_finalize(mut object: *mut GObject) {
    let mut proxy: *mut GXdpDocumentsProxy =
        object as *mut ::core::ffi::c_void as *mut GXdpDocumentsProxy;
    g_datalist_clear(&raw mut (*(*proxy).priv_0).qdata);
    (*(safe_c2rust_gxdp_documents_proxy_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_gxdp_documents_proxy_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut info: *const _ExtendedGDBusPropertyInfo =
        ::core::ptr::null::<_ExtendedGDBusPropertyInfo>();
    let mut variant: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if prop_id != 0 as guint && prop_id.wrapping_sub(1 as guint) < 1 as guint {
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
            b"gio/xdp-dbus.c\0" as *const u8 as *const ::core::ffi::c_char,
            3531 as ::core::ffi::c_int,
            G_STRFUNC,
            b"prop_id != 0 && prop_id - 1 < 1\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    info = safe_c2rust__gxdp_documents_property_info_pointers
        [prop_id.wrapping_sub(1 as guint) as usize] as *const _ExtendedGDBusPropertyInfo;
    variant = g_dbus_proxy_get_cached_property(
        object as *mut ::core::ffi::c_void as *mut GDBusProxy,
        (*info).parent_struct.name,
    );
    if (*info).use_gvariant() != 0 {
        g_value_set_variant(value, variant);
    } else if !variant.is_null() {
        g_dbus_gvariant_to_gvalue(variant, value);
    }
    if !variant.is_null() {
        g_variant_unref(variant);
    }
}
unsafe extern "C" fn safe_c2rust_gxdp_documents_proxy_set_property_cb(
    mut proxy: *mut GDBusProxy,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut info: *const _ExtendedGDBusPropertyInfo =
        user_data as *const _ExtendedGDBusPropertyInfo;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    error = ::core::ptr::null_mut::<GError>();
    _ret = g_dbus_proxy_call_finish(proxy, res, &raw mut error);
    if _ret.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Error setting property '%s' on interface org.freedesktop.portal.Documents: %s (%s, %d)\0"
                as *const u8 as *const gchar,
            (*info).parent_struct.name,
            (*error).message,
            g_quark_to_string((*error).domain),
            (*error).code,
        );
        g_error_free(error);
    } else {
        g_variant_unref(_ret);
    };
}
unsafe extern "C" fn safe_c2rust_gxdp_documents_proxy_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut info: *const _ExtendedGDBusPropertyInfo =
        ::core::ptr::null::<_ExtendedGDBusPropertyInfo>();
    let mut variant: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if prop_id != 0 as guint && prop_id.wrapping_sub(1 as guint) < 1 as guint {
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
            b"gio/xdp-dbus.c\0" as *const u8 as *const ::core::ffi::c_char,
            3578 as ::core::ffi::c_int,
            G_STRFUNC,
            b"prop_id != 0 && prop_id - 1 < 1\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    info = safe_c2rust__gxdp_documents_property_info_pointers
        [prop_id.wrapping_sub(1 as guint) as usize] as *const _ExtendedGDBusPropertyInfo;
    variant = g_dbus_gvalue_to_gvariant(
        value,
        g_variant_type_checked_((*info).parent_struct.signature),
    );
    g_dbus_proxy_call(
        object as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"org.freedesktop.DBus.Properties.Set\0" as *const u8 as *const gchar,
        g_variant_new(
            b"(ssv)\0" as *const u8 as *const gchar,
            b"org.freedesktop.portal.Documents\0" as *const u8 as *const ::core::ffi::c_char,
            (*info).parent_struct.name,
            variant,
        ),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        ::core::ptr::null_mut::<GCancellable>(),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GDBusProxy, *mut GAsyncResult, gpointer) -> ()>,
            GAsyncReadyCallback,
        >(Some(
            safe_c2rust_gxdp_documents_proxy_set_property_cb
                as unsafe extern "C" fn(*mut GDBusProxy, *mut GAsyncResult, gpointer) -> (),
        )),
        &raw const (*info).parent_struct as *mut GDBusPropertyInfo as gpointer,
    );
    g_variant_unref(variant);
}
unsafe extern "C" fn safe_c2rust_gxdp_documents_proxy_g_signal(
    mut proxy: *mut GDBusProxy,
    mut sender_name: *const gchar,
    mut signal_name: *const gchar,
    mut parameters: *mut GVariant,
) {
    let mut info: *mut _ExtendedGDBusSignalInfo =
        ::core::ptr::null_mut::<_ExtendedGDBusSignalInfo>();
    let mut iter: GVariantIter = _GVariantIter { x: [0; 16] };
    let mut child: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut paramv: *mut GValue = ::core::ptr::null_mut::<GValue>();
    let mut num_params: gsize = 0;
    let mut n: gsize = 0;
    let mut signal_id: guint = 0;
    info = g_dbus_interface_info_lookup_signal(
        &raw const safe_c2rust__gxdp_documents_interface_info.parent_struct
            as *mut GDBusInterfaceInfo,
        signal_name,
    ) as *mut _ExtendedGDBusSignalInfo;
    if info.is_null() {
        return;
    }
    num_params = g_variant_n_children(parameters);
    paramv = ({
        let mut __n: gsize = num_params.wrapping_add(1 as gsize);
        let mut __s: gsize = ::core::mem::size_of::<GValue>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GValue;
    g_value_init(
        paramv.offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        safe_c2rust_gxdp_documents_get_type(),
    );
    g_value_set_object(
        paramv.offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        proxy as gpointer,
    );
    g_variant_iter_init(&raw mut iter, parameters);
    n = 1 as gsize;
    loop {
        child = g_variant_iter_next_value(&raw mut iter);
        if child.is_null() {
            break;
        }
        let mut arg_info: *mut _ExtendedGDBusArgInfo = *(*info)
            .parent_struct
            .args
            .offset(n.wrapping_sub(1 as gsize) as isize)
            as *mut _ExtendedGDBusArgInfo;
        if (*arg_info).use_gvariant != 0 {
            g_value_init(paramv.offset(n as isize) as *mut GValue, G_TYPE_VARIANT);
            g_value_set_variant(paramv.offset(n as isize) as *mut GValue, child);
            n = n.wrapping_add(1);
        } else {
            let fresh1 = n;
            n = n.wrapping_add(1);
            g_dbus_gvariant_to_gvalue(child, paramv.offset(fresh1 as isize) as *mut GValue);
        }
        g_variant_unref(child);
    }
    signal_id = g_signal_lookup((*info).signal_name, safe_c2rust_gxdp_documents_get_type());
    g_signal_emitv(
        paramv,
        signal_id,
        0 as GQuark,
        ::core::ptr::null_mut::<GValue>(),
    );
    n = 0 as gsize;
    while n < num_params.wrapping_add(1 as gsize) {
        g_value_unset(paramv.offset(n as isize) as *mut GValue);
        n = n.wrapping_add(1);
    }
    g_free(paramv as gpointer);
}
unsafe extern "C" fn safe_c2rust_gxdp_documents_proxy_g_properties_changed(
    mut _proxy: *mut GDBusProxy,
    mut changed_properties: *mut GVariant,
    mut invalidated_properties: *const *const gchar,
) {
    let mut proxy: *mut GXdpDocumentsProxy =
        _proxy as *mut ::core::ffi::c_void as *mut GXdpDocumentsProxy;
    let mut n: guint = 0;
    let mut key: *const gchar = ::core::ptr::null::<gchar>();
    let mut iter: *mut GVariantIter = ::core::ptr::null_mut::<GVariantIter>();
    let mut info: *mut _ExtendedGDBusPropertyInfo =
        ::core::ptr::null_mut::<_ExtendedGDBusPropertyInfo>();
    g_variant_get(
        changed_properties,
        b"a{sv}\0" as *const u8 as *const gchar,
        &raw mut iter,
    );
    while g_variant_iter_next(
        iter,
        b"{&sv}\0" as *const u8 as *const gchar,
        &raw mut key,
        NULL,
    ) != 0
    {
        info = g_dbus_interface_info_lookup_property(
            &raw const safe_c2rust__gxdp_documents_interface_info.parent_struct
                as *mut GDBusInterfaceInfo,
            key,
        ) as *mut _ExtendedGDBusPropertyInfo;
        g_datalist_id_set_data_full(
            &raw mut (*(*proxy).priv_0).qdata,
            g_quark_try_string(key),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
        );
        if !info.is_null() {
            g_object_notify(
                proxy as *mut ::core::ffi::c_void as *mut GObject,
                (*info).hyphen_name,
            );
        }
    }
    g_variant_iter_free(iter);
    n = 0 as guint;
    while !(*invalidated_properties.offset(n as isize)).is_null() {
        info = g_dbus_interface_info_lookup_property(
            &raw const safe_c2rust__gxdp_documents_interface_info.parent_struct
                as *mut GDBusInterfaceInfo,
            *invalidated_properties.offset(n as isize),
        ) as *mut _ExtendedGDBusPropertyInfo;
        g_datalist_id_set_data_full(
            &raw mut (*(*proxy).priv_0).qdata,
            g_quark_try_string(*invalidated_properties.offset(n as isize)),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
        );
        if !info.is_null() {
            g_object_notify(
                proxy as *mut ::core::ffi::c_void as *mut GObject,
                (*info).hyphen_name,
            );
        }
        n = n.wrapping_add(1);
    }
}
unsafe extern "C" fn safe_c2rust_gxdp_documents_proxy_get_version(
    mut object: *mut GXdpDocuments,
) -> guint {
    let mut proxy: *mut GXdpDocumentsProxy =
        object as *mut ::core::ffi::c_void as *mut GXdpDocumentsProxy;
    let mut variant: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut value: guint = 0 as guint;
    variant = g_dbus_proxy_get_cached_property(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"version\0" as *const u8 as *const gchar,
    );
    if !variant.is_null() {
        value = g_variant_get_uint32(variant) as guint;
        g_variant_unref(variant);
    }
    return value;
}
unsafe extern "C" fn safe_c2rust_gxdp_documents_proxy_init(mut proxy: *mut GXdpDocumentsProxy) {
    (*proxy).priv_0 = safe_c2rust_gxdp_documents_proxy_get_instance_private(proxy)
        as *mut GXdpDocumentsProxyPrivate;
    g_dbus_proxy_set_interface_info(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        safe_c2rust_gxdp_documents_interface_info(),
    );
}
unsafe extern "C" fn safe_c2rust_gxdp_documents_proxy_class_init(
    mut klass: *mut GXdpDocumentsProxyClass,
) {
    let mut gobject_class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    let mut proxy_class: *mut GDBusProxyClass = ::core::ptr::null_mut::<GDBusProxyClass>();
    gobject_class = klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize =
        Some(safe_c2rust_gxdp_documents_proxy_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_gxdp_documents_proxy_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_gxdp_documents_proxy_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    proxy_class = klass as *mut ::core::ffi::c_void as *mut GDBusProxyClass;
    (*proxy_class).g_signal = Some(
        safe_c2rust_gxdp_documents_proxy_g_signal
            as unsafe extern "C" fn(
                *mut GDBusProxy,
                *const gchar,
                *const gchar,
                *mut GVariant,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(*mut GDBusProxy, *const gchar, *const gchar, *mut GVariant) -> (),
        >;
    (*proxy_class).g_properties_changed = Some(
        safe_c2rust_gxdp_documents_proxy_g_properties_changed
            as unsafe extern "C" fn(*mut GDBusProxy, *mut GVariant, *const *const gchar) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GDBusProxy, *mut GVariant, *const *const gchar) -> ()>;
    safe_c2rust_gxdp_documents_override_properties(gobject_class, 1 as guint);
}
unsafe extern "C" fn safe_c2rust_gxdp_documents_proxy_iface_init(
    mut iface: *mut GXdpDocumentsIface,
) {
    (*iface).get_version = Some(
        safe_c2rust_gxdp_documents_proxy_get_version
            as unsafe extern "C" fn(*mut GXdpDocuments) -> guint,
    ) as Option<unsafe extern "C" fn(*mut GXdpDocuments) -> guint>;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_proxy_new(
    mut connection: *mut GDBusConnection,
    mut flags: GDBusProxyFlags,
    mut name: *const gchar,
    mut object_path: *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_async_initable_new_async(
        safe_c2rust_gxdp_documents_proxy_get_type(),
        G_PRIORITY_DEFAULT,
        cancellable,
        callback,
        user_data,
        b"g-flags\0" as *const u8 as *const gchar,
        flags as ::core::ffi::c_uint,
        b"g-name\0" as *const u8 as *const ::core::ffi::c_char,
        name,
        b"g-connection\0" as *const u8 as *const ::core::ffi::c_char,
        connection,
        b"g-object-path\0" as *const u8 as *const ::core::ffi::c_char,
        object_path,
        b"g-interface-name\0" as *const u8 as *const ::core::ffi::c_char,
        b"org.freedesktop.portal.Documents\0" as *const u8 as *const ::core::ffi::c_char,
        NULL,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_proxy_new_finish(
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GXdpDocuments {
    let mut ret: *mut GObject = ::core::ptr::null_mut::<GObject>();
    let mut source_object: *mut GObject = ::core::ptr::null_mut::<GObject>();
    source_object = g_async_result_get_source_object(res);
    ret = g_async_initable_new_finish(
        source_object as *mut ::core::ffi::c_void as *mut GAsyncInitable,
        res,
        error,
    );
    g_object_unref(source_object as gpointer);
    if !ret.is_null() {
        return ret as *mut ::core::ffi::c_void as *mut GXdpDocuments;
    } else {
        return ::core::ptr::null_mut::<GXdpDocuments>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_proxy_new_sync(
    mut connection: *mut GDBusConnection,
    mut flags: GDBusProxyFlags,
    mut name: *const gchar,
    mut object_path: *const gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GXdpDocuments {
    let mut ret: *mut GInitable = ::core::ptr::null_mut::<GInitable>();
    ret = g_initable_new(
        safe_c2rust_gxdp_documents_proxy_get_type(),
        cancellable,
        error,
        b"g-flags\0" as *const u8 as *const gchar,
        flags as ::core::ffi::c_uint,
        b"g-name\0" as *const u8 as *const ::core::ffi::c_char,
        name,
        b"g-connection\0" as *const u8 as *const ::core::ffi::c_char,
        connection,
        b"g-object-path\0" as *const u8 as *const ::core::ffi::c_char,
        object_path,
        b"g-interface-name\0" as *const u8 as *const ::core::ffi::c_char,
        b"org.freedesktop.portal.Documents\0" as *const u8 as *const ::core::ffi::c_char,
        NULL,
    ) as *mut GInitable;
    if !ret.is_null() {
        return ret as *mut ::core::ffi::c_void as *mut GXdpDocuments;
    } else {
        return ::core::ptr::null_mut::<GXdpDocuments>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_proxy_new_for_bus(
    mut bus_type: GBusType,
    mut flags: GDBusProxyFlags,
    mut name: *const gchar,
    mut object_path: *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_async_initable_new_async(
        safe_c2rust_gxdp_documents_proxy_get_type(),
        G_PRIORITY_DEFAULT,
        cancellable,
        callback,
        user_data,
        b"g-flags\0" as *const u8 as *const gchar,
        flags as ::core::ffi::c_uint,
        b"g-name\0" as *const u8 as *const ::core::ffi::c_char,
        name,
        b"g-bus-type\0" as *const u8 as *const ::core::ffi::c_char,
        bus_type as ::core::ffi::c_int,
        b"g-object-path\0" as *const u8 as *const ::core::ffi::c_char,
        object_path,
        b"g-interface-name\0" as *const u8 as *const ::core::ffi::c_char,
        b"org.freedesktop.portal.Documents\0" as *const u8 as *const ::core::ffi::c_char,
        NULL,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_proxy_new_for_bus_finish(
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GXdpDocuments {
    let mut ret: *mut GObject = ::core::ptr::null_mut::<GObject>();
    let mut source_object: *mut GObject = ::core::ptr::null_mut::<GObject>();
    source_object = g_async_result_get_source_object(res);
    ret = g_async_initable_new_finish(
        source_object as *mut ::core::ffi::c_void as *mut GAsyncInitable,
        res,
        error,
    );
    g_object_unref(source_object as gpointer);
    if !ret.is_null() {
        return ret as *mut ::core::ffi::c_void as *mut GXdpDocuments;
    } else {
        return ::core::ptr::null_mut::<GXdpDocuments>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_proxy_new_for_bus_sync(
    mut bus_type: GBusType,
    mut flags: GDBusProxyFlags,
    mut name: *const gchar,
    mut object_path: *const gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GXdpDocuments {
    let mut ret: *mut GInitable = ::core::ptr::null_mut::<GInitable>();
    ret = g_initable_new(
        safe_c2rust_gxdp_documents_proxy_get_type(),
        cancellable,
        error,
        b"g-flags\0" as *const u8 as *const gchar,
        flags as ::core::ffi::c_uint,
        b"g-name\0" as *const u8 as *const ::core::ffi::c_char,
        name,
        b"g-bus-type\0" as *const u8 as *const ::core::ffi::c_char,
        bus_type as ::core::ffi::c_int,
        b"g-object-path\0" as *const u8 as *const ::core::ffi::c_char,
        object_path,
        b"g-interface-name\0" as *const u8 as *const ::core::ffi::c_char,
        b"org.freedesktop.portal.Documents\0" as *const u8 as *const ::core::ffi::c_char,
        NULL,
    ) as *mut GInitable;
    if !ret.is_null() {
        return ret as *mut ::core::ffi::c_void as *mut GXdpDocuments;
    } else {
        return ::core::ptr::null_mut::<GXdpDocuments>();
    };
}
unsafe extern "C" fn safe_c2rust__gxdp_documents_skeleton_handle_method_call(
    mut connection: *mut GDBusConnection,
    mut sender: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut method_name: *const gchar,
    mut parameters: *mut GVariant,
    mut invocation: *mut GDBusMethodInvocation,
    mut user_data: gpointer,
) {
    let mut skeleton: *mut GXdpDocumentsSkeleton = user_data as *mut GXdpDocumentsSkeleton;
    let mut info: *mut _ExtendedGDBusMethodInfo =
        ::core::ptr::null_mut::<_ExtendedGDBusMethodInfo>();
    let mut iter: GVariantIter = _GVariantIter { x: [0; 16] };
    let mut child: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut paramv: *mut GValue = ::core::ptr::null_mut::<GValue>();
    let mut num_params: gsize = 0;
    let mut num_extra: guint = 0;
    let mut n: gsize = 0;
    let mut signal_id: guint = 0;
    let mut return_value: GValue = _GValue {
        g_type: 0 as GType,
        data: [
            C2RustUnnamed_1 {
                v_int: 0 as ::core::ffi::c_int,
            },
            C2RustUnnamed_1 { v_int: 0 },
        ],
    };
    info = g_dbus_method_invocation_get_method_info(invocation) as *mut _ExtendedGDBusMethodInfo;
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if !info.is_null() {
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
            b"gio/xdp-dbus.c\0" as *const u8 as *const ::core::ffi::c_char,
            3941 as ::core::ffi::c_int,
            G_STRFUNC,
            b"info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    num_params = g_variant_n_children(parameters);
    num_extra = (if (*info).pass_fdlist != 0 {
        3 as ::core::ffi::c_int
    } else {
        2 as ::core::ffi::c_int
    }) as guint;
    paramv = ({
        let mut __n: gsize = num_params.wrapping_add(num_extra as gsize);
        let mut __s: gsize = ::core::mem::size_of::<GValue>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GValue;
    n = 0 as gsize;
    g_value_init(
        paramv.offset(n as isize) as *mut GValue,
        safe_c2rust_gxdp_documents_get_type(),
    );
    let fresh2 = n;
    n = n.wrapping_add(1);
    g_value_set_object(
        paramv.offset(fresh2 as isize) as *mut GValue,
        skeleton as gpointer,
    );
    g_value_init(
        paramv.offset(n as isize) as *mut GValue,
        g_dbus_method_invocation_get_type(),
    );
    let fresh3 = n;
    n = n.wrapping_add(1);
    g_value_set_object(
        paramv.offset(fresh3 as isize) as *mut GValue,
        invocation as gpointer,
    );
    if (*info).pass_fdlist != 0 {
        g_value_init(
            paramv.offset(n as isize) as *mut GValue,
            g_unix_fd_list_get_type(),
        );
        let fresh4 = n;
        n = n.wrapping_add(1);
        g_value_set_object(
            paramv.offset(fresh4 as isize) as *mut GValue,
            g_dbus_message_get_unix_fd_list(g_dbus_method_invocation_get_message(invocation))
                as gpointer,
        );
    }
    g_variant_iter_init(&raw mut iter, parameters);
    loop {
        child = g_variant_iter_next_value(&raw mut iter);
        if child.is_null() {
            break;
        }
        let mut arg_info: *mut _ExtendedGDBusArgInfo = *(*info)
            .parent_struct
            .in_args
            .offset(n.wrapping_sub(num_extra as gsize) as isize)
            as *mut _ExtendedGDBusArgInfo;
        if (*arg_info).use_gvariant != 0 {
            g_value_init(paramv.offset(n as isize) as *mut GValue, G_TYPE_VARIANT);
            g_value_set_variant(paramv.offset(n as isize) as *mut GValue, child);
            n = n.wrapping_add(1);
        } else {
            let fresh5 = n;
            n = n.wrapping_add(1);
            g_dbus_gvariant_to_gvalue(child, paramv.offset(fresh5 as isize) as *mut GValue);
        }
        g_variant_unref(child);
    }
    signal_id = g_signal_lookup((*info).signal_name, safe_c2rust_gxdp_documents_get_type());
    g_value_init(&raw mut return_value, G_TYPE_BOOLEAN);
    g_signal_emitv(paramv, signal_id, 0 as GQuark, &raw mut return_value);
    if g_value_get_boolean(&raw mut return_value) == 0 {
        g_dbus_method_invocation_return_error(
            invocation,
            g_dbus_error_quark(),
            G_DBUS_ERROR_UNKNOWN_METHOD as ::core::ffi::c_int as gint,
            b"Method %s is not implemented on interface %s\0" as *const u8 as *const gchar,
            method_name,
            interface_name,
        );
    }
    g_value_unset(&raw mut return_value);
    n = 0 as gsize;
    while n < num_params.wrapping_add(num_extra as gsize) {
        g_value_unset(paramv.offset(n as isize) as *mut GValue);
        n = n.wrapping_add(1);
    }
    g_free(paramv as gpointer);
}
unsafe extern "C" fn safe_c2rust__gxdp_documents_skeleton_handle_get_property(
    mut connection: *mut GDBusConnection,
    mut sender: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut property_name: *const gchar,
    mut error: *mut *mut GError,
    mut user_data: gpointer,
) -> *mut GVariant {
    let mut skeleton: *mut GXdpDocumentsSkeleton = user_data as *mut GXdpDocumentsSkeleton;
    let mut value: GValue = _GValue {
        g_type: 0 as GType,
        data: [
            C2RustUnnamed_1 {
                v_int: 0 as ::core::ffi::c_int,
            },
            C2RustUnnamed_1 { v_int: 0 },
        ],
    };
    let mut pspec: *mut GParamSpec = ::core::ptr::null_mut::<GParamSpec>();
    let mut info: *mut _ExtendedGDBusPropertyInfo =
        ::core::ptr::null_mut::<_ExtendedGDBusPropertyInfo>();
    let mut ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    ret = ::core::ptr::null_mut::<GVariant>();
    info = g_dbus_interface_info_lookup_property(
        &raw const safe_c2rust__gxdp_documents_interface_info.parent_struct
            as *mut GDBusInterfaceInfo,
        property_name,
    ) as *mut _ExtendedGDBusPropertyInfo;
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if !info.is_null() {
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
            b"gio/xdp-dbus.c\0" as *const u8 as *const ::core::ffi::c_char,
            4000 as ::core::ffi::c_int,
            G_STRFUNC,
            b"info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    pspec = g_object_class_find_property(
        (*(skeleton as *mut GTypeInstance)).g_class as *mut GObjectClass,
        (*info).hyphen_name,
    );
    if pspec.is_null() {
        g_set_error(
            error,
            g_dbus_error_quark(),
            G_DBUS_ERROR_INVALID_ARGS as ::core::ffi::c_int as gint,
            b"No property with name %s\0" as *const u8 as *const gchar,
            property_name,
        );
    } else {
        g_value_init(&raw mut value, (*pspec).value_type);
        g_object_get_property(
            skeleton as *mut ::core::ffi::c_void as *mut GObject,
            (*info).hyphen_name,
            &raw mut value,
        );
        ret = g_dbus_gvalue_to_gvariant(
            &raw mut value,
            g_variant_type_checked_((*info).parent_struct.signature),
        );
        g_value_unset(&raw mut value);
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust__gxdp_documents_skeleton_handle_set_property(
    mut connection: *mut GDBusConnection,
    mut sender: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut property_name: *const gchar,
    mut variant: *mut GVariant,
    mut error: *mut *mut GError,
    mut user_data: gpointer,
) -> gboolean {
    let mut skeleton: *mut GXdpDocumentsSkeleton = user_data as *mut GXdpDocumentsSkeleton;
    let mut value: GValue = _GValue {
        g_type: 0 as GType,
        data: [
            C2RustUnnamed_1 {
                v_int: 0 as ::core::ffi::c_int,
            },
            C2RustUnnamed_1 { v_int: 0 },
        ],
    };
    let mut pspec: *mut GParamSpec = ::core::ptr::null_mut::<GParamSpec>();
    let mut info: *mut _ExtendedGDBusPropertyInfo =
        ::core::ptr::null_mut::<_ExtendedGDBusPropertyInfo>();
    let mut ret: gboolean = 0;
    ret = FALSE as gboolean;
    info = g_dbus_interface_info_lookup_property(
        &raw const safe_c2rust__gxdp_documents_interface_info.parent_struct
            as *mut GDBusInterfaceInfo,
        property_name,
    ) as *mut _ExtendedGDBusPropertyInfo;
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if !info.is_null() {
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
            b"gio/xdp-dbus.c\0" as *const u8 as *const ::core::ffi::c_char,
            4034 as ::core::ffi::c_int,
            G_STRFUNC,
            b"info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    pspec = g_object_class_find_property(
        (*(skeleton as *mut GTypeInstance)).g_class as *mut GObjectClass,
        (*info).hyphen_name,
    );
    if pspec.is_null() {
        g_set_error(
            error,
            g_dbus_error_quark(),
            G_DBUS_ERROR_INVALID_ARGS as ::core::ffi::c_int as gint,
            b"No property with name %s\0" as *const u8 as *const gchar,
            property_name,
        );
    } else {
        if (*info).use_gvariant() != 0 {
            g_value_set_variant(&raw mut value, variant);
        } else {
            g_dbus_gvariant_to_gvalue(variant, &raw mut value);
        }
        g_object_set_property(
            skeleton as *mut ::core::ffi::c_void as *mut GObject,
            (*info).hyphen_name,
            &raw mut value,
        );
        g_value_unset(&raw mut value);
        ret = TRUE as gboolean;
    }
    return ret;
}
static mut safe_c2rust__gxdp_documents_skeleton_vtable: GDBusInterfaceVTable = unsafe {
    _GDBusInterfaceVTable {
        method_call: Some(
            safe_c2rust__gxdp_documents_skeleton_handle_method_call
                as unsafe extern "C" fn(
                    *mut GDBusConnection,
                    *const gchar,
                    *const gchar,
                    *const gchar,
                    *const gchar,
                    *mut GVariant,
                    *mut GDBusMethodInvocation,
                    gpointer,
                ) -> (),
        ),
        get_property: Some(
            safe_c2rust__gxdp_documents_skeleton_handle_get_property
                as unsafe extern "C" fn(
                    *mut GDBusConnection,
                    *const gchar,
                    *const gchar,
                    *const gchar,
                    *const gchar,
                    *mut *mut GError,
                    gpointer,
                ) -> *mut GVariant,
        ),
        set_property: Some(
            safe_c2rust__gxdp_documents_skeleton_handle_set_property
                as unsafe extern "C" fn(
                    *mut GDBusConnection,
                    *const gchar,
                    *const gchar,
                    *const gchar,
                    *const gchar,
                    *mut GVariant,
                    *mut *mut GError,
                    gpointer,
                ) -> gboolean,
        ),
        padding: [
            NULL,
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        ],
    }
};
unsafe extern "C" fn safe_c2rust_gxdp_documents_skeleton_dbus_interface_get_info(
    mut skeleton: *mut GDBusInterfaceSkeleton,
) -> *mut GDBusInterfaceInfo {
    return safe_c2rust_gxdp_documents_interface_info();
}
unsafe extern "C" fn safe_c2rust_gxdp_documents_skeleton_dbus_interface_get_vtable(
    mut skeleton: *mut GDBusInterfaceSkeleton,
) -> *mut GDBusInterfaceVTable {
    return &raw const safe_c2rust__gxdp_documents_skeleton_vtable as *mut GDBusInterfaceVTable;
}
unsafe extern "C" fn safe_c2rust_gxdp_documents_skeleton_dbus_interface_get_properties(
    mut _skeleton: *mut GDBusInterfaceSkeleton,
) -> *mut GVariant {
    let mut skeleton: *mut GXdpDocumentsSkeleton =
        _skeleton as *mut ::core::ffi::c_void as *mut GXdpDocumentsSkeleton;
    let mut builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut n: guint = 0;
    g_variant_builder_init(
        &raw mut builder,
        g_variant_type_checked_(b"a{sv}\0" as *const u8 as *const gchar),
    );
    if !safe_c2rust__gxdp_documents_interface_info
        .parent_struct
        .properties
        .is_null()
    {
        n = 0 as guint;
        while !(*safe_c2rust__gxdp_documents_interface_info
            .parent_struct
            .properties
            .offset(n as isize))
        .is_null()
        {
            let mut info: *mut GDBusPropertyInfo = *safe_c2rust__gxdp_documents_interface_info
                .parent_struct
                .properties
                .offset(n as isize);
            if (*info).flags as ::core::ffi::c_uint
                & G_DBUS_PROPERTY_INFO_FLAGS_READABLE as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
            {
                let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
                value = safe_c2rust__gxdp_documents_skeleton_handle_get_property(
                    g_dbus_interface_skeleton_get_connection(
                        skeleton as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton,
                    ),
                    ::core::ptr::null::<gchar>(),
                    g_dbus_interface_skeleton_get_object_path(
                        skeleton as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton,
                    ),
                    b"org.freedesktop.portal.Documents\0" as *const u8 as *const gchar,
                    (*info).name,
                    ::core::ptr::null_mut::<*mut GError>(),
                    skeleton as gpointer,
                );
                if !value.is_null() {
                    g_variant_take_ref(value);
                    g_variant_builder_add(
                        &raw mut builder,
                        b"{sv}\0" as *const u8 as *const gchar,
                        (*info).name,
                        value,
                    );
                    g_variant_unref(value);
                }
            }
            n = n.wrapping_add(1);
        }
    }
    return g_variant_builder_end(&raw mut builder);
}
unsafe extern "C" fn safe_c2rust_gxdp_documents_skeleton_dbus_interface_flush(
    mut _skeleton: *mut GDBusInterfaceSkeleton,
) {
    let mut skeleton: *mut GXdpDocumentsSkeleton =
        _skeleton as *mut ::core::ffi::c_void as *mut GXdpDocumentsSkeleton;
    let mut emit_changed: gboolean = FALSE;
    g_mutex_lock(&raw mut (*(*skeleton).priv_0).lock);
    if !(*(*skeleton).priv_0)
        .changed_properties_idle_source
        .is_null()
    {
        g_source_destroy((*(*skeleton).priv_0).changed_properties_idle_source);
        (*(*skeleton).priv_0).changed_properties_idle_source = ::core::ptr::null_mut::<GSource>();
        emit_changed = TRUE as gboolean;
    }
    g_mutex_unlock(&raw mut (*(*skeleton).priv_0).lock);
    if emit_changed != 0 {
        safe_c2rust__gxdp_documents_emit_changed(skeleton as gpointer);
    }
}
#[inline]
unsafe extern "C" fn safe_c2rust_gxdp_documents_skeleton_get_instance_private(
    mut self_0: *mut GXdpDocumentsSkeleton,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GXdpDocumentsSkeleton_private_offset as glong as isize)
        as gpointer;
}
static mut safe_c2rust_GXdpDocumentsSkeleton_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_gxdp_documents_skeleton_class_intern_init(mut klass: gpointer) {
    safe_c2rust_gxdp_documents_skeleton_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GXdpDocumentsSkeleton_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GXdpDocumentsSkeleton_private_offset,
        );
    }
    safe_c2rust_gxdp_documents_skeleton_class_init(klass as *mut GXdpDocumentsSkeletonClass);
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_gxdp_documents_skeleton_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_dbus_interface_skeleton_get_type(),
        g_intern_static_string(b"GXdpDocumentsSkeleton\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GXdpDocumentsSkeletonClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_gxdp_documents_skeleton_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GXdpDocumentsSkeleton>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GXdpDocumentsSkeleton) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_gxdp_documents_skeleton_init
                    as unsafe extern "C" fn(*mut GXdpDocumentsSkeleton) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GXdpDocumentsSkeleton_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GXdpDocumentsSkeletonPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GXdpDocumentsIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_gxdp_documents_skeleton_iface_init
                as unsafe extern "C" fn(*mut GXdpDocumentsIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        safe_c2rust_gxdp_documents_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_skeleton_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_gxdp_documents_skeleton_get_type_once();
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
static mut safe_c2rust_gxdp_documents_skeleton_parent_class: gpointer = NULL;
unsafe extern "C" fn safe_c2rust_gxdp_documents_skeleton_finalize(mut object: *mut GObject) {
    let mut skeleton: *mut GXdpDocumentsSkeleton =
        object as *mut ::core::ffi::c_void as *mut GXdpDocumentsSkeleton;
    let mut n: guint = 0;
    n = 0 as guint;
    while n < 1 as guint {
        g_value_unset((*(*skeleton).priv_0).properties.offset(n as isize) as *mut GValue);
        n = n.wrapping_add(1);
    }
    g_free((*(*skeleton).priv_0).properties as gpointer);
    g_list_free_full(
        (*(*skeleton).priv_0).changed_properties,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut ChangedProperty) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust__changed_property_free as unsafe extern "C" fn(*mut ChangedProperty) -> (),
        )),
    );
    if !(*(*skeleton).priv_0)
        .changed_properties_idle_source
        .is_null()
    {
        g_source_destroy((*(*skeleton).priv_0).changed_properties_idle_source);
    }
    g_main_context_unref((*(*skeleton).priv_0).context);
    g_mutex_clear(&raw mut (*(*skeleton).priv_0).lock);
    (*(safe_c2rust_gxdp_documents_skeleton_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_gxdp_documents_skeleton_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut skeleton: *mut GXdpDocumentsSkeleton =
        object as *mut ::core::ffi::c_void as *mut GXdpDocumentsSkeleton;
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if prop_id != 0 as guint && prop_id.wrapping_sub(1 as guint) < 1 as guint {
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
            b"gio/xdp-dbus.c\0" as *const u8 as *const ::core::ffi::c_char,
            4157 as ::core::ffi::c_int,
            G_STRFUNC,
            b"prop_id != 0 && prop_id - 1 < 1\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_mutex_lock(&raw mut (*(*skeleton).priv_0).lock);
    g_value_copy(
        (*(*skeleton).priv_0)
            .properties
            .offset(prop_id.wrapping_sub(1 as guint) as isize) as *mut GValue,
        value,
    );
    g_mutex_unlock(&raw mut (*(*skeleton).priv_0).lock);
}
unsafe extern "C" fn safe_c2rust__gxdp_documents_emit_changed(mut user_data: gpointer) -> gboolean {
    let mut skeleton: *mut GXdpDocumentsSkeleton = user_data as *mut GXdpDocumentsSkeleton;
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut invalidated_builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut num_changes: guint = 0;
    g_mutex_lock(&raw mut (*(*skeleton).priv_0).lock);
    g_variant_builder_init(
        &raw mut builder,
        g_variant_type_checked_(b"a{sv}\0" as *const u8 as *const gchar),
    );
    g_variant_builder_init(
        &raw mut invalidated_builder,
        g_variant_type_checked_(b"as\0" as *const u8 as *const gchar),
    );
    l = (*(*skeleton).priv_0).changed_properties;
    num_changes = 0 as guint;
    while !l.is_null() {
        let mut cp: *mut ChangedProperty = (*l).data as *mut ChangedProperty;
        let mut variant: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        let mut cur_value: *const GValue = ::core::ptr::null::<GValue>();
        cur_value = (*(*skeleton).priv_0)
            .properties
            .offset((*cp).prop_id.wrapping_sub(1 as guint) as isize)
            as *mut GValue;
        if safe_c2rust__g_value_equal(cur_value, &raw mut (*cp).orig_value) == 0 {
            variant = g_dbus_gvalue_to_gvariant(
                cur_value,
                g_variant_type_checked_((*(*cp).info).parent_struct.signature),
            );
            g_variant_builder_add(
                &raw mut builder,
                b"{sv}\0" as *const u8 as *const gchar,
                (*(*cp).info).parent_struct.name,
                variant,
            );
            g_variant_unref(variant);
            num_changes = num_changes.wrapping_add(1);
        }
        l = (*l).next;
    }
    if num_changes > 0 as guint {
        let mut connections: *mut GList = ::core::ptr::null_mut::<GList>();
        let mut ll: *mut GList = ::core::ptr::null_mut::<GList>();
        let mut signal_variant: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        signal_variant = g_variant_ref_sink(g_variant_new(
            b"(sa{sv}as)\0" as *const u8 as *const gchar,
            b"org.freedesktop.portal.Documents\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut builder,
            &raw mut invalidated_builder,
        ));
        connections = g_dbus_interface_skeleton_get_connections(
            skeleton as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton,
        );
        ll = connections;
        while !ll.is_null() {
            let mut connection: *mut GDBusConnection = (*ll).data as *mut GDBusConnection;
            g_dbus_connection_emit_signal(
                connection,
                ::core::ptr::null::<gchar>(),
                g_dbus_interface_skeleton_get_object_path(
                    skeleton as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton,
                ),
                b"org.freedesktop.DBus.Properties\0" as *const u8 as *const gchar,
                b"PropertiesChanged\0" as *const u8 as *const gchar,
                signal_variant,
                ::core::ptr::null_mut::<*mut GError>(),
            );
            ll = (*ll).next;
        }
        g_variant_unref(signal_variant);
        g_list_free_full(
            connections,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    } else {
        g_variant_builder_clear(&raw mut builder);
        g_variant_builder_clear(&raw mut invalidated_builder);
    }
    g_list_free_full(
        (*(*skeleton).priv_0).changed_properties,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut ChangedProperty) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust__changed_property_free as unsafe extern "C" fn(*mut ChangedProperty) -> (),
        )),
    );
    (*(*skeleton).priv_0).changed_properties = ::core::ptr::null_mut::<GList>();
    (*(*skeleton).priv_0).changed_properties_idle_source = ::core::ptr::null_mut::<GSource>();
    g_mutex_unlock(&raw mut (*(*skeleton).priv_0).lock);
    return FALSE;
}
unsafe extern "C" fn safe_c2rust__gxdp_documents_schedule_emit_changed(
    mut skeleton: *mut GXdpDocumentsSkeleton,
    mut info: *const _ExtendedGDBusPropertyInfo,
    mut prop_id: guint,
    mut orig_value: *const GValue,
) {
    let mut cp: *mut ChangedProperty = ::core::ptr::null_mut::<ChangedProperty>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    cp = ::core::ptr::null_mut::<ChangedProperty>();
    l = (*(*skeleton).priv_0).changed_properties;
    while !l.is_null() {
        let mut i_cp: *mut ChangedProperty = (*l).data as *mut ChangedProperty;
        if (*i_cp).info == info {
            cp = i_cp;
            break;
        } else {
            l = (*l).next;
        }
    }
    if cp.is_null() {
        cp = ({
            let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
            let mut __s: gsize = ::core::mem::size_of::<ChangedProperty>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc0(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc0(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc0_n(__n, __s);
            }
            __p
        }) as *mut ChangedProperty;
        (*cp).prop_id = prop_id;
        (*cp).info = info;
        (*(*skeleton).priv_0).changed_properties =
            g_list_prepend((*(*skeleton).priv_0).changed_properties, cp as gpointer);
        g_value_init(
            &raw mut (*cp).orig_value,
            (*(orig_value as *mut GValue)).g_type,
        );
        g_value_copy(orig_value, &raw mut (*cp).orig_value);
    }
}
unsafe extern "C" fn safe_c2rust_gxdp_documents_skeleton_notify(
    mut object: *mut GObject,
    mut pspec: *mut GParamSpec,
) {
    let mut skeleton: *mut GXdpDocumentsSkeleton =
        object as *mut ::core::ffi::c_void as *mut GXdpDocumentsSkeleton;
    g_mutex_lock(&raw mut (*(*skeleton).priv_0).lock);
    if !(*(*skeleton).priv_0).changed_properties.is_null()
        && (*(*skeleton).priv_0)
            .changed_properties_idle_source
            .is_null()
    {
        (*(*skeleton).priv_0).changed_properties_idle_source = g_idle_source_new();
        g_source_set_priority(
            (*(*skeleton).priv_0).changed_properties_idle_source,
            G_PRIORITY_DEFAULT,
        );
        g_source_set_callback(
            (*(*skeleton).priv_0).changed_properties_idle_source,
            Some(
                safe_c2rust__gxdp_documents_emit_changed
                    as unsafe extern "C" fn(gpointer) -> gboolean,
            ),
            g_object_ref(skeleton as gpointer) as *mut GXdpDocumentsSkeleton as gpointer,
            ::core::mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> ()>, GDestroyNotify>(
                Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
            ),
        );
        g_source_set_name(
            (*(*skeleton).priv_0).changed_properties_idle_source,
            b"[generated] _gxdp_documents_emit_changed\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        g_source_attach(
            (*(*skeleton).priv_0).changed_properties_idle_source,
            (*(*skeleton).priv_0).context,
        );
        g_source_unref((*(*skeleton).priv_0).changed_properties_idle_source);
    }
    g_mutex_unlock(&raw mut (*(*skeleton).priv_0).lock);
}
unsafe extern "C" fn safe_c2rust_gxdp_documents_skeleton_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut info: *const _ExtendedGDBusPropertyInfo =
        ::core::ptr::null::<_ExtendedGDBusPropertyInfo>();
    let mut skeleton: *mut GXdpDocumentsSkeleton =
        object as *mut ::core::ffi::c_void as *mut GXdpDocumentsSkeleton;
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if prop_id != 0 as guint && prop_id.wrapping_sub(1 as guint) < 1 as guint {
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
            b"gio/xdp-dbus.c\0" as *const u8 as *const ::core::ffi::c_char,
            4276 as ::core::ffi::c_int,
            G_STRFUNC,
            b"prop_id != 0 && prop_id - 1 < 1\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    info = safe_c2rust__gxdp_documents_property_info_pointers
        [prop_id.wrapping_sub(1 as guint) as usize] as *const _ExtendedGDBusPropertyInfo;
    g_mutex_lock(&raw mut (*(*skeleton).priv_0).lock);
    g_object_freeze_notify(object);
    if safe_c2rust__g_value_equal(
        value,
        (*(*skeleton).priv_0)
            .properties
            .offset(prop_id.wrapping_sub(1 as guint) as isize) as *mut GValue,
    ) == 0
    {
        if !g_dbus_interface_skeleton_get_connection(
            skeleton as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton,
        )
        .is_null()
            && (*info).emits_changed_signal() as ::core::ffi::c_int != 0
        {
            safe_c2rust__gxdp_documents_schedule_emit_changed(
                skeleton,
                info,
                prop_id,
                (*(*skeleton).priv_0)
                    .properties
                    .offset(prop_id.wrapping_sub(1 as guint) as isize)
                    as *mut GValue,
            );
        }
        g_value_copy(
            value,
            (*(*skeleton).priv_0)
                .properties
                .offset(prop_id.wrapping_sub(1 as guint) as isize) as *mut GValue,
        );
        g_object_notify_by_pspec(object, pspec);
    }
    g_mutex_unlock(&raw mut (*(*skeleton).priv_0).lock);
    g_object_thaw_notify(object);
}
unsafe extern "C" fn safe_c2rust_gxdp_documents_skeleton_init(
    mut skeleton: *mut GXdpDocumentsSkeleton,
) {
    (*skeleton).priv_0 = safe_c2rust_gxdp_documents_skeleton_get_instance_private(skeleton)
        as *mut GXdpDocumentsSkeletonPrivate;
    g_mutex_init(&raw mut (*(*skeleton).priv_0).lock);
    (*(*skeleton).priv_0).context = g_main_context_ref_thread_default();
    (*(*skeleton).priv_0).properties = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GValue>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GValue;
    g_value_init(
        (*(*skeleton).priv_0)
            .properties
            .offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        G_TYPE_UINT,
    );
}
unsafe extern "C" fn safe_c2rust_gxdp_documents_skeleton_get_version(
    mut object: *mut GXdpDocuments,
) -> guint {
    let mut skeleton: *mut GXdpDocumentsSkeleton =
        object as *mut ::core::ffi::c_void as *mut GXdpDocumentsSkeleton;
    let mut value: guint = 0;
    g_mutex_lock(&raw mut (*(*skeleton).priv_0).lock);
    value = (*(*(*skeleton).priv_0)
        .properties
        .offset(0 as ::core::ffi::c_int as isize))
    .data[0 as ::core::ffi::c_int as usize]
        .v_uint;
    g_mutex_unlock(&raw mut (*(*skeleton).priv_0).lock);
    return value;
}
unsafe extern "C" fn safe_c2rust_gxdp_documents_skeleton_class_init(
    mut klass: *mut GXdpDocumentsSkeletonClass,
) {
    let mut gobject_class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    let mut skeleton_class: *mut GDBusInterfaceSkeletonClass =
        ::core::ptr::null_mut::<GDBusInterfaceSkeletonClass>();
    gobject_class = klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize = Some(
        safe_c2rust_gxdp_documents_skeleton_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_gxdp_documents_skeleton_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_gxdp_documents_skeleton_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).notify = Some(
        safe_c2rust_gxdp_documents_skeleton_notify
            as unsafe extern "C" fn(*mut GObject, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, *mut GParamSpec) -> ()>;
    safe_c2rust_gxdp_documents_override_properties(gobject_class, 1 as guint);
    skeleton_class = klass as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeletonClass;
    (*skeleton_class).get_info = Some(
        safe_c2rust_gxdp_documents_skeleton_dbus_interface_get_info
            as unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GDBusInterfaceInfo,
    )
        as Option<unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GDBusInterfaceInfo>;
    (*skeleton_class).get_properties = Some(
        safe_c2rust_gxdp_documents_skeleton_dbus_interface_get_properties
            as unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GVariant,
    )
        as Option<unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GVariant>;
    (*skeleton_class).flush = Some(
        safe_c2rust_gxdp_documents_skeleton_dbus_interface_flush
            as unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> ()>;
    (*skeleton_class).get_vtable = Some(
        safe_c2rust_gxdp_documents_skeleton_dbus_interface_get_vtable
            as unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GDBusInterfaceVTable,
    )
        as Option<unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GDBusInterfaceVTable>;
}
unsafe extern "C" fn safe_c2rust_gxdp_documents_skeleton_iface_init(
    mut iface: *mut GXdpDocumentsIface,
) {
    (*iface).get_version = Some(
        safe_c2rust_gxdp_documents_skeleton_get_version
            as unsafe extern "C" fn(*mut GXdpDocuments) -> guint,
    ) as Option<unsafe extern "C" fn(*mut GXdpDocuments) -> guint>;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_documents_skeleton_new() -> *mut GXdpDocuments {
    return g_object_new(
        safe_c2rust_gxdp_documents_skeleton_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GXdpDocuments;
}
static mut safe_c2rust__gxdp_open_uri_method_info_open_uri_IN_ARG_parent_window:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"parent_window\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust__gxdp_open_uri_method_info_open_uri_IN_ARG_uri: _ExtendedGDBusArgInfo =
    _ExtendedGDBusArgInfo {
        parent_struct: _GDBusArgInfo {
            ref_count: -(1 as gint),
            name: b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        use_gvariant: FALSE,
    };
static mut safe_c2rust__gxdp_open_uri_method_info_open_uri_IN_ARG_options: _ExtendedGDBusArgInfo =
    _ExtendedGDBusArgInfo {
        parent_struct: _GDBusArgInfo {
            ref_count: -(1 as gint),
            name: b"options\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            signature: b"a{sv}\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        use_gvariant: FALSE,
    };
static mut safe_c2rust__gxdp_open_uri_method_info_open_uri_IN_ARG_pointers: [*const GDBusArgInfo;
    4] = [::core::ptr::null::<GDBusArgInfo>(); 4];
static mut safe_c2rust__gxdp_open_uri_method_info_open_uri_OUT_ARG_handle: _ExtendedGDBusArgInfo =
    _ExtendedGDBusArgInfo {
        parent_struct: _GDBusArgInfo {
            ref_count: -(1 as gint),
            name: b"handle\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            signature: b"o\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        use_gvariant: FALSE,
    };
static mut safe_c2rust__gxdp_open_uri_method_info_open_uri_OUT_ARG_pointers: [*const GDBusArgInfo;
    2] = [::core::ptr::null::<GDBusArgInfo>(); 2];
static mut safe_c2rust__gxdp_open_uri_method_info_open_uri: _ExtendedGDBusMethodInfo = unsafe {
    _ExtendedGDBusMethodInfo {
        parent_struct: _GDBusMethodInfo {
            ref_count: -(1 as gint),
            name: b"OpenURI\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            in_args: &raw const safe_c2rust__gxdp_open_uri_method_info_open_uri_IN_ARG_pointers
                as *mut *mut GDBusArgInfo,
            out_args: &raw const safe_c2rust__gxdp_open_uri_method_info_open_uri_OUT_ARG_pointers
                as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"handle-open-uri\0" as *const u8 as *const gchar,
        pass_fdlist: FALSE,
    }
};
static mut safe_c2rust__gxdp_open_uri_method_info_open_file_IN_ARG_parent_window:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"parent_window\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust__gxdp_open_uri_method_info_open_file_IN_ARG_fd: _ExtendedGDBusArgInfo =
    _ExtendedGDBusArgInfo {
        parent_struct: _GDBusArgInfo {
            ref_count: -(1 as gint),
            name: b"fd\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            signature: b"h\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        use_gvariant: FALSE,
    };
static mut safe_c2rust__gxdp_open_uri_method_info_open_file_IN_ARG_options: _ExtendedGDBusArgInfo =
    _ExtendedGDBusArgInfo {
        parent_struct: _GDBusArgInfo {
            ref_count: -(1 as gint),
            name: b"options\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            signature: b"a{sv}\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        use_gvariant: FALSE,
    };
static mut safe_c2rust__gxdp_open_uri_method_info_open_file_IN_ARG_pointers: [*const GDBusArgInfo;
    4] = [::core::ptr::null::<GDBusArgInfo>(); 4];
static mut safe_c2rust__gxdp_open_uri_method_info_open_file_OUT_ARG_handle: _ExtendedGDBusArgInfo =
    _ExtendedGDBusArgInfo {
        parent_struct: _GDBusArgInfo {
            ref_count: -(1 as gint),
            name: b"handle\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            signature: b"o\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        use_gvariant: FALSE,
    };
static mut safe_c2rust__gxdp_open_uri_method_info_open_file_OUT_ARG_pointers:
    [*const GDBusArgInfo; 2] = [::core::ptr::null::<GDBusArgInfo>(); 2];
static mut safe_c2rust__gxdp_open_uri_method_info_open_file: _ExtendedGDBusMethodInfo = unsafe {
    _ExtendedGDBusMethodInfo {
        parent_struct: _GDBusMethodInfo {
            ref_count: -(1 as gint),
            name: b"OpenFile\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            in_args: &raw const safe_c2rust__gxdp_open_uri_method_info_open_file_IN_ARG_pointers
                as *mut *mut GDBusArgInfo,
            out_args: &raw const safe_c2rust__gxdp_open_uri_method_info_open_file_OUT_ARG_pointers
                as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"handle-open-file\0" as *const u8 as *const gchar,
        pass_fdlist: TRUE,
    }
};
static mut safe_c2rust__gxdp_open_uri_method_info_open_directory_IN_ARG_parent_window:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"parent_window\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust__gxdp_open_uri_method_info_open_directory_IN_ARG_fd: _ExtendedGDBusArgInfo =
    _ExtendedGDBusArgInfo {
        parent_struct: _GDBusArgInfo {
            ref_count: -(1 as gint),
            name: b"fd\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            signature: b"h\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        use_gvariant: FALSE,
    };
static mut safe_c2rust__gxdp_open_uri_method_info_open_directory_IN_ARG_options:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"options\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"a{sv}\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust__gxdp_open_uri_method_info_open_directory_IN_ARG_pointers:
    [*const GDBusArgInfo; 4] = [::core::ptr::null::<GDBusArgInfo>(); 4];
static mut safe_c2rust__gxdp_open_uri_method_info_open_directory_OUT_ARG_handle:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"handle\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"o\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust__gxdp_open_uri_method_info_open_directory_OUT_ARG_pointers:
    [*const GDBusArgInfo; 2] = [::core::ptr::null::<GDBusArgInfo>(); 2];
static mut safe_c2rust__gxdp_open_uri_method_info_open_directory: _ExtendedGDBusMethodInfo = unsafe {
    _ExtendedGDBusMethodInfo {
        parent_struct: _GDBusMethodInfo {
            ref_count: -(1 as gint),
            name: b"OpenDirectory\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            in_args:
                &raw const safe_c2rust__gxdp_open_uri_method_info_open_directory_IN_ARG_pointers
                    as *mut *mut GDBusArgInfo,
            out_args:
                &raw const safe_c2rust__gxdp_open_uri_method_info_open_directory_OUT_ARG_pointers
                    as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"handle-open-directory\0" as *const u8 as *const gchar,
        pass_fdlist: TRUE,
    }
};
static mut safe_c2rust__gxdp_open_uri_method_info_pointers: [*const GDBusMethodInfo; 4] =
    [::core::ptr::null::<GDBusMethodInfo>(); 4];
static mut safe_c2rust__gxdp_open_uri_property_info_version: _ExtendedGDBusPropertyInfo =
    _ExtendedGDBusPropertyInfo {
        parent_struct: _GDBusPropertyInfo {
            ref_count: 0,
            name: ::core::ptr::null::<gchar>() as *mut gchar,
            signature: ::core::ptr::null::<gchar>() as *mut gchar,
            flags: G_DBUS_PROPERTY_INFO_FLAGS_NONE,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        hyphen_name: ::core::ptr::null::<gchar>(),
        use_gvariant_emits_changed_signal: [0; 1],
        c2rust_padding: [0; 7],
    };
static mut safe_c2rust__gxdp_open_uri_property_info_pointers: [*const GDBusPropertyInfo; 2] =
    [::core::ptr::null::<GDBusPropertyInfo>(); 2];
static mut safe_c2rust__gxdp_open_uri_interface_info: _ExtendedGDBusInterfaceInfo = unsafe {
    _ExtendedGDBusInterfaceInfo {
        parent_struct: _GDBusInterfaceInfo {
            ref_count: -(1 as gint),
            name: b"org.freedesktop.portal.OpenURI\0" as *const u8 as *const ::core::ffi::c_char
                as *mut gchar,
            methods: &raw const safe_c2rust__gxdp_open_uri_method_info_pointers
                as *mut *mut GDBusMethodInfo,
            signals: ::core::ptr::null::<*mut GDBusSignalInfo>() as *mut *mut GDBusSignalInfo,
            properties: &raw const safe_c2rust__gxdp_open_uri_property_info_pointers
                as *mut *mut GDBusPropertyInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        hyphen_name: b"open-uri\0" as *const u8 as *const gchar,
    }
};
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_open_uri_interface_info() -> *mut GDBusInterfaceInfo {
    return &raw const safe_c2rust__gxdp_open_uri_interface_info.parent_struct
        as *mut GDBusInterfaceInfo;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_open_uri_override_properties(
    mut klass: *mut GObjectClass,
    mut property_id_begin: guint,
) -> guint {
    let fresh6 = property_id_begin;
    property_id_begin = property_id_begin.wrapping_add(1);
    g_object_class_override_property(klass, fresh6, b"version\0" as *const u8 as *const gchar);
    return property_id_begin.wrapping_sub(1 as guint);
}
#[inline]
unsafe extern "C" fn safe_c2rust_gxdp_open_uri_method_marshal_open_uri(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: ::core::ffi::c_uint,
    mut param_values: *const GValue,
    mut invocation_hint: *mut ::core::ffi::c_void,
    mut marshal_data: *mut ::core::ffi::c_void,
) {
    safe_c2rust__g_dbus_codegen_marshal_BOOLEAN__OBJECT_STRING_STRING_VARIANT(
        closure,
        return_value,
        n_param_values,
        param_values,
        invocation_hint,
        marshal_data,
    );
}
#[inline]
unsafe extern "C" fn safe_c2rust_gxdp_open_uri_method_marshal_open_file(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: ::core::ffi::c_uint,
    mut param_values: *const GValue,
    mut invocation_hint: *mut ::core::ffi::c_void,
    mut marshal_data: *mut ::core::ffi::c_void,
) {
    safe_c2rust__g_dbus_codegen_marshal_BOOLEAN__OBJECT_OBJECT_STRING_VARIANT_VARIANT(
        closure,
        return_value,
        n_param_values,
        param_values,
        invocation_hint,
        marshal_data,
    );
}
#[inline]
unsafe extern "C" fn safe_c2rust_gxdp_open_uri_method_marshal_open_directory(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: ::core::ffi::c_uint,
    mut param_values: *const GValue,
    mut invocation_hint: *mut ::core::ffi::c_void,
    mut marshal_data: *mut ::core::ffi::c_void,
) {
    safe_c2rust__g_dbus_codegen_marshal_BOOLEAN__OBJECT_OBJECT_STRING_VARIANT_VARIANT(
        closure,
        return_value,
        n_param_values,
        param_values,
        invocation_hint,
        marshal_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_open_uri_get_type() -> GType {
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
        let mut g_define_type_id: GType = g_type_register_static_simple(
            G_TYPE_INTERFACE,
            g_intern_static_string(b"GXdpOpenURI\0" as *const u8 as *const gchar),
            ::core::mem::size_of::<GXdpOpenURIInterface>() as guint,
            ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GXdpOpenURIIface) -> ()>,
                    Option<unsafe extern "C" fn() -> ()>,
                >(Some(
                    safe_c2rust_gxdp_open_uri_default_init
                        as unsafe extern "C" fn(*mut GXdpOpenURIIface) -> (),
                )),
            ),
            0 as guint,
            ::core::mem::transmute::<*mut ::core::ffi::c_void, GInstanceInitFunc>(NULL),
            G_TYPE_FLAG_NONE,
        );
        if ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType != G_TYPE_INVALID {
            g_type_interface_add_prerequisite(
                g_define_type_id,
                ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            );
        }
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
unsafe extern "C" fn safe_c2rust_gxdp_open_uri_default_init(mut iface: *mut GXdpOpenURIIface) {
    g_signal_new(
        b"handle-open-uri\0" as *const u8 as *const gchar,
        (*(iface as *mut GTypeInterface)).g_type,
        G_SIGNAL_RUN_LAST,
        32 as ::core::ffi::c_ulong as glong as guint,
        Some(
            g_signal_accumulator_true_handled
                as unsafe extern "C" fn(
                    *mut GSignalInvocationHint,
                    *mut GValue,
                    *const GValue,
                    gpointer,
                ) -> gboolean,
        ),
        NULL,
        Some(
            safe_c2rust_gxdp_open_uri_method_marshal_open_uri
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    ::core::ffi::c_uint,
                    *const GValue,
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        G_TYPE_BOOLEAN,
        4 as guint,
        g_dbus_method_invocation_get_type(),
        G_TYPE_STRING,
        G_TYPE_STRING,
        G_TYPE_VARIANT,
    );
    g_signal_new(
        b"handle-open-file\0" as *const u8 as *const gchar,
        (*(iface as *mut GTypeInterface)).g_type,
        G_SIGNAL_RUN_LAST,
        24 as ::core::ffi::c_ulong as glong as guint,
        Some(
            g_signal_accumulator_true_handled
                as unsafe extern "C" fn(
                    *mut GSignalInvocationHint,
                    *mut GValue,
                    *const GValue,
                    gpointer,
                ) -> gboolean,
        ),
        NULL,
        Some(
            safe_c2rust_gxdp_open_uri_method_marshal_open_file
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    ::core::ffi::c_uint,
                    *const GValue,
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        G_TYPE_BOOLEAN,
        5 as guint,
        g_dbus_method_invocation_get_type(),
        g_unix_fd_list_get_type(),
        G_TYPE_STRING,
        G_TYPE_VARIANT,
        G_TYPE_VARIANT,
    );
    g_signal_new(
        b"handle-open-directory\0" as *const u8 as *const gchar,
        (*(iface as *mut GTypeInterface)).g_type,
        G_SIGNAL_RUN_LAST,
        16 as ::core::ffi::c_ulong as glong as guint,
        Some(
            g_signal_accumulator_true_handled
                as unsafe extern "C" fn(
                    *mut GSignalInvocationHint,
                    *mut GValue,
                    *const GValue,
                    gpointer,
                ) -> gboolean,
        ),
        NULL,
        Some(
            safe_c2rust_gxdp_open_uri_method_marshal_open_directory
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    ::core::ffi::c_uint,
                    *const GValue,
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        G_TYPE_BOOLEAN,
        5 as guint,
        g_dbus_method_invocation_get_type(),
        g_unix_fd_list_get_type(),
        G_TYPE_STRING,
        G_TYPE_VARIANT,
        G_TYPE_VARIANT,
    );
    g_object_interface_install_property(
        iface as gpointer,
        g_param_spec_uint(
            b"version\0" as *const u8 as *const gchar,
            b"version\0" as *const u8 as *const gchar,
            b"version\0" as *const u8 as *const gchar,
            0 as guint,
            G_MAXUINT32,
            0 as guint,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_open_uri_get_version(
    mut object: *mut GXdpOpenURI,
) -> guint {
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = object as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_gxdp_open_uri_get_type();
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
            b"GXDP_IS_OPEN_URI (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    return (*(g_type_interface_peek(
        (*(object as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_gxdp_open_uri_get_type(),
    ) as *mut GXdpOpenURIIface))
        .get_version
        .expect("non-null function pointer")(object);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_open_uri_set_version(
    mut object: *mut GXdpOpenURI,
    mut value: guint,
) {
    g_object_set(
        object as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"version\0" as *const u8 as *const gchar,
        value,
        NULL,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_open_uri_call_open_uri(
    mut proxy: *mut GXdpOpenURI,
    mut arg_parent_window: *const gchar,
    mut arg_uri: *const gchar,
    mut arg_options: *mut GVariant,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_dbus_proxy_call(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"OpenURI\0" as *const u8 as *const gchar,
        g_variant_new(
            b"(ss@a{sv})\0" as *const u8 as *const gchar,
            arg_parent_window,
            arg_uri,
            arg_options,
        ),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_open_uri_call_open_uri_finish(
    mut proxy: *mut GXdpOpenURI,
    mut out_handle: *mut *mut gchar,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_finish(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        res,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(_ret, b"(o)\0" as *const u8 as *const gchar, out_handle);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_open_uri_call_open_uri_sync(
    mut proxy: *mut GXdpOpenURI,
    mut arg_parent_window: *const gchar,
    mut arg_uri: *const gchar,
    mut arg_options: *mut GVariant,
    mut out_handle: *mut *mut gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_sync(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"OpenURI\0" as *const u8 as *const gchar,
        g_variant_new(
            b"(ss@a{sv})\0" as *const u8 as *const gchar,
            arg_parent_window,
            arg_uri,
            arg_options,
        ),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(_ret, b"(o)\0" as *const u8 as *const gchar, out_handle);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_open_uri_call_open_file(
    mut proxy: *mut GXdpOpenURI,
    mut arg_parent_window: *const gchar,
    mut arg_fd: *mut GVariant,
    mut arg_options: *mut GVariant,
    mut fd_list: *mut GUnixFDList,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_dbus_proxy_call_with_unix_fd_list(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"OpenFile\0" as *const u8 as *const gchar,
        g_variant_new(
            b"(s@h@a{sv})\0" as *const u8 as *const gchar,
            arg_parent_window,
            arg_fd,
            arg_options,
        ),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        fd_list,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_open_uri_call_open_file_finish(
    mut proxy: *mut GXdpOpenURI,
    mut out_handle: *mut *mut gchar,
    mut out_fd_list: *mut *mut GUnixFDList,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_with_unix_fd_list_finish(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        out_fd_list,
        res,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(_ret, b"(o)\0" as *const u8 as *const gchar, out_handle);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_open_uri_call_open_file_sync(
    mut proxy: *mut GXdpOpenURI,
    mut arg_parent_window: *const gchar,
    mut arg_fd: *mut GVariant,
    mut arg_options: *mut GVariant,
    mut fd_list: *mut GUnixFDList,
    mut out_handle: *mut *mut gchar,
    mut out_fd_list: *mut *mut GUnixFDList,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_with_unix_fd_list_sync(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"OpenFile\0" as *const u8 as *const gchar,
        g_variant_new(
            b"(s@h@a{sv})\0" as *const u8 as *const gchar,
            arg_parent_window,
            arg_fd,
            arg_options,
        ),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        fd_list,
        out_fd_list,
        cancellable,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(_ret, b"(o)\0" as *const u8 as *const gchar, out_handle);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_open_uri_call_open_directory(
    mut proxy: *mut GXdpOpenURI,
    mut arg_parent_window: *const gchar,
    mut arg_fd: *mut GVariant,
    mut arg_options: *mut GVariant,
    mut fd_list: *mut GUnixFDList,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_dbus_proxy_call_with_unix_fd_list(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"OpenDirectory\0" as *const u8 as *const gchar,
        g_variant_new(
            b"(s@h@a{sv})\0" as *const u8 as *const gchar,
            arg_parent_window,
            arg_fd,
            arg_options,
        ),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        fd_list,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_open_uri_call_open_directory_finish(
    mut proxy: *mut GXdpOpenURI,
    mut out_handle: *mut *mut gchar,
    mut out_fd_list: *mut *mut GUnixFDList,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_with_unix_fd_list_finish(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        out_fd_list,
        res,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(_ret, b"(o)\0" as *const u8 as *const gchar, out_handle);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_open_uri_call_open_directory_sync(
    mut proxy: *mut GXdpOpenURI,
    mut arg_parent_window: *const gchar,
    mut arg_fd: *mut GVariant,
    mut arg_options: *mut GVariant,
    mut fd_list: *mut GUnixFDList,
    mut out_handle: *mut *mut gchar,
    mut out_fd_list: *mut *mut GUnixFDList,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_with_unix_fd_list_sync(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"OpenDirectory\0" as *const u8 as *const gchar,
        g_variant_new(
            b"(s@h@a{sv})\0" as *const u8 as *const gchar,
            arg_parent_window,
            arg_fd,
            arg_options,
        ),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        fd_list,
        out_fd_list,
        cancellable,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(_ret, b"(o)\0" as *const u8 as *const gchar, out_handle);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_open_uri_complete_open_uri(
    mut object: *mut GXdpOpenURI,
    mut invocation: *mut GDBusMethodInvocation,
    mut handle: *const gchar,
) {
    g_dbus_method_invocation_return_value(
        invocation,
        g_variant_new(b"(o)\0" as *const u8 as *const gchar, handle),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_open_uri_complete_open_file(
    mut object: *mut GXdpOpenURI,
    mut invocation: *mut GDBusMethodInvocation,
    mut fd_list: *mut GUnixFDList,
    mut handle: *const gchar,
) {
    g_dbus_method_invocation_return_value_with_unix_fd_list(
        invocation,
        g_variant_new(b"(o)\0" as *const u8 as *const gchar, handle),
        fd_list,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_open_uri_complete_open_directory(
    mut object: *mut GXdpOpenURI,
    mut invocation: *mut GDBusMethodInvocation,
    mut fd_list: *mut GUnixFDList,
    mut handle: *const gchar,
) {
    g_dbus_method_invocation_return_value_with_unix_fd_list(
        invocation,
        g_variant_new(b"(o)\0" as *const u8 as *const gchar, handle),
        fd_list,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_open_uri_proxy_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_gxdp_open_uri_proxy_get_type_once();
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
unsafe extern "C" fn safe_c2rust_gxdp_open_uri_proxy_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_dbus_proxy_get_type(),
        g_intern_static_string(b"GXdpOpenURIProxy\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GXdpOpenURIProxyClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_gxdp_open_uri_proxy_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GXdpOpenURIProxy>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GXdpOpenURIProxy) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_gxdp_open_uri_proxy_init
                    as unsafe extern "C" fn(*mut GXdpOpenURIProxy) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GXdpOpenURIProxy_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GXdpOpenURIProxyPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GXdpOpenURIIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_gxdp_open_uri_proxy_iface_init
                as unsafe extern "C" fn(*mut GXdpOpenURIIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        safe_c2rust_gxdp_open_uri_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
#[inline]
unsafe extern "C" fn safe_c2rust_gxdp_open_uri_proxy_get_instance_private(
    mut self_0: *mut GXdpOpenURIProxy,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GXdpOpenURIProxy_private_offset as glong as isize)
        as gpointer;
}
static mut safe_c2rust_GXdpOpenURIProxy_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_gxdp_open_uri_proxy_class_intern_init(mut klass: gpointer) {
    safe_c2rust_gxdp_open_uri_proxy_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GXdpOpenURIProxy_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GXdpOpenURIProxy_private_offset,
        );
    }
    safe_c2rust_gxdp_open_uri_proxy_class_init(klass as *mut GXdpOpenURIProxyClass);
}
static mut safe_c2rust_gxdp_open_uri_proxy_parent_class: gpointer = NULL;
unsafe extern "C" fn safe_c2rust_gxdp_open_uri_proxy_finalize(mut object: *mut GObject) {
    let mut proxy: *mut GXdpOpenURIProxy =
        object as *mut ::core::ffi::c_void as *mut GXdpOpenURIProxy;
    g_datalist_clear(&raw mut (*(*proxy).priv_0).qdata);
    (*(safe_c2rust_gxdp_open_uri_proxy_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_gxdp_open_uri_proxy_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut info: *const _ExtendedGDBusPropertyInfo =
        ::core::ptr::null::<_ExtendedGDBusPropertyInfo>();
    let mut variant: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if prop_id != 0 as guint && prop_id.wrapping_sub(1 as guint) < 1 as guint {
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
            b"gio/xdp-dbus.c\0" as *const u8 as *const ::core::ffi::c_char,
            5337 as ::core::ffi::c_int,
            G_STRFUNC,
            b"prop_id != 0 && prop_id - 1 < 1\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    info = safe_c2rust__gxdp_open_uri_property_info_pointers
        [prop_id.wrapping_sub(1 as guint) as usize] as *const _ExtendedGDBusPropertyInfo;
    variant = g_dbus_proxy_get_cached_property(
        object as *mut ::core::ffi::c_void as *mut GDBusProxy,
        (*info).parent_struct.name,
    );
    if (*info).use_gvariant() != 0 {
        g_value_set_variant(value, variant);
    } else if !variant.is_null() {
        g_dbus_gvariant_to_gvalue(variant, value);
    }
    if !variant.is_null() {
        g_variant_unref(variant);
    }
}
unsafe extern "C" fn safe_c2rust_gxdp_open_uri_proxy_set_property_cb(
    mut proxy: *mut GDBusProxy,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut info: *const _ExtendedGDBusPropertyInfo =
        user_data as *const _ExtendedGDBusPropertyInfo;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    error = ::core::ptr::null_mut::<GError>();
    _ret = g_dbus_proxy_call_finish(proxy, res, &raw mut error);
    if _ret.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Error setting property '%s' on interface org.freedesktop.portal.OpenURI: %s (%s, %d)\0"
                as *const u8 as *const gchar,
            (*info).parent_struct.name,
            (*error).message,
            g_quark_to_string((*error).domain),
            (*error).code,
        );
        g_error_free(error);
    } else {
        g_variant_unref(_ret);
    };
}
unsafe extern "C" fn safe_c2rust_gxdp_open_uri_proxy_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut info: *const _ExtendedGDBusPropertyInfo =
        ::core::ptr::null::<_ExtendedGDBusPropertyInfo>();
    let mut variant: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if prop_id != 0 as guint && prop_id.wrapping_sub(1 as guint) < 1 as guint {
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
            b"gio/xdp-dbus.c\0" as *const u8 as *const ::core::ffi::c_char,
            5384 as ::core::ffi::c_int,
            G_STRFUNC,
            b"prop_id != 0 && prop_id - 1 < 1\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    info = safe_c2rust__gxdp_open_uri_property_info_pointers
        [prop_id.wrapping_sub(1 as guint) as usize] as *const _ExtendedGDBusPropertyInfo;
    variant = g_dbus_gvalue_to_gvariant(
        value,
        g_variant_type_checked_((*info).parent_struct.signature),
    );
    g_dbus_proxy_call(
        object as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"org.freedesktop.DBus.Properties.Set\0" as *const u8 as *const gchar,
        g_variant_new(
            b"(ssv)\0" as *const u8 as *const gchar,
            b"org.freedesktop.portal.OpenURI\0" as *const u8 as *const ::core::ffi::c_char,
            (*info).parent_struct.name,
            variant,
        ),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        ::core::ptr::null_mut::<GCancellable>(),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GDBusProxy, *mut GAsyncResult, gpointer) -> ()>,
            GAsyncReadyCallback,
        >(Some(
            safe_c2rust_gxdp_open_uri_proxy_set_property_cb
                as unsafe extern "C" fn(*mut GDBusProxy, *mut GAsyncResult, gpointer) -> (),
        )),
        &raw const (*info).parent_struct as *mut GDBusPropertyInfo as gpointer,
    );
    g_variant_unref(variant);
}
unsafe extern "C" fn safe_c2rust_gxdp_open_uri_proxy_g_signal(
    mut proxy: *mut GDBusProxy,
    mut sender_name: *const gchar,
    mut signal_name: *const gchar,
    mut parameters: *mut GVariant,
) {
    let mut info: *mut _ExtendedGDBusSignalInfo =
        ::core::ptr::null_mut::<_ExtendedGDBusSignalInfo>();
    let mut iter: GVariantIter = _GVariantIter { x: [0; 16] };
    let mut child: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut paramv: *mut GValue = ::core::ptr::null_mut::<GValue>();
    let mut num_params: gsize = 0;
    let mut n: gsize = 0;
    let mut signal_id: guint = 0;
    info = g_dbus_interface_info_lookup_signal(
        &raw const safe_c2rust__gxdp_open_uri_interface_info.parent_struct
            as *mut GDBusInterfaceInfo,
        signal_name,
    ) as *mut _ExtendedGDBusSignalInfo;
    if info.is_null() {
        return;
    }
    num_params = g_variant_n_children(parameters);
    paramv = ({
        let mut __n: gsize = num_params.wrapping_add(1 as gsize);
        let mut __s: gsize = ::core::mem::size_of::<GValue>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GValue;
    g_value_init(
        paramv.offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        safe_c2rust_gxdp_open_uri_get_type(),
    );
    g_value_set_object(
        paramv.offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        proxy as gpointer,
    );
    g_variant_iter_init(&raw mut iter, parameters);
    n = 1 as gsize;
    loop {
        child = g_variant_iter_next_value(&raw mut iter);
        if child.is_null() {
            break;
        }
        let mut arg_info: *mut _ExtendedGDBusArgInfo = *(*info)
            .parent_struct
            .args
            .offset(n.wrapping_sub(1 as gsize) as isize)
            as *mut _ExtendedGDBusArgInfo;
        if (*arg_info).use_gvariant != 0 {
            g_value_init(paramv.offset(n as isize) as *mut GValue, G_TYPE_VARIANT);
            g_value_set_variant(paramv.offset(n as isize) as *mut GValue, child);
            n = n.wrapping_add(1);
        } else {
            let fresh7 = n;
            n = n.wrapping_add(1);
            g_dbus_gvariant_to_gvalue(child, paramv.offset(fresh7 as isize) as *mut GValue);
        }
        g_variant_unref(child);
    }
    signal_id = g_signal_lookup((*info).signal_name, safe_c2rust_gxdp_open_uri_get_type());
    g_signal_emitv(
        paramv,
        signal_id,
        0 as GQuark,
        ::core::ptr::null_mut::<GValue>(),
    );
    n = 0 as gsize;
    while n < num_params.wrapping_add(1 as gsize) {
        g_value_unset(paramv.offset(n as isize) as *mut GValue);
        n = n.wrapping_add(1);
    }
    g_free(paramv as gpointer);
}
unsafe extern "C" fn safe_c2rust_gxdp_open_uri_proxy_g_properties_changed(
    mut _proxy: *mut GDBusProxy,
    mut changed_properties: *mut GVariant,
    mut invalidated_properties: *const *const gchar,
) {
    let mut proxy: *mut GXdpOpenURIProxy =
        _proxy as *mut ::core::ffi::c_void as *mut GXdpOpenURIProxy;
    let mut n: guint = 0;
    let mut key: *const gchar = ::core::ptr::null::<gchar>();
    let mut iter: *mut GVariantIter = ::core::ptr::null_mut::<GVariantIter>();
    let mut info: *mut _ExtendedGDBusPropertyInfo =
        ::core::ptr::null_mut::<_ExtendedGDBusPropertyInfo>();
    g_variant_get(
        changed_properties,
        b"a{sv}\0" as *const u8 as *const gchar,
        &raw mut iter,
    );
    while g_variant_iter_next(
        iter,
        b"{&sv}\0" as *const u8 as *const gchar,
        &raw mut key,
        NULL,
    ) != 0
    {
        info = g_dbus_interface_info_lookup_property(
            &raw const safe_c2rust__gxdp_open_uri_interface_info.parent_struct
                as *mut GDBusInterfaceInfo,
            key,
        ) as *mut _ExtendedGDBusPropertyInfo;
        g_datalist_id_set_data_full(
            &raw mut (*(*proxy).priv_0).qdata,
            g_quark_try_string(key),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
        );
        if !info.is_null() {
            g_object_notify(
                proxy as *mut ::core::ffi::c_void as *mut GObject,
                (*info).hyphen_name,
            );
        }
    }
    g_variant_iter_free(iter);
    n = 0 as guint;
    while !(*invalidated_properties.offset(n as isize)).is_null() {
        info = g_dbus_interface_info_lookup_property(
            &raw const safe_c2rust__gxdp_open_uri_interface_info.parent_struct
                as *mut GDBusInterfaceInfo,
            *invalidated_properties.offset(n as isize),
        ) as *mut _ExtendedGDBusPropertyInfo;
        g_datalist_id_set_data_full(
            &raw mut (*(*proxy).priv_0).qdata,
            g_quark_try_string(*invalidated_properties.offset(n as isize)),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
        );
        if !info.is_null() {
            g_object_notify(
                proxy as *mut ::core::ffi::c_void as *mut GObject,
                (*info).hyphen_name,
            );
        }
        n = n.wrapping_add(1);
    }
}
unsafe extern "C" fn safe_c2rust_gxdp_open_uri_proxy_get_version(
    mut object: *mut GXdpOpenURI,
) -> guint {
    let mut proxy: *mut GXdpOpenURIProxy =
        object as *mut ::core::ffi::c_void as *mut GXdpOpenURIProxy;
    let mut variant: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut value: guint = 0 as guint;
    variant = g_dbus_proxy_get_cached_property(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"version\0" as *const u8 as *const gchar,
    );
    if !variant.is_null() {
        value = g_variant_get_uint32(variant) as guint;
        g_variant_unref(variant);
    }
    return value;
}
unsafe extern "C" fn safe_c2rust_gxdp_open_uri_proxy_init(mut proxy: *mut GXdpOpenURIProxy) {
    (*proxy).priv_0 =
        safe_c2rust_gxdp_open_uri_proxy_get_instance_private(proxy) as *mut GXdpOpenURIProxyPrivate;
    g_dbus_proxy_set_interface_info(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        safe_c2rust_gxdp_open_uri_interface_info(),
    );
}
unsafe extern "C" fn safe_c2rust_gxdp_open_uri_proxy_class_init(
    mut klass: *mut GXdpOpenURIProxyClass,
) {
    let mut gobject_class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    let mut proxy_class: *mut GDBusProxyClass = ::core::ptr::null_mut::<GDBusProxyClass>();
    gobject_class = klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize =
        Some(safe_c2rust_gxdp_open_uri_proxy_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_gxdp_open_uri_proxy_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_gxdp_open_uri_proxy_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    proxy_class = klass as *mut ::core::ffi::c_void as *mut GDBusProxyClass;
    (*proxy_class).g_signal = Some(
        safe_c2rust_gxdp_open_uri_proxy_g_signal
            as unsafe extern "C" fn(
                *mut GDBusProxy,
                *const gchar,
                *const gchar,
                *mut GVariant,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(*mut GDBusProxy, *const gchar, *const gchar, *mut GVariant) -> (),
        >;
    (*proxy_class).g_properties_changed = Some(
        safe_c2rust_gxdp_open_uri_proxy_g_properties_changed
            as unsafe extern "C" fn(*mut GDBusProxy, *mut GVariant, *const *const gchar) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GDBusProxy, *mut GVariant, *const *const gchar) -> ()>;
    safe_c2rust_gxdp_open_uri_override_properties(gobject_class, 1 as guint);
}
unsafe extern "C" fn safe_c2rust_gxdp_open_uri_proxy_iface_init(mut iface: *mut GXdpOpenURIIface) {
    (*iface).get_version = Some(
        safe_c2rust_gxdp_open_uri_proxy_get_version
            as unsafe extern "C" fn(*mut GXdpOpenURI) -> guint,
    ) as Option<unsafe extern "C" fn(*mut GXdpOpenURI) -> guint>;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_open_uri_proxy_new(
    mut connection: *mut GDBusConnection,
    mut flags: GDBusProxyFlags,
    mut name: *const gchar,
    mut object_path: *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_async_initable_new_async(
        safe_c2rust_gxdp_open_uri_proxy_get_type(),
        G_PRIORITY_DEFAULT,
        cancellable,
        callback,
        user_data,
        b"g-flags\0" as *const u8 as *const gchar,
        flags as ::core::ffi::c_uint,
        b"g-name\0" as *const u8 as *const ::core::ffi::c_char,
        name,
        b"g-connection\0" as *const u8 as *const ::core::ffi::c_char,
        connection,
        b"g-object-path\0" as *const u8 as *const ::core::ffi::c_char,
        object_path,
        b"g-interface-name\0" as *const u8 as *const ::core::ffi::c_char,
        b"org.freedesktop.portal.OpenURI\0" as *const u8 as *const ::core::ffi::c_char,
        NULL,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_open_uri_proxy_new_finish(
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GXdpOpenURI {
    let mut ret: *mut GObject = ::core::ptr::null_mut::<GObject>();
    let mut source_object: *mut GObject = ::core::ptr::null_mut::<GObject>();
    source_object = g_async_result_get_source_object(res);
    ret = g_async_initable_new_finish(
        source_object as *mut ::core::ffi::c_void as *mut GAsyncInitable,
        res,
        error,
    );
    g_object_unref(source_object as gpointer);
    if !ret.is_null() {
        return ret as *mut ::core::ffi::c_void as *mut GXdpOpenURI;
    } else {
        return ::core::ptr::null_mut::<GXdpOpenURI>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_open_uri_proxy_new_sync(
    mut connection: *mut GDBusConnection,
    mut flags: GDBusProxyFlags,
    mut name: *const gchar,
    mut object_path: *const gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GXdpOpenURI {
    let mut ret: *mut GInitable = ::core::ptr::null_mut::<GInitable>();
    ret = g_initable_new(
        safe_c2rust_gxdp_open_uri_proxy_get_type(),
        cancellable,
        error,
        b"g-flags\0" as *const u8 as *const gchar,
        flags as ::core::ffi::c_uint,
        b"g-name\0" as *const u8 as *const ::core::ffi::c_char,
        name,
        b"g-connection\0" as *const u8 as *const ::core::ffi::c_char,
        connection,
        b"g-object-path\0" as *const u8 as *const ::core::ffi::c_char,
        object_path,
        b"g-interface-name\0" as *const u8 as *const ::core::ffi::c_char,
        b"org.freedesktop.portal.OpenURI\0" as *const u8 as *const ::core::ffi::c_char,
        NULL,
    ) as *mut GInitable;
    if !ret.is_null() {
        return ret as *mut ::core::ffi::c_void as *mut GXdpOpenURI;
    } else {
        return ::core::ptr::null_mut::<GXdpOpenURI>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_open_uri_proxy_new_for_bus(
    mut bus_type: GBusType,
    mut flags: GDBusProxyFlags,
    mut name: *const gchar,
    mut object_path: *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_async_initable_new_async(
        safe_c2rust_gxdp_open_uri_proxy_get_type(),
        G_PRIORITY_DEFAULT,
        cancellable,
        callback,
        user_data,
        b"g-flags\0" as *const u8 as *const gchar,
        flags as ::core::ffi::c_uint,
        b"g-name\0" as *const u8 as *const ::core::ffi::c_char,
        name,
        b"g-bus-type\0" as *const u8 as *const ::core::ffi::c_char,
        bus_type as ::core::ffi::c_int,
        b"g-object-path\0" as *const u8 as *const ::core::ffi::c_char,
        object_path,
        b"g-interface-name\0" as *const u8 as *const ::core::ffi::c_char,
        b"org.freedesktop.portal.OpenURI\0" as *const u8 as *const ::core::ffi::c_char,
        NULL,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_open_uri_proxy_new_for_bus_finish(
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GXdpOpenURI {
    let mut ret: *mut GObject = ::core::ptr::null_mut::<GObject>();
    let mut source_object: *mut GObject = ::core::ptr::null_mut::<GObject>();
    source_object = g_async_result_get_source_object(res);
    ret = g_async_initable_new_finish(
        source_object as *mut ::core::ffi::c_void as *mut GAsyncInitable,
        res,
        error,
    );
    g_object_unref(source_object as gpointer);
    if !ret.is_null() {
        return ret as *mut ::core::ffi::c_void as *mut GXdpOpenURI;
    } else {
        return ::core::ptr::null_mut::<GXdpOpenURI>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_open_uri_proxy_new_for_bus_sync(
    mut bus_type: GBusType,
    mut flags: GDBusProxyFlags,
    mut name: *const gchar,
    mut object_path: *const gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GXdpOpenURI {
    let mut ret: *mut GInitable = ::core::ptr::null_mut::<GInitable>();
    ret = g_initable_new(
        safe_c2rust_gxdp_open_uri_proxy_get_type(),
        cancellable,
        error,
        b"g-flags\0" as *const u8 as *const gchar,
        flags as ::core::ffi::c_uint,
        b"g-name\0" as *const u8 as *const ::core::ffi::c_char,
        name,
        b"g-bus-type\0" as *const u8 as *const ::core::ffi::c_char,
        bus_type as ::core::ffi::c_int,
        b"g-object-path\0" as *const u8 as *const ::core::ffi::c_char,
        object_path,
        b"g-interface-name\0" as *const u8 as *const ::core::ffi::c_char,
        b"org.freedesktop.portal.OpenURI\0" as *const u8 as *const ::core::ffi::c_char,
        NULL,
    ) as *mut GInitable;
    if !ret.is_null() {
        return ret as *mut ::core::ffi::c_void as *mut GXdpOpenURI;
    } else {
        return ::core::ptr::null_mut::<GXdpOpenURI>();
    };
}
unsafe extern "C" fn safe_c2rust__gxdp_open_uri_skeleton_handle_method_call(
    mut connection: *mut GDBusConnection,
    mut sender: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut method_name: *const gchar,
    mut parameters: *mut GVariant,
    mut invocation: *mut GDBusMethodInvocation,
    mut user_data: gpointer,
) {
    let mut skeleton: *mut GXdpOpenURISkeleton = user_data as *mut GXdpOpenURISkeleton;
    let mut info: *mut _ExtendedGDBusMethodInfo =
        ::core::ptr::null_mut::<_ExtendedGDBusMethodInfo>();
    let mut iter: GVariantIter = _GVariantIter { x: [0; 16] };
    let mut child: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut paramv: *mut GValue = ::core::ptr::null_mut::<GValue>();
    let mut num_params: gsize = 0;
    let mut num_extra: guint = 0;
    let mut n: gsize = 0;
    let mut signal_id: guint = 0;
    let mut return_value: GValue = _GValue {
        g_type: 0 as GType,
        data: [
            C2RustUnnamed_1 {
                v_int: 0 as ::core::ffi::c_int,
            },
            C2RustUnnamed_1 { v_int: 0 },
        ],
    };
    info = g_dbus_method_invocation_get_method_info(invocation) as *mut _ExtendedGDBusMethodInfo;
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if !info.is_null() {
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
            b"gio/xdp-dbus.c\0" as *const u8 as *const ::core::ffi::c_char,
            5747 as ::core::ffi::c_int,
            G_STRFUNC,
            b"info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    num_params = g_variant_n_children(parameters);
    num_extra = (if (*info).pass_fdlist != 0 {
        3 as ::core::ffi::c_int
    } else {
        2 as ::core::ffi::c_int
    }) as guint;
    paramv = ({
        let mut __n: gsize = num_params.wrapping_add(num_extra as gsize);
        let mut __s: gsize = ::core::mem::size_of::<GValue>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GValue;
    n = 0 as gsize;
    g_value_init(
        paramv.offset(n as isize) as *mut GValue,
        safe_c2rust_gxdp_open_uri_get_type(),
    );
    let fresh8 = n;
    n = n.wrapping_add(1);
    g_value_set_object(
        paramv.offset(fresh8 as isize) as *mut GValue,
        skeleton as gpointer,
    );
    g_value_init(
        paramv.offset(n as isize) as *mut GValue,
        g_dbus_method_invocation_get_type(),
    );
    let fresh9 = n;
    n = n.wrapping_add(1);
    g_value_set_object(
        paramv.offset(fresh9 as isize) as *mut GValue,
        invocation as gpointer,
    );
    if (*info).pass_fdlist != 0 {
        g_value_init(
            paramv.offset(n as isize) as *mut GValue,
            g_unix_fd_list_get_type(),
        );
        let fresh10 = n;
        n = n.wrapping_add(1);
        g_value_set_object(
            paramv.offset(fresh10 as isize) as *mut GValue,
            g_dbus_message_get_unix_fd_list(g_dbus_method_invocation_get_message(invocation))
                as gpointer,
        );
    }
    g_variant_iter_init(&raw mut iter, parameters);
    loop {
        child = g_variant_iter_next_value(&raw mut iter);
        if child.is_null() {
            break;
        }
        let mut arg_info: *mut _ExtendedGDBusArgInfo = *(*info)
            .parent_struct
            .in_args
            .offset(n.wrapping_sub(num_extra as gsize) as isize)
            as *mut _ExtendedGDBusArgInfo;
        if (*arg_info).use_gvariant != 0 {
            g_value_init(paramv.offset(n as isize) as *mut GValue, G_TYPE_VARIANT);
            g_value_set_variant(paramv.offset(n as isize) as *mut GValue, child);
            n = n.wrapping_add(1);
        } else {
            let fresh11 = n;
            n = n.wrapping_add(1);
            g_dbus_gvariant_to_gvalue(child, paramv.offset(fresh11 as isize) as *mut GValue);
        }
        g_variant_unref(child);
    }
    signal_id = g_signal_lookup((*info).signal_name, safe_c2rust_gxdp_open_uri_get_type());
    g_value_init(&raw mut return_value, G_TYPE_BOOLEAN);
    g_signal_emitv(paramv, signal_id, 0 as GQuark, &raw mut return_value);
    if g_value_get_boolean(&raw mut return_value) == 0 {
        g_dbus_method_invocation_return_error(
            invocation,
            g_dbus_error_quark(),
            G_DBUS_ERROR_UNKNOWN_METHOD as ::core::ffi::c_int as gint,
            b"Method %s is not implemented on interface %s\0" as *const u8 as *const gchar,
            method_name,
            interface_name,
        );
    }
    g_value_unset(&raw mut return_value);
    n = 0 as gsize;
    while n < num_params.wrapping_add(num_extra as gsize) {
        g_value_unset(paramv.offset(n as isize) as *mut GValue);
        n = n.wrapping_add(1);
    }
    g_free(paramv as gpointer);
}
unsafe extern "C" fn safe_c2rust__gxdp_open_uri_skeleton_handle_get_property(
    mut connection: *mut GDBusConnection,
    mut sender: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut property_name: *const gchar,
    mut error: *mut *mut GError,
    mut user_data: gpointer,
) -> *mut GVariant {
    let mut skeleton: *mut GXdpOpenURISkeleton = user_data as *mut GXdpOpenURISkeleton;
    let mut value: GValue = _GValue {
        g_type: 0 as GType,
        data: [
            C2RustUnnamed_1 {
                v_int: 0 as ::core::ffi::c_int,
            },
            C2RustUnnamed_1 { v_int: 0 },
        ],
    };
    let mut pspec: *mut GParamSpec = ::core::ptr::null_mut::<GParamSpec>();
    let mut info: *mut _ExtendedGDBusPropertyInfo =
        ::core::ptr::null_mut::<_ExtendedGDBusPropertyInfo>();
    let mut ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    ret = ::core::ptr::null_mut::<GVariant>();
    info = g_dbus_interface_info_lookup_property(
        &raw const safe_c2rust__gxdp_open_uri_interface_info.parent_struct
            as *mut GDBusInterfaceInfo,
        property_name,
    ) as *mut _ExtendedGDBusPropertyInfo;
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if !info.is_null() {
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
            b"gio/xdp-dbus.c\0" as *const u8 as *const ::core::ffi::c_char,
            5806 as ::core::ffi::c_int,
            G_STRFUNC,
            b"info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    pspec = g_object_class_find_property(
        (*(skeleton as *mut GTypeInstance)).g_class as *mut GObjectClass,
        (*info).hyphen_name,
    );
    if pspec.is_null() {
        g_set_error(
            error,
            g_dbus_error_quark(),
            G_DBUS_ERROR_INVALID_ARGS as ::core::ffi::c_int as gint,
            b"No property with name %s\0" as *const u8 as *const gchar,
            property_name,
        );
    } else {
        g_value_init(&raw mut value, (*pspec).value_type);
        g_object_get_property(
            skeleton as *mut ::core::ffi::c_void as *mut GObject,
            (*info).hyphen_name,
            &raw mut value,
        );
        ret = g_dbus_gvalue_to_gvariant(
            &raw mut value,
            g_variant_type_checked_((*info).parent_struct.signature),
        );
        g_value_unset(&raw mut value);
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust__gxdp_open_uri_skeleton_handle_set_property(
    mut connection: *mut GDBusConnection,
    mut sender: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut property_name: *const gchar,
    mut variant: *mut GVariant,
    mut error: *mut *mut GError,
    mut user_data: gpointer,
) -> gboolean {
    let mut skeleton: *mut GXdpOpenURISkeleton = user_data as *mut GXdpOpenURISkeleton;
    let mut value: GValue = _GValue {
        g_type: 0 as GType,
        data: [
            C2RustUnnamed_1 {
                v_int: 0 as ::core::ffi::c_int,
            },
            C2RustUnnamed_1 { v_int: 0 },
        ],
    };
    let mut pspec: *mut GParamSpec = ::core::ptr::null_mut::<GParamSpec>();
    let mut info: *mut _ExtendedGDBusPropertyInfo =
        ::core::ptr::null_mut::<_ExtendedGDBusPropertyInfo>();
    let mut ret: gboolean = 0;
    ret = FALSE as gboolean;
    info = g_dbus_interface_info_lookup_property(
        &raw const safe_c2rust__gxdp_open_uri_interface_info.parent_struct
            as *mut GDBusInterfaceInfo,
        property_name,
    ) as *mut _ExtendedGDBusPropertyInfo;
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if !info.is_null() {
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
            b"gio/xdp-dbus.c\0" as *const u8 as *const ::core::ffi::c_char,
            5840 as ::core::ffi::c_int,
            G_STRFUNC,
            b"info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    pspec = g_object_class_find_property(
        (*(skeleton as *mut GTypeInstance)).g_class as *mut GObjectClass,
        (*info).hyphen_name,
    );
    if pspec.is_null() {
        g_set_error(
            error,
            g_dbus_error_quark(),
            G_DBUS_ERROR_INVALID_ARGS as ::core::ffi::c_int as gint,
            b"No property with name %s\0" as *const u8 as *const gchar,
            property_name,
        );
    } else {
        if (*info).use_gvariant() != 0 {
            g_value_set_variant(&raw mut value, variant);
        } else {
            g_dbus_gvariant_to_gvalue(variant, &raw mut value);
        }
        g_object_set_property(
            skeleton as *mut ::core::ffi::c_void as *mut GObject,
            (*info).hyphen_name,
            &raw mut value,
        );
        g_value_unset(&raw mut value);
        ret = TRUE as gboolean;
    }
    return ret;
}
static mut safe_c2rust__gxdp_open_uri_skeleton_vtable: GDBusInterfaceVTable = unsafe {
    _GDBusInterfaceVTable {
        method_call: Some(
            safe_c2rust__gxdp_open_uri_skeleton_handle_method_call
                as unsafe extern "C" fn(
                    *mut GDBusConnection,
                    *const gchar,
                    *const gchar,
                    *const gchar,
                    *const gchar,
                    *mut GVariant,
                    *mut GDBusMethodInvocation,
                    gpointer,
                ) -> (),
        ),
        get_property: Some(
            safe_c2rust__gxdp_open_uri_skeleton_handle_get_property
                as unsafe extern "C" fn(
                    *mut GDBusConnection,
                    *const gchar,
                    *const gchar,
                    *const gchar,
                    *const gchar,
                    *mut *mut GError,
                    gpointer,
                ) -> *mut GVariant,
        ),
        set_property: Some(
            safe_c2rust__gxdp_open_uri_skeleton_handle_set_property
                as unsafe extern "C" fn(
                    *mut GDBusConnection,
                    *const gchar,
                    *const gchar,
                    *const gchar,
                    *const gchar,
                    *mut GVariant,
                    *mut *mut GError,
                    gpointer,
                ) -> gboolean,
        ),
        padding: [
            NULL,
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        ],
    }
};
unsafe extern "C" fn safe_c2rust_gxdp_open_uri_skeleton_dbus_interface_get_info(
    mut skeleton: *mut GDBusInterfaceSkeleton,
) -> *mut GDBusInterfaceInfo {
    return safe_c2rust_gxdp_open_uri_interface_info();
}
unsafe extern "C" fn safe_c2rust_gxdp_open_uri_skeleton_dbus_interface_get_vtable(
    mut skeleton: *mut GDBusInterfaceSkeleton,
) -> *mut GDBusInterfaceVTable {
    return &raw const safe_c2rust__gxdp_open_uri_skeleton_vtable as *mut GDBusInterfaceVTable;
}
unsafe extern "C" fn safe_c2rust_gxdp_open_uri_skeleton_dbus_interface_get_properties(
    mut _skeleton: *mut GDBusInterfaceSkeleton,
) -> *mut GVariant {
    let mut skeleton: *mut GXdpOpenURISkeleton =
        _skeleton as *mut ::core::ffi::c_void as *mut GXdpOpenURISkeleton;
    let mut builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut n: guint = 0;
    g_variant_builder_init(
        &raw mut builder,
        g_variant_type_checked_(b"a{sv}\0" as *const u8 as *const gchar),
    );
    if !safe_c2rust__gxdp_open_uri_interface_info
        .parent_struct
        .properties
        .is_null()
    {
        n = 0 as guint;
        while !(*safe_c2rust__gxdp_open_uri_interface_info
            .parent_struct
            .properties
            .offset(n as isize))
        .is_null()
        {
            let mut info: *mut GDBusPropertyInfo = *safe_c2rust__gxdp_open_uri_interface_info
                .parent_struct
                .properties
                .offset(n as isize);
            if (*info).flags as ::core::ffi::c_uint
                & G_DBUS_PROPERTY_INFO_FLAGS_READABLE as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
            {
                let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
                value = safe_c2rust__gxdp_open_uri_skeleton_handle_get_property(
                    g_dbus_interface_skeleton_get_connection(
                        skeleton as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton,
                    ),
                    ::core::ptr::null::<gchar>(),
                    g_dbus_interface_skeleton_get_object_path(
                        skeleton as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton,
                    ),
                    b"org.freedesktop.portal.OpenURI\0" as *const u8 as *const gchar,
                    (*info).name,
                    ::core::ptr::null_mut::<*mut GError>(),
                    skeleton as gpointer,
                );
                if !value.is_null() {
                    g_variant_take_ref(value);
                    g_variant_builder_add(
                        &raw mut builder,
                        b"{sv}\0" as *const u8 as *const gchar,
                        (*info).name,
                        value,
                    );
                    g_variant_unref(value);
                }
            }
            n = n.wrapping_add(1);
        }
    }
    return g_variant_builder_end(&raw mut builder);
}
unsafe extern "C" fn safe_c2rust_gxdp_open_uri_skeleton_dbus_interface_flush(
    mut _skeleton: *mut GDBusInterfaceSkeleton,
) {
    let mut skeleton: *mut GXdpOpenURISkeleton =
        _skeleton as *mut ::core::ffi::c_void as *mut GXdpOpenURISkeleton;
    let mut emit_changed: gboolean = FALSE;
    g_mutex_lock(&raw mut (*(*skeleton).priv_0).lock);
    if !(*(*skeleton).priv_0)
        .changed_properties_idle_source
        .is_null()
    {
        g_source_destroy((*(*skeleton).priv_0).changed_properties_idle_source);
        (*(*skeleton).priv_0).changed_properties_idle_source = ::core::ptr::null_mut::<GSource>();
        emit_changed = TRUE as gboolean;
    }
    g_mutex_unlock(&raw mut (*(*skeleton).priv_0).lock);
    if emit_changed != 0 {
        safe_c2rust__gxdp_open_uri_emit_changed(skeleton as gpointer);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_open_uri_skeleton_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_gxdp_open_uri_skeleton_get_type_once();
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
unsafe extern "C" fn safe_c2rust_gxdp_open_uri_skeleton_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_dbus_interface_skeleton_get_type(),
        g_intern_static_string(b"GXdpOpenURISkeleton\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GXdpOpenURISkeletonClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_gxdp_open_uri_skeleton_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GXdpOpenURISkeleton>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GXdpOpenURISkeleton) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_gxdp_open_uri_skeleton_init
                    as unsafe extern "C" fn(*mut GXdpOpenURISkeleton) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GXdpOpenURISkeleton_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GXdpOpenURISkeletonPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GXdpOpenURIIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_gxdp_open_uri_skeleton_iface_init
                as unsafe extern "C" fn(*mut GXdpOpenURIIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        safe_c2rust_gxdp_open_uri_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
#[inline]
unsafe extern "C" fn safe_c2rust_gxdp_open_uri_skeleton_get_instance_private(
    mut self_0: *mut GXdpOpenURISkeleton,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GXdpOpenURISkeleton_private_offset as glong as isize)
        as gpointer;
}
static mut safe_c2rust_GXdpOpenURISkeleton_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_gxdp_open_uri_skeleton_class_intern_init(mut klass: gpointer) {
    safe_c2rust_gxdp_open_uri_skeleton_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GXdpOpenURISkeleton_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GXdpOpenURISkeleton_private_offset,
        );
    }
    safe_c2rust_gxdp_open_uri_skeleton_class_init(klass as *mut GXdpOpenURISkeletonClass);
}
static mut safe_c2rust_gxdp_open_uri_skeleton_parent_class: gpointer = NULL;
unsafe extern "C" fn safe_c2rust_gxdp_open_uri_skeleton_finalize(mut object: *mut GObject) {
    let mut skeleton: *mut GXdpOpenURISkeleton =
        object as *mut ::core::ffi::c_void as *mut GXdpOpenURISkeleton;
    let mut n: guint = 0;
    n = 0 as guint;
    while n < 1 as guint {
        g_value_unset((*(*skeleton).priv_0).properties.offset(n as isize) as *mut GValue);
        n = n.wrapping_add(1);
    }
    g_free((*(*skeleton).priv_0).properties as gpointer);
    g_list_free_full(
        (*(*skeleton).priv_0).changed_properties,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut ChangedProperty) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust__changed_property_free as unsafe extern "C" fn(*mut ChangedProperty) -> (),
        )),
    );
    if !(*(*skeleton).priv_0)
        .changed_properties_idle_source
        .is_null()
    {
        g_source_destroy((*(*skeleton).priv_0).changed_properties_idle_source);
    }
    g_main_context_unref((*(*skeleton).priv_0).context);
    g_mutex_clear(&raw mut (*(*skeleton).priv_0).lock);
    (*(safe_c2rust_gxdp_open_uri_skeleton_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_gxdp_open_uri_skeleton_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut skeleton: *mut GXdpOpenURISkeleton =
        object as *mut ::core::ffi::c_void as *mut GXdpOpenURISkeleton;
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if prop_id != 0 as guint && prop_id.wrapping_sub(1 as guint) < 1 as guint {
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
            b"gio/xdp-dbus.c\0" as *const u8 as *const ::core::ffi::c_char,
            5963 as ::core::ffi::c_int,
            G_STRFUNC,
            b"prop_id != 0 && prop_id - 1 < 1\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_mutex_lock(&raw mut (*(*skeleton).priv_0).lock);
    g_value_copy(
        (*(*skeleton).priv_0)
            .properties
            .offset(prop_id.wrapping_sub(1 as guint) as isize) as *mut GValue,
        value,
    );
    g_mutex_unlock(&raw mut (*(*skeleton).priv_0).lock);
}
unsafe extern "C" fn safe_c2rust__gxdp_open_uri_emit_changed(mut user_data: gpointer) -> gboolean {
    let mut skeleton: *mut GXdpOpenURISkeleton = user_data as *mut GXdpOpenURISkeleton;
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut invalidated_builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut num_changes: guint = 0;
    g_mutex_lock(&raw mut (*(*skeleton).priv_0).lock);
    g_variant_builder_init(
        &raw mut builder,
        g_variant_type_checked_(b"a{sv}\0" as *const u8 as *const gchar),
    );
    g_variant_builder_init(
        &raw mut invalidated_builder,
        g_variant_type_checked_(b"as\0" as *const u8 as *const gchar),
    );
    l = (*(*skeleton).priv_0).changed_properties;
    num_changes = 0 as guint;
    while !l.is_null() {
        let mut cp: *mut ChangedProperty = (*l).data as *mut ChangedProperty;
        let mut variant: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        let mut cur_value: *const GValue = ::core::ptr::null::<GValue>();
        cur_value = (*(*skeleton).priv_0)
            .properties
            .offset((*cp).prop_id.wrapping_sub(1 as guint) as isize)
            as *mut GValue;
        if safe_c2rust__g_value_equal(cur_value, &raw mut (*cp).orig_value) == 0 {
            variant = g_dbus_gvalue_to_gvariant(
                cur_value,
                g_variant_type_checked_((*(*cp).info).parent_struct.signature),
            );
            g_variant_builder_add(
                &raw mut builder,
                b"{sv}\0" as *const u8 as *const gchar,
                (*(*cp).info).parent_struct.name,
                variant,
            );
            g_variant_unref(variant);
            num_changes = num_changes.wrapping_add(1);
        }
        l = (*l).next;
    }
    if num_changes > 0 as guint {
        let mut connections: *mut GList = ::core::ptr::null_mut::<GList>();
        let mut ll: *mut GList = ::core::ptr::null_mut::<GList>();
        let mut signal_variant: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        signal_variant = g_variant_ref_sink(g_variant_new(
            b"(sa{sv}as)\0" as *const u8 as *const gchar,
            b"org.freedesktop.portal.OpenURI\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut builder,
            &raw mut invalidated_builder,
        ));
        connections = g_dbus_interface_skeleton_get_connections(
            skeleton as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton,
        );
        ll = connections;
        while !ll.is_null() {
            let mut connection: *mut GDBusConnection = (*ll).data as *mut GDBusConnection;
            g_dbus_connection_emit_signal(
                connection,
                ::core::ptr::null::<gchar>(),
                g_dbus_interface_skeleton_get_object_path(
                    skeleton as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton,
                ),
                b"org.freedesktop.DBus.Properties\0" as *const u8 as *const gchar,
                b"PropertiesChanged\0" as *const u8 as *const gchar,
                signal_variant,
                ::core::ptr::null_mut::<*mut GError>(),
            );
            ll = (*ll).next;
        }
        g_variant_unref(signal_variant);
        g_list_free_full(
            connections,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    } else {
        g_variant_builder_clear(&raw mut builder);
        g_variant_builder_clear(&raw mut invalidated_builder);
    }
    g_list_free_full(
        (*(*skeleton).priv_0).changed_properties,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut ChangedProperty) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust__changed_property_free as unsafe extern "C" fn(*mut ChangedProperty) -> (),
        )),
    );
    (*(*skeleton).priv_0).changed_properties = ::core::ptr::null_mut::<GList>();
    (*(*skeleton).priv_0).changed_properties_idle_source = ::core::ptr::null_mut::<GSource>();
    g_mutex_unlock(&raw mut (*(*skeleton).priv_0).lock);
    return FALSE;
}
unsafe extern "C" fn safe_c2rust__gxdp_open_uri_schedule_emit_changed(
    mut skeleton: *mut GXdpOpenURISkeleton,
    mut info: *const _ExtendedGDBusPropertyInfo,
    mut prop_id: guint,
    mut orig_value: *const GValue,
) {
    let mut cp: *mut ChangedProperty = ::core::ptr::null_mut::<ChangedProperty>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    cp = ::core::ptr::null_mut::<ChangedProperty>();
    l = (*(*skeleton).priv_0).changed_properties;
    while !l.is_null() {
        let mut i_cp: *mut ChangedProperty = (*l).data as *mut ChangedProperty;
        if (*i_cp).info == info {
            cp = i_cp;
            break;
        } else {
            l = (*l).next;
        }
    }
    if cp.is_null() {
        cp = ({
            let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
            let mut __s: gsize = ::core::mem::size_of::<ChangedProperty>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc0(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc0(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc0_n(__n, __s);
            }
            __p
        }) as *mut ChangedProperty;
        (*cp).prop_id = prop_id;
        (*cp).info = info;
        (*(*skeleton).priv_0).changed_properties =
            g_list_prepend((*(*skeleton).priv_0).changed_properties, cp as gpointer);
        g_value_init(
            &raw mut (*cp).orig_value,
            (*(orig_value as *mut GValue)).g_type,
        );
        g_value_copy(orig_value, &raw mut (*cp).orig_value);
    }
}
unsafe extern "C" fn safe_c2rust_gxdp_open_uri_skeleton_notify(
    mut object: *mut GObject,
    mut pspec: *mut GParamSpec,
) {
    let mut skeleton: *mut GXdpOpenURISkeleton =
        object as *mut ::core::ffi::c_void as *mut GXdpOpenURISkeleton;
    g_mutex_lock(&raw mut (*(*skeleton).priv_0).lock);
    if !(*(*skeleton).priv_0).changed_properties.is_null()
        && (*(*skeleton).priv_0)
            .changed_properties_idle_source
            .is_null()
    {
        (*(*skeleton).priv_0).changed_properties_idle_source = g_idle_source_new();
        g_source_set_priority(
            (*(*skeleton).priv_0).changed_properties_idle_source,
            G_PRIORITY_DEFAULT,
        );
        g_source_set_callback(
            (*(*skeleton).priv_0).changed_properties_idle_source,
            Some(
                safe_c2rust__gxdp_open_uri_emit_changed
                    as unsafe extern "C" fn(gpointer) -> gboolean,
            ),
            g_object_ref(skeleton as gpointer) as *mut GXdpOpenURISkeleton as gpointer,
            ::core::mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> ()>, GDestroyNotify>(
                Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
            ),
        );
        g_source_set_name(
            (*(*skeleton).priv_0).changed_properties_idle_source,
            b"[generated] _gxdp_open_uri_emit_changed\0" as *const u8 as *const ::core::ffi::c_char,
        );
        g_source_attach(
            (*(*skeleton).priv_0).changed_properties_idle_source,
            (*(*skeleton).priv_0).context,
        );
        g_source_unref((*(*skeleton).priv_0).changed_properties_idle_source);
    }
    g_mutex_unlock(&raw mut (*(*skeleton).priv_0).lock);
}
unsafe extern "C" fn safe_c2rust_gxdp_open_uri_skeleton_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut info: *const _ExtendedGDBusPropertyInfo =
        ::core::ptr::null::<_ExtendedGDBusPropertyInfo>();
    let mut skeleton: *mut GXdpOpenURISkeleton =
        object as *mut ::core::ffi::c_void as *mut GXdpOpenURISkeleton;
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if prop_id != 0 as guint && prop_id.wrapping_sub(1 as guint) < 1 as guint {
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
            b"gio/xdp-dbus.c\0" as *const u8 as *const ::core::ffi::c_char,
            6082 as ::core::ffi::c_int,
            G_STRFUNC,
            b"prop_id != 0 && prop_id - 1 < 1\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    info = safe_c2rust__gxdp_open_uri_property_info_pointers
        [prop_id.wrapping_sub(1 as guint) as usize] as *const _ExtendedGDBusPropertyInfo;
    g_mutex_lock(&raw mut (*(*skeleton).priv_0).lock);
    g_object_freeze_notify(object);
    if safe_c2rust__g_value_equal(
        value,
        (*(*skeleton).priv_0)
            .properties
            .offset(prop_id.wrapping_sub(1 as guint) as isize) as *mut GValue,
    ) == 0
    {
        if !g_dbus_interface_skeleton_get_connection(
            skeleton as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton,
        )
        .is_null()
            && (*info).emits_changed_signal() as ::core::ffi::c_int != 0
        {
            safe_c2rust__gxdp_open_uri_schedule_emit_changed(
                skeleton,
                info,
                prop_id,
                (*(*skeleton).priv_0)
                    .properties
                    .offset(prop_id.wrapping_sub(1 as guint) as isize)
                    as *mut GValue,
            );
        }
        g_value_copy(
            value,
            (*(*skeleton).priv_0)
                .properties
                .offset(prop_id.wrapping_sub(1 as guint) as isize) as *mut GValue,
        );
        g_object_notify_by_pspec(object, pspec);
    }
    g_mutex_unlock(&raw mut (*(*skeleton).priv_0).lock);
    g_object_thaw_notify(object);
}
unsafe extern "C" fn safe_c2rust_gxdp_open_uri_skeleton_init(
    mut skeleton: *mut GXdpOpenURISkeleton,
) {
    (*skeleton).priv_0 = safe_c2rust_gxdp_open_uri_skeleton_get_instance_private(skeleton)
        as *mut GXdpOpenURISkeletonPrivate;
    g_mutex_init(&raw mut (*(*skeleton).priv_0).lock);
    (*(*skeleton).priv_0).context = g_main_context_ref_thread_default();
    (*(*skeleton).priv_0).properties = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GValue>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GValue;
    g_value_init(
        (*(*skeleton).priv_0)
            .properties
            .offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        G_TYPE_UINT,
    );
}
unsafe extern "C" fn safe_c2rust_gxdp_open_uri_skeleton_get_version(
    mut object: *mut GXdpOpenURI,
) -> guint {
    let mut skeleton: *mut GXdpOpenURISkeleton =
        object as *mut ::core::ffi::c_void as *mut GXdpOpenURISkeleton;
    let mut value: guint = 0;
    g_mutex_lock(&raw mut (*(*skeleton).priv_0).lock);
    value = (*(*(*skeleton).priv_0)
        .properties
        .offset(0 as ::core::ffi::c_int as isize))
    .data[0 as ::core::ffi::c_int as usize]
        .v_uint;
    g_mutex_unlock(&raw mut (*(*skeleton).priv_0).lock);
    return value;
}
unsafe extern "C" fn safe_c2rust_gxdp_open_uri_skeleton_class_init(
    mut klass: *mut GXdpOpenURISkeletonClass,
) {
    let mut gobject_class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    let mut skeleton_class: *mut GDBusInterfaceSkeletonClass =
        ::core::ptr::null_mut::<GDBusInterfaceSkeletonClass>();
    gobject_class = klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize = Some(
        safe_c2rust_gxdp_open_uri_skeleton_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_gxdp_open_uri_skeleton_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_gxdp_open_uri_skeleton_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).notify = Some(
        safe_c2rust_gxdp_open_uri_skeleton_notify
            as unsafe extern "C" fn(*mut GObject, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, *mut GParamSpec) -> ()>;
    safe_c2rust_gxdp_open_uri_override_properties(gobject_class, 1 as guint);
    skeleton_class = klass as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeletonClass;
    (*skeleton_class).get_info = Some(
        safe_c2rust_gxdp_open_uri_skeleton_dbus_interface_get_info
            as unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GDBusInterfaceInfo,
    )
        as Option<unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GDBusInterfaceInfo>;
    (*skeleton_class).get_properties = Some(
        safe_c2rust_gxdp_open_uri_skeleton_dbus_interface_get_properties
            as unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GVariant,
    )
        as Option<unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GVariant>;
    (*skeleton_class).flush = Some(
        safe_c2rust_gxdp_open_uri_skeleton_dbus_interface_flush
            as unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> ()>;
    (*skeleton_class).get_vtable = Some(
        safe_c2rust_gxdp_open_uri_skeleton_dbus_interface_get_vtable
            as unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GDBusInterfaceVTable,
    )
        as Option<unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GDBusInterfaceVTable>;
}
unsafe extern "C" fn safe_c2rust_gxdp_open_uri_skeleton_iface_init(
    mut iface: *mut GXdpOpenURIIface,
) {
    (*iface).get_version = Some(
        safe_c2rust_gxdp_open_uri_skeleton_get_version
            as unsafe extern "C" fn(*mut GXdpOpenURI) -> guint,
    ) as Option<unsafe extern "C" fn(*mut GXdpOpenURI) -> guint>;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_open_uri_skeleton_new() -> *mut GXdpOpenURI {
    return g_object_new(
        safe_c2rust_gxdp_open_uri_skeleton_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GXdpOpenURI;
}
static mut safe_c2rust__gxdp_proxy_resolver_method_info_lookup_IN_ARG_uri: _ExtendedGDBusArgInfo =
    _ExtendedGDBusArgInfo {
        parent_struct: _GDBusArgInfo {
            ref_count: -(1 as gint),
            name: b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        use_gvariant: FALSE,
    };
static mut safe_c2rust__gxdp_proxy_resolver_method_info_lookup_IN_ARG_pointers:
    [*const GDBusArgInfo; 2] = [::core::ptr::null::<GDBusArgInfo>(); 2];
static mut safe_c2rust__gxdp_proxy_resolver_method_info_lookup_OUT_ARG_proxies:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"proxies\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"as\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust__gxdp_proxy_resolver_method_info_lookup_OUT_ARG_pointers:
    [*const GDBusArgInfo; 2] = [::core::ptr::null::<GDBusArgInfo>(); 2];
static mut safe_c2rust__gxdp_proxy_resolver_method_info_lookup: _ExtendedGDBusMethodInfo = unsafe {
    _ExtendedGDBusMethodInfo {
        parent_struct: _GDBusMethodInfo {
            ref_count: -(1 as gint),
            name: b"Lookup\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            in_args: &raw const safe_c2rust__gxdp_proxy_resolver_method_info_lookup_IN_ARG_pointers
                as *mut *mut GDBusArgInfo,
            out_args:
                &raw const safe_c2rust__gxdp_proxy_resolver_method_info_lookup_OUT_ARG_pointers
                    as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"handle-lookup\0" as *const u8 as *const gchar,
        pass_fdlist: FALSE,
    }
};
static mut safe_c2rust__gxdp_proxy_resolver_method_info_pointers: [*const GDBusMethodInfo; 2] =
    [::core::ptr::null::<GDBusMethodInfo>(); 2];
static mut safe_c2rust__gxdp_proxy_resolver_property_info_version: _ExtendedGDBusPropertyInfo =
    _ExtendedGDBusPropertyInfo {
        parent_struct: _GDBusPropertyInfo {
            ref_count: 0,
            name: ::core::ptr::null::<gchar>() as *mut gchar,
            signature: ::core::ptr::null::<gchar>() as *mut gchar,
            flags: G_DBUS_PROPERTY_INFO_FLAGS_NONE,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        hyphen_name: ::core::ptr::null::<gchar>(),
        use_gvariant_emits_changed_signal: [0; 1],
        c2rust_padding: [0; 7],
    };
static mut safe_c2rust__gxdp_proxy_resolver_property_info_pointers: [*const GDBusPropertyInfo; 2] =
    [::core::ptr::null::<GDBusPropertyInfo>(); 2];
static mut safe_c2rust__gxdp_proxy_resolver_interface_info: _ExtendedGDBusInterfaceInfo = unsafe {
    _ExtendedGDBusInterfaceInfo {
        parent_struct: _GDBusInterfaceInfo {
            ref_count: -(1 as gint),
            name: b"org.freedesktop.portal.ProxyResolver\0" as *const u8
                as *const ::core::ffi::c_char as *mut gchar,
            methods: &raw const safe_c2rust__gxdp_proxy_resolver_method_info_pointers
                as *mut *mut GDBusMethodInfo,
            signals: ::core::ptr::null::<*mut GDBusSignalInfo>() as *mut *mut GDBusSignalInfo,
            properties: &raw const safe_c2rust__gxdp_proxy_resolver_property_info_pointers
                as *mut *mut GDBusPropertyInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        hyphen_name: b"proxy-resolver\0" as *const u8 as *const gchar,
    }
};
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_interface_info() -> *mut GDBusInterfaceInfo
{
    return &raw const safe_c2rust__gxdp_proxy_resolver_interface_info.parent_struct
        as *mut GDBusInterfaceInfo;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_override_properties(
    mut klass: *mut GObjectClass,
    mut property_id_begin: guint,
) -> guint {
    let fresh12 = property_id_begin;
    property_id_begin = property_id_begin.wrapping_add(1);
    g_object_class_override_property(klass, fresh12, b"version\0" as *const u8 as *const gchar);
    return property_id_begin.wrapping_sub(1 as guint);
}
#[inline]
unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_method_marshal_lookup(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: ::core::ffi::c_uint,
    mut param_values: *const GValue,
    mut invocation_hint: *mut ::core::ffi::c_void,
    mut marshal_data: *mut ::core::ffi::c_void,
) {
    safe_c2rust__g_dbus_codegen_marshal_BOOLEAN__OBJECT_STRING(
        closure,
        return_value,
        n_param_values,
        param_values,
        invocation_hint,
        marshal_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_get_type() -> GType {
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
        let mut g_define_type_id: GType = g_type_register_static_simple(
            G_TYPE_INTERFACE,
            g_intern_static_string(b"GXdpProxyResolver\0" as *const u8 as *const gchar),
            ::core::mem::size_of::<GXdpProxyResolverInterface>() as guint,
            ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GXdpProxyResolverIface) -> ()>,
                    Option<unsafe extern "C" fn() -> ()>,
                >(Some(
                    safe_c2rust_gxdp_proxy_resolver_default_init
                        as unsafe extern "C" fn(*mut GXdpProxyResolverIface) -> (),
                )),
            ),
            0 as guint,
            ::core::mem::transmute::<*mut ::core::ffi::c_void, GInstanceInitFunc>(NULL),
            G_TYPE_FLAG_NONE,
        );
        if ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType != G_TYPE_INVALID {
            g_type_interface_add_prerequisite(
                g_define_type_id,
                ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            );
        }
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
unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_default_init(
    mut iface: *mut GXdpProxyResolverIface,
) {
    g_signal_new(
        b"handle-lookup\0" as *const u8 as *const gchar,
        (*(iface as *mut GTypeInterface)).g_type,
        G_SIGNAL_RUN_LAST,
        16 as ::core::ffi::c_ulong as glong as guint,
        Some(
            g_signal_accumulator_true_handled
                as unsafe extern "C" fn(
                    *mut GSignalInvocationHint,
                    *mut GValue,
                    *const GValue,
                    gpointer,
                ) -> gboolean,
        ),
        NULL,
        Some(
            safe_c2rust_gxdp_proxy_resolver_method_marshal_lookup
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    ::core::ffi::c_uint,
                    *const GValue,
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        G_TYPE_BOOLEAN,
        2 as guint,
        g_dbus_method_invocation_get_type(),
        G_TYPE_STRING,
    );
    g_object_interface_install_property(
        iface as gpointer,
        g_param_spec_uint(
            b"version\0" as *const u8 as *const gchar,
            b"version\0" as *const u8 as *const gchar,
            b"version\0" as *const u8 as *const gchar,
            0 as guint,
            G_MAXUINT32,
            0 as guint,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_get_version(
    mut object: *mut GXdpProxyResolver,
) -> guint {
    if ({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = object as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_gxdp_proxy_resolver_get_type();
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
            b"GXDP_IS_PROXY_RESOLVER (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    return (*(g_type_interface_peek(
        (*(object as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_gxdp_proxy_resolver_get_type(),
    ) as *mut GXdpProxyResolverIface))
        .get_version
        .expect("non-null function pointer")(object);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_set_version(
    mut object: *mut GXdpProxyResolver,
    mut value: guint,
) {
    g_object_set(
        object as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"version\0" as *const u8 as *const gchar,
        value,
        NULL,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_call_lookup(
    mut proxy: *mut GXdpProxyResolver,
    mut arg_uri: *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_dbus_proxy_call(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"Lookup\0" as *const u8 as *const gchar,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, arg_uri),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_call_lookup_finish(
    mut proxy: *mut GXdpProxyResolver,
    mut out_proxies: *mut *mut *mut gchar,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_finish(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        res,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(_ret, b"(^as)\0" as *const u8 as *const gchar, out_proxies);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_call_lookup_sync(
    mut proxy: *mut GXdpProxyResolver,
    mut arg_uri: *const gchar,
    mut out_proxies: *mut *mut *mut gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_sync(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"Lookup\0" as *const u8 as *const gchar,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, arg_uri),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(_ret, b"(^as)\0" as *const u8 as *const gchar, out_proxies);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_complete_lookup(
    mut object: *mut GXdpProxyResolver,
    mut invocation: *mut GDBusMethodInvocation,
    mut proxies: *const *const gchar,
) {
    g_dbus_method_invocation_return_value(
        invocation,
        g_variant_new(b"(^as)\0" as *const u8 as *const gchar, proxies),
    );
}
static mut safe_c2rust_gxdp_proxy_resolver_proxy_parent_class: gpointer = NULL;
#[inline]
unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_proxy_get_instance_private(
    mut self_0: *mut GXdpProxyResolverProxy,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GXdpProxyResolverProxy_private_offset as glong as isize)
        as gpointer;
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_proxy_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_dbus_proxy_get_type(),
        g_intern_static_string(b"GXdpProxyResolverProxy\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GXdpProxyResolverProxyClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_gxdp_proxy_resolver_proxy_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GXdpProxyResolverProxy>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GXdpProxyResolverProxy) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_gxdp_proxy_resolver_proxy_init
                    as unsafe extern "C" fn(*mut GXdpProxyResolverProxy) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GXdpProxyResolverProxy_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GXdpProxyResolverProxyPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GXdpProxyResolverIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_gxdp_proxy_resolver_proxy_iface_init
                as unsafe extern "C" fn(*mut GXdpProxyResolverIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        safe_c2rust_gxdp_proxy_resolver_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_proxy_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_gxdp_proxy_resolver_proxy_get_type_once();
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
unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_proxy_class_intern_init(mut klass: gpointer) {
    safe_c2rust_gxdp_proxy_resolver_proxy_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GXdpProxyResolverProxy_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GXdpProxyResolverProxy_private_offset,
        );
    }
    safe_c2rust_gxdp_proxy_resolver_proxy_class_init(klass as *mut GXdpProxyResolverProxyClass);
}
static mut safe_c2rust_GXdpProxyResolverProxy_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_proxy_finalize(mut object: *mut GObject) {
    let mut proxy: *mut GXdpProxyResolverProxy =
        object as *mut ::core::ffi::c_void as *mut GXdpProxyResolverProxy;
    g_datalist_clear(&raw mut (*(*proxy).priv_0).qdata);
    (*(safe_c2rust_gxdp_proxy_resolver_proxy_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_proxy_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut info: *const _ExtendedGDBusPropertyInfo =
        ::core::ptr::null::<_ExtendedGDBusPropertyInfo>();
    let mut variant: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if prop_id != 0 as guint && prop_id.wrapping_sub(1 as guint) < 1 as guint {
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
            b"gio/xdp-dbus.c\0" as *const u8 as *const ::core::ffi::c_char,
            6579 as ::core::ffi::c_int,
            G_STRFUNC,
            b"prop_id != 0 && prop_id - 1 < 1\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    info = safe_c2rust__gxdp_proxy_resolver_property_info_pointers
        [prop_id.wrapping_sub(1 as guint) as usize] as *const _ExtendedGDBusPropertyInfo;
    variant = g_dbus_proxy_get_cached_property(
        object as *mut ::core::ffi::c_void as *mut GDBusProxy,
        (*info).parent_struct.name,
    );
    if (*info).use_gvariant() != 0 {
        g_value_set_variant(value, variant);
    } else if !variant.is_null() {
        g_dbus_gvariant_to_gvalue(variant, value);
    }
    if !variant.is_null() {
        g_variant_unref(variant);
    }
}
unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_proxy_set_property_cb(
    mut proxy: *mut GDBusProxy,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut info: *const _ExtendedGDBusPropertyInfo =
        user_data as *const _ExtendedGDBusPropertyInfo;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    error = ::core::ptr::null_mut::<GError>();
    _ret = g_dbus_proxy_call_finish(proxy, res, &raw mut error);
    if _ret.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Error setting property '%s' on interface org.freedesktop.portal.ProxyResolver: %s (%s, %d)\0"
                as *const u8 as *const gchar,
            (*info).parent_struct.name,
            (*error).message,
            g_quark_to_string((*error).domain),
            (*error).code,
        );
        g_error_free(error);
    } else {
        g_variant_unref(_ret);
    };
}
unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_proxy_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut info: *const _ExtendedGDBusPropertyInfo =
        ::core::ptr::null::<_ExtendedGDBusPropertyInfo>();
    let mut variant: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if ({
        let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
        if prop_id != 0 as guint && prop_id.wrapping_sub(1 as guint) < 1 as guint {
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
            b"gio/xdp-dbus.c\0" as *const u8 as *const ::core::ffi::c_char,
            6626 as ::core::ffi::c_int,
            G_STRFUNC,
            b"prop_id != 0 && prop_id - 1 < 1\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    info = safe_c2rust__gxdp_proxy_resolver_property_info_pointers
        [prop_id.wrapping_sub(1 as guint) as usize] as *const _ExtendedGDBusPropertyInfo;
    variant = g_dbus_gvalue_to_gvariant(
        value,
        g_variant_type_checked_((*info).parent_struct.signature),
    );
    g_dbus_proxy_call(
        object as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"org.freedesktop.DBus.Properties.Set\0" as *const u8 as *const gchar,
        g_variant_new(
            b"(ssv)\0" as *const u8 as *const gchar,
            b"org.freedesktop.portal.ProxyResolver\0" as *const u8 as *const ::core::ffi::c_char,
            (*info).parent_struct.name,
            variant,
        ),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        ::core::ptr::null_mut::<GCancellable>(),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GDBusProxy, *mut GAsyncResult, gpointer) -> ()>,
            GAsyncReadyCallback,
        >(Some(
            safe_c2rust_gxdp_proxy_resolver_proxy_set_property_cb
                as unsafe extern "C" fn(*mut GDBusProxy, *mut GAsyncResult, gpointer) -> (),
        )),
        &raw const (*info).parent_struct as *mut GDBusPropertyInfo as gpointer,
    );
    g_variant_unref(variant);
}
unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_proxy_g_signal(
    mut proxy: *mut GDBusProxy,
    mut sender_name: *const gchar,
    mut signal_name: *const gchar,
    mut parameters: *mut GVariant,
) {
    let mut info: *mut _ExtendedGDBusSignalInfo =
        ::core::ptr::null_mut::<_ExtendedGDBusSignalInfo>();
    let mut iter: GVariantIter = _GVariantIter { x: [0; 16] };
    let mut child: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut paramv: *mut GValue = ::core::ptr::null_mut::<GValue>();
    let mut num_params: gsize = 0;
    let mut n: gsize = 0;
    let mut signal_id: guint = 0;
    info = g_dbus_interface_info_lookup_signal(
        &raw const safe_c2rust__gxdp_proxy_resolver_interface_info.parent_struct
            as *mut GDBusInterfaceInfo,
        signal_name,
    ) as *mut _ExtendedGDBusSignalInfo;
    if info.is_null() {
        return;
    }
    num_params = g_variant_n_children(parameters);
    paramv = ({
        let mut __n: gsize = num_params.wrapping_add(1 as gsize);
        let mut __s: gsize = ::core::mem::size_of::<GValue>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GValue;
    g_value_init(
        paramv.offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        safe_c2rust_gxdp_proxy_resolver_get_type(),
    );
    g_value_set_object(
        paramv.offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        proxy as gpointer,
    );
    g_variant_iter_init(&raw mut iter, parameters);
    n = 1 as gsize;
    loop {
        child = g_variant_iter_next_value(&raw mut iter);
        if child.is_null() {
            break;
        }
        let mut arg_info: *mut _ExtendedGDBusArgInfo = *(*info)
            .parent_struct
            .args
            .offset(n.wrapping_sub(1 as gsize) as isize)
            as *mut _ExtendedGDBusArgInfo;
        if (*arg_info).use_gvariant != 0 {
            g_value_init(paramv.offset(n as isize) as *mut GValue, G_TYPE_VARIANT);
            g_value_set_variant(paramv.offset(n as isize) as *mut GValue, child);
            n = n.wrapping_add(1);
        } else {
            let fresh13 = n;
            n = n.wrapping_add(1);
            g_dbus_gvariant_to_gvalue(child, paramv.offset(fresh13 as isize) as *mut GValue);
        }
        g_variant_unref(child);
    }
    signal_id = g_signal_lookup(
        (*info).signal_name,
        safe_c2rust_gxdp_proxy_resolver_get_type(),
    );
    g_signal_emitv(
        paramv,
        signal_id,
        0 as GQuark,
        ::core::ptr::null_mut::<GValue>(),
    );
    n = 0 as gsize;
    while n < num_params.wrapping_add(1 as gsize) {
        g_value_unset(paramv.offset(n as isize) as *mut GValue);
        n = n.wrapping_add(1);
    }
    g_free(paramv as gpointer);
}
unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_proxy_g_properties_changed(
    mut _proxy: *mut GDBusProxy,
    mut changed_properties: *mut GVariant,
    mut invalidated_properties: *const *const gchar,
) {
    let mut proxy: *mut GXdpProxyResolverProxy =
        _proxy as *mut ::core::ffi::c_void as *mut GXdpProxyResolverProxy;
    let mut n: guint = 0;
    let mut key: *const gchar = ::core::ptr::null::<gchar>();
    let mut iter: *mut GVariantIter = ::core::ptr::null_mut::<GVariantIter>();
    let mut info: *mut _ExtendedGDBusPropertyInfo =
        ::core::ptr::null_mut::<_ExtendedGDBusPropertyInfo>();
    g_variant_get(
        changed_properties,
        b"a{sv}\0" as *const u8 as *const gchar,
        &raw mut iter,
    );
    while g_variant_iter_next(
        iter,
        b"{&sv}\0" as *const u8 as *const gchar,
        &raw mut key,
        NULL,
    ) != 0
    {
        info = g_dbus_interface_info_lookup_property(
            &raw const safe_c2rust__gxdp_proxy_resolver_interface_info.parent_struct
                as *mut GDBusInterfaceInfo,
            key,
        ) as *mut _ExtendedGDBusPropertyInfo;
        g_datalist_id_set_data_full(
            &raw mut (*(*proxy).priv_0).qdata,
            g_quark_try_string(key),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
        );
        if !info.is_null() {
            g_object_notify(
                proxy as *mut ::core::ffi::c_void as *mut GObject,
                (*info).hyphen_name,
            );
        }
    }
    g_variant_iter_free(iter);
    n = 0 as guint;
    while !(*invalidated_properties.offset(n as isize)).is_null() {
        info = g_dbus_interface_info_lookup_property(
            &raw const safe_c2rust__gxdp_proxy_resolver_interface_info.parent_struct
                as *mut GDBusInterfaceInfo,
            *invalidated_properties.offset(n as isize),
        ) as *mut _ExtendedGDBusPropertyInfo;
        g_datalist_id_set_data_full(
            &raw mut (*(*proxy).priv_0).qdata,
            g_quark_try_string(*invalidated_properties.offset(n as isize)),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
        );
        if !info.is_null() {
            g_object_notify(
                proxy as *mut ::core::ffi::c_void as *mut GObject,
                (*info).hyphen_name,
            );
        }
        n = n.wrapping_add(1);
    }
}
unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_proxy_get_version(
    mut object: *mut GXdpProxyResolver,
) -> guint {
    let mut proxy: *mut GXdpProxyResolverProxy =
        object as *mut ::core::ffi::c_void as *mut GXdpProxyResolverProxy;
    let mut variant: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut value: guint = 0 as guint;
    variant = g_dbus_proxy_get_cached_property(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"version\0" as *const u8 as *const gchar,
    );
    if !variant.is_null() {
        value = g_variant_get_uint32(variant) as guint;
        g_variant_unref(variant);
    }
    return value;
}
unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_proxy_init(
    mut proxy: *mut GXdpProxyResolverProxy,
) {
    (*proxy).priv_0 = safe_c2rust_gxdp_proxy_resolver_proxy_get_instance_private(proxy)
        as *mut GXdpProxyResolverProxyPrivate;
    g_dbus_proxy_set_interface_info(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        safe_c2rust_gxdp_proxy_resolver_interface_info(),
    );
}
unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_proxy_class_init(
    mut klass: *mut GXdpProxyResolverProxyClass,
) {
    let mut gobject_class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    let mut proxy_class: *mut GDBusProxyClass = ::core::ptr::null_mut::<GDBusProxyClass>();
    gobject_class = klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize = Some(
        safe_c2rust_gxdp_proxy_resolver_proxy_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_gxdp_proxy_resolver_proxy_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_gxdp_proxy_resolver_proxy_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    proxy_class = klass as *mut ::core::ffi::c_void as *mut GDBusProxyClass;
    (*proxy_class).g_signal = Some(
        safe_c2rust_gxdp_proxy_resolver_proxy_g_signal
            as unsafe extern "C" fn(
                *mut GDBusProxy,
                *const gchar,
                *const gchar,
                *mut GVariant,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(*mut GDBusProxy, *const gchar, *const gchar, *mut GVariant) -> (),
        >;
    (*proxy_class).g_properties_changed = Some(
        safe_c2rust_gxdp_proxy_resolver_proxy_g_properties_changed
            as unsafe extern "C" fn(*mut GDBusProxy, *mut GVariant, *const *const gchar) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GDBusProxy, *mut GVariant, *const *const gchar) -> ()>;
    safe_c2rust_gxdp_proxy_resolver_override_properties(gobject_class, 1 as guint);
}
unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_proxy_iface_init(
    mut iface: *mut GXdpProxyResolverIface,
) {
    (*iface).get_version = Some(
        safe_c2rust_gxdp_proxy_resolver_proxy_get_version
            as unsafe extern "C" fn(*mut GXdpProxyResolver) -> guint,
    ) as Option<unsafe extern "C" fn(*mut GXdpProxyResolver) -> guint>;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_proxy_new(
    mut connection: *mut GDBusConnection,
    mut flags: GDBusProxyFlags,
    mut name: *const gchar,
    mut object_path: *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_async_initable_new_async(
        safe_c2rust_gxdp_proxy_resolver_proxy_get_type(),
        G_PRIORITY_DEFAULT,
        cancellable,
        callback,
        user_data,
        b"g-flags\0" as *const u8 as *const gchar,
        flags as ::core::ffi::c_uint,
        b"g-name\0" as *const u8 as *const ::core::ffi::c_char,
        name,
        b"g-connection\0" as *const u8 as *const ::core::ffi::c_char,
        connection,
        b"g-object-path\0" as *const u8 as *const ::core::ffi::c_char,
        object_path,
        b"g-interface-name\0" as *const u8 as *const ::core::ffi::c_char,
        b"org.freedesktop.portal.ProxyResolver\0" as *const u8 as *const ::core::ffi::c_char,
        NULL,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_proxy_new_finish(
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GXdpProxyResolver {
    let mut ret: *mut GObject = ::core::ptr::null_mut::<GObject>();
    let mut source_object: *mut GObject = ::core::ptr::null_mut::<GObject>();
    source_object = g_async_result_get_source_object(res);
    ret = g_async_initable_new_finish(
        source_object as *mut ::core::ffi::c_void as *mut GAsyncInitable,
        res,
        error,
    );
    g_object_unref(source_object as gpointer);
    if !ret.is_null() {
        return ret as *mut ::core::ffi::c_void as *mut GXdpProxyResolver;
    } else {
        return ::core::ptr::null_mut::<GXdpProxyResolver>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_proxy_new_sync(
    mut connection: *mut GDBusConnection,
    mut flags: GDBusProxyFlags,
    mut name: *const gchar,
    mut object_path: *const gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GXdpProxyResolver {
    let mut ret: *mut GInitable = ::core::ptr::null_mut::<GInitable>();
    ret = g_initable_new(
        safe_c2rust_gxdp_proxy_resolver_proxy_get_type(),
        cancellable,
        error,
        b"g-flags\0" as *const u8 as *const gchar,
        flags as ::core::ffi::c_uint,
        b"g-name\0" as *const u8 as *const ::core::ffi::c_char,
        name,
        b"g-connection\0" as *const u8 as *const ::core::ffi::c_char,
        connection,
        b"g-object-path\0" as *const u8 as *const ::core::ffi::c_char,
        object_path,
        b"g-interface-name\0" as *const u8 as *const ::core::ffi::c_char,
        b"org.freedesktop.portal.ProxyResolver\0" as *const u8 as *const ::core::ffi::c_char,
        NULL,
    ) as *mut GInitable;
    if !ret.is_null() {
        return ret as *mut ::core::ffi::c_void as *mut GXdpProxyResolver;
    } else {
        return ::core::ptr::null_mut::<GXdpProxyResolver>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_proxy_new_for_bus(
    mut bus_type: GBusType,
    mut flags: GDBusProxyFlags,
    mut name: *const gchar,
    mut object_path: *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_async_initable_new_async(
        safe_c2rust_gxdp_proxy_resolver_proxy_get_type(),
        G_PRIORITY_DEFAULT,
        cancellable,
        callback,
        user_data,
        b"g-flags\0" as *const u8 as *const gchar,
        flags as ::core::ffi::c_uint,
        b"g-name\0" as *const u8 as *const ::core::ffi::c_char,
        name,
        b"g-bus-type\0" as *const u8 as *const ::core::ffi::c_char,
        bus_type as ::core::ffi::c_int,
        b"g-object-path\0" as *const u8 as *const ::core::ffi::c_char,
        object_path,
        b"g-interface-name\0" as *const u8 as *const ::core::ffi::c_char,
        b"org.freedesktop.portal.ProxyResolver\0" as *const u8 as *const ::core::ffi::c_char,
        NULL,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_proxy_new_for_bus_finish(
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GXdpProxyResolver {
    let mut ret: *mut GObject = ::core::ptr::null_mut::<GObject>();
    let mut source_object: *mut GObject = ::core::ptr::null_mut::<GObject>();
    source_object = g_async_result_get_source_object(res);
    ret = g_async_initable_new_finish(
        source_object as *mut ::core::ffi::c_void as *mut GAsyncInitable,
        res,
        error,
    );
    g_object_unref(source_object as gpointer);
    if !ret.is_null() {
        return ret as *mut ::core::ffi::c_void as *mut GXdpProxyResolver;
    } else {
        return ::core::ptr::null_mut::<GXdpProxyResolver>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_proxy_new_for_bus_sync(
    mut bus_type: GBusType,
    mut flags: GDBusProxyFlags,
    mut name: *const gchar,
    mut object_path: *const gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GXdpProxyResolver {
    let mut ret: *mut GInitable = ::core::ptr::null_mut::<GInitable>();
    ret = g_initable_new(
        safe_c2rust_gxdp_proxy_resolver_proxy_get_type(),
        cancellable,
        error,
        b"g-flags\0" as *const u8 as *const gchar,
        flags as ::core::ffi::c_uint,
        b"g-name\0" as *const u8 as *const ::core::ffi::c_char,
        name,
        b"g-bus-type\0" as *const u8 as *const ::core::ffi::c_char,
        bus_type as ::core::ffi::c_int,
        b"g-object-path\0" as *const u8 as *const ::core::ffi::c_char,
        object_path,
        b"g-interface-name\0" as *const u8 as *const ::core::ffi::c_char,
        b"org.freedesktop.portal.ProxyResolver\0" as *const u8 as *const ::core::ffi::c_char,
        NULL,
    ) as *mut GInitable;
    if !ret.is_null() {
        return ret as *mut ::core::ffi::c_void as *mut GXdpProxyResolver;
    } else {
        return ::core::ptr::null_mut::<GXdpProxyResolver>();
    };
}
unsafe extern "C" fn safe_c2rust__gxdp_proxy_resolver_skeleton_handle_method_call(
    mut connection: *mut GDBusConnection,
    mut sender: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut method_name: *const gchar,
    mut parameters: *mut GVariant,
    mut invocation: *mut GDBusMethodInvocation,
    mut user_data: gpointer,
) {
    let mut skeleton: *mut GXdpProxyResolverSkeleton = user_data as *mut GXdpProxyResolverSkeleton;
    let mut info: *mut _ExtendedGDBusMethodInfo =
        ::core::ptr::null_mut::<_ExtendedGDBusMethodInfo>();
    let mut iter: GVariantIter = _GVariantIter { x: [0; 16] };
    let mut child: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut paramv: *mut GValue = ::core::ptr::null_mut::<GValue>();
    let mut num_params: gsize = 0;
    let mut num_extra: guint = 0;
    let mut n: gsize = 0;
    let mut signal_id: guint = 0;
    let mut return_value: GValue = _GValue {
        g_type: 0 as GType,
        data: [
            C2RustUnnamed_1 {
                v_int: 0 as ::core::ffi::c_int,
            },
            C2RustUnnamed_1 { v_int: 0 },
        ],
    };
    info = g_dbus_method_invocation_get_method_info(invocation) as *mut _ExtendedGDBusMethodInfo;
    if ({
        let mut _g_boolean_var_50: ::core::ffi::c_int = 0;
        if !info.is_null() {
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
            b"gio/xdp-dbus.c\0" as *const u8 as *const ::core::ffi::c_char,
            6989 as ::core::ffi::c_int,
            G_STRFUNC,
            b"info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    num_params = g_variant_n_children(parameters);
    num_extra = (if (*info).pass_fdlist != 0 {
        3 as ::core::ffi::c_int
    } else {
        2 as ::core::ffi::c_int
    }) as guint;
    paramv = ({
        let mut __n: gsize = num_params.wrapping_add(num_extra as gsize);
        let mut __s: gsize = ::core::mem::size_of::<GValue>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GValue;
    n = 0 as gsize;
    g_value_init(
        paramv.offset(n as isize) as *mut GValue,
        safe_c2rust_gxdp_proxy_resolver_get_type(),
    );
    let fresh14 = n;
    n = n.wrapping_add(1);
    g_value_set_object(
        paramv.offset(fresh14 as isize) as *mut GValue,
        skeleton as gpointer,
    );
    g_value_init(
        paramv.offset(n as isize) as *mut GValue,
        g_dbus_method_invocation_get_type(),
    );
    let fresh15 = n;
    n = n.wrapping_add(1);
    g_value_set_object(
        paramv.offset(fresh15 as isize) as *mut GValue,
        invocation as gpointer,
    );
    if (*info).pass_fdlist != 0 {
        g_value_init(
            paramv.offset(n as isize) as *mut GValue,
            g_unix_fd_list_get_type(),
        );
        let fresh16 = n;
        n = n.wrapping_add(1);
        g_value_set_object(
            paramv.offset(fresh16 as isize) as *mut GValue,
            g_dbus_message_get_unix_fd_list(g_dbus_method_invocation_get_message(invocation))
                as gpointer,
        );
    }
    g_variant_iter_init(&raw mut iter, parameters);
    loop {
        child = g_variant_iter_next_value(&raw mut iter);
        if child.is_null() {
            break;
        }
        let mut arg_info: *mut _ExtendedGDBusArgInfo = *(*info)
            .parent_struct
            .in_args
            .offset(n.wrapping_sub(num_extra as gsize) as isize)
            as *mut _ExtendedGDBusArgInfo;
        if (*arg_info).use_gvariant != 0 {
            g_value_init(paramv.offset(n as isize) as *mut GValue, G_TYPE_VARIANT);
            g_value_set_variant(paramv.offset(n as isize) as *mut GValue, child);
            n = n.wrapping_add(1);
        } else {
            let fresh17 = n;
            n = n.wrapping_add(1);
            g_dbus_gvariant_to_gvalue(child, paramv.offset(fresh17 as isize) as *mut GValue);
        }
        g_variant_unref(child);
    }
    signal_id = g_signal_lookup(
        (*info).signal_name,
        safe_c2rust_gxdp_proxy_resolver_get_type(),
    );
    g_value_init(&raw mut return_value, G_TYPE_BOOLEAN);
    g_signal_emitv(paramv, signal_id, 0 as GQuark, &raw mut return_value);
    if g_value_get_boolean(&raw mut return_value) == 0 {
        g_dbus_method_invocation_return_error(
            invocation,
            g_dbus_error_quark(),
            G_DBUS_ERROR_UNKNOWN_METHOD as ::core::ffi::c_int as gint,
            b"Method %s is not implemented on interface %s\0" as *const u8 as *const gchar,
            method_name,
            interface_name,
        );
    }
    g_value_unset(&raw mut return_value);
    n = 0 as gsize;
    while n < num_params.wrapping_add(num_extra as gsize) {
        g_value_unset(paramv.offset(n as isize) as *mut GValue);
        n = n.wrapping_add(1);
    }
    g_free(paramv as gpointer);
}
unsafe extern "C" fn safe_c2rust__gxdp_proxy_resolver_skeleton_handle_get_property(
    mut connection: *mut GDBusConnection,
    mut sender: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut property_name: *const gchar,
    mut error: *mut *mut GError,
    mut user_data: gpointer,
) -> *mut GVariant {
    let mut skeleton: *mut GXdpProxyResolverSkeleton = user_data as *mut GXdpProxyResolverSkeleton;
    let mut value: GValue = _GValue {
        g_type: 0 as GType,
        data: [
            C2RustUnnamed_1 {
                v_int: 0 as ::core::ffi::c_int,
            },
            C2RustUnnamed_1 { v_int: 0 },
        ],
    };
    let mut pspec: *mut GParamSpec = ::core::ptr::null_mut::<GParamSpec>();
    let mut info: *mut _ExtendedGDBusPropertyInfo =
        ::core::ptr::null_mut::<_ExtendedGDBusPropertyInfo>();
    let mut ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    ret = ::core::ptr::null_mut::<GVariant>();
    info = g_dbus_interface_info_lookup_property(
        &raw const safe_c2rust__gxdp_proxy_resolver_interface_info.parent_struct
            as *mut GDBusInterfaceInfo,
        property_name,
    ) as *mut _ExtendedGDBusPropertyInfo;
    if ({
        let mut _g_boolean_var_51: ::core::ffi::c_int = 0;
        if !info.is_null() {
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
            b"gio/xdp-dbus.c\0" as *const u8 as *const ::core::ffi::c_char,
            7048 as ::core::ffi::c_int,
            G_STRFUNC,
            b"info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    pspec = g_object_class_find_property(
        (*(skeleton as *mut GTypeInstance)).g_class as *mut GObjectClass,
        (*info).hyphen_name,
    );
    if pspec.is_null() {
        g_set_error(
            error,
            g_dbus_error_quark(),
            G_DBUS_ERROR_INVALID_ARGS as ::core::ffi::c_int as gint,
            b"No property with name %s\0" as *const u8 as *const gchar,
            property_name,
        );
    } else {
        g_value_init(&raw mut value, (*pspec).value_type);
        g_object_get_property(
            skeleton as *mut ::core::ffi::c_void as *mut GObject,
            (*info).hyphen_name,
            &raw mut value,
        );
        ret = g_dbus_gvalue_to_gvariant(
            &raw mut value,
            g_variant_type_checked_((*info).parent_struct.signature),
        );
        g_value_unset(&raw mut value);
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust__gxdp_proxy_resolver_skeleton_handle_set_property(
    mut connection: *mut GDBusConnection,
    mut sender: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut property_name: *const gchar,
    mut variant: *mut GVariant,
    mut error: *mut *mut GError,
    mut user_data: gpointer,
) -> gboolean {
    let mut skeleton: *mut GXdpProxyResolverSkeleton = user_data as *mut GXdpProxyResolverSkeleton;
    let mut value: GValue = _GValue {
        g_type: 0 as GType,
        data: [
            C2RustUnnamed_1 {
                v_int: 0 as ::core::ffi::c_int,
            },
            C2RustUnnamed_1 { v_int: 0 },
        ],
    };
    let mut pspec: *mut GParamSpec = ::core::ptr::null_mut::<GParamSpec>();
    let mut info: *mut _ExtendedGDBusPropertyInfo =
        ::core::ptr::null_mut::<_ExtendedGDBusPropertyInfo>();
    let mut ret: gboolean = 0;
    ret = FALSE as gboolean;
    info = g_dbus_interface_info_lookup_property(
        &raw const safe_c2rust__gxdp_proxy_resolver_interface_info.parent_struct
            as *mut GDBusInterfaceInfo,
        property_name,
    ) as *mut _ExtendedGDBusPropertyInfo;
    if ({
        let mut _g_boolean_var_52: ::core::ffi::c_int = 0;
        if !info.is_null() {
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
            b"gio/xdp-dbus.c\0" as *const u8 as *const ::core::ffi::c_char,
            7082 as ::core::ffi::c_int,
            G_STRFUNC,
            b"info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    pspec = g_object_class_find_property(
        (*(skeleton as *mut GTypeInstance)).g_class as *mut GObjectClass,
        (*info).hyphen_name,
    );
    if pspec.is_null() {
        g_set_error(
            error,
            g_dbus_error_quark(),
            G_DBUS_ERROR_INVALID_ARGS as ::core::ffi::c_int as gint,
            b"No property with name %s\0" as *const u8 as *const gchar,
            property_name,
        );
    } else {
        if (*info).use_gvariant() != 0 {
            g_value_set_variant(&raw mut value, variant);
        } else {
            g_dbus_gvariant_to_gvalue(variant, &raw mut value);
        }
        g_object_set_property(
            skeleton as *mut ::core::ffi::c_void as *mut GObject,
            (*info).hyphen_name,
            &raw mut value,
        );
        g_value_unset(&raw mut value);
        ret = TRUE as gboolean;
    }
    return ret;
}
static mut safe_c2rust__gxdp_proxy_resolver_skeleton_vtable: GDBusInterfaceVTable = unsafe {
    _GDBusInterfaceVTable {
        method_call: Some(
            safe_c2rust__gxdp_proxy_resolver_skeleton_handle_method_call
                as unsafe extern "C" fn(
                    *mut GDBusConnection,
                    *const gchar,
                    *const gchar,
                    *const gchar,
                    *const gchar,
                    *mut GVariant,
                    *mut GDBusMethodInvocation,
                    gpointer,
                ) -> (),
        ),
        get_property: Some(
            safe_c2rust__gxdp_proxy_resolver_skeleton_handle_get_property
                as unsafe extern "C" fn(
                    *mut GDBusConnection,
                    *const gchar,
                    *const gchar,
                    *const gchar,
                    *const gchar,
                    *mut *mut GError,
                    gpointer,
                ) -> *mut GVariant,
        ),
        set_property: Some(
            safe_c2rust__gxdp_proxy_resolver_skeleton_handle_set_property
                as unsafe extern "C" fn(
                    *mut GDBusConnection,
                    *const gchar,
                    *const gchar,
                    *const gchar,
                    *const gchar,
                    *mut GVariant,
                    *mut *mut GError,
                    gpointer,
                ) -> gboolean,
        ),
        padding: [
            NULL,
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        ],
    }
};
unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_skeleton_dbus_interface_get_info(
    mut skeleton: *mut GDBusInterfaceSkeleton,
) -> *mut GDBusInterfaceInfo {
    return safe_c2rust_gxdp_proxy_resolver_interface_info();
}
unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_skeleton_dbus_interface_get_vtable(
    mut skeleton: *mut GDBusInterfaceSkeleton,
) -> *mut GDBusInterfaceVTable {
    return &raw const safe_c2rust__gxdp_proxy_resolver_skeleton_vtable
        as *mut GDBusInterfaceVTable;
}
unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_skeleton_dbus_interface_get_properties(
    mut _skeleton: *mut GDBusInterfaceSkeleton,
) -> *mut GVariant {
    let mut skeleton: *mut GXdpProxyResolverSkeleton =
        _skeleton as *mut ::core::ffi::c_void as *mut GXdpProxyResolverSkeleton;
    let mut builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut n: guint = 0;
    g_variant_builder_init(
        &raw mut builder,
        g_variant_type_checked_(b"a{sv}\0" as *const u8 as *const gchar),
    );
    if !safe_c2rust__gxdp_proxy_resolver_interface_info
        .parent_struct
        .properties
        .is_null()
    {
        n = 0 as guint;
        while !(*safe_c2rust__gxdp_proxy_resolver_interface_info
            .parent_struct
            .properties
            .offset(n as isize))
        .is_null()
        {
            let mut info: *mut GDBusPropertyInfo = *safe_c2rust__gxdp_proxy_resolver_interface_info
                .parent_struct
                .properties
                .offset(n as isize);
            if (*info).flags as ::core::ffi::c_uint
                & G_DBUS_PROPERTY_INFO_FLAGS_READABLE as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
            {
                let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
                value = safe_c2rust__gxdp_proxy_resolver_skeleton_handle_get_property(
                    g_dbus_interface_skeleton_get_connection(
                        skeleton as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton,
                    ),
                    ::core::ptr::null::<gchar>(),
                    g_dbus_interface_skeleton_get_object_path(
                        skeleton as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton,
                    ),
                    b"org.freedesktop.portal.ProxyResolver\0" as *const u8 as *const gchar,
                    (*info).name,
                    ::core::ptr::null_mut::<*mut GError>(),
                    skeleton as gpointer,
                );
                if !value.is_null() {
                    g_variant_take_ref(value);
                    g_variant_builder_add(
                        &raw mut builder,
                        b"{sv}\0" as *const u8 as *const gchar,
                        (*info).name,
                        value,
                    );
                    g_variant_unref(value);
                }
            }
            n = n.wrapping_add(1);
        }
    }
    return g_variant_builder_end(&raw mut builder);
}
unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_skeleton_dbus_interface_flush(
    mut _skeleton: *mut GDBusInterfaceSkeleton,
) {
    let mut skeleton: *mut GXdpProxyResolverSkeleton =
        _skeleton as *mut ::core::ffi::c_void as *mut GXdpProxyResolverSkeleton;
    let mut emit_changed: gboolean = FALSE;
    g_mutex_lock(&raw mut (*(*skeleton).priv_0).lock);
    if !(*(*skeleton).priv_0)
        .changed_properties_idle_source
        .is_null()
    {
        g_source_destroy((*(*skeleton).priv_0).changed_properties_idle_source);
        (*(*skeleton).priv_0).changed_properties_idle_source = ::core::ptr::null_mut::<GSource>();
        emit_changed = TRUE as gboolean;
    }
    g_mutex_unlock(&raw mut (*(*skeleton).priv_0).lock);
    if emit_changed != 0 {
        safe_c2rust__gxdp_proxy_resolver_emit_changed(skeleton as gpointer);
    }
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_skeleton_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_dbus_interface_skeleton_get_type(),
        g_intern_static_string(b"GXdpProxyResolverSkeleton\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GXdpProxyResolverSkeletonClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_gxdp_proxy_resolver_skeleton_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GXdpProxyResolverSkeleton>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GXdpProxyResolverSkeleton) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_gxdp_proxy_resolver_skeleton_init
                    as unsafe extern "C" fn(*mut GXdpProxyResolverSkeleton) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GXdpProxyResolverSkeleton_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GXdpProxyResolverSkeletonPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GXdpProxyResolverIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_gxdp_proxy_resolver_skeleton_iface_init
                as unsafe extern "C" fn(*mut GXdpProxyResolverIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        safe_c2rust_gxdp_proxy_resolver_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
static mut safe_c2rust_gxdp_proxy_resolver_skeleton_parent_class: gpointer = NULL;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_skeleton_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_gxdp_proxy_resolver_skeleton_get_type_once();
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
unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_skeleton_class_intern_init(
    mut klass: gpointer,
) {
    safe_c2rust_gxdp_proxy_resolver_skeleton_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GXdpProxyResolverSkeleton_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GXdpProxyResolverSkeleton_private_offset,
        );
    }
    safe_c2rust_gxdp_proxy_resolver_skeleton_class_init(
        klass as *mut GXdpProxyResolverSkeletonClass,
    );
}
static mut safe_c2rust_GXdpProxyResolverSkeleton_private_offset: gint = 0;
#[inline]
unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_skeleton_get_instance_private(
    mut self_0: *mut GXdpProxyResolverSkeleton,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GXdpProxyResolverSkeleton_private_offset as glong as isize)
        as gpointer;
}
unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_skeleton_finalize(mut object: *mut GObject) {
    let mut skeleton: *mut GXdpProxyResolverSkeleton =
        object as *mut ::core::ffi::c_void as *mut GXdpProxyResolverSkeleton;
    let mut n: guint = 0;
    n = 0 as guint;
    while n < 1 as guint {
        g_value_unset((*(*skeleton).priv_0).properties.offset(n as isize) as *mut GValue);
        n = n.wrapping_add(1);
    }
    g_free((*(*skeleton).priv_0).properties as gpointer);
    g_list_free_full(
        (*(*skeleton).priv_0).changed_properties,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut ChangedProperty) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust__changed_property_free as unsafe extern "C" fn(*mut ChangedProperty) -> (),
        )),
    );
    if !(*(*skeleton).priv_0)
        .changed_properties_idle_source
        .is_null()
    {
        g_source_destroy((*(*skeleton).priv_0).changed_properties_idle_source);
    }
    g_main_context_unref((*(*skeleton).priv_0).context);
    g_mutex_clear(&raw mut (*(*skeleton).priv_0).lock);
    (*(safe_c2rust_gxdp_proxy_resolver_skeleton_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_skeleton_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut skeleton: *mut GXdpProxyResolverSkeleton =
        object as *mut ::core::ffi::c_void as *mut GXdpProxyResolverSkeleton;
    if ({
        let mut _g_boolean_var_53: ::core::ffi::c_int = 0;
        if prop_id != 0 as guint && prop_id.wrapping_sub(1 as guint) < 1 as guint {
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
            b"gio/xdp-dbus.c\0" as *const u8 as *const ::core::ffi::c_char,
            7205 as ::core::ffi::c_int,
            G_STRFUNC,
            b"prop_id != 0 && prop_id - 1 < 1\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_mutex_lock(&raw mut (*(*skeleton).priv_0).lock);
    g_value_copy(
        (*(*skeleton).priv_0)
            .properties
            .offset(prop_id.wrapping_sub(1 as guint) as isize) as *mut GValue,
        value,
    );
    g_mutex_unlock(&raw mut (*(*skeleton).priv_0).lock);
}
unsafe extern "C" fn safe_c2rust__gxdp_proxy_resolver_emit_changed(
    mut user_data: gpointer,
) -> gboolean {
    let mut skeleton: *mut GXdpProxyResolverSkeleton = user_data as *mut GXdpProxyResolverSkeleton;
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut invalidated_builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut num_changes: guint = 0;
    g_mutex_lock(&raw mut (*(*skeleton).priv_0).lock);
    g_variant_builder_init(
        &raw mut builder,
        g_variant_type_checked_(b"a{sv}\0" as *const u8 as *const gchar),
    );
    g_variant_builder_init(
        &raw mut invalidated_builder,
        g_variant_type_checked_(b"as\0" as *const u8 as *const gchar),
    );
    l = (*(*skeleton).priv_0).changed_properties;
    num_changes = 0 as guint;
    while !l.is_null() {
        let mut cp: *mut ChangedProperty = (*l).data as *mut ChangedProperty;
        let mut variant: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        let mut cur_value: *const GValue = ::core::ptr::null::<GValue>();
        cur_value = (*(*skeleton).priv_0)
            .properties
            .offset((*cp).prop_id.wrapping_sub(1 as guint) as isize)
            as *mut GValue;
        if safe_c2rust__g_value_equal(cur_value, &raw mut (*cp).orig_value) == 0 {
            variant = g_dbus_gvalue_to_gvariant(
                cur_value,
                g_variant_type_checked_((*(*cp).info).parent_struct.signature),
            );
            g_variant_builder_add(
                &raw mut builder,
                b"{sv}\0" as *const u8 as *const gchar,
                (*(*cp).info).parent_struct.name,
                variant,
            );
            g_variant_unref(variant);
            num_changes = num_changes.wrapping_add(1);
        }
        l = (*l).next;
    }
    if num_changes > 0 as guint {
        let mut connections: *mut GList = ::core::ptr::null_mut::<GList>();
        let mut ll: *mut GList = ::core::ptr::null_mut::<GList>();
        let mut signal_variant: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        signal_variant = g_variant_ref_sink(g_variant_new(
            b"(sa{sv}as)\0" as *const u8 as *const gchar,
            b"org.freedesktop.portal.ProxyResolver\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut builder,
            &raw mut invalidated_builder,
        ));
        connections = g_dbus_interface_skeleton_get_connections(
            skeleton as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton,
        );
        ll = connections;
        while !ll.is_null() {
            let mut connection: *mut GDBusConnection = (*ll).data as *mut GDBusConnection;
            g_dbus_connection_emit_signal(
                connection,
                ::core::ptr::null::<gchar>(),
                g_dbus_interface_skeleton_get_object_path(
                    skeleton as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton,
                ),
                b"org.freedesktop.DBus.Properties\0" as *const u8 as *const gchar,
                b"PropertiesChanged\0" as *const u8 as *const gchar,
                signal_variant,
                ::core::ptr::null_mut::<*mut GError>(),
            );
            ll = (*ll).next;
        }
        g_variant_unref(signal_variant);
        g_list_free_full(
            connections,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    } else {
        g_variant_builder_clear(&raw mut builder);
        g_variant_builder_clear(&raw mut invalidated_builder);
    }
    g_list_free_full(
        (*(*skeleton).priv_0).changed_properties,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut ChangedProperty) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust__changed_property_free as unsafe extern "C" fn(*mut ChangedProperty) -> (),
        )),
    );
    (*(*skeleton).priv_0).changed_properties = ::core::ptr::null_mut::<GList>();
    (*(*skeleton).priv_0).changed_properties_idle_source = ::core::ptr::null_mut::<GSource>();
    g_mutex_unlock(&raw mut (*(*skeleton).priv_0).lock);
    return FALSE;
}
unsafe extern "C" fn safe_c2rust__gxdp_proxy_resolver_schedule_emit_changed(
    mut skeleton: *mut GXdpProxyResolverSkeleton,
    mut info: *const _ExtendedGDBusPropertyInfo,
    mut prop_id: guint,
    mut orig_value: *const GValue,
) {
    let mut cp: *mut ChangedProperty = ::core::ptr::null_mut::<ChangedProperty>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    cp = ::core::ptr::null_mut::<ChangedProperty>();
    l = (*(*skeleton).priv_0).changed_properties;
    while !l.is_null() {
        let mut i_cp: *mut ChangedProperty = (*l).data as *mut ChangedProperty;
        if (*i_cp).info == info {
            cp = i_cp;
            break;
        } else {
            l = (*l).next;
        }
    }
    if cp.is_null() {
        cp = ({
            let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
            let mut __s: gsize = ::core::mem::size_of::<ChangedProperty>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc0(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc0(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc0_n(__n, __s);
            }
            __p
        }) as *mut ChangedProperty;
        (*cp).prop_id = prop_id;
        (*cp).info = info;
        (*(*skeleton).priv_0).changed_properties =
            g_list_prepend((*(*skeleton).priv_0).changed_properties, cp as gpointer);
        g_value_init(
            &raw mut (*cp).orig_value,
            (*(orig_value as *mut GValue)).g_type,
        );
        g_value_copy(orig_value, &raw mut (*cp).orig_value);
    }
}
unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_skeleton_notify(
    mut object: *mut GObject,
    mut pspec: *mut GParamSpec,
) {
    let mut skeleton: *mut GXdpProxyResolverSkeleton =
        object as *mut ::core::ffi::c_void as *mut GXdpProxyResolverSkeleton;
    g_mutex_lock(&raw mut (*(*skeleton).priv_0).lock);
    if !(*(*skeleton).priv_0).changed_properties.is_null()
        && (*(*skeleton).priv_0)
            .changed_properties_idle_source
            .is_null()
    {
        (*(*skeleton).priv_0).changed_properties_idle_source = g_idle_source_new();
        g_source_set_priority(
            (*(*skeleton).priv_0).changed_properties_idle_source,
            G_PRIORITY_DEFAULT,
        );
        g_source_set_callback(
            (*(*skeleton).priv_0).changed_properties_idle_source,
            Some(
                safe_c2rust__gxdp_proxy_resolver_emit_changed
                    as unsafe extern "C" fn(gpointer) -> gboolean,
            ),
            g_object_ref(skeleton as gpointer) as *mut GXdpProxyResolverSkeleton as gpointer,
            ::core::mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> ()>, GDestroyNotify>(
                Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
            ),
        );
        g_source_set_name(
            (*(*skeleton).priv_0).changed_properties_idle_source,
            b"[generated] _gxdp_proxy_resolver_emit_changed\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        g_source_attach(
            (*(*skeleton).priv_0).changed_properties_idle_source,
            (*(*skeleton).priv_0).context,
        );
        g_source_unref((*(*skeleton).priv_0).changed_properties_idle_source);
    }
    g_mutex_unlock(&raw mut (*(*skeleton).priv_0).lock);
}
unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_skeleton_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut info: *const _ExtendedGDBusPropertyInfo =
        ::core::ptr::null::<_ExtendedGDBusPropertyInfo>();
    let mut skeleton: *mut GXdpProxyResolverSkeleton =
        object as *mut ::core::ffi::c_void as *mut GXdpProxyResolverSkeleton;
    if ({
        let mut _g_boolean_var_54: ::core::ffi::c_int = 0;
        if prop_id != 0 as guint && prop_id.wrapping_sub(1 as guint) < 1 as guint {
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
            b"gio/xdp-dbus.c\0" as *const u8 as *const ::core::ffi::c_char,
            7324 as ::core::ffi::c_int,
            G_STRFUNC,
            b"prop_id != 0 && prop_id - 1 < 1\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    info = safe_c2rust__gxdp_proxy_resolver_property_info_pointers
        [prop_id.wrapping_sub(1 as guint) as usize] as *const _ExtendedGDBusPropertyInfo;
    g_mutex_lock(&raw mut (*(*skeleton).priv_0).lock);
    g_object_freeze_notify(object);
    if safe_c2rust__g_value_equal(
        value,
        (*(*skeleton).priv_0)
            .properties
            .offset(prop_id.wrapping_sub(1 as guint) as isize) as *mut GValue,
    ) == 0
    {
        if !g_dbus_interface_skeleton_get_connection(
            skeleton as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton,
        )
        .is_null()
            && (*info).emits_changed_signal() as ::core::ffi::c_int != 0
        {
            safe_c2rust__gxdp_proxy_resolver_schedule_emit_changed(
                skeleton,
                info,
                prop_id,
                (*(*skeleton).priv_0)
                    .properties
                    .offset(prop_id.wrapping_sub(1 as guint) as isize)
                    as *mut GValue,
            );
        }
        g_value_copy(
            value,
            (*(*skeleton).priv_0)
                .properties
                .offset(prop_id.wrapping_sub(1 as guint) as isize) as *mut GValue,
        );
        g_object_notify_by_pspec(object, pspec);
    }
    g_mutex_unlock(&raw mut (*(*skeleton).priv_0).lock);
    g_object_thaw_notify(object);
}
unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_skeleton_init(
    mut skeleton: *mut GXdpProxyResolverSkeleton,
) {
    (*skeleton).priv_0 = safe_c2rust_gxdp_proxy_resolver_skeleton_get_instance_private(skeleton)
        as *mut GXdpProxyResolverSkeletonPrivate;
    g_mutex_init(&raw mut (*(*skeleton).priv_0).lock);
    (*(*skeleton).priv_0).context = g_main_context_ref_thread_default();
    (*(*skeleton).priv_0).properties = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GValue>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GValue;
    g_value_init(
        (*(*skeleton).priv_0)
            .properties
            .offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        G_TYPE_UINT,
    );
}
unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_skeleton_get_version(
    mut object: *mut GXdpProxyResolver,
) -> guint {
    let mut skeleton: *mut GXdpProxyResolverSkeleton =
        object as *mut ::core::ffi::c_void as *mut GXdpProxyResolverSkeleton;
    let mut value: guint = 0;
    g_mutex_lock(&raw mut (*(*skeleton).priv_0).lock);
    value = (*(*(*skeleton).priv_0)
        .properties
        .offset(0 as ::core::ffi::c_int as isize))
    .data[0 as ::core::ffi::c_int as usize]
        .v_uint;
    g_mutex_unlock(&raw mut (*(*skeleton).priv_0).lock);
    return value;
}
unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_skeleton_class_init(
    mut klass: *mut GXdpProxyResolverSkeletonClass,
) {
    let mut gobject_class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    let mut skeleton_class: *mut GDBusInterfaceSkeletonClass =
        ::core::ptr::null_mut::<GDBusInterfaceSkeletonClass>();
    gobject_class = klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize = Some(
        safe_c2rust_gxdp_proxy_resolver_skeleton_finalize
            as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_gxdp_proxy_resolver_skeleton_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_gxdp_proxy_resolver_skeleton_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).notify = Some(
        safe_c2rust_gxdp_proxy_resolver_skeleton_notify
            as unsafe extern "C" fn(*mut GObject, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, *mut GParamSpec) -> ()>;
    safe_c2rust_gxdp_proxy_resolver_override_properties(gobject_class, 1 as guint);
    skeleton_class = klass as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeletonClass;
    (*skeleton_class).get_info = Some(
        safe_c2rust_gxdp_proxy_resolver_skeleton_dbus_interface_get_info
            as unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GDBusInterfaceInfo,
    )
        as Option<unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GDBusInterfaceInfo>;
    (*skeleton_class).get_properties = Some(
        safe_c2rust_gxdp_proxy_resolver_skeleton_dbus_interface_get_properties
            as unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GVariant,
    )
        as Option<unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GVariant>;
    (*skeleton_class).flush = Some(
        safe_c2rust_gxdp_proxy_resolver_skeleton_dbus_interface_flush
            as unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> ()>;
    (*skeleton_class).get_vtable = Some(
        safe_c2rust_gxdp_proxy_resolver_skeleton_dbus_interface_get_vtable
            as unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GDBusInterfaceVTable,
    )
        as Option<unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GDBusInterfaceVTable>;
}
unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_skeleton_iface_init(
    mut iface: *mut GXdpProxyResolverIface,
) {
    (*iface).get_version = Some(
        safe_c2rust_gxdp_proxy_resolver_skeleton_get_version
            as unsafe extern "C" fn(*mut GXdpProxyResolver) -> guint,
    ) as Option<unsafe extern "C" fn(*mut GXdpProxyResolver) -> guint>;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_proxy_resolver_skeleton_new() -> *mut GXdpProxyResolver {
    return g_object_new(
        safe_c2rust_gxdp_proxy_resolver_skeleton_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GXdpProxyResolver;
}
static mut safe_c2rust__gxdp_trash_method_info_trash_file_IN_ARG_fd: _ExtendedGDBusArgInfo =
    _ExtendedGDBusArgInfo {
        parent_struct: _GDBusArgInfo {
            ref_count: -(1 as gint),
            name: b"fd\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            signature: b"h\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        use_gvariant: FALSE,
    };
static mut safe_c2rust__gxdp_trash_method_info_trash_file_IN_ARG_pointers: [*const GDBusArgInfo;
    2] = [::core::ptr::null::<GDBusArgInfo>(); 2];
static mut safe_c2rust__gxdp_trash_method_info_trash_file_OUT_ARG_result: _ExtendedGDBusArgInfo =
    _ExtendedGDBusArgInfo {
        parent_struct: _GDBusArgInfo {
            ref_count: -(1 as gint),
            name: b"result\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            signature: b"u\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        use_gvariant: FALSE,
    };
static mut safe_c2rust__gxdp_trash_method_info_trash_file_OUT_ARG_pointers: [*const GDBusArgInfo;
    2] = [::core::ptr::null::<GDBusArgInfo>(); 2];
static mut safe_c2rust__gxdp_trash_method_info_trash_file: _ExtendedGDBusMethodInfo = unsafe {
    _ExtendedGDBusMethodInfo {
        parent_struct: _GDBusMethodInfo {
            ref_count: -(1 as gint),
            name: b"TrashFile\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            in_args: &raw const safe_c2rust__gxdp_trash_method_info_trash_file_IN_ARG_pointers
                as *mut *mut GDBusArgInfo,
            out_args: &raw const safe_c2rust__gxdp_trash_method_info_trash_file_OUT_ARG_pointers
                as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"handle-trash-file\0" as *const u8 as *const gchar,
        pass_fdlist: TRUE,
    }
};
static mut safe_c2rust__gxdp_trash_method_info_pointers: [*const GDBusMethodInfo; 2] =
    [::core::ptr::null::<GDBusMethodInfo>(); 2];
static mut safe_c2rust__gxdp_trash_property_info_version: _ExtendedGDBusPropertyInfo =
    _ExtendedGDBusPropertyInfo {
        parent_struct: _GDBusPropertyInfo {
            ref_count: 0,
            name: ::core::ptr::null::<gchar>() as *mut gchar,
            signature: ::core::ptr::null::<gchar>() as *mut gchar,
            flags: G_DBUS_PROPERTY_INFO_FLAGS_NONE,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        hyphen_name: ::core::ptr::null::<gchar>(),
        use_gvariant_emits_changed_signal: [0; 1],
        c2rust_padding: [0; 7],
    };
static mut safe_c2rust__gxdp_trash_property_info_pointers: [*const GDBusPropertyInfo; 2] =
    [::core::ptr::null::<GDBusPropertyInfo>(); 2];
static mut safe_c2rust__gxdp_trash_interface_info: _ExtendedGDBusInterfaceInfo = unsafe {
    _ExtendedGDBusInterfaceInfo {
        parent_struct: _GDBusInterfaceInfo {
            ref_count: -(1 as gint),
            name: b"org.freedesktop.portal.Trash\0" as *const u8 as *const ::core::ffi::c_char
                as *mut gchar,
            methods: &raw const safe_c2rust__gxdp_trash_method_info_pointers
                as *mut *mut GDBusMethodInfo,
            signals: ::core::ptr::null::<*mut GDBusSignalInfo>() as *mut *mut GDBusSignalInfo,
            properties: &raw const safe_c2rust__gxdp_trash_property_info_pointers
                as *mut *mut GDBusPropertyInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        hyphen_name: b"trash\0" as *const u8 as *const gchar,
    }
};
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_trash_interface_info() -> *mut GDBusInterfaceInfo {
    return &raw const safe_c2rust__gxdp_trash_interface_info.parent_struct
        as *mut GDBusInterfaceInfo;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_trash_override_properties(
    mut klass: *mut GObjectClass,
    mut property_id_begin: guint,
) -> guint {
    let fresh18 = property_id_begin;
    property_id_begin = property_id_begin.wrapping_add(1);
    g_object_class_override_property(klass, fresh18, b"version\0" as *const u8 as *const gchar);
    return property_id_begin.wrapping_sub(1 as guint);
}
#[inline]
unsafe extern "C" fn safe_c2rust_gxdp_trash_method_marshal_trash_file(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: ::core::ffi::c_uint,
    mut param_values: *const GValue,
    mut invocation_hint: *mut ::core::ffi::c_void,
    mut marshal_data: *mut ::core::ffi::c_void,
) {
    safe_c2rust__g_dbus_codegen_marshal_BOOLEAN__OBJECT_OBJECT_VARIANT(
        closure,
        return_value,
        n_param_values,
        param_values,
        invocation_hint,
        marshal_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_trash_get_type() -> GType {
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
        let mut g_define_type_id: GType = g_type_register_static_simple(
            G_TYPE_INTERFACE,
            g_intern_static_string(b"GXdpTrash\0" as *const u8 as *const gchar),
            ::core::mem::size_of::<GXdpTrashInterface>() as guint,
            ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GXdpTrashIface) -> ()>,
                    Option<unsafe extern "C" fn() -> ()>,
                >(Some(
                    safe_c2rust_gxdp_trash_default_init
                        as unsafe extern "C" fn(*mut GXdpTrashIface) -> (),
                )),
            ),
            0 as guint,
            ::core::mem::transmute::<*mut ::core::ffi::c_void, GInstanceInitFunc>(NULL),
            G_TYPE_FLAG_NONE,
        );
        if ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType != G_TYPE_INVALID {
            g_type_interface_add_prerequisite(
                g_define_type_id,
                ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            );
        }
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
unsafe extern "C" fn safe_c2rust_gxdp_trash_default_init(mut iface: *mut GXdpTrashIface) {
    g_signal_new(
        b"handle-trash-file\0" as *const u8 as *const gchar,
        (*(iface as *mut GTypeInterface)).g_type,
        G_SIGNAL_RUN_LAST,
        16 as ::core::ffi::c_ulong as glong as guint,
        Some(
            g_signal_accumulator_true_handled
                as unsafe extern "C" fn(
                    *mut GSignalInvocationHint,
                    *mut GValue,
                    *const GValue,
                    gpointer,
                ) -> gboolean,
        ),
        NULL,
        Some(
            safe_c2rust_gxdp_trash_method_marshal_trash_file
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    ::core::ffi::c_uint,
                    *const GValue,
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        G_TYPE_BOOLEAN,
        3 as guint,
        g_dbus_method_invocation_get_type(),
        g_unix_fd_list_get_type(),
        G_TYPE_VARIANT,
    );
    g_object_interface_install_property(
        iface as gpointer,
        g_param_spec_uint(
            b"version\0" as *const u8 as *const gchar,
            b"version\0" as *const u8 as *const gchar,
            b"version\0" as *const u8 as *const gchar,
            0 as guint,
            G_MAXUINT32,
            0 as guint,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_trash_get_version(mut object: *mut GXdpTrash) -> guint {
    if ({
        let mut _g_boolean_var_55: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = object as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_gxdp_trash_get_type();
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
            b"GXDP_IS_TRASH (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    return (*(g_type_interface_peek(
        (*(object as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_gxdp_trash_get_type(),
    ) as *mut GXdpTrashIface))
        .get_version
        .expect("non-null function pointer")(object);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_trash_set_version(
    mut object: *mut GXdpTrash,
    mut value: guint,
) {
    g_object_set(
        object as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"version\0" as *const u8 as *const gchar,
        value,
        NULL,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_trash_call_trash_file(
    mut proxy: *mut GXdpTrash,
    mut arg_fd: *mut GVariant,
    mut fd_list: *mut GUnixFDList,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_dbus_proxy_call_with_unix_fd_list(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"TrashFile\0" as *const u8 as *const gchar,
        g_variant_new(b"(@h)\0" as *const u8 as *const gchar, arg_fd),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        fd_list,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_trash_call_trash_file_finish(
    mut proxy: *mut GXdpTrash,
    mut out_result: *mut guint,
    mut out_fd_list: *mut *mut GUnixFDList,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_with_unix_fd_list_finish(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        out_fd_list,
        res,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(_ret, b"(u)\0" as *const u8 as *const gchar, out_result);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_trash_call_trash_file_sync(
    mut proxy: *mut GXdpTrash,
    mut arg_fd: *mut GVariant,
    mut fd_list: *mut GUnixFDList,
    mut out_result: *mut guint,
    mut out_fd_list: *mut *mut GUnixFDList,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_with_unix_fd_list_sync(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"TrashFile\0" as *const u8 as *const gchar,
        g_variant_new(b"(@h)\0" as *const u8 as *const gchar, arg_fd),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        fd_list,
        out_fd_list,
        cancellable,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(_ret, b"(u)\0" as *const u8 as *const gchar, out_result);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_trash_complete_trash_file(
    mut object: *mut GXdpTrash,
    mut invocation: *mut GDBusMethodInvocation,
    mut fd_list: *mut GUnixFDList,
    mut result: guint,
) {
    g_dbus_method_invocation_return_value_with_unix_fd_list(
        invocation,
        g_variant_new(b"(u)\0" as *const u8 as *const gchar, result),
        fd_list,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_trash_proxy_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_gxdp_trash_proxy_get_type_once();
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
#[inline]
unsafe extern "C" fn safe_c2rust_gxdp_trash_proxy_get_instance_private(
    mut self_0: *mut GXdpTrashProxy,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GXdpTrashProxy_private_offset as glong as isize)
        as gpointer;
}
static mut safe_c2rust_GXdpTrashProxy_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_gxdp_trash_proxy_class_intern_init(mut klass: gpointer) {
    safe_c2rust_gxdp_trash_proxy_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GXdpTrashProxy_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GXdpTrashProxy_private_offset,
        );
    }
    safe_c2rust_gxdp_trash_proxy_class_init(klass as *mut GXdpTrashProxyClass);
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_gxdp_trash_proxy_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_dbus_proxy_get_type(),
        g_intern_static_string(b"GXdpTrashProxy\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GXdpTrashProxyClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_gxdp_trash_proxy_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GXdpTrashProxy>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GXdpTrashProxy) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_gxdp_trash_proxy_init
                    as unsafe extern "C" fn(*mut GXdpTrashProxy) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GXdpTrashProxy_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GXdpTrashProxyPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GXdpTrashIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_gxdp_trash_proxy_iface_init
                as unsafe extern "C" fn(*mut GXdpTrashIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        safe_c2rust_gxdp_trash_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
static mut safe_c2rust_gxdp_trash_proxy_parent_class: gpointer = NULL;
unsafe extern "C" fn safe_c2rust_gxdp_trash_proxy_finalize(mut object: *mut GObject) {
    let mut proxy: *mut GXdpTrashProxy = object as *mut ::core::ffi::c_void as *mut GXdpTrashProxy;
    g_datalist_clear(&raw mut (*(*proxy).priv_0).qdata);
    (*(safe_c2rust_gxdp_trash_proxy_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_gxdp_trash_proxy_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut info: *const _ExtendedGDBusPropertyInfo =
        ::core::ptr::null::<_ExtendedGDBusPropertyInfo>();
    let mut variant: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if ({
        let mut _g_boolean_var_56: ::core::ffi::c_int = 0;
        if prop_id != 0 as guint && prop_id.wrapping_sub(1 as guint) < 1 as guint {
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
            b"gio/xdp-dbus.c\0" as *const u8 as *const ::core::ffi::c_char,
            7836 as ::core::ffi::c_int,
            G_STRFUNC,
            b"prop_id != 0 && prop_id - 1 < 1\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    info = safe_c2rust__gxdp_trash_property_info_pointers[prop_id.wrapping_sub(1 as guint) as usize]
        as *const _ExtendedGDBusPropertyInfo;
    variant = g_dbus_proxy_get_cached_property(
        object as *mut ::core::ffi::c_void as *mut GDBusProxy,
        (*info).parent_struct.name,
    );
    if (*info).use_gvariant() != 0 {
        g_value_set_variant(value, variant);
    } else if !variant.is_null() {
        g_dbus_gvariant_to_gvalue(variant, value);
    }
    if !variant.is_null() {
        g_variant_unref(variant);
    }
}
unsafe extern "C" fn safe_c2rust_gxdp_trash_proxy_set_property_cb(
    mut proxy: *mut GDBusProxy,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut info: *const _ExtendedGDBusPropertyInfo =
        user_data as *const _ExtendedGDBusPropertyInfo;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    error = ::core::ptr::null_mut::<GError>();
    _ret = g_dbus_proxy_call_finish(proxy, res, &raw mut error);
    if _ret.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Error setting property '%s' on interface org.freedesktop.portal.Trash: %s (%s, %d)\0"
                as *const u8 as *const gchar,
            (*info).parent_struct.name,
            (*error).message,
            g_quark_to_string((*error).domain),
            (*error).code,
        );
        g_error_free(error);
    } else {
        g_variant_unref(_ret);
    };
}
unsafe extern "C" fn safe_c2rust_gxdp_trash_proxy_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut info: *const _ExtendedGDBusPropertyInfo =
        ::core::ptr::null::<_ExtendedGDBusPropertyInfo>();
    let mut variant: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if ({
        let mut _g_boolean_var_57: ::core::ffi::c_int = 0;
        if prop_id != 0 as guint && prop_id.wrapping_sub(1 as guint) < 1 as guint {
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
            b"gio/xdp-dbus.c\0" as *const u8 as *const ::core::ffi::c_char,
            7883 as ::core::ffi::c_int,
            G_STRFUNC,
            b"prop_id != 0 && prop_id - 1 < 1\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    info = safe_c2rust__gxdp_trash_property_info_pointers[prop_id.wrapping_sub(1 as guint) as usize]
        as *const _ExtendedGDBusPropertyInfo;
    variant = g_dbus_gvalue_to_gvariant(
        value,
        g_variant_type_checked_((*info).parent_struct.signature),
    );
    g_dbus_proxy_call(
        object as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"org.freedesktop.DBus.Properties.Set\0" as *const u8 as *const gchar,
        g_variant_new(
            b"(ssv)\0" as *const u8 as *const gchar,
            b"org.freedesktop.portal.Trash\0" as *const u8 as *const ::core::ffi::c_char,
            (*info).parent_struct.name,
            variant,
        ),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        ::core::ptr::null_mut::<GCancellable>(),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GDBusProxy, *mut GAsyncResult, gpointer) -> ()>,
            GAsyncReadyCallback,
        >(Some(
            safe_c2rust_gxdp_trash_proxy_set_property_cb
                as unsafe extern "C" fn(*mut GDBusProxy, *mut GAsyncResult, gpointer) -> (),
        )),
        &raw const (*info).parent_struct as *mut GDBusPropertyInfo as gpointer,
    );
    g_variant_unref(variant);
}
unsafe extern "C" fn safe_c2rust_gxdp_trash_proxy_g_signal(
    mut proxy: *mut GDBusProxy,
    mut sender_name: *const gchar,
    mut signal_name: *const gchar,
    mut parameters: *mut GVariant,
) {
    let mut info: *mut _ExtendedGDBusSignalInfo =
        ::core::ptr::null_mut::<_ExtendedGDBusSignalInfo>();
    let mut iter: GVariantIter = _GVariantIter { x: [0; 16] };
    let mut child: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut paramv: *mut GValue = ::core::ptr::null_mut::<GValue>();
    let mut num_params: gsize = 0;
    let mut n: gsize = 0;
    let mut signal_id: guint = 0;
    info = g_dbus_interface_info_lookup_signal(
        &raw const safe_c2rust__gxdp_trash_interface_info.parent_struct as *mut GDBusInterfaceInfo,
        signal_name,
    ) as *mut _ExtendedGDBusSignalInfo;
    if info.is_null() {
        return;
    }
    num_params = g_variant_n_children(parameters);
    paramv = ({
        let mut __n: gsize = num_params.wrapping_add(1 as gsize);
        let mut __s: gsize = ::core::mem::size_of::<GValue>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GValue;
    g_value_init(
        paramv.offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        safe_c2rust_gxdp_trash_get_type(),
    );
    g_value_set_object(
        paramv.offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        proxy as gpointer,
    );
    g_variant_iter_init(&raw mut iter, parameters);
    n = 1 as gsize;
    loop {
        child = g_variant_iter_next_value(&raw mut iter);
        if child.is_null() {
            break;
        }
        let mut arg_info: *mut _ExtendedGDBusArgInfo = *(*info)
            .parent_struct
            .args
            .offset(n.wrapping_sub(1 as gsize) as isize)
            as *mut _ExtendedGDBusArgInfo;
        if (*arg_info).use_gvariant != 0 {
            g_value_init(paramv.offset(n as isize) as *mut GValue, G_TYPE_VARIANT);
            g_value_set_variant(paramv.offset(n as isize) as *mut GValue, child);
            n = n.wrapping_add(1);
        } else {
            let fresh19 = n;
            n = n.wrapping_add(1);
            g_dbus_gvariant_to_gvalue(child, paramv.offset(fresh19 as isize) as *mut GValue);
        }
        g_variant_unref(child);
    }
    signal_id = g_signal_lookup((*info).signal_name, safe_c2rust_gxdp_trash_get_type());
    g_signal_emitv(
        paramv,
        signal_id,
        0 as GQuark,
        ::core::ptr::null_mut::<GValue>(),
    );
    n = 0 as gsize;
    while n < num_params.wrapping_add(1 as gsize) {
        g_value_unset(paramv.offset(n as isize) as *mut GValue);
        n = n.wrapping_add(1);
    }
    g_free(paramv as gpointer);
}
unsafe extern "C" fn safe_c2rust_gxdp_trash_proxy_g_properties_changed(
    mut _proxy: *mut GDBusProxy,
    mut changed_properties: *mut GVariant,
    mut invalidated_properties: *const *const gchar,
) {
    let mut proxy: *mut GXdpTrashProxy = _proxy as *mut ::core::ffi::c_void as *mut GXdpTrashProxy;
    let mut n: guint = 0;
    let mut key: *const gchar = ::core::ptr::null::<gchar>();
    let mut iter: *mut GVariantIter = ::core::ptr::null_mut::<GVariantIter>();
    let mut info: *mut _ExtendedGDBusPropertyInfo =
        ::core::ptr::null_mut::<_ExtendedGDBusPropertyInfo>();
    g_variant_get(
        changed_properties,
        b"a{sv}\0" as *const u8 as *const gchar,
        &raw mut iter,
    );
    while g_variant_iter_next(
        iter,
        b"{&sv}\0" as *const u8 as *const gchar,
        &raw mut key,
        NULL,
    ) != 0
    {
        info = g_dbus_interface_info_lookup_property(
            &raw const safe_c2rust__gxdp_trash_interface_info.parent_struct
                as *mut GDBusInterfaceInfo,
            key,
        ) as *mut _ExtendedGDBusPropertyInfo;
        g_datalist_id_set_data_full(
            &raw mut (*(*proxy).priv_0).qdata,
            g_quark_try_string(key),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
        );
        if !info.is_null() {
            g_object_notify(
                proxy as *mut ::core::ffi::c_void as *mut GObject,
                (*info).hyphen_name,
            );
        }
    }
    g_variant_iter_free(iter);
    n = 0 as guint;
    while !(*invalidated_properties.offset(n as isize)).is_null() {
        info = g_dbus_interface_info_lookup_property(
            &raw const safe_c2rust__gxdp_trash_interface_info.parent_struct
                as *mut GDBusInterfaceInfo,
            *invalidated_properties.offset(n as isize),
        ) as *mut _ExtendedGDBusPropertyInfo;
        g_datalist_id_set_data_full(
            &raw mut (*(*proxy).priv_0).qdata,
            g_quark_try_string(*invalidated_properties.offset(n as isize)),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
        );
        if !info.is_null() {
            g_object_notify(
                proxy as *mut ::core::ffi::c_void as *mut GObject,
                (*info).hyphen_name,
            );
        }
        n = n.wrapping_add(1);
    }
}
unsafe extern "C" fn safe_c2rust_gxdp_trash_proxy_get_version(mut object: *mut GXdpTrash) -> guint {
    let mut proxy: *mut GXdpTrashProxy = object as *mut ::core::ffi::c_void as *mut GXdpTrashProxy;
    let mut variant: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut value: guint = 0 as guint;
    variant = g_dbus_proxy_get_cached_property(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"version\0" as *const u8 as *const gchar,
    );
    if !variant.is_null() {
        value = g_variant_get_uint32(variant) as guint;
        g_variant_unref(variant);
    }
    return value;
}
unsafe extern "C" fn safe_c2rust_gxdp_trash_proxy_init(mut proxy: *mut GXdpTrashProxy) {
    (*proxy).priv_0 =
        safe_c2rust_gxdp_trash_proxy_get_instance_private(proxy) as *mut GXdpTrashProxyPrivate;
    g_dbus_proxy_set_interface_info(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        safe_c2rust_gxdp_trash_interface_info(),
    );
}
unsafe extern "C" fn safe_c2rust_gxdp_trash_proxy_class_init(mut klass: *mut GXdpTrashProxyClass) {
    let mut gobject_class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    let mut proxy_class: *mut GDBusProxyClass = ::core::ptr::null_mut::<GDBusProxyClass>();
    gobject_class = klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize =
        Some(safe_c2rust_gxdp_trash_proxy_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_gxdp_trash_proxy_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_gxdp_trash_proxy_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    proxy_class = klass as *mut ::core::ffi::c_void as *mut GDBusProxyClass;
    (*proxy_class).g_signal = Some(
        safe_c2rust_gxdp_trash_proxy_g_signal
            as unsafe extern "C" fn(
                *mut GDBusProxy,
                *const gchar,
                *const gchar,
                *mut GVariant,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(*mut GDBusProxy, *const gchar, *const gchar, *mut GVariant) -> (),
        >;
    (*proxy_class).g_properties_changed = Some(
        safe_c2rust_gxdp_trash_proxy_g_properties_changed
            as unsafe extern "C" fn(*mut GDBusProxy, *mut GVariant, *const *const gchar) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GDBusProxy, *mut GVariant, *const *const gchar) -> ()>;
    safe_c2rust_gxdp_trash_override_properties(gobject_class, 1 as guint);
}
unsafe extern "C" fn safe_c2rust_gxdp_trash_proxy_iface_init(mut iface: *mut GXdpTrashIface) {
    (*iface).get_version = Some(
        safe_c2rust_gxdp_trash_proxy_get_version as unsafe extern "C" fn(*mut GXdpTrash) -> guint,
    ) as Option<unsafe extern "C" fn(*mut GXdpTrash) -> guint>;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_trash_proxy_new(
    mut connection: *mut GDBusConnection,
    mut flags: GDBusProxyFlags,
    mut name: *const gchar,
    mut object_path: *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_async_initable_new_async(
        safe_c2rust_gxdp_trash_proxy_get_type(),
        G_PRIORITY_DEFAULT,
        cancellable,
        callback,
        user_data,
        b"g-flags\0" as *const u8 as *const gchar,
        flags as ::core::ffi::c_uint,
        b"g-name\0" as *const u8 as *const ::core::ffi::c_char,
        name,
        b"g-connection\0" as *const u8 as *const ::core::ffi::c_char,
        connection,
        b"g-object-path\0" as *const u8 as *const ::core::ffi::c_char,
        object_path,
        b"g-interface-name\0" as *const u8 as *const ::core::ffi::c_char,
        b"org.freedesktop.portal.Trash\0" as *const u8 as *const ::core::ffi::c_char,
        NULL,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_trash_proxy_new_finish(
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GXdpTrash {
    let mut ret: *mut GObject = ::core::ptr::null_mut::<GObject>();
    let mut source_object: *mut GObject = ::core::ptr::null_mut::<GObject>();
    source_object = g_async_result_get_source_object(res);
    ret = g_async_initable_new_finish(
        source_object as *mut ::core::ffi::c_void as *mut GAsyncInitable,
        res,
        error,
    );
    g_object_unref(source_object as gpointer);
    if !ret.is_null() {
        return ret as *mut ::core::ffi::c_void as *mut GXdpTrash;
    } else {
        return ::core::ptr::null_mut::<GXdpTrash>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_trash_proxy_new_sync(
    mut connection: *mut GDBusConnection,
    mut flags: GDBusProxyFlags,
    mut name: *const gchar,
    mut object_path: *const gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GXdpTrash {
    let mut ret: *mut GInitable = ::core::ptr::null_mut::<GInitable>();
    ret = g_initable_new(
        safe_c2rust_gxdp_trash_proxy_get_type(),
        cancellable,
        error,
        b"g-flags\0" as *const u8 as *const gchar,
        flags as ::core::ffi::c_uint,
        b"g-name\0" as *const u8 as *const ::core::ffi::c_char,
        name,
        b"g-connection\0" as *const u8 as *const ::core::ffi::c_char,
        connection,
        b"g-object-path\0" as *const u8 as *const ::core::ffi::c_char,
        object_path,
        b"g-interface-name\0" as *const u8 as *const ::core::ffi::c_char,
        b"org.freedesktop.portal.Trash\0" as *const u8 as *const ::core::ffi::c_char,
        NULL,
    ) as *mut GInitable;
    if !ret.is_null() {
        return ret as *mut ::core::ffi::c_void as *mut GXdpTrash;
    } else {
        return ::core::ptr::null_mut::<GXdpTrash>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_trash_proxy_new_for_bus(
    mut bus_type: GBusType,
    mut flags: GDBusProxyFlags,
    mut name: *const gchar,
    mut object_path: *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_async_initable_new_async(
        safe_c2rust_gxdp_trash_proxy_get_type(),
        G_PRIORITY_DEFAULT,
        cancellable,
        callback,
        user_data,
        b"g-flags\0" as *const u8 as *const gchar,
        flags as ::core::ffi::c_uint,
        b"g-name\0" as *const u8 as *const ::core::ffi::c_char,
        name,
        b"g-bus-type\0" as *const u8 as *const ::core::ffi::c_char,
        bus_type as ::core::ffi::c_int,
        b"g-object-path\0" as *const u8 as *const ::core::ffi::c_char,
        object_path,
        b"g-interface-name\0" as *const u8 as *const ::core::ffi::c_char,
        b"org.freedesktop.portal.Trash\0" as *const u8 as *const ::core::ffi::c_char,
        NULL,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_trash_proxy_new_for_bus_finish(
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GXdpTrash {
    let mut ret: *mut GObject = ::core::ptr::null_mut::<GObject>();
    let mut source_object: *mut GObject = ::core::ptr::null_mut::<GObject>();
    source_object = g_async_result_get_source_object(res);
    ret = g_async_initable_new_finish(
        source_object as *mut ::core::ffi::c_void as *mut GAsyncInitable,
        res,
        error,
    );
    g_object_unref(source_object as gpointer);
    if !ret.is_null() {
        return ret as *mut ::core::ffi::c_void as *mut GXdpTrash;
    } else {
        return ::core::ptr::null_mut::<GXdpTrash>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_trash_proxy_new_for_bus_sync(
    mut bus_type: GBusType,
    mut flags: GDBusProxyFlags,
    mut name: *const gchar,
    mut object_path: *const gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GXdpTrash {
    let mut ret: *mut GInitable = ::core::ptr::null_mut::<GInitable>();
    ret = g_initable_new(
        safe_c2rust_gxdp_trash_proxy_get_type(),
        cancellable,
        error,
        b"g-flags\0" as *const u8 as *const gchar,
        flags as ::core::ffi::c_uint,
        b"g-name\0" as *const u8 as *const ::core::ffi::c_char,
        name,
        b"g-bus-type\0" as *const u8 as *const ::core::ffi::c_char,
        bus_type as ::core::ffi::c_int,
        b"g-object-path\0" as *const u8 as *const ::core::ffi::c_char,
        object_path,
        b"g-interface-name\0" as *const u8 as *const ::core::ffi::c_char,
        b"org.freedesktop.portal.Trash\0" as *const u8 as *const ::core::ffi::c_char,
        NULL,
    ) as *mut GInitable;
    if !ret.is_null() {
        return ret as *mut ::core::ffi::c_void as *mut GXdpTrash;
    } else {
        return ::core::ptr::null_mut::<GXdpTrash>();
    };
}
unsafe extern "C" fn safe_c2rust__gxdp_trash_skeleton_handle_method_call(
    mut connection: *mut GDBusConnection,
    mut sender: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut method_name: *const gchar,
    mut parameters: *mut GVariant,
    mut invocation: *mut GDBusMethodInvocation,
    mut user_data: gpointer,
) {
    let mut skeleton: *mut GXdpTrashSkeleton = user_data as *mut GXdpTrashSkeleton;
    let mut info: *mut _ExtendedGDBusMethodInfo =
        ::core::ptr::null_mut::<_ExtendedGDBusMethodInfo>();
    let mut iter: GVariantIter = _GVariantIter { x: [0; 16] };
    let mut child: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut paramv: *mut GValue = ::core::ptr::null_mut::<GValue>();
    let mut num_params: gsize = 0;
    let mut num_extra: guint = 0;
    let mut n: gsize = 0;
    let mut signal_id: guint = 0;
    let mut return_value: GValue = _GValue {
        g_type: 0 as GType,
        data: [
            C2RustUnnamed_1 {
                v_int: 0 as ::core::ffi::c_int,
            },
            C2RustUnnamed_1 { v_int: 0 },
        ],
    };
    info = g_dbus_method_invocation_get_method_info(invocation) as *mut _ExtendedGDBusMethodInfo;
    if ({
        let mut _g_boolean_var_58: ::core::ffi::c_int = 0;
        if !info.is_null() {
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
            b"gio/xdp-dbus.c\0" as *const u8 as *const ::core::ffi::c_char,
            8246 as ::core::ffi::c_int,
            G_STRFUNC,
            b"info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    num_params = g_variant_n_children(parameters);
    num_extra = (if (*info).pass_fdlist != 0 {
        3 as ::core::ffi::c_int
    } else {
        2 as ::core::ffi::c_int
    }) as guint;
    paramv = ({
        let mut __n: gsize = num_params.wrapping_add(num_extra as gsize);
        let mut __s: gsize = ::core::mem::size_of::<GValue>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GValue;
    n = 0 as gsize;
    g_value_init(
        paramv.offset(n as isize) as *mut GValue,
        safe_c2rust_gxdp_trash_get_type(),
    );
    let fresh20 = n;
    n = n.wrapping_add(1);
    g_value_set_object(
        paramv.offset(fresh20 as isize) as *mut GValue,
        skeleton as gpointer,
    );
    g_value_init(
        paramv.offset(n as isize) as *mut GValue,
        g_dbus_method_invocation_get_type(),
    );
    let fresh21 = n;
    n = n.wrapping_add(1);
    g_value_set_object(
        paramv.offset(fresh21 as isize) as *mut GValue,
        invocation as gpointer,
    );
    if (*info).pass_fdlist != 0 {
        g_value_init(
            paramv.offset(n as isize) as *mut GValue,
            g_unix_fd_list_get_type(),
        );
        let fresh22 = n;
        n = n.wrapping_add(1);
        g_value_set_object(
            paramv.offset(fresh22 as isize) as *mut GValue,
            g_dbus_message_get_unix_fd_list(g_dbus_method_invocation_get_message(invocation))
                as gpointer,
        );
    }
    g_variant_iter_init(&raw mut iter, parameters);
    loop {
        child = g_variant_iter_next_value(&raw mut iter);
        if child.is_null() {
            break;
        }
        let mut arg_info: *mut _ExtendedGDBusArgInfo = *(*info)
            .parent_struct
            .in_args
            .offset(n.wrapping_sub(num_extra as gsize) as isize)
            as *mut _ExtendedGDBusArgInfo;
        if (*arg_info).use_gvariant != 0 {
            g_value_init(paramv.offset(n as isize) as *mut GValue, G_TYPE_VARIANT);
            g_value_set_variant(paramv.offset(n as isize) as *mut GValue, child);
            n = n.wrapping_add(1);
        } else {
            let fresh23 = n;
            n = n.wrapping_add(1);
            g_dbus_gvariant_to_gvalue(child, paramv.offset(fresh23 as isize) as *mut GValue);
        }
        g_variant_unref(child);
    }
    signal_id = g_signal_lookup((*info).signal_name, safe_c2rust_gxdp_trash_get_type());
    g_value_init(&raw mut return_value, G_TYPE_BOOLEAN);
    g_signal_emitv(paramv, signal_id, 0 as GQuark, &raw mut return_value);
    if g_value_get_boolean(&raw mut return_value) == 0 {
        g_dbus_method_invocation_return_error(
            invocation,
            g_dbus_error_quark(),
            G_DBUS_ERROR_UNKNOWN_METHOD as ::core::ffi::c_int as gint,
            b"Method %s is not implemented on interface %s\0" as *const u8 as *const gchar,
            method_name,
            interface_name,
        );
    }
    g_value_unset(&raw mut return_value);
    n = 0 as gsize;
    while n < num_params.wrapping_add(num_extra as gsize) {
        g_value_unset(paramv.offset(n as isize) as *mut GValue);
        n = n.wrapping_add(1);
    }
    g_free(paramv as gpointer);
}
unsafe extern "C" fn safe_c2rust__gxdp_trash_skeleton_handle_get_property(
    mut connection: *mut GDBusConnection,
    mut sender: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut property_name: *const gchar,
    mut error: *mut *mut GError,
    mut user_data: gpointer,
) -> *mut GVariant {
    let mut skeleton: *mut GXdpTrashSkeleton = user_data as *mut GXdpTrashSkeleton;
    let mut value: GValue = _GValue {
        g_type: 0 as GType,
        data: [
            C2RustUnnamed_1 {
                v_int: 0 as ::core::ffi::c_int,
            },
            C2RustUnnamed_1 { v_int: 0 },
        ],
    };
    let mut pspec: *mut GParamSpec = ::core::ptr::null_mut::<GParamSpec>();
    let mut info: *mut _ExtendedGDBusPropertyInfo =
        ::core::ptr::null_mut::<_ExtendedGDBusPropertyInfo>();
    let mut ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    ret = ::core::ptr::null_mut::<GVariant>();
    info = g_dbus_interface_info_lookup_property(
        &raw const safe_c2rust__gxdp_trash_interface_info.parent_struct as *mut GDBusInterfaceInfo,
        property_name,
    ) as *mut _ExtendedGDBusPropertyInfo;
    if ({
        let mut _g_boolean_var_59: ::core::ffi::c_int = 0;
        if !info.is_null() {
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
            b"gio/xdp-dbus.c\0" as *const u8 as *const ::core::ffi::c_char,
            8305 as ::core::ffi::c_int,
            G_STRFUNC,
            b"info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    pspec = g_object_class_find_property(
        (*(skeleton as *mut GTypeInstance)).g_class as *mut GObjectClass,
        (*info).hyphen_name,
    );
    if pspec.is_null() {
        g_set_error(
            error,
            g_dbus_error_quark(),
            G_DBUS_ERROR_INVALID_ARGS as ::core::ffi::c_int as gint,
            b"No property with name %s\0" as *const u8 as *const gchar,
            property_name,
        );
    } else {
        g_value_init(&raw mut value, (*pspec).value_type);
        g_object_get_property(
            skeleton as *mut ::core::ffi::c_void as *mut GObject,
            (*info).hyphen_name,
            &raw mut value,
        );
        ret = g_dbus_gvalue_to_gvariant(
            &raw mut value,
            g_variant_type_checked_((*info).parent_struct.signature),
        );
        g_value_unset(&raw mut value);
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust__gxdp_trash_skeleton_handle_set_property(
    mut connection: *mut GDBusConnection,
    mut sender: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut property_name: *const gchar,
    mut variant: *mut GVariant,
    mut error: *mut *mut GError,
    mut user_data: gpointer,
) -> gboolean {
    let mut skeleton: *mut GXdpTrashSkeleton = user_data as *mut GXdpTrashSkeleton;
    let mut value: GValue = _GValue {
        g_type: 0 as GType,
        data: [
            C2RustUnnamed_1 {
                v_int: 0 as ::core::ffi::c_int,
            },
            C2RustUnnamed_1 { v_int: 0 },
        ],
    };
    let mut pspec: *mut GParamSpec = ::core::ptr::null_mut::<GParamSpec>();
    let mut info: *mut _ExtendedGDBusPropertyInfo =
        ::core::ptr::null_mut::<_ExtendedGDBusPropertyInfo>();
    let mut ret: gboolean = 0;
    ret = FALSE as gboolean;
    info = g_dbus_interface_info_lookup_property(
        &raw const safe_c2rust__gxdp_trash_interface_info.parent_struct as *mut GDBusInterfaceInfo,
        property_name,
    ) as *mut _ExtendedGDBusPropertyInfo;
    if ({
        let mut _g_boolean_var_60: ::core::ffi::c_int = 0;
        if !info.is_null() {
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
            b"gio/xdp-dbus.c\0" as *const u8 as *const ::core::ffi::c_char,
            8339 as ::core::ffi::c_int,
            G_STRFUNC,
            b"info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    pspec = g_object_class_find_property(
        (*(skeleton as *mut GTypeInstance)).g_class as *mut GObjectClass,
        (*info).hyphen_name,
    );
    if pspec.is_null() {
        g_set_error(
            error,
            g_dbus_error_quark(),
            G_DBUS_ERROR_INVALID_ARGS as ::core::ffi::c_int as gint,
            b"No property with name %s\0" as *const u8 as *const gchar,
            property_name,
        );
    } else {
        if (*info).use_gvariant() != 0 {
            g_value_set_variant(&raw mut value, variant);
        } else {
            g_dbus_gvariant_to_gvalue(variant, &raw mut value);
        }
        g_object_set_property(
            skeleton as *mut ::core::ffi::c_void as *mut GObject,
            (*info).hyphen_name,
            &raw mut value,
        );
        g_value_unset(&raw mut value);
        ret = TRUE as gboolean;
    }
    return ret;
}
static mut safe_c2rust__gxdp_trash_skeleton_vtable: GDBusInterfaceVTable = unsafe {
    _GDBusInterfaceVTable {
        method_call: Some(
            safe_c2rust__gxdp_trash_skeleton_handle_method_call
                as unsafe extern "C" fn(
                    *mut GDBusConnection,
                    *const gchar,
                    *const gchar,
                    *const gchar,
                    *const gchar,
                    *mut GVariant,
                    *mut GDBusMethodInvocation,
                    gpointer,
                ) -> (),
        ),
        get_property: Some(
            safe_c2rust__gxdp_trash_skeleton_handle_get_property
                as unsafe extern "C" fn(
                    *mut GDBusConnection,
                    *const gchar,
                    *const gchar,
                    *const gchar,
                    *const gchar,
                    *mut *mut GError,
                    gpointer,
                ) -> *mut GVariant,
        ),
        set_property: Some(
            safe_c2rust__gxdp_trash_skeleton_handle_set_property
                as unsafe extern "C" fn(
                    *mut GDBusConnection,
                    *const gchar,
                    *const gchar,
                    *const gchar,
                    *const gchar,
                    *mut GVariant,
                    *mut *mut GError,
                    gpointer,
                ) -> gboolean,
        ),
        padding: [
            NULL,
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        ],
    }
};
unsafe extern "C" fn safe_c2rust_gxdp_trash_skeleton_dbus_interface_get_info(
    mut skeleton: *mut GDBusInterfaceSkeleton,
) -> *mut GDBusInterfaceInfo {
    return safe_c2rust_gxdp_trash_interface_info();
}
unsafe extern "C" fn safe_c2rust_gxdp_trash_skeleton_dbus_interface_get_vtable(
    mut skeleton: *mut GDBusInterfaceSkeleton,
) -> *mut GDBusInterfaceVTable {
    return &raw const safe_c2rust__gxdp_trash_skeleton_vtable as *mut GDBusInterfaceVTable;
}
unsafe extern "C" fn safe_c2rust_gxdp_trash_skeleton_dbus_interface_get_properties(
    mut _skeleton: *mut GDBusInterfaceSkeleton,
) -> *mut GVariant {
    let mut skeleton: *mut GXdpTrashSkeleton =
        _skeleton as *mut ::core::ffi::c_void as *mut GXdpTrashSkeleton;
    let mut builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut n: guint = 0;
    g_variant_builder_init(
        &raw mut builder,
        g_variant_type_checked_(b"a{sv}\0" as *const u8 as *const gchar),
    );
    if !safe_c2rust__gxdp_trash_interface_info
        .parent_struct
        .properties
        .is_null()
    {
        n = 0 as guint;
        while !(*safe_c2rust__gxdp_trash_interface_info
            .parent_struct
            .properties
            .offset(n as isize))
        .is_null()
        {
            let mut info: *mut GDBusPropertyInfo = *safe_c2rust__gxdp_trash_interface_info
                .parent_struct
                .properties
                .offset(n as isize);
            if (*info).flags as ::core::ffi::c_uint
                & G_DBUS_PROPERTY_INFO_FLAGS_READABLE as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
            {
                let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
                value = safe_c2rust__gxdp_trash_skeleton_handle_get_property(
                    g_dbus_interface_skeleton_get_connection(
                        skeleton as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton,
                    ),
                    ::core::ptr::null::<gchar>(),
                    g_dbus_interface_skeleton_get_object_path(
                        skeleton as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton,
                    ),
                    b"org.freedesktop.portal.Trash\0" as *const u8 as *const gchar,
                    (*info).name,
                    ::core::ptr::null_mut::<*mut GError>(),
                    skeleton as gpointer,
                );
                if !value.is_null() {
                    g_variant_take_ref(value);
                    g_variant_builder_add(
                        &raw mut builder,
                        b"{sv}\0" as *const u8 as *const gchar,
                        (*info).name,
                        value,
                    );
                    g_variant_unref(value);
                }
            }
            n = n.wrapping_add(1);
        }
    }
    return g_variant_builder_end(&raw mut builder);
}
unsafe extern "C" fn safe_c2rust_gxdp_trash_skeleton_dbus_interface_flush(
    mut _skeleton: *mut GDBusInterfaceSkeleton,
) {
    let mut skeleton: *mut GXdpTrashSkeleton =
        _skeleton as *mut ::core::ffi::c_void as *mut GXdpTrashSkeleton;
    let mut emit_changed: gboolean = FALSE;
    g_mutex_lock(&raw mut (*(*skeleton).priv_0).lock);
    if !(*(*skeleton).priv_0)
        .changed_properties_idle_source
        .is_null()
    {
        g_source_destroy((*(*skeleton).priv_0).changed_properties_idle_source);
        (*(*skeleton).priv_0).changed_properties_idle_source = ::core::ptr::null_mut::<GSource>();
        emit_changed = TRUE as gboolean;
    }
    g_mutex_unlock(&raw mut (*(*skeleton).priv_0).lock);
    if emit_changed != 0 {
        safe_c2rust__gxdp_trash_emit_changed(skeleton as gpointer);
    }
}
static mut safe_c2rust_gxdp_trash_skeleton_parent_class: gpointer = NULL;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_trash_skeleton_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_gxdp_trash_skeleton_get_type_once();
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
unsafe extern "C" fn safe_c2rust_gxdp_trash_skeleton_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_dbus_interface_skeleton_get_type(),
        g_intern_static_string(b"GXdpTrashSkeleton\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GXdpTrashSkeletonClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_gxdp_trash_skeleton_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GXdpTrashSkeleton>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GXdpTrashSkeleton) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_gxdp_trash_skeleton_init
                    as unsafe extern "C" fn(*mut GXdpTrashSkeleton) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GXdpTrashSkeleton_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GXdpTrashSkeletonPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GXdpTrashIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_gxdp_trash_skeleton_iface_init
                as unsafe extern "C" fn(*mut GXdpTrashIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        safe_c2rust_gxdp_trash_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
#[inline]
unsafe extern "C" fn safe_c2rust_gxdp_trash_skeleton_get_instance_private(
    mut self_0: *mut GXdpTrashSkeleton,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GXdpTrashSkeleton_private_offset as glong as isize)
        as gpointer;
}
static mut safe_c2rust_GXdpTrashSkeleton_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_gxdp_trash_skeleton_class_intern_init(mut klass: gpointer) {
    safe_c2rust_gxdp_trash_skeleton_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GXdpTrashSkeleton_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GXdpTrashSkeleton_private_offset,
        );
    }
    safe_c2rust_gxdp_trash_skeleton_class_init(klass as *mut GXdpTrashSkeletonClass);
}
unsafe extern "C" fn safe_c2rust_gxdp_trash_skeleton_finalize(mut object: *mut GObject) {
    let mut skeleton: *mut GXdpTrashSkeleton =
        object as *mut ::core::ffi::c_void as *mut GXdpTrashSkeleton;
    let mut n: guint = 0;
    n = 0 as guint;
    while n < 1 as guint {
        g_value_unset((*(*skeleton).priv_0).properties.offset(n as isize) as *mut GValue);
        n = n.wrapping_add(1);
    }
    g_free((*(*skeleton).priv_0).properties as gpointer);
    g_list_free_full(
        (*(*skeleton).priv_0).changed_properties,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut ChangedProperty) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust__changed_property_free as unsafe extern "C" fn(*mut ChangedProperty) -> (),
        )),
    );
    if !(*(*skeleton).priv_0)
        .changed_properties_idle_source
        .is_null()
    {
        g_source_destroy((*(*skeleton).priv_0).changed_properties_idle_source);
    }
    g_main_context_unref((*(*skeleton).priv_0).context);
    g_mutex_clear(&raw mut (*(*skeleton).priv_0).lock);
    (*(safe_c2rust_gxdp_trash_skeleton_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_gxdp_trash_skeleton_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut skeleton: *mut GXdpTrashSkeleton =
        object as *mut ::core::ffi::c_void as *mut GXdpTrashSkeleton;
    if ({
        let mut _g_boolean_var_61: ::core::ffi::c_int = 0;
        if prop_id != 0 as guint && prop_id.wrapping_sub(1 as guint) < 1 as guint {
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
            b"gio/xdp-dbus.c\0" as *const u8 as *const ::core::ffi::c_char,
            8462 as ::core::ffi::c_int,
            G_STRFUNC,
            b"prop_id != 0 && prop_id - 1 < 1\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_mutex_lock(&raw mut (*(*skeleton).priv_0).lock);
    g_value_copy(
        (*(*skeleton).priv_0)
            .properties
            .offset(prop_id.wrapping_sub(1 as guint) as isize) as *mut GValue,
        value,
    );
    g_mutex_unlock(&raw mut (*(*skeleton).priv_0).lock);
}
unsafe extern "C" fn safe_c2rust__gxdp_trash_emit_changed(mut user_data: gpointer) -> gboolean {
    let mut skeleton: *mut GXdpTrashSkeleton = user_data as *mut GXdpTrashSkeleton;
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut invalidated_builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut num_changes: guint = 0;
    g_mutex_lock(&raw mut (*(*skeleton).priv_0).lock);
    g_variant_builder_init(
        &raw mut builder,
        g_variant_type_checked_(b"a{sv}\0" as *const u8 as *const gchar),
    );
    g_variant_builder_init(
        &raw mut invalidated_builder,
        g_variant_type_checked_(b"as\0" as *const u8 as *const gchar),
    );
    l = (*(*skeleton).priv_0).changed_properties;
    num_changes = 0 as guint;
    while !l.is_null() {
        let mut cp: *mut ChangedProperty = (*l).data as *mut ChangedProperty;
        let mut variant: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        let mut cur_value: *const GValue = ::core::ptr::null::<GValue>();
        cur_value = (*(*skeleton).priv_0)
            .properties
            .offset((*cp).prop_id.wrapping_sub(1 as guint) as isize)
            as *mut GValue;
        if safe_c2rust__g_value_equal(cur_value, &raw mut (*cp).orig_value) == 0 {
            variant = g_dbus_gvalue_to_gvariant(
                cur_value,
                g_variant_type_checked_((*(*cp).info).parent_struct.signature),
            );
            g_variant_builder_add(
                &raw mut builder,
                b"{sv}\0" as *const u8 as *const gchar,
                (*(*cp).info).parent_struct.name,
                variant,
            );
            g_variant_unref(variant);
            num_changes = num_changes.wrapping_add(1);
        }
        l = (*l).next;
    }
    if num_changes > 0 as guint {
        let mut connections: *mut GList = ::core::ptr::null_mut::<GList>();
        let mut ll: *mut GList = ::core::ptr::null_mut::<GList>();
        let mut signal_variant: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        signal_variant = g_variant_ref_sink(g_variant_new(
            b"(sa{sv}as)\0" as *const u8 as *const gchar,
            b"org.freedesktop.portal.Trash\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut builder,
            &raw mut invalidated_builder,
        ));
        connections = g_dbus_interface_skeleton_get_connections(
            skeleton as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton,
        );
        ll = connections;
        while !ll.is_null() {
            let mut connection: *mut GDBusConnection = (*ll).data as *mut GDBusConnection;
            g_dbus_connection_emit_signal(
                connection,
                ::core::ptr::null::<gchar>(),
                g_dbus_interface_skeleton_get_object_path(
                    skeleton as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton,
                ),
                b"org.freedesktop.DBus.Properties\0" as *const u8 as *const gchar,
                b"PropertiesChanged\0" as *const u8 as *const gchar,
                signal_variant,
                ::core::ptr::null_mut::<*mut GError>(),
            );
            ll = (*ll).next;
        }
        g_variant_unref(signal_variant);
        g_list_free_full(
            connections,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    } else {
        g_variant_builder_clear(&raw mut builder);
        g_variant_builder_clear(&raw mut invalidated_builder);
    }
    g_list_free_full(
        (*(*skeleton).priv_0).changed_properties,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut ChangedProperty) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust__changed_property_free as unsafe extern "C" fn(*mut ChangedProperty) -> (),
        )),
    );
    (*(*skeleton).priv_0).changed_properties = ::core::ptr::null_mut::<GList>();
    (*(*skeleton).priv_0).changed_properties_idle_source = ::core::ptr::null_mut::<GSource>();
    g_mutex_unlock(&raw mut (*(*skeleton).priv_0).lock);
    return FALSE;
}
unsafe extern "C" fn safe_c2rust__gxdp_trash_schedule_emit_changed(
    mut skeleton: *mut GXdpTrashSkeleton,
    mut info: *const _ExtendedGDBusPropertyInfo,
    mut prop_id: guint,
    mut orig_value: *const GValue,
) {
    let mut cp: *mut ChangedProperty = ::core::ptr::null_mut::<ChangedProperty>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    cp = ::core::ptr::null_mut::<ChangedProperty>();
    l = (*(*skeleton).priv_0).changed_properties;
    while !l.is_null() {
        let mut i_cp: *mut ChangedProperty = (*l).data as *mut ChangedProperty;
        if (*i_cp).info == info {
            cp = i_cp;
            break;
        } else {
            l = (*l).next;
        }
    }
    if cp.is_null() {
        cp = ({
            let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
            let mut __s: gsize = ::core::mem::size_of::<ChangedProperty>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc0(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc0(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc0_n(__n, __s);
            }
            __p
        }) as *mut ChangedProperty;
        (*cp).prop_id = prop_id;
        (*cp).info = info;
        (*(*skeleton).priv_0).changed_properties =
            g_list_prepend((*(*skeleton).priv_0).changed_properties, cp as gpointer);
        g_value_init(
            &raw mut (*cp).orig_value,
            (*(orig_value as *mut GValue)).g_type,
        );
        g_value_copy(orig_value, &raw mut (*cp).orig_value);
    }
}
unsafe extern "C" fn safe_c2rust_gxdp_trash_skeleton_notify(
    mut object: *mut GObject,
    mut pspec: *mut GParamSpec,
) {
    let mut skeleton: *mut GXdpTrashSkeleton =
        object as *mut ::core::ffi::c_void as *mut GXdpTrashSkeleton;
    g_mutex_lock(&raw mut (*(*skeleton).priv_0).lock);
    if !(*(*skeleton).priv_0).changed_properties.is_null()
        && (*(*skeleton).priv_0)
            .changed_properties_idle_source
            .is_null()
    {
        (*(*skeleton).priv_0).changed_properties_idle_source = g_idle_source_new();
        g_source_set_priority(
            (*(*skeleton).priv_0).changed_properties_idle_source,
            G_PRIORITY_DEFAULT,
        );
        g_source_set_callback(
            (*(*skeleton).priv_0).changed_properties_idle_source,
            Some(
                safe_c2rust__gxdp_trash_emit_changed as unsafe extern "C" fn(gpointer) -> gboolean,
            ),
            g_object_ref(skeleton as gpointer) as *mut GXdpTrashSkeleton as gpointer,
            ::core::mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> ()>, GDestroyNotify>(
                Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
            ),
        );
        g_source_set_name(
            (*(*skeleton).priv_0).changed_properties_idle_source,
            b"[generated] _gxdp_trash_emit_changed\0" as *const u8 as *const ::core::ffi::c_char,
        );
        g_source_attach(
            (*(*skeleton).priv_0).changed_properties_idle_source,
            (*(*skeleton).priv_0).context,
        );
        g_source_unref((*(*skeleton).priv_0).changed_properties_idle_source);
    }
    g_mutex_unlock(&raw mut (*(*skeleton).priv_0).lock);
}
unsafe extern "C" fn safe_c2rust_gxdp_trash_skeleton_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut info: *const _ExtendedGDBusPropertyInfo =
        ::core::ptr::null::<_ExtendedGDBusPropertyInfo>();
    let mut skeleton: *mut GXdpTrashSkeleton =
        object as *mut ::core::ffi::c_void as *mut GXdpTrashSkeleton;
    if ({
        let mut _g_boolean_var_62: ::core::ffi::c_int = 0;
        if prop_id != 0 as guint && prop_id.wrapping_sub(1 as guint) < 1 as guint {
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
            b"gio/xdp-dbus.c\0" as *const u8 as *const ::core::ffi::c_char,
            8581 as ::core::ffi::c_int,
            G_STRFUNC,
            b"prop_id != 0 && prop_id - 1 < 1\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    info = safe_c2rust__gxdp_trash_property_info_pointers[prop_id.wrapping_sub(1 as guint) as usize]
        as *const _ExtendedGDBusPropertyInfo;
    g_mutex_lock(&raw mut (*(*skeleton).priv_0).lock);
    g_object_freeze_notify(object);
    if safe_c2rust__g_value_equal(
        value,
        (*(*skeleton).priv_0)
            .properties
            .offset(prop_id.wrapping_sub(1 as guint) as isize) as *mut GValue,
    ) == 0
    {
        if !g_dbus_interface_skeleton_get_connection(
            skeleton as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton,
        )
        .is_null()
            && (*info).emits_changed_signal() as ::core::ffi::c_int != 0
        {
            safe_c2rust__gxdp_trash_schedule_emit_changed(
                skeleton,
                info,
                prop_id,
                (*(*skeleton).priv_0)
                    .properties
                    .offset(prop_id.wrapping_sub(1 as guint) as isize)
                    as *mut GValue,
            );
        }
        g_value_copy(
            value,
            (*(*skeleton).priv_0)
                .properties
                .offset(prop_id.wrapping_sub(1 as guint) as isize) as *mut GValue,
        );
        g_object_notify_by_pspec(object, pspec);
    }
    g_mutex_unlock(&raw mut (*(*skeleton).priv_0).lock);
    g_object_thaw_notify(object);
}
unsafe extern "C" fn safe_c2rust_gxdp_trash_skeleton_init(mut skeleton: *mut GXdpTrashSkeleton) {
    (*skeleton).priv_0 = safe_c2rust_gxdp_trash_skeleton_get_instance_private(skeleton)
        as *mut GXdpTrashSkeletonPrivate;
    g_mutex_init(&raw mut (*(*skeleton).priv_0).lock);
    (*(*skeleton).priv_0).context = g_main_context_ref_thread_default();
    (*(*skeleton).priv_0).properties = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GValue>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GValue;
    g_value_init(
        (*(*skeleton).priv_0)
            .properties
            .offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        G_TYPE_UINT,
    );
}
unsafe extern "C" fn safe_c2rust_gxdp_trash_skeleton_get_version(
    mut object: *mut GXdpTrash,
) -> guint {
    let mut skeleton: *mut GXdpTrashSkeleton =
        object as *mut ::core::ffi::c_void as *mut GXdpTrashSkeleton;
    let mut value: guint = 0;
    g_mutex_lock(&raw mut (*(*skeleton).priv_0).lock);
    value = (*(*(*skeleton).priv_0)
        .properties
        .offset(0 as ::core::ffi::c_int as isize))
    .data[0 as ::core::ffi::c_int as usize]
        .v_uint;
    g_mutex_unlock(&raw mut (*(*skeleton).priv_0).lock);
    return value;
}
unsafe extern "C" fn safe_c2rust_gxdp_trash_skeleton_class_init(
    mut klass: *mut GXdpTrashSkeletonClass,
) {
    let mut gobject_class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    let mut skeleton_class: *mut GDBusInterfaceSkeletonClass =
        ::core::ptr::null_mut::<GDBusInterfaceSkeletonClass>();
    gobject_class = klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize =
        Some(safe_c2rust_gxdp_trash_skeleton_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_gxdp_trash_skeleton_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_gxdp_trash_skeleton_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).notify = Some(
        safe_c2rust_gxdp_trash_skeleton_notify
            as unsafe extern "C" fn(*mut GObject, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, *mut GParamSpec) -> ()>;
    safe_c2rust_gxdp_trash_override_properties(gobject_class, 1 as guint);
    skeleton_class = klass as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeletonClass;
    (*skeleton_class).get_info = Some(
        safe_c2rust_gxdp_trash_skeleton_dbus_interface_get_info
            as unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GDBusInterfaceInfo,
    )
        as Option<unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GDBusInterfaceInfo>;
    (*skeleton_class).get_properties = Some(
        safe_c2rust_gxdp_trash_skeleton_dbus_interface_get_properties
            as unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GVariant,
    )
        as Option<unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GVariant>;
    (*skeleton_class).flush = Some(
        safe_c2rust_gxdp_trash_skeleton_dbus_interface_flush
            as unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> ()>;
    (*skeleton_class).get_vtable = Some(
        safe_c2rust_gxdp_trash_skeleton_dbus_interface_get_vtable
            as unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GDBusInterfaceVTable,
    )
        as Option<unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GDBusInterfaceVTable>;
}
unsafe extern "C" fn safe_c2rust_gxdp_trash_skeleton_iface_init(mut iface: *mut GXdpTrashIface) {
    (*iface).get_version = Some(
        safe_c2rust_gxdp_trash_skeleton_get_version
            as unsafe extern "C" fn(*mut GXdpTrash) -> guint,
    ) as Option<unsafe extern "C" fn(*mut GXdpTrash) -> guint>;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_gxdp_trash_skeleton_new() -> *mut GXdpTrash {
    return g_object_new(
        safe_c2rust_gxdp_trash_skeleton_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GXdpTrash;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
unsafe extern "C" fn run_static_initializers() {
    safe_c2rust__gxdp_documents_property_info_pointers = [
        &raw const safe_c2rust__gxdp_documents_property_info_version.parent_struct,
        ::core::ptr::null::<GDBusPropertyInfo>(),
    ];
    safe_c2rust__gxdp_documents_property_info_version = {
        let mut init = _ExtendedGDBusPropertyInfo {
            use_gvariant_emits_changed_signal: [0; 1],
            c2rust_padding: [0; 7],
            parent_struct: _GDBusPropertyInfo {
                ref_count: -(1 as gint),
                name: b"version\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
                signature: b"u\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
                flags: G_DBUS_PROPERTY_INFO_FLAGS_READABLE,
                annotations: ::core::ptr::null_mut::<*mut GDBusAnnotationInfo>(),
            },
            hyphen_name: b"version\0" as *const u8 as *const gchar,
        };
        init.set_use_gvariant(FALSE as guint);
        init.set_emits_changed_signal(TRUE as guint);
        init
    };
    safe_c2rust__gxdp_documents_method_info_pointers = [
        &raw const safe_c2rust__gxdp_documents_method_info_get_mount_point.parent_struct,
        &raw const safe_c2rust__gxdp_documents_method_info_add.parent_struct,
        &raw const safe_c2rust__gxdp_documents_method_info_add_named.parent_struct,
        &raw const safe_c2rust__gxdp_documents_method_info_add_full.parent_struct,
        &raw const safe_c2rust__gxdp_documents_method_info_add_named_full.parent_struct,
        &raw const safe_c2rust__gxdp_documents_method_info_grant_permissions.parent_struct,
        &raw const safe_c2rust__gxdp_documents_method_info_revoke_permissions.parent_struct,
        &raw const safe_c2rust__gxdp_documents_method_info_delete.parent_struct,
        &raw const safe_c2rust__gxdp_documents_method_info_lookup.parent_struct,
        &raw const safe_c2rust__gxdp_documents_method_info_info.parent_struct,
        &raw const safe_c2rust__gxdp_documents_method_info_list.parent_struct,
        ::core::ptr::null::<GDBusMethodInfo>(),
    ];
    safe_c2rust__gxdp_documents_method_info_list_OUT_ARG_pointers = [
        &raw const safe_c2rust__gxdp_documents_method_info_list_OUT_ARG_docs.parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust__gxdp_documents_method_info_list_IN_ARG_pointers = [
        &raw const safe_c2rust__gxdp_documents_method_info_list_IN_ARG_app_id.parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust__gxdp_documents_method_info_info_OUT_ARG_pointers = [
        &raw const safe_c2rust__gxdp_documents_method_info_info_OUT_ARG_path.parent_struct,
        &raw const safe_c2rust__gxdp_documents_method_info_info_OUT_ARG_apps.parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust__gxdp_documents_method_info_info_IN_ARG_pointers = [
        &raw const safe_c2rust__gxdp_documents_method_info_info_IN_ARG_doc_id.parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust__gxdp_documents_method_info_lookup_OUT_ARG_pointers = [
        &raw const safe_c2rust__gxdp_documents_method_info_lookup_OUT_ARG_doc_id.parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust__gxdp_documents_method_info_lookup_IN_ARG_pointers = [
        &raw const safe_c2rust__gxdp_documents_method_info_lookup_IN_ARG_filename.parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust__gxdp_documents_method_info_delete_IN_ARG_pointers = [
        &raw const safe_c2rust__gxdp_documents_method_info_delete_IN_ARG_doc_id.parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust__gxdp_documents_method_info_revoke_permissions_IN_ARG_pointers = [
        &raw const safe_c2rust__gxdp_documents_method_info_revoke_permissions_IN_ARG_doc_id
            .parent_struct,
        &raw const safe_c2rust__gxdp_documents_method_info_revoke_permissions_IN_ARG_app_id
            .parent_struct,
        &raw const safe_c2rust__gxdp_documents_method_info_revoke_permissions_IN_ARG_permissions
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust__gxdp_documents_method_info_grant_permissions_IN_ARG_pointers = [
        &raw const safe_c2rust__gxdp_documents_method_info_grant_permissions_IN_ARG_doc_id
            .parent_struct,
        &raw const safe_c2rust__gxdp_documents_method_info_grant_permissions_IN_ARG_app_id
            .parent_struct,
        &raw const safe_c2rust__gxdp_documents_method_info_grant_permissions_IN_ARG_permissions
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust__gxdp_documents_method_info_add_named_full_OUT_ARG_pointers = [
        &raw const safe_c2rust__gxdp_documents_method_info_add_named_full_OUT_ARG_doc_id
            .parent_struct,
        &raw const safe_c2rust__gxdp_documents_method_info_add_named_full_OUT_ARG_extra_out
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust__gxdp_documents_method_info_add_named_full_IN_ARG_pointers = [
        &raw const safe_c2rust__gxdp_documents_method_info_add_named_full_IN_ARG_o_path_fd
            .parent_struct,
        &raw const safe_c2rust__gxdp_documents_method_info_add_named_full_IN_ARG_filename
            .parent_struct,
        &raw const safe_c2rust__gxdp_documents_method_info_add_named_full_IN_ARG_flags
            .parent_struct,
        &raw const safe_c2rust__gxdp_documents_method_info_add_named_full_IN_ARG_app_id
            .parent_struct,
        &raw const safe_c2rust__gxdp_documents_method_info_add_named_full_IN_ARG_permissions
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust__gxdp_documents_method_info_add_full_OUT_ARG_pointers = [
        &raw const safe_c2rust__gxdp_documents_method_info_add_full_OUT_ARG_doc_ids.parent_struct,
        &raw const safe_c2rust__gxdp_documents_method_info_add_full_OUT_ARG_extra_out.parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust__gxdp_documents_method_info_add_full_IN_ARG_pointers = [
        &raw const safe_c2rust__gxdp_documents_method_info_add_full_IN_ARG_o_path_fds.parent_struct,
        &raw const safe_c2rust__gxdp_documents_method_info_add_full_IN_ARG_flags.parent_struct,
        &raw const safe_c2rust__gxdp_documents_method_info_add_full_IN_ARG_app_id.parent_struct,
        &raw const safe_c2rust__gxdp_documents_method_info_add_full_IN_ARG_permissions
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust__gxdp_documents_method_info_add_named_OUT_ARG_pointers = [
        &raw const safe_c2rust__gxdp_documents_method_info_add_named_OUT_ARG_doc_id.parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust__gxdp_documents_method_info_add_named_IN_ARG_pointers = [
        &raw const safe_c2rust__gxdp_documents_method_info_add_named_IN_ARG_o_path_parent_fd
            .parent_struct,
        &raw const safe_c2rust__gxdp_documents_method_info_add_named_IN_ARG_filename.parent_struct,
        &raw const safe_c2rust__gxdp_documents_method_info_add_named_IN_ARG_reuse_existing
            .parent_struct,
        &raw const safe_c2rust__gxdp_documents_method_info_add_named_IN_ARG_persistent
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust__gxdp_documents_method_info_add_OUT_ARG_pointers = [
        &raw const safe_c2rust__gxdp_documents_method_info_add_OUT_ARG_doc_id.parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust__gxdp_documents_method_info_add_IN_ARG_pointers = [
        &raw const safe_c2rust__gxdp_documents_method_info_add_IN_ARG_o_path_fd.parent_struct,
        &raw const safe_c2rust__gxdp_documents_method_info_add_IN_ARG_reuse_existing.parent_struct,
        &raw const safe_c2rust__gxdp_documents_method_info_add_IN_ARG_persistent.parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust__gxdp_documents_method_info_get_mount_point_OUT_ARG_pointers = [
        &raw const safe_c2rust__gxdp_documents_method_info_get_mount_point_OUT_ARG_path
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust__gxdp_open_uri_property_info_pointers = [
        &raw const safe_c2rust__gxdp_open_uri_property_info_version.parent_struct,
        ::core::ptr::null::<GDBusPropertyInfo>(),
    ];
    safe_c2rust__gxdp_open_uri_property_info_version = {
        let mut init = _ExtendedGDBusPropertyInfo {
            use_gvariant_emits_changed_signal: [0; 1],
            c2rust_padding: [0; 7],
            parent_struct: _GDBusPropertyInfo {
                ref_count: -(1 as gint),
                name: b"version\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
                signature: b"u\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
                flags: G_DBUS_PROPERTY_INFO_FLAGS_READABLE,
                annotations: ::core::ptr::null_mut::<*mut GDBusAnnotationInfo>(),
            },
            hyphen_name: b"version\0" as *const u8 as *const gchar,
        };
        init.set_use_gvariant(FALSE as guint);
        init.set_emits_changed_signal(TRUE as guint);
        init
    };
    safe_c2rust__gxdp_open_uri_method_info_pointers = [
        &raw const safe_c2rust__gxdp_open_uri_method_info_open_uri.parent_struct,
        &raw const safe_c2rust__gxdp_open_uri_method_info_open_file.parent_struct,
        &raw const safe_c2rust__gxdp_open_uri_method_info_open_directory.parent_struct,
        ::core::ptr::null::<GDBusMethodInfo>(),
    ];
    safe_c2rust__gxdp_open_uri_method_info_open_directory_OUT_ARG_pointers = [
        &raw const safe_c2rust__gxdp_open_uri_method_info_open_directory_OUT_ARG_handle
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust__gxdp_open_uri_method_info_open_directory_IN_ARG_pointers = [
        &raw const safe_c2rust__gxdp_open_uri_method_info_open_directory_IN_ARG_parent_window
            .parent_struct,
        &raw const safe_c2rust__gxdp_open_uri_method_info_open_directory_IN_ARG_fd.parent_struct,
        &raw const safe_c2rust__gxdp_open_uri_method_info_open_directory_IN_ARG_options
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust__gxdp_open_uri_method_info_open_file_OUT_ARG_pointers = [
        &raw const safe_c2rust__gxdp_open_uri_method_info_open_file_OUT_ARG_handle.parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust__gxdp_open_uri_method_info_open_file_IN_ARG_pointers = [
        &raw const safe_c2rust__gxdp_open_uri_method_info_open_file_IN_ARG_parent_window
            .parent_struct,
        &raw const safe_c2rust__gxdp_open_uri_method_info_open_file_IN_ARG_fd.parent_struct,
        &raw const safe_c2rust__gxdp_open_uri_method_info_open_file_IN_ARG_options.parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust__gxdp_open_uri_method_info_open_uri_OUT_ARG_pointers = [
        &raw const safe_c2rust__gxdp_open_uri_method_info_open_uri_OUT_ARG_handle.parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust__gxdp_open_uri_method_info_open_uri_IN_ARG_pointers = [
        &raw const safe_c2rust__gxdp_open_uri_method_info_open_uri_IN_ARG_parent_window
            .parent_struct,
        &raw const safe_c2rust__gxdp_open_uri_method_info_open_uri_IN_ARG_uri.parent_struct,
        &raw const safe_c2rust__gxdp_open_uri_method_info_open_uri_IN_ARG_options.parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust__gxdp_proxy_resolver_property_info_pointers = [
        &raw const safe_c2rust__gxdp_proxy_resolver_property_info_version.parent_struct,
        ::core::ptr::null::<GDBusPropertyInfo>(),
    ];
    safe_c2rust__gxdp_proxy_resolver_property_info_version = {
        let mut init = _ExtendedGDBusPropertyInfo {
            use_gvariant_emits_changed_signal: [0; 1],
            c2rust_padding: [0; 7],
            parent_struct: _GDBusPropertyInfo {
                ref_count: -(1 as gint),
                name: b"version\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
                signature: b"u\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
                flags: G_DBUS_PROPERTY_INFO_FLAGS_READABLE,
                annotations: ::core::ptr::null_mut::<*mut GDBusAnnotationInfo>(),
            },
            hyphen_name: b"version\0" as *const u8 as *const gchar,
        };
        init.set_use_gvariant(FALSE as guint);
        init.set_emits_changed_signal(TRUE as guint);
        init
    };
    safe_c2rust__gxdp_proxy_resolver_method_info_pointers = [
        &raw const safe_c2rust__gxdp_proxy_resolver_method_info_lookup.parent_struct,
        ::core::ptr::null::<GDBusMethodInfo>(),
    ];
    safe_c2rust__gxdp_proxy_resolver_method_info_lookup_OUT_ARG_pointers = [
        &raw const safe_c2rust__gxdp_proxy_resolver_method_info_lookup_OUT_ARG_proxies
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust__gxdp_proxy_resolver_method_info_lookup_IN_ARG_pointers = [
        &raw const safe_c2rust__gxdp_proxy_resolver_method_info_lookup_IN_ARG_uri.parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust__gxdp_trash_property_info_pointers = [
        &raw const safe_c2rust__gxdp_trash_property_info_version.parent_struct,
        ::core::ptr::null::<GDBusPropertyInfo>(),
    ];
    safe_c2rust__gxdp_trash_property_info_version = {
        let mut init = _ExtendedGDBusPropertyInfo {
            use_gvariant_emits_changed_signal: [0; 1],
            c2rust_padding: [0; 7],
            parent_struct: _GDBusPropertyInfo {
                ref_count: -(1 as gint),
                name: b"version\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
                signature: b"u\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
                flags: G_DBUS_PROPERTY_INFO_FLAGS_READABLE,
                annotations: ::core::ptr::null_mut::<*mut GDBusAnnotationInfo>(),
            },
            hyphen_name: b"version\0" as *const u8 as *const gchar,
        };
        init.set_use_gvariant(FALSE as guint);
        init.set_emits_changed_signal(TRUE as guint);
        init
    };
    safe_c2rust__gxdp_trash_method_info_pointers = [
        &raw const safe_c2rust__gxdp_trash_method_info_trash_file.parent_struct,
        ::core::ptr::null::<GDBusMethodInfo>(),
    ];
    safe_c2rust__gxdp_trash_method_info_trash_file_OUT_ARG_pointers = [
        &raw const safe_c2rust__gxdp_trash_method_info_trash_file_OUT_ARG_result.parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust__gxdp_trash_method_info_trash_file_IN_ARG_pointers = [
        &raw const safe_c2rust__gxdp_trash_method_info_trash_file_IN_ARG_fd.parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
