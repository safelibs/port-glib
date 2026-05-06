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
    pub type __GFreedesktopDBus;
    fn g_quark_try_string(string: *const gchar) -> GQuark;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_mutex_init(mutex: *mut GMutex);
    fn g_mutex_clear(mutex: *mut GMutex);
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
    fn g_main_context_unref(context: *mut GMainContext);
    fn g_main_context_ref_thread_default() -> *mut GMainContext;
    fn g_source_destroy(source: *mut GSource);
    fn g_variant_type_checked_(type_string: *const gchar) -> *const GVariantType;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_ref_sink(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_take_ref(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_n_children(value: *mut GVariant) -> gsize;
    fn g_variant_iter_init(iter: *mut GVariantIter, value: *mut GVariant) -> gsize;
    fn g_variant_iter_free(iter: *mut GVariantIter);
    fn g_variant_iter_next_value(iter: *mut GVariantIter) -> *mut GVariant;
    fn g_variant_iter_next(iter: *mut GVariantIter, format_string: *const gchar, ...) -> gboolean;
    fn g_variant_builder_init(builder: *mut GVariantBuilder, type_0: *const GVariantType);
    fn g_variant_builder_end(builder: *mut GVariantBuilder) -> *mut GVariant;
    fn g_variant_builder_add(builder: *mut GVariantBuilder, format_string: *const gchar, ...);
    fn g_variant_new(format_string: *const gchar, ...) -> *mut GVariant;
    fn g_variant_get(value: *mut GVariant, format_string: *const gchar, ...);
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
    fn g_type_add_interface_static(
        instance_type: GType,
        interface_type: GType,
        info: *const GInterfaceInfo,
    );
    fn g_type_interface_add_prerequisite(interface_type: GType, prerequisite_type: GType);
    fn g_type_add_instance_private(class_type: GType, private_size: gsize) -> gint;
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_value_init(value: *mut GValue, g_type: GType) -> *mut GValue;
    fn g_value_unset(value: *mut GValue);
    fn g_value_peek_pointer(value: *const GValue) -> gpointer;
    fn g_cclosure_marshal_VOID__STRING(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
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
    fn g_signal_emit(instance: gpointer, signal_id: guint, detail: GQuark, ...);
    fn g_signal_lookup(name: *const gchar, itype: GType) -> guint;
    fn g_signal_accumulator_true_handled(
        ihint: *mut GSignalInvocationHint,
        return_accu: *mut GValue,
        handler_return: *const GValue,
        dummy: gpointer,
    ) -> gboolean;
    fn g_object_class_find_property(
        oclass: *mut GObjectClass,
        property_name: *const gchar,
    ) -> *mut GParamSpec;
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_set_property(
        object: *mut GObject,
        property_name: *const gchar,
        value: *const GValue,
    );
    fn g_object_get_property(object: *mut GObject, property_name: *const gchar, value: *mut GValue);
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
    fn g_object_unref(object: gpointer);
    fn g_value_set_object(value: *mut GValue, v_object: gpointer);
    fn g_value_set_boolean(value: *mut GValue, v_boolean: gboolean);
    fn g_value_get_boolean(value: *const GValue) -> gboolean;
    fn g_value_set_variant(value: *mut GValue, variant: *mut GVariant);
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
    fn g_dbus_method_invocation_return_error(
        invocation: *mut GDBusMethodInvocation,
        domain: GQuark,
        code: gint,
        format: *const gchar,
        ...
    );
    fn g_dbus_proxy_get_type() -> GType;
    fn g_dbus_proxy_set_interface_info(proxy: *mut GDBusProxy, info: *mut GDBusInterfaceInfo);
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
    fn g_dbus_gvariant_to_gvalue(value: *mut GVariant, out_gvalue: *mut GValue);
    fn g_dbus_gvalue_to_gvariant(
        gvalue: *const GValue,
        type_0: *const GVariantType,
    ) -> *mut GVariant;
    fn g_unix_fd_list_get_type() -> GType;
}
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
pub type _GFreedesktopDBus = __GFreedesktopDBus;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __GFreedesktopDBusIface {
    pub parent_iface: GTypeInterface,
    pub handle_add_match: Option<
        unsafe extern "C" fn(
            *mut _GFreedesktopDBus,
            *mut GDBusMethodInvocation,
            *const gchar,
        ) -> gboolean,
    >,
    pub handle_get_connection_selinux_security_context: Option<
        unsafe extern "C" fn(
            *mut _GFreedesktopDBus,
            *mut GDBusMethodInvocation,
            *const gchar,
        ) -> gboolean,
    >,
    pub handle_get_connection_unix_process_id: Option<
        unsafe extern "C" fn(
            *mut _GFreedesktopDBus,
            *mut GDBusMethodInvocation,
            *const gchar,
        ) -> gboolean,
    >,
    pub handle_get_connection_unix_user: Option<
        unsafe extern "C" fn(
            *mut _GFreedesktopDBus,
            *mut GDBusMethodInvocation,
            *const gchar,
        ) -> gboolean,
    >,
    pub handle_get_id: Option<
        unsafe extern "C" fn(*mut _GFreedesktopDBus, *mut GDBusMethodInvocation) -> gboolean,
    >,
    pub handle_get_name_owner: Option<
        unsafe extern "C" fn(
            *mut _GFreedesktopDBus,
            *mut GDBusMethodInvocation,
            *const gchar,
        ) -> gboolean,
    >,
    pub handle_hello: Option<
        unsafe extern "C" fn(*mut _GFreedesktopDBus, *mut GDBusMethodInvocation) -> gboolean,
    >,
    pub handle_list_activatable_names: Option<
        unsafe extern "C" fn(*mut _GFreedesktopDBus, *mut GDBusMethodInvocation) -> gboolean,
    >,
    pub handle_list_names: Option<
        unsafe extern "C" fn(*mut _GFreedesktopDBus, *mut GDBusMethodInvocation) -> gboolean,
    >,
    pub handle_list_queued_owners: Option<
        unsafe extern "C" fn(
            *mut _GFreedesktopDBus,
            *mut GDBusMethodInvocation,
            *const gchar,
        ) -> gboolean,
    >,
    pub handle_name_has_owner: Option<
        unsafe extern "C" fn(
            *mut _GFreedesktopDBus,
            *mut GDBusMethodInvocation,
            *const gchar,
        ) -> gboolean,
    >,
    pub handle_release_name: Option<
        unsafe extern "C" fn(
            *mut _GFreedesktopDBus,
            *mut GDBusMethodInvocation,
            *const gchar,
        ) -> gboolean,
    >,
    pub handle_reload_config: Option<
        unsafe extern "C" fn(*mut _GFreedesktopDBus, *mut GDBusMethodInvocation) -> gboolean,
    >,
    pub handle_remove_match: Option<
        unsafe extern "C" fn(
            *mut _GFreedesktopDBus,
            *mut GDBusMethodInvocation,
            *const gchar,
        ) -> gboolean,
    >,
    pub handle_request_name: Option<
        unsafe extern "C" fn(
            *mut _GFreedesktopDBus,
            *mut GDBusMethodInvocation,
            *const gchar,
            guint,
        ) -> gboolean,
    >,
    pub handle_start_service_by_name: Option<
        unsafe extern "C" fn(
            *mut _GFreedesktopDBus,
            *mut GDBusMethodInvocation,
            *const gchar,
            guint,
        ) -> gboolean,
    >,
    pub handle_update_activation_environment: Option<
        unsafe extern "C" fn(
            *mut _GFreedesktopDBus,
            *mut GDBusMethodInvocation,
            *mut GVariant,
        ) -> gboolean,
    >,
    pub name_acquired: Option<unsafe extern "C" fn(*mut _GFreedesktopDBus, *const gchar) -> ()>,
    pub name_lost: Option<unsafe extern "C" fn(*mut _GFreedesktopDBus, *const gchar) -> ()>,
    pub name_owner_changed: Option<
        unsafe extern "C" fn(
            *mut _GFreedesktopDBus,
            *const gchar,
            *const gchar,
            *const gchar,
        ) -> (),
    >,
}
pub type _GFreedesktopDBusIface = __GFreedesktopDBusIface;
pub type _GFreedesktopDBusInterface = _GFreedesktopDBusIface;
pub const _G__FREEDESKTOP_DBUS_NAME_ACQUIRED: C2RustUnnamed_3 = 2;
pub const _G__FREEDESKTOP_DBUS_NAME_LOST: C2RustUnnamed_3 = 1;
pub type _GDbusCodegenMarshalVoid_StringStringStringFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const gchar,
        *const gchar,
        *const gchar,
        *mut ::core::ffi::c_void,
    ) -> (),
>;
pub const _G__FREEDESKTOP_DBUS_NAME_OWNER_CHANGED: C2RustUnnamed_3 = 0;
pub type _GDbusCodegenMarshalBoolean_ObjectFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *mut GDBusMethodInvocation,
        *mut ::core::ffi::c_void,
    ) -> gboolean,
>;
pub type _GDbusCodegenMarshalBoolean_ObjectVariantFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *mut GDBusMethodInvocation,
        *mut GVariant,
        *mut ::core::ffi::c_void,
    ) -> gboolean,
>;
pub type _GDbusCodegenMarshalBoolean_ObjectStringFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *mut GDBusMethodInvocation,
        *const gchar,
        *mut ::core::ffi::c_void,
    ) -> gboolean,
>;
pub type _GDbusCodegenMarshalBoolean_ObjectStringUintFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *mut GDBusMethodInvocation,
        *const gchar,
        guint,
        *mut ::core::ffi::c_void,
    ) -> gboolean,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ExtendedGDBusInterfaceInfo {
    pub parent_struct: GDBusInterfaceInfo,
    pub hyphen_name: *const gchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ExtendedGDBusSignalInfo {
    pub parent_struct: GDBusSignalInfo,
    pub signal_name: *const gchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ExtendedGDBusArgInfo {
    pub parent_struct: GDBusArgInfo,
    pub use_gvariant: gboolean,
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
pub struct __GFreedesktopDBusProxy {
    pub parent_instance: GDBusProxy,
    pub priv_0: *mut _GFreedesktopDBusProxyPrivate,
}
pub type _GFreedesktopDBusProxyPrivate = __GFreedesktopDBusProxyPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __GFreedesktopDBusProxyPrivate {
    pub qdata: *mut GData,
}
pub type _GFreedesktopDBusProxy = __GFreedesktopDBusProxy;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __GFreedesktopDBusProxyClass {
    pub parent_class: GDBusProxyClass,
}
pub type _GFreedesktopDBusProxyClass = __GFreedesktopDBusProxyClass;
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
pub struct __GFreedesktopDBusSkeleton {
    pub parent_instance: GDBusInterfaceSkeleton,
    pub priv_0: *mut _GFreedesktopDBusSkeletonPrivate,
}
pub type _GFreedesktopDBusSkeletonPrivate = __GFreedesktopDBusSkeletonPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __GFreedesktopDBusSkeletonPrivate {
    pub properties: *mut GValue,
    pub changed_properties: *mut GList,
    pub changed_properties_idle_source: *mut GSource,
    pub context: *mut GMainContext,
    pub lock: GMutex,
}
pub type _GFreedesktopDBusSkeleton = __GFreedesktopDBusSkeleton;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __GFreedesktopDBusSkeletonClass {
    pub parent_class: GDBusInterfaceSkeletonClass,
}
pub type _GFreedesktopDBusSkeletonClass = __GFreedesktopDBusSkeletonClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ChangedProperty {
    pub info: *const _ExtendedGDBusPropertyInfo,
    pub prop_id: guint,
    pub orig_value: GValue,
}
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_PRIORITY_DEFAULT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_INVALID: GType = ((0 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_NONE: GType = ((1 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_INTERFACE: GType =
    ((2 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_BOOLEAN: GType = ((5 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_UINT: GType = ((7 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_STRING: GType = ((16 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_VARIANT: GType = ((21 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
unsafe extern "C" fn safe_c2rust__changed_property_free(mut data: *mut ChangedProperty) {
    g_value_unset(&raw mut (*data).orig_value);
    g_free(data as gpointer);
}
unsafe extern "C" fn safe_c2rust__g_dbus_codegen_marshal_VOID__STRING_STRING_STRING(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: ::core::ffi::c_uint,
    mut param_values: *const GValue,
    mut invocation_hint: *mut ::core::ffi::c_void,
    mut marshal_data: *mut ::core::ffi::c_void,
) {
    let mut callback: _GDbusCodegenMarshalVoid_StringStringStringFunc = None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if n_param_values == 4 as ::core::ffi::c_uint {
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
        _GDbusCodegenMarshalVoid_StringStringStringFunc,
    >(if !marshal_data.is_null() {
        marshal_data
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *const gchar,
        (*param_values.offset(2 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *const gchar,
        (*param_values.offset(3 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer as *const gchar,
        data2,
    );
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
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !return_value.is_null() {
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
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if n_param_values == 2 as ::core::ffi::c_uint {
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
unsafe extern "C" fn safe_c2rust__g_dbus_codegen_marshal_BOOLEAN__OBJECT_STRING_UINT(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: ::core::ffi::c_uint,
    mut param_values: *const GValue,
    mut invocation_hint: *mut ::core::ffi::c_void,
    mut marshal_data: *mut ::core::ffi::c_void,
) {
    let mut callback: _GDbusCodegenMarshalBoolean_ObjectStringUintFunc = None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut v_return: gboolean = 0;
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !return_value.is_null() {
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
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if n_param_values == 4 as ::core::ffi::c_uint {
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
        _GDbusCodegenMarshalBoolean_ObjectStringUintFunc,
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
            .v_uint,
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
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !return_value.is_null() {
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
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if n_param_values == 3 as ::core::ffi::c_uint {
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
unsafe extern "C" fn safe_c2rust__g_dbus_codegen_marshal_BOOLEAN__OBJECT_VARIANT(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: ::core::ffi::c_uint,
    mut param_values: *const GValue,
    mut invocation_hint: *mut ::core::ffi::c_void,
    mut marshal_data: *mut ::core::ffi::c_void,
) {
    let mut callback: _GDbusCodegenMarshalBoolean_ObjectVariantFunc = None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut v_return: gboolean = 0;
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !return_value.is_null() {
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
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if n_param_values == 3 as ::core::ffi::c_uint {
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
        _GDbusCodegenMarshalBoolean_ObjectVariantFunc,
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
            .v_pointer as *mut GVariant,
        data2,
    );
    g_value_set_boolean(return_value, v_return);
}
static mut safe_c2rust__G__FREEDESKTOP_DBUS_SIGNALS: [::core::ffi::c_uint; 3] =
    [0 as ::core::ffi::c_int as ::core::ffi::c_uint, 0, 0];
static mut safe_c2rust___g_freedesktop_dbus_method_info_hello_OUT_ARG_assigned_name:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"assigned_name\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_hello_OUT_ARG_pointers:
    [*const GDBusArgInfo; 2] = [::core::ptr::null::<GDBusArgInfo>(); 2];
static mut safe_c2rust___g_freedesktop_dbus_method_info_hello: _ExtendedGDBusMethodInfo = unsafe {
    _ExtendedGDBusMethodInfo {
        parent_struct: _GDBusMethodInfo {
            ref_count: -(1 as gint),
            name: b"Hello\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            in_args: ::core::ptr::null::<*mut GDBusArgInfo>() as *mut *mut GDBusArgInfo,
            out_args: &raw const safe_c2rust___g_freedesktop_dbus_method_info_hello_OUT_ARG_pointers
                as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"handle-hello\0" as *const u8 as *const gchar,
        pass_fdlist: FALSE,
    }
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_request_name_IN_ARG_name:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"name\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_request_name_IN_ARG_flags:
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
static mut safe_c2rust___g_freedesktop_dbus_method_info_request_name_IN_ARG_pointers:
    [*const GDBusArgInfo; 3] = [::core::ptr::null::<GDBusArgInfo>(); 3];
static mut safe_c2rust___g_freedesktop_dbus_method_info_request_name_OUT_ARG_value:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"value\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"u\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_request_name_OUT_ARG_pointers:
    [*const GDBusArgInfo; 2] = [::core::ptr::null::<GDBusArgInfo>(); 2];
static mut safe_c2rust___g_freedesktop_dbus_method_info_request_name: _ExtendedGDBusMethodInfo = unsafe {
    _ExtendedGDBusMethodInfo {
        parent_struct: _GDBusMethodInfo {
            ref_count: -(1 as gint),
            name: b"RequestName\0" as *const u8 as *const ::core::ffi::c_char
                as *mut gchar,
            in_args: &raw const safe_c2rust___g_freedesktop_dbus_method_info_request_name_IN_ARG_pointers
                as *mut *mut GDBusArgInfo,
            out_args: &raw const safe_c2rust___g_freedesktop_dbus_method_info_request_name_OUT_ARG_pointers
                as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"handle-request-name\0" as *const u8 as *const gchar,
        pass_fdlist: FALSE,
    }
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_release_name_IN_ARG_name:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"name\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_release_name_IN_ARG_pointers:
    [*const GDBusArgInfo; 2] = [::core::ptr::null::<GDBusArgInfo>(); 2];
static mut safe_c2rust___g_freedesktop_dbus_method_info_release_name_OUT_ARG_value:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"value\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"u\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_release_name_OUT_ARG_pointers:
    [*const GDBusArgInfo; 2] = [::core::ptr::null::<GDBusArgInfo>(); 2];
static mut safe_c2rust___g_freedesktop_dbus_method_info_release_name: _ExtendedGDBusMethodInfo = unsafe {
    _ExtendedGDBusMethodInfo {
        parent_struct: _GDBusMethodInfo {
            ref_count: -(1 as gint),
            name: b"ReleaseName\0" as *const u8 as *const ::core::ffi::c_char
                as *mut gchar,
            in_args: &raw const safe_c2rust___g_freedesktop_dbus_method_info_release_name_IN_ARG_pointers
                as *mut *mut GDBusArgInfo,
            out_args: &raw const safe_c2rust___g_freedesktop_dbus_method_info_release_name_OUT_ARG_pointers
                as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"handle-release-name\0" as *const u8 as *const gchar,
        pass_fdlist: FALSE,
    }
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_start_service_by_name_IN_ARG_name:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"name\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_start_service_by_name_IN_ARG_flags:
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
static mut safe_c2rust___g_freedesktop_dbus_method_info_start_service_by_name_IN_ARG_pointers:
    [*const GDBusArgInfo; 3] = [::core::ptr::null::<GDBusArgInfo>(); 3];
static mut safe_c2rust___g_freedesktop_dbus_method_info_start_service_by_name_OUT_ARG_value:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"value\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"u\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_start_service_by_name_OUT_ARG_pointers:
    [*const GDBusArgInfo; 2] = [::core::ptr::null::<GDBusArgInfo>(); 2];
static mut safe_c2rust___g_freedesktop_dbus_method_info_start_service_by_name:
    _ExtendedGDBusMethodInfo = unsafe {
    _ExtendedGDBusMethodInfo {
        parent_struct: _GDBusMethodInfo {
            ref_count: -(1 as gint),
            name: b"StartServiceByName\0" as *const u8 as *const ::core::ffi::c_char
                as *mut gchar,
            in_args: &raw const safe_c2rust___g_freedesktop_dbus_method_info_start_service_by_name_IN_ARG_pointers
                as *mut *mut GDBusArgInfo,
            out_args: &raw const safe_c2rust___g_freedesktop_dbus_method_info_start_service_by_name_OUT_ARG_pointers
                as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"handle-start-service-by-name\0" as *const u8 as *const gchar,
        pass_fdlist: FALSE,
    }
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_name_has_owner_IN_ARG_name:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"name\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_name_has_owner_IN_ARG_pointers:
    [*const GDBusArgInfo; 2] = [::core::ptr::null::<GDBusArgInfo>(); 2];
static mut safe_c2rust___g_freedesktop_dbus_method_info_name_has_owner_OUT_ARG_has_owner:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"has_owner\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"b\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_name_has_owner_OUT_ARG_pointers:
    [*const GDBusArgInfo; 2] = [::core::ptr::null::<GDBusArgInfo>(); 2];
static mut safe_c2rust___g_freedesktop_dbus_method_info_name_has_owner: _ExtendedGDBusMethodInfo = unsafe {
    _ExtendedGDBusMethodInfo {
        parent_struct: _GDBusMethodInfo {
            ref_count: -(1 as gint),
            name: b"NameHasOwner\0" as *const u8 as *const ::core::ffi::c_char
                as *mut gchar,
            in_args: &raw const safe_c2rust___g_freedesktop_dbus_method_info_name_has_owner_IN_ARG_pointers
                as *mut *mut GDBusArgInfo,
            out_args: &raw const safe_c2rust___g_freedesktop_dbus_method_info_name_has_owner_OUT_ARG_pointers
                as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"handle-name-has-owner\0" as *const u8 as *const gchar,
        pass_fdlist: FALSE,
    }
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_list_names_OUT_ARG_names:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"names\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"as\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_list_names_OUT_ARG_pointers:
    [*const GDBusArgInfo; 2] = [::core::ptr::null::<GDBusArgInfo>(); 2];
static mut safe_c2rust___g_freedesktop_dbus_method_info_list_names: _ExtendedGDBusMethodInfo = unsafe {
    _ExtendedGDBusMethodInfo {
        parent_struct: _GDBusMethodInfo {
            ref_count: -(1 as gint),
            name: b"ListNames\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            in_args: ::core::ptr::null::<*mut GDBusArgInfo>() as *mut *mut GDBusArgInfo,
            out_args:
                &raw const safe_c2rust___g_freedesktop_dbus_method_info_list_names_OUT_ARG_pointers
                    as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"handle-list-names\0" as *const u8 as *const gchar,
        pass_fdlist: FALSE,
    }
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_list_activatable_names_OUT_ARG_activatable_names: _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"activatable_names\0" as *const u8 as *const ::core::ffi::c_char
            as *mut gchar,
        signature: b"as\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_list_activatable_names_OUT_ARG_pointers:
    [*const GDBusArgInfo; 2] = [::core::ptr::null::<GDBusArgInfo>(); 2];
static mut safe_c2rust___g_freedesktop_dbus_method_info_list_activatable_names:
    _ExtendedGDBusMethodInfo = unsafe {
    _ExtendedGDBusMethodInfo {
        parent_struct: _GDBusMethodInfo {
            ref_count: -(1 as gint),
            name: b"ListActivatableNames\0" as *const u8 as *const ::core::ffi::c_char
                as *mut gchar,
            in_args: ::core::ptr::null::<*mut GDBusArgInfo>() as *mut *mut GDBusArgInfo,
            out_args: &raw const safe_c2rust___g_freedesktop_dbus_method_info_list_activatable_names_OUT_ARG_pointers
                as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"handle-list-activatable-names\0" as *const u8 as *const gchar,
        pass_fdlist: FALSE,
    }
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_add_match_IN_ARG_rule:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"rule\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_add_match_IN_ARG_pointers:
    [*const GDBusArgInfo; 2] = [::core::ptr::null::<GDBusArgInfo>(); 2];
static mut safe_c2rust___g_freedesktop_dbus_method_info_add_match: _ExtendedGDBusMethodInfo = unsafe {
    _ExtendedGDBusMethodInfo {
        parent_struct: _GDBusMethodInfo {
            ref_count: -(1 as gint),
            name: b"AddMatch\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            in_args:
                &raw const safe_c2rust___g_freedesktop_dbus_method_info_add_match_IN_ARG_pointers
                    as *mut *mut GDBusArgInfo,
            out_args: ::core::ptr::null::<*mut GDBusArgInfo>() as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"handle-add-match\0" as *const u8 as *const gchar,
        pass_fdlist: FALSE,
    }
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_remove_match_IN_ARG_rule:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"rule\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_remove_match_IN_ARG_pointers:
    [*const GDBusArgInfo; 2] = [::core::ptr::null::<GDBusArgInfo>(); 2];
static mut safe_c2rust___g_freedesktop_dbus_method_info_remove_match: _ExtendedGDBusMethodInfo = unsafe {
    _ExtendedGDBusMethodInfo {
        parent_struct: _GDBusMethodInfo {
            ref_count: -(1 as gint),
            name: b"RemoveMatch\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            in_args:
                &raw const safe_c2rust___g_freedesktop_dbus_method_info_remove_match_IN_ARG_pointers
                    as *mut *mut GDBusArgInfo,
            out_args: ::core::ptr::null::<*mut GDBusArgInfo>() as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"handle-remove-match\0" as *const u8 as *const gchar,
        pass_fdlist: FALSE,
    }
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_get_name_owner_IN_ARG_name:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"name\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_get_name_owner_IN_ARG_pointers:
    [*const GDBusArgInfo; 2] = [::core::ptr::null::<GDBusArgInfo>(); 2];
static mut safe_c2rust___g_freedesktop_dbus_method_info_get_name_owner_OUT_ARG_unique_name:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"unique_name\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_get_name_owner_OUT_ARG_pointers:
    [*const GDBusArgInfo; 2] = [::core::ptr::null::<GDBusArgInfo>(); 2];
static mut safe_c2rust___g_freedesktop_dbus_method_info_get_name_owner: _ExtendedGDBusMethodInfo = unsafe {
    _ExtendedGDBusMethodInfo {
        parent_struct: _GDBusMethodInfo {
            ref_count: -(1 as gint),
            name: b"GetNameOwner\0" as *const u8 as *const ::core::ffi::c_char
                as *mut gchar,
            in_args: &raw const safe_c2rust___g_freedesktop_dbus_method_info_get_name_owner_IN_ARG_pointers
                as *mut *mut GDBusArgInfo,
            out_args: &raw const safe_c2rust___g_freedesktop_dbus_method_info_get_name_owner_OUT_ARG_pointers
                as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"handle-get-name-owner\0" as *const u8 as *const gchar,
        pass_fdlist: FALSE,
    }
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_list_queued_owners_IN_ARG_name:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"name\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_list_queued_owners_IN_ARG_pointers:
    [*const GDBusArgInfo; 2] = [::core::ptr::null::<GDBusArgInfo>(); 2];
static mut safe_c2rust___g_freedesktop_dbus_method_info_list_queued_owners_OUT_ARG_queued_owners:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"queued_owners\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"as\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_list_queued_owners_OUT_ARG_pointers:
    [*const GDBusArgInfo; 2] = [::core::ptr::null::<GDBusArgInfo>(); 2];
static mut safe_c2rust___g_freedesktop_dbus_method_info_list_queued_owners:
    _ExtendedGDBusMethodInfo = unsafe {
    _ExtendedGDBusMethodInfo {
        parent_struct: _GDBusMethodInfo {
            ref_count: -(1 as gint),
            name: b"ListQueuedOwners\0" as *const u8 as *const ::core::ffi::c_char
                as *mut gchar,
            in_args: &raw const safe_c2rust___g_freedesktop_dbus_method_info_list_queued_owners_IN_ARG_pointers
                as *mut *mut GDBusArgInfo,
            out_args: &raw const safe_c2rust___g_freedesktop_dbus_method_info_list_queued_owners_OUT_ARG_pointers
                as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"handle-list-queued-owners\0" as *const u8 as *const gchar,
        pass_fdlist: FALSE,
    }
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_get_connection_unix_user_IN_ARG_name:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"name\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_get_connection_unix_user_IN_ARG_pointers:
    [*const GDBusArgInfo; 2] = [::core::ptr::null::<GDBusArgInfo>(); 2];
static mut safe_c2rust___g_freedesktop_dbus_method_info_get_connection_unix_user_OUT_ARG_uid:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"uid\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"u\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_get_connection_unix_user_OUT_ARG_pointers: [*const GDBusArgInfo; 2] = [::core::ptr::null::<
    GDBusArgInfo,
>(); 2];
static mut safe_c2rust___g_freedesktop_dbus_method_info_get_connection_unix_user:
    _ExtendedGDBusMethodInfo = unsafe {
    _ExtendedGDBusMethodInfo {
        parent_struct: _GDBusMethodInfo {
            ref_count: -(1 as gint),
            name: b"GetConnectionUnixUser\0" as *const u8 as *const ::core::ffi::c_char
                as *mut gchar,
            in_args: &raw const safe_c2rust___g_freedesktop_dbus_method_info_get_connection_unix_user_IN_ARG_pointers
                as *mut *mut GDBusArgInfo,
            out_args: &raw const safe_c2rust___g_freedesktop_dbus_method_info_get_connection_unix_user_OUT_ARG_pointers
                as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"handle-get-connection-unix-user\0" as *const u8 as *const gchar,
        pass_fdlist: FALSE,
    }
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_get_connection_unix_process_id_IN_ARG_name: _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"name\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_get_connection_unix_process_id_IN_ARG_pointers: [*const GDBusArgInfo; 2] = [::core::ptr::null::<
    GDBusArgInfo,
>(); 2];
static mut safe_c2rust___g_freedesktop_dbus_method_info_get_connection_unix_process_id_OUT_ARG_pid: _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"pid\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"u\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_get_connection_unix_process_id_OUT_ARG_pointers: [*const GDBusArgInfo; 2] = [::core::ptr::null::<
    GDBusArgInfo,
>(); 2];
static mut safe_c2rust___g_freedesktop_dbus_method_info_get_connection_unix_process_id:
    _ExtendedGDBusMethodInfo = unsafe {
    _ExtendedGDBusMethodInfo {
        parent_struct: _GDBusMethodInfo {
            ref_count: -(1 as gint),
            name: b"GetConnectionUnixProcessID\0" as *const u8
                as *const ::core::ffi::c_char as *mut gchar,
            in_args: &raw const safe_c2rust___g_freedesktop_dbus_method_info_get_connection_unix_process_id_IN_ARG_pointers
                as *mut *mut GDBusArgInfo,
            out_args: &raw const safe_c2rust___g_freedesktop_dbus_method_info_get_connection_unix_process_id_OUT_ARG_pointers
                as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"handle-get-connection-unix-process-id\0" as *const u8
            as *const gchar,
        pass_fdlist: FALSE,
    }
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_get_connection_selinux_security_context_IN_ARG_name: _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"name\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_get_connection_selinux_security_context_IN_ARG_pointers: [*const GDBusArgInfo; 2] = [::core::ptr::null::<
    GDBusArgInfo,
>(); 2];
static mut safe_c2rust___g_freedesktop_dbus_method_info_get_connection_selinux_security_context_OUT_ARG_security_context: _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"security_context\0" as *const u8 as *const ::core::ffi::c_char
            as *mut gchar,
        signature: b"ay\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_get_connection_selinux_security_context_OUT_ARG_pointers: [*const GDBusArgInfo; 2] = [::core::ptr::null::<
    GDBusArgInfo,
>(); 2];
static mut safe_c2rust___g_freedesktop_dbus_method_info_get_connection_selinux_security_context:
    _ExtendedGDBusMethodInfo = unsafe {
    _ExtendedGDBusMethodInfo {
        parent_struct: _GDBusMethodInfo {
            ref_count: -(1 as gint),
            name: b"GetConnectionSELinuxSecurityContext\0" as *const u8
                as *const ::core::ffi::c_char as *mut gchar,
            in_args: &raw const safe_c2rust___g_freedesktop_dbus_method_info_get_connection_selinux_security_context_IN_ARG_pointers
                as *mut *mut GDBusArgInfo,
            out_args: &raw const safe_c2rust___g_freedesktop_dbus_method_info_get_connection_selinux_security_context_OUT_ARG_pointers
                as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"handle-get-connection-selinux-security-context\0" as *const u8
            as *const gchar,
        pass_fdlist: FALSE,
    }
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_update_activation_environment_IN_ARG_environment: _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"environment\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"a{ss}\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_update_activation_environment_IN_ARG_pointers: [*const GDBusArgInfo; 2] = [::core::ptr::null::<
    GDBusArgInfo,
>(); 2];
static mut safe_c2rust___g_freedesktop_dbus_method_info_update_activation_environment:
    _ExtendedGDBusMethodInfo = unsafe {
    _ExtendedGDBusMethodInfo {
        parent_struct: _GDBusMethodInfo {
            ref_count: -(1 as gint),
            name: b"UpdateActivationEnvironment\0" as *const u8
                as *const ::core::ffi::c_char as *mut gchar,
            in_args: &raw const safe_c2rust___g_freedesktop_dbus_method_info_update_activation_environment_IN_ARG_pointers
                as *mut *mut GDBusArgInfo,
            out_args: ::core::ptr::null::<*mut GDBusArgInfo>() as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"handle-update-activation-environment\0" as *const u8
            as *const gchar,
        pass_fdlist: FALSE,
    }
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_reload_config: _ExtendedGDBusMethodInfo =
    _ExtendedGDBusMethodInfo {
        parent_struct: _GDBusMethodInfo {
            ref_count: -(1 as gint),
            name: b"ReloadConfig\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            in_args: ::core::ptr::null::<*mut GDBusArgInfo>() as *mut *mut GDBusArgInfo,
            out_args: ::core::ptr::null::<*mut GDBusArgInfo>() as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"handle-reload-config\0" as *const u8 as *const gchar,
        pass_fdlist: FALSE,
    };
static mut safe_c2rust___g_freedesktop_dbus_method_info_get_id_OUT_ARG_unique_id:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"unique_id\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_get_id_OUT_ARG_pointers:
    [*const GDBusArgInfo; 2] = [::core::ptr::null::<GDBusArgInfo>(); 2];
static mut safe_c2rust___g_freedesktop_dbus_method_info_get_id: _ExtendedGDBusMethodInfo = unsafe {
    _ExtendedGDBusMethodInfo {
        parent_struct: _GDBusMethodInfo {
            ref_count: -(1 as gint),
            name: b"GetId\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            in_args: ::core::ptr::null::<*mut GDBusArgInfo>() as *mut *mut GDBusArgInfo,
            out_args:
                &raw const safe_c2rust___g_freedesktop_dbus_method_info_get_id_OUT_ARG_pointers
                    as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"handle-get-id\0" as *const u8 as *const gchar,
        pass_fdlist: FALSE,
    }
};
static mut safe_c2rust___g_freedesktop_dbus_method_info_pointers: [*const GDBusMethodInfo; 18] =
    [::core::ptr::null::<GDBusMethodInfo>(); 18];
static mut safe_c2rust___g_freedesktop_dbus_signal_info_name_owner_changed_ARG_name:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"name\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust___g_freedesktop_dbus_signal_info_name_owner_changed_ARG_old_owner:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"old_owner\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust___g_freedesktop_dbus_signal_info_name_owner_changed_ARG_new_owner:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"new_owner\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust___g_freedesktop_dbus_signal_info_name_owner_changed_ARG_pointers:
    [*const GDBusArgInfo; 4] = [::core::ptr::null::<GDBusArgInfo>(); 4];
static mut safe_c2rust___g_freedesktop_dbus_signal_info_name_owner_changed:
    _ExtendedGDBusSignalInfo = unsafe {
    _ExtendedGDBusSignalInfo {
        parent_struct: _GDBusSignalInfo {
            ref_count: -(1 as gint),
            name: b"NameOwnerChanged\0" as *const u8 as *const ::core::ffi::c_char
                as *mut gchar,
            args: &raw const safe_c2rust___g_freedesktop_dbus_signal_info_name_owner_changed_ARG_pointers
                as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"name-owner-changed\0" as *const u8 as *const gchar,
    }
};
static mut safe_c2rust___g_freedesktop_dbus_signal_info_name_lost_ARG_name: _ExtendedGDBusArgInfo =
    _ExtendedGDBusArgInfo {
        parent_struct: _GDBusArgInfo {
            ref_count: -(1 as gint),
            name: b"name\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        use_gvariant: FALSE,
    };
static mut safe_c2rust___g_freedesktop_dbus_signal_info_name_lost_ARG_pointers:
    [*const GDBusArgInfo; 2] = [::core::ptr::null::<GDBusArgInfo>(); 2];
static mut safe_c2rust___g_freedesktop_dbus_signal_info_name_lost: _ExtendedGDBusSignalInfo = unsafe {
    _ExtendedGDBusSignalInfo {
        parent_struct: _GDBusSignalInfo {
            ref_count: -(1 as gint),
            name: b"NameLost\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            args: &raw const safe_c2rust___g_freedesktop_dbus_signal_info_name_lost_ARG_pointers
                as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"name-lost\0" as *const u8 as *const gchar,
    }
};
static mut safe_c2rust___g_freedesktop_dbus_signal_info_name_acquired_ARG_name:
    _ExtendedGDBusArgInfo = _ExtendedGDBusArgInfo {
    parent_struct: _GDBusArgInfo {
        ref_count: -(1 as gint),
        name: b"name\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        signature: b"s\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
            as *mut *mut GDBusAnnotationInfo,
    },
    use_gvariant: FALSE,
};
static mut safe_c2rust___g_freedesktop_dbus_signal_info_name_acquired_ARG_pointers:
    [*const GDBusArgInfo; 2] = [::core::ptr::null::<GDBusArgInfo>(); 2];
static mut safe_c2rust___g_freedesktop_dbus_signal_info_name_acquired: _ExtendedGDBusSignalInfo = unsafe {
    _ExtendedGDBusSignalInfo {
        parent_struct: _GDBusSignalInfo {
            ref_count: -(1 as gint),
            name: b"NameAcquired\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
            args: &raw const safe_c2rust___g_freedesktop_dbus_signal_info_name_acquired_ARG_pointers
                as *mut *mut GDBusArgInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        signal_name: b"name-acquired\0" as *const u8 as *const gchar,
    }
};
static mut safe_c2rust___g_freedesktop_dbus_signal_info_pointers: [*const GDBusSignalInfo; 4] =
    [::core::ptr::null::<GDBusSignalInfo>(); 4];
static mut safe_c2rust___g_freedesktop_dbus_interface_info: _ExtendedGDBusInterfaceInfo = unsafe {
    _ExtendedGDBusInterfaceInfo {
        parent_struct: _GDBusInterfaceInfo {
            ref_count: -(1 as gint),
            name: b"org.freedesktop.DBus\0" as *const u8 as *const ::core::ffi::c_char
                as *mut gchar,
            methods: &raw const safe_c2rust___g_freedesktop_dbus_method_info_pointers
                as *mut *mut GDBusMethodInfo,
            signals: &raw const safe_c2rust___g_freedesktop_dbus_signal_info_pointers
                as *mut *mut GDBusSignalInfo,
            properties: ::core::ptr::null::<*mut GDBusPropertyInfo>()
                as *mut *mut GDBusPropertyInfo,
            annotations: ::core::ptr::null::<*mut GDBusAnnotationInfo>()
                as *mut *mut GDBusAnnotationInfo,
        },
        hyphen_name: b"freedesktop-dbus\0" as *const u8 as *const gchar,
    }
};
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_interface_info() -> *mut GDBusInterfaceInfo
{
    return &raw const safe_c2rust___g_freedesktop_dbus_interface_info.parent_struct
        as *mut GDBusInterfaceInfo;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_override_properties(
    mut klass: *mut GObjectClass,
    mut property_id_begin: guint,
) -> guint {
    return property_id_begin.wrapping_sub(1 as guint);
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_signal_marshal_name_owner_changed(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: ::core::ffi::c_uint,
    mut param_values: *const GValue,
    mut invocation_hint: *mut ::core::ffi::c_void,
    mut marshal_data: *mut ::core::ffi::c_void,
) {
    safe_c2rust__g_dbus_codegen_marshal_VOID__STRING_STRING_STRING(
        closure,
        return_value,
        n_param_values,
        param_values,
        invocation_hint,
        marshal_data,
    );
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_signal_marshal_name_lost(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: ::core::ffi::c_uint,
    mut param_values: *const GValue,
    mut invocation_hint: *mut ::core::ffi::c_void,
    mut marshal_data: *mut ::core::ffi::c_void,
) {
    g_cclosure_marshal_VOID__STRING(
        closure,
        return_value,
        n_param_values as guint,
        param_values,
        invocation_hint as gpointer,
        marshal_data as gpointer,
    );
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_signal_marshal_name_acquired(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: ::core::ffi::c_uint,
    mut param_values: *const GValue,
    mut invocation_hint: *mut ::core::ffi::c_void,
    mut marshal_data: *mut ::core::ffi::c_void,
) {
    g_cclosure_marshal_VOID__STRING(
        closure,
        return_value,
        n_param_values as guint,
        param_values,
        invocation_hint as gpointer,
        marshal_data as gpointer,
    );
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_method_marshal_hello(
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
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_method_marshal_request_name(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: ::core::ffi::c_uint,
    mut param_values: *const GValue,
    mut invocation_hint: *mut ::core::ffi::c_void,
    mut marshal_data: *mut ::core::ffi::c_void,
) {
    safe_c2rust__g_dbus_codegen_marshal_BOOLEAN__OBJECT_STRING_UINT(
        closure,
        return_value,
        n_param_values,
        param_values,
        invocation_hint,
        marshal_data,
    );
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_method_marshal_release_name(
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
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_method_marshal_start_service_by_name(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: ::core::ffi::c_uint,
    mut param_values: *const GValue,
    mut invocation_hint: *mut ::core::ffi::c_void,
    mut marshal_data: *mut ::core::ffi::c_void,
) {
    safe_c2rust__g_dbus_codegen_marshal_BOOLEAN__OBJECT_STRING_UINT(
        closure,
        return_value,
        n_param_values,
        param_values,
        invocation_hint,
        marshal_data,
    );
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_method_marshal_name_has_owner(
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
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_method_marshal_list_names(
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
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_method_marshal_list_activatable_names(
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
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_method_marshal_add_match(
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
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_method_marshal_remove_match(
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
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_method_marshal_get_name_owner(
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
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_method_marshal_list_queued_owners(
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
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_method_marshal_get_connection_unix_user(
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
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_method_marshal_get_connection_unix_process_id(
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
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_method_marshal_get_connection_selinux_security_context(
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
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_method_marshal_update_activation_environment(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: ::core::ffi::c_uint,
    mut param_values: *const GValue,
    mut invocation_hint: *mut ::core::ffi::c_void,
    mut marshal_data: *mut ::core::ffi::c_void,
) {
    safe_c2rust__g_dbus_codegen_marshal_BOOLEAN__OBJECT_VARIANT(
        closure,
        return_value,
        n_param_values,
        param_values,
        invocation_hint,
        marshal_data,
    );
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_method_marshal_reload_config(
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
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_method_marshal_get_id(
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_get_type() -> GType {
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
            g_intern_static_string(b"_GFreedesktopDBus\0" as *const u8 as *const gchar),
            ::core::mem::size_of::<_GFreedesktopDBusInterface>() as guint,
            ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut _GFreedesktopDBusIface) -> ()>,
                    Option<unsafe extern "C" fn() -> ()>,
                >(Some(
                    safe_c2rust__g_freedesktop_dbus_default_init
                        as unsafe extern "C" fn(*mut _GFreedesktopDBusIface) -> (),
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
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_default_init(
    mut iface: *mut _GFreedesktopDBusIface,
) {
    g_signal_new(
        b"handle-hello\0" as *const u8 as *const gchar,
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
            safe_c2rust__g_freedesktop_dbus_method_marshal_hello
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
        b"handle-request-name\0" as *const u8 as *const gchar,
        (*(iface as *mut GTypeInterface)).g_type,
        G_SIGNAL_RUN_LAST,
        128 as ::core::ffi::c_ulong as glong as guint,
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
            safe_c2rust__g_freedesktop_dbus_method_marshal_request_name
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
        G_TYPE_STRING,
        G_TYPE_UINT,
    );
    g_signal_new(
        b"handle-release-name\0" as *const u8 as *const gchar,
        (*(iface as *mut GTypeInterface)).g_type,
        G_SIGNAL_RUN_LAST,
        104 as ::core::ffi::c_ulong as glong as guint,
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
            safe_c2rust__g_freedesktop_dbus_method_marshal_release_name
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
        b"handle-start-service-by-name\0" as *const u8 as *const gchar,
        (*(iface as *mut GTypeInterface)).g_type,
        G_SIGNAL_RUN_LAST,
        136 as ::core::ffi::c_ulong as glong as guint,
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
            safe_c2rust__g_freedesktop_dbus_method_marshal_start_service_by_name
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
        G_TYPE_STRING,
        G_TYPE_UINT,
    );
    g_signal_new(
        b"handle-name-has-owner\0" as *const u8 as *const gchar,
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
            safe_c2rust__g_freedesktop_dbus_method_marshal_name_has_owner
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
        b"handle-list-names\0" as *const u8 as *const gchar,
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
            safe_c2rust__g_freedesktop_dbus_method_marshal_list_names
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
        b"handle-list-activatable-names\0" as *const u8 as *const gchar,
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
            safe_c2rust__g_freedesktop_dbus_method_marshal_list_activatable_names
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
        b"handle-add-match\0" as *const u8 as *const gchar,
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
            safe_c2rust__g_freedesktop_dbus_method_marshal_add_match
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
        b"handle-remove-match\0" as *const u8 as *const gchar,
        (*(iface as *mut GTypeInterface)).g_type,
        G_SIGNAL_RUN_LAST,
        120 as ::core::ffi::c_ulong as glong as guint,
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
            safe_c2rust__g_freedesktop_dbus_method_marshal_remove_match
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
        b"handle-get-name-owner\0" as *const u8 as *const gchar,
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
            safe_c2rust__g_freedesktop_dbus_method_marshal_get_name_owner
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
        b"handle-list-queued-owners\0" as *const u8 as *const gchar,
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
            safe_c2rust__g_freedesktop_dbus_method_marshal_list_queued_owners
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
        b"handle-get-connection-unix-user\0" as *const u8 as *const gchar,
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
            safe_c2rust__g_freedesktop_dbus_method_marshal_get_connection_unix_user
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
        b"handle-get-connection-unix-process-id\0" as *const u8 as *const gchar,
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
            safe_c2rust__g_freedesktop_dbus_method_marshal_get_connection_unix_process_id
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
        b"handle-get-connection-selinux-security-context\0" as *const u8 as *const gchar,
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
            safe_c2rust__g_freedesktop_dbus_method_marshal_get_connection_selinux_security_context
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
        b"handle-update-activation-environment\0" as *const u8 as *const gchar,
        (*(iface as *mut GTypeInterface)).g_type,
        G_SIGNAL_RUN_LAST,
        144 as ::core::ffi::c_ulong as glong as guint,
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
            safe_c2rust__g_freedesktop_dbus_method_marshal_update_activation_environment
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
        G_TYPE_VARIANT,
    );
    g_signal_new(
        b"handle-reload-config\0" as *const u8 as *const gchar,
        (*(iface as *mut GTypeInterface)).g_type,
        G_SIGNAL_RUN_LAST,
        112 as ::core::ffi::c_ulong as glong as guint,
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
            safe_c2rust__g_freedesktop_dbus_method_marshal_reload_config
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
        b"handle-get-id\0" as *const u8 as *const gchar,
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
            safe_c2rust__g_freedesktop_dbus_method_marshal_get_id
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
    safe_c2rust__G__FREEDESKTOP_DBUS_SIGNALS
        [_G__FREEDESKTOP_DBUS_NAME_OWNER_CHANGED as ::core::ffi::c_int as usize] = g_signal_new(
        b"name-owner-changed\0" as *const u8 as *const gchar,
        (*(iface as *mut GTypeInterface)).g_type,
        G_SIGNAL_RUN_LAST,
        168 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL,
        Some(
            safe_c2rust__g_freedesktop_dbus_signal_marshal_name_owner_changed
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    ::core::ffi::c_uint,
                    *const GValue,
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        G_TYPE_NONE,
        3 as guint,
        G_TYPE_STRING,
        G_TYPE_STRING,
        G_TYPE_STRING,
    )
        as ::core::ffi::c_uint;
    safe_c2rust__G__FREEDESKTOP_DBUS_SIGNALS
        [_G__FREEDESKTOP_DBUS_NAME_LOST as ::core::ffi::c_int as usize] = g_signal_new(
        b"name-lost\0" as *const u8 as *const gchar,
        (*(iface as *mut GTypeInterface)).g_type,
        G_SIGNAL_RUN_LAST,
        160 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL,
        Some(
            safe_c2rust__g_freedesktop_dbus_signal_marshal_name_lost
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    ::core::ffi::c_uint,
                    *const GValue,
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        G_TYPE_NONE,
        1 as guint,
        G_TYPE_STRING,
    )
        as ::core::ffi::c_uint;
    safe_c2rust__G__FREEDESKTOP_DBUS_SIGNALS
        [_G__FREEDESKTOP_DBUS_NAME_ACQUIRED as ::core::ffi::c_int as usize] = g_signal_new(
        b"name-acquired\0" as *const u8 as *const gchar,
        (*(iface as *mut GTypeInterface)).g_type,
        G_SIGNAL_RUN_LAST,
        152 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL,
        Some(
            safe_c2rust__g_freedesktop_dbus_signal_marshal_name_acquired
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    ::core::ffi::c_uint,
                    *const GValue,
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        G_TYPE_NONE,
        1 as guint,
        G_TYPE_STRING,
    )
        as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_emit_name_owner_changed(
    mut object: *mut _GFreedesktopDBus,
    mut arg_name: *const gchar,
    mut arg_old_owner: *const gchar,
    mut arg_new_owner: *const gchar,
) {
    g_signal_emit(
        object as gpointer,
        safe_c2rust__G__FREEDESKTOP_DBUS_SIGNALS
            [_G__FREEDESKTOP_DBUS_NAME_OWNER_CHANGED as ::core::ffi::c_int as usize],
        0 as GQuark,
        arg_name,
        arg_old_owner,
        arg_new_owner,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_emit_name_lost(
    mut object: *mut _GFreedesktopDBus,
    mut arg_name: *const gchar,
) {
    g_signal_emit(
        object as gpointer,
        safe_c2rust__G__FREEDESKTOP_DBUS_SIGNALS
            [_G__FREEDESKTOP_DBUS_NAME_LOST as ::core::ffi::c_int as usize],
        0 as GQuark,
        arg_name,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_emit_name_acquired(
    mut object: *mut _GFreedesktopDBus,
    mut arg_name: *const gchar,
) {
    g_signal_emit(
        object as gpointer,
        safe_c2rust__G__FREEDESKTOP_DBUS_SIGNALS
            [_G__FREEDESKTOP_DBUS_NAME_ACQUIRED as ::core::ffi::c_int as usize],
        0 as GQuark,
        arg_name,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_hello(
    mut proxy: *mut _GFreedesktopDBus,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_dbus_proxy_call(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"Hello\0" as *const u8 as *const gchar,
        g_variant_new(b"()\0" as *const u8 as *const gchar),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_hello_finish(
    mut proxy: *mut _GFreedesktopDBus,
    mut out_assigned_name: *mut *mut gchar,
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
            b"(s)\0" as *const u8 as *const gchar,
            out_assigned_name,
        );
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_hello_sync(
    mut proxy: *mut _GFreedesktopDBus,
    mut out_assigned_name: *mut *mut gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_sync(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"Hello\0" as *const u8 as *const gchar,
        g_variant_new(b"()\0" as *const u8 as *const gchar),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(
            _ret,
            b"(s)\0" as *const u8 as *const gchar,
            out_assigned_name,
        );
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_request_name(
    mut proxy: *mut _GFreedesktopDBus,
    mut arg_name: *const gchar,
    mut arg_flags: guint,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_dbus_proxy_call(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"RequestName\0" as *const u8 as *const gchar,
        g_variant_new(b"(su)\0" as *const u8 as *const gchar, arg_name, arg_flags),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_request_name_finish(
    mut proxy: *mut _GFreedesktopDBus,
    mut out_value: *mut guint,
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
        g_variant_get(_ret, b"(u)\0" as *const u8 as *const gchar, out_value);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_request_name_sync(
    mut proxy: *mut _GFreedesktopDBus,
    mut arg_name: *const gchar,
    mut arg_flags: guint,
    mut out_value: *mut guint,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_sync(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"RequestName\0" as *const u8 as *const gchar,
        g_variant_new(b"(su)\0" as *const u8 as *const gchar, arg_name, arg_flags),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(_ret, b"(u)\0" as *const u8 as *const gchar, out_value);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_release_name(
    mut proxy: *mut _GFreedesktopDBus,
    mut arg_name: *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_dbus_proxy_call(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"ReleaseName\0" as *const u8 as *const gchar,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, arg_name),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_release_name_finish(
    mut proxy: *mut _GFreedesktopDBus,
    mut out_value: *mut guint,
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
        g_variant_get(_ret, b"(u)\0" as *const u8 as *const gchar, out_value);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_release_name_sync(
    mut proxy: *mut _GFreedesktopDBus,
    mut arg_name: *const gchar,
    mut out_value: *mut guint,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_sync(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"ReleaseName\0" as *const u8 as *const gchar,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, arg_name),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(_ret, b"(u)\0" as *const u8 as *const gchar, out_value);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_start_service_by_name(
    mut proxy: *mut _GFreedesktopDBus,
    mut arg_name: *const gchar,
    mut arg_flags: guint,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_dbus_proxy_call(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"StartServiceByName\0" as *const u8 as *const gchar,
        g_variant_new(b"(su)\0" as *const u8 as *const gchar, arg_name, arg_flags),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_start_service_by_name_finish(
    mut proxy: *mut _GFreedesktopDBus,
    mut out_value: *mut guint,
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
        g_variant_get(_ret, b"(u)\0" as *const u8 as *const gchar, out_value);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_start_service_by_name_sync(
    mut proxy: *mut _GFreedesktopDBus,
    mut arg_name: *const gchar,
    mut arg_flags: guint,
    mut out_value: *mut guint,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_sync(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"StartServiceByName\0" as *const u8 as *const gchar,
        g_variant_new(b"(su)\0" as *const u8 as *const gchar, arg_name, arg_flags),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(_ret, b"(u)\0" as *const u8 as *const gchar, out_value);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_name_has_owner(
    mut proxy: *mut _GFreedesktopDBus,
    mut arg_name: *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_dbus_proxy_call(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"NameHasOwner\0" as *const u8 as *const gchar,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, arg_name),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_name_has_owner_finish(
    mut proxy: *mut _GFreedesktopDBus,
    mut out_has_owner: *mut gboolean,
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
        g_variant_get(_ret, b"(b)\0" as *const u8 as *const gchar, out_has_owner);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_name_has_owner_sync(
    mut proxy: *mut _GFreedesktopDBus,
    mut arg_name: *const gchar,
    mut out_has_owner: *mut gboolean,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_sync(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"NameHasOwner\0" as *const u8 as *const gchar,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, arg_name),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(_ret, b"(b)\0" as *const u8 as *const gchar, out_has_owner);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_list_names(
    mut proxy: *mut _GFreedesktopDBus,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_dbus_proxy_call(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"ListNames\0" as *const u8 as *const gchar,
        g_variant_new(b"()\0" as *const u8 as *const gchar),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_list_names_finish(
    mut proxy: *mut _GFreedesktopDBus,
    mut out_names: *mut *mut *mut gchar,
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
        g_variant_get(_ret, b"(^as)\0" as *const u8 as *const gchar, out_names);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_list_names_sync(
    mut proxy: *mut _GFreedesktopDBus,
    mut out_names: *mut *mut *mut gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_sync(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"ListNames\0" as *const u8 as *const gchar,
        g_variant_new(b"()\0" as *const u8 as *const gchar),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(_ret, b"(^as)\0" as *const u8 as *const gchar, out_names);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_list_activatable_names(
    mut proxy: *mut _GFreedesktopDBus,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_dbus_proxy_call(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"ListActivatableNames\0" as *const u8 as *const gchar,
        g_variant_new(b"()\0" as *const u8 as *const gchar),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_list_activatable_names_finish(
    mut proxy: *mut _GFreedesktopDBus,
    mut out_activatable_names: *mut *mut *mut gchar,
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
            b"(^as)\0" as *const u8 as *const gchar,
            out_activatable_names,
        );
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_list_activatable_names_sync(
    mut proxy: *mut _GFreedesktopDBus,
    mut out_activatable_names: *mut *mut *mut gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_sync(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"ListActivatableNames\0" as *const u8 as *const gchar,
        g_variant_new(b"()\0" as *const u8 as *const gchar),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(
            _ret,
            b"(^as)\0" as *const u8 as *const gchar,
            out_activatable_names,
        );
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_add_match(
    mut proxy: *mut _GFreedesktopDBus,
    mut arg_rule: *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_dbus_proxy_call(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"AddMatch\0" as *const u8 as *const gchar,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, arg_rule),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_add_match_finish(
    mut proxy: *mut _GFreedesktopDBus,
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
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_add_match_sync(
    mut proxy: *mut _GFreedesktopDBus,
    mut arg_rule: *const gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_sync(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"AddMatch\0" as *const u8 as *const gchar,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, arg_rule),
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
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_remove_match(
    mut proxy: *mut _GFreedesktopDBus,
    mut arg_rule: *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_dbus_proxy_call(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"RemoveMatch\0" as *const u8 as *const gchar,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, arg_rule),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_remove_match_finish(
    mut proxy: *mut _GFreedesktopDBus,
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
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_remove_match_sync(
    mut proxy: *mut _GFreedesktopDBus,
    mut arg_rule: *const gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_sync(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"RemoveMatch\0" as *const u8 as *const gchar,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, arg_rule),
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
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_get_name_owner(
    mut proxy: *mut _GFreedesktopDBus,
    mut arg_name: *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_dbus_proxy_call(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"GetNameOwner\0" as *const u8 as *const gchar,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, arg_name),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_get_name_owner_finish(
    mut proxy: *mut _GFreedesktopDBus,
    mut out_unique_name: *mut *mut gchar,
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
        g_variant_get(_ret, b"(s)\0" as *const u8 as *const gchar, out_unique_name);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_get_name_owner_sync(
    mut proxy: *mut _GFreedesktopDBus,
    mut arg_name: *const gchar,
    mut out_unique_name: *mut *mut gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_sync(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"GetNameOwner\0" as *const u8 as *const gchar,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, arg_name),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(_ret, b"(s)\0" as *const u8 as *const gchar, out_unique_name);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_list_queued_owners(
    mut proxy: *mut _GFreedesktopDBus,
    mut arg_name: *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_dbus_proxy_call(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"ListQueuedOwners\0" as *const u8 as *const gchar,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, arg_name),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_list_queued_owners_finish(
    mut proxy: *mut _GFreedesktopDBus,
    mut out_queued_owners: *mut *mut *mut gchar,
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
            b"(^as)\0" as *const u8 as *const gchar,
            out_queued_owners,
        );
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_list_queued_owners_sync(
    mut proxy: *mut _GFreedesktopDBus,
    mut arg_name: *const gchar,
    mut out_queued_owners: *mut *mut *mut gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_sync(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"ListQueuedOwners\0" as *const u8 as *const gchar,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, arg_name),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(
            _ret,
            b"(^as)\0" as *const u8 as *const gchar,
            out_queued_owners,
        );
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_get_connection_unix_user(
    mut proxy: *mut _GFreedesktopDBus,
    mut arg_name: *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_dbus_proxy_call(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"GetConnectionUnixUser\0" as *const u8 as *const gchar,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, arg_name),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_get_connection_unix_user_finish(
    mut proxy: *mut _GFreedesktopDBus,
    mut out_uid: *mut guint,
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
        g_variant_get(_ret, b"(u)\0" as *const u8 as *const gchar, out_uid);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_get_connection_unix_user_sync(
    mut proxy: *mut _GFreedesktopDBus,
    mut arg_name: *const gchar,
    mut out_uid: *mut guint,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_sync(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"GetConnectionUnixUser\0" as *const u8 as *const gchar,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, arg_name),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(_ret, b"(u)\0" as *const u8 as *const gchar, out_uid);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_get_connection_unix_process_id(
    mut proxy: *mut _GFreedesktopDBus,
    mut arg_name: *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_dbus_proxy_call(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"GetConnectionUnixProcessID\0" as *const u8 as *const gchar,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, arg_name),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_get_connection_unix_process_id_finish(
    mut proxy: *mut _GFreedesktopDBus,
    mut out_pid: *mut guint,
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
        g_variant_get(_ret, b"(u)\0" as *const u8 as *const gchar, out_pid);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_get_connection_unix_process_id_sync(
    mut proxy: *mut _GFreedesktopDBus,
    mut arg_name: *const gchar,
    mut out_pid: *mut guint,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_sync(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"GetConnectionUnixProcessID\0" as *const u8 as *const gchar,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, arg_name),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(_ret, b"(u)\0" as *const u8 as *const gchar, out_pid);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_get_connection_selinux_security_context(
    mut proxy: *mut _GFreedesktopDBus,
    mut arg_name: *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_dbus_proxy_call(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"GetConnectionSELinuxSecurityContext\0" as *const u8 as *const gchar,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, arg_name),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_get_connection_selinux_security_context_finish(
    mut proxy: *mut _GFreedesktopDBus,
    mut out_security_context: *mut *mut gchar,
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
            b"(^ay)\0" as *const u8 as *const gchar,
            out_security_context,
        );
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_get_connection_selinux_security_context_sync(
    mut proxy: *mut _GFreedesktopDBus,
    mut arg_name: *const gchar,
    mut out_security_context: *mut *mut gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_sync(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"GetConnectionSELinuxSecurityContext\0" as *const u8 as *const gchar,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, arg_name),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(
            _ret,
            b"(^ay)\0" as *const u8 as *const gchar,
            out_security_context,
        );
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_update_activation_environment(
    mut proxy: *mut _GFreedesktopDBus,
    mut arg_environment: *mut GVariant,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_dbus_proxy_call(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"UpdateActivationEnvironment\0" as *const u8 as *const gchar,
        g_variant_new(b"(@a{ss})\0" as *const u8 as *const gchar, arg_environment),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_update_activation_environment_finish(
    mut proxy: *mut _GFreedesktopDBus,
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
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_update_activation_environment_sync(
    mut proxy: *mut _GFreedesktopDBus,
    mut arg_environment: *mut GVariant,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_sync(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"UpdateActivationEnvironment\0" as *const u8 as *const gchar,
        g_variant_new(b"(@a{ss})\0" as *const u8 as *const gchar, arg_environment),
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
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_reload_config(
    mut proxy: *mut _GFreedesktopDBus,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_dbus_proxy_call(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"ReloadConfig\0" as *const u8 as *const gchar,
        g_variant_new(b"()\0" as *const u8 as *const gchar),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_reload_config_finish(
    mut proxy: *mut _GFreedesktopDBus,
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
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_reload_config_sync(
    mut proxy: *mut _GFreedesktopDBus,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_sync(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"ReloadConfig\0" as *const u8 as *const gchar,
        g_variant_new(b"()\0" as *const u8 as *const gchar),
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
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_get_id(
    mut proxy: *mut _GFreedesktopDBus,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_dbus_proxy_call(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"GetId\0" as *const u8 as *const gchar,
        g_variant_new(b"()\0" as *const u8 as *const gchar),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_get_id_finish(
    mut proxy: *mut _GFreedesktopDBus,
    mut out_unique_id: *mut *mut gchar,
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
        g_variant_get(_ret, b"(s)\0" as *const u8 as *const gchar, out_unique_id);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_call_get_id_sync(
    mut proxy: *mut _GFreedesktopDBus,
    mut out_unique_id: *mut *mut gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    _ret = g_dbus_proxy_call_sync(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        b"GetId\0" as *const u8 as *const gchar,
        g_variant_new(b"()\0" as *const u8 as *const gchar),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        error,
    );
    if !_ret.is_null() {
        g_variant_get(_ret, b"(s)\0" as *const u8 as *const gchar, out_unique_id);
        g_variant_unref(_ret);
    }
    return (_ret != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_complete_hello(
    mut object: *mut _GFreedesktopDBus,
    mut invocation: *mut GDBusMethodInvocation,
    mut assigned_name: *const gchar,
) {
    g_dbus_method_invocation_return_value(
        invocation,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, assigned_name),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_complete_request_name(
    mut object: *mut _GFreedesktopDBus,
    mut invocation: *mut GDBusMethodInvocation,
    mut value: guint,
) {
    g_dbus_method_invocation_return_value(
        invocation,
        g_variant_new(b"(u)\0" as *const u8 as *const gchar, value),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_complete_release_name(
    mut object: *mut _GFreedesktopDBus,
    mut invocation: *mut GDBusMethodInvocation,
    mut value: guint,
) {
    g_dbus_method_invocation_return_value(
        invocation,
        g_variant_new(b"(u)\0" as *const u8 as *const gchar, value),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_complete_start_service_by_name(
    mut object: *mut _GFreedesktopDBus,
    mut invocation: *mut GDBusMethodInvocation,
    mut value: guint,
) {
    g_dbus_method_invocation_return_value(
        invocation,
        g_variant_new(b"(u)\0" as *const u8 as *const gchar, value),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_complete_name_has_owner(
    mut object: *mut _GFreedesktopDBus,
    mut invocation: *mut GDBusMethodInvocation,
    mut has_owner: gboolean,
) {
    g_dbus_method_invocation_return_value(
        invocation,
        g_variant_new(b"(b)\0" as *const u8 as *const gchar, has_owner),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_complete_list_names(
    mut object: *mut _GFreedesktopDBus,
    mut invocation: *mut GDBusMethodInvocation,
    mut names: *const *const gchar,
) {
    g_dbus_method_invocation_return_value(
        invocation,
        g_variant_new(b"(^as)\0" as *const u8 as *const gchar, names),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_complete_list_activatable_names(
    mut object: *mut _GFreedesktopDBus,
    mut invocation: *mut GDBusMethodInvocation,
    mut activatable_names: *const *const gchar,
) {
    g_dbus_method_invocation_return_value(
        invocation,
        g_variant_new(b"(^as)\0" as *const u8 as *const gchar, activatable_names),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_complete_add_match(
    mut object: *mut _GFreedesktopDBus,
    mut invocation: *mut GDBusMethodInvocation,
) {
    g_dbus_method_invocation_return_value(
        invocation,
        g_variant_new(b"()\0" as *const u8 as *const gchar),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_complete_remove_match(
    mut object: *mut _GFreedesktopDBus,
    mut invocation: *mut GDBusMethodInvocation,
) {
    g_dbus_method_invocation_return_value(
        invocation,
        g_variant_new(b"()\0" as *const u8 as *const gchar),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_complete_get_name_owner(
    mut object: *mut _GFreedesktopDBus,
    mut invocation: *mut GDBusMethodInvocation,
    mut unique_name: *const gchar,
) {
    g_dbus_method_invocation_return_value(
        invocation,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, unique_name),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_complete_list_queued_owners(
    mut object: *mut _GFreedesktopDBus,
    mut invocation: *mut GDBusMethodInvocation,
    mut queued_owners: *const *const gchar,
) {
    g_dbus_method_invocation_return_value(
        invocation,
        g_variant_new(b"(^as)\0" as *const u8 as *const gchar, queued_owners),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_complete_get_connection_unix_user(
    mut object: *mut _GFreedesktopDBus,
    mut invocation: *mut GDBusMethodInvocation,
    mut uid: guint,
) {
    g_dbus_method_invocation_return_value(
        invocation,
        g_variant_new(b"(u)\0" as *const u8 as *const gchar, uid),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_complete_get_connection_unix_process_id(
    mut object: *mut _GFreedesktopDBus,
    mut invocation: *mut GDBusMethodInvocation,
    mut pid: guint,
) {
    g_dbus_method_invocation_return_value(
        invocation,
        g_variant_new(b"(u)\0" as *const u8 as *const gchar, pid),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_complete_get_connection_selinux_security_context(
    mut object: *mut _GFreedesktopDBus,
    mut invocation: *mut GDBusMethodInvocation,
    mut security_context: *const gchar,
) {
    g_dbus_method_invocation_return_value(
        invocation,
        g_variant_new(b"(^ay)\0" as *const u8 as *const gchar, security_context),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_complete_update_activation_environment(
    mut object: *mut _GFreedesktopDBus,
    mut invocation: *mut GDBusMethodInvocation,
) {
    g_dbus_method_invocation_return_value(
        invocation,
        g_variant_new(b"()\0" as *const u8 as *const gchar),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_complete_reload_config(
    mut object: *mut _GFreedesktopDBus,
    mut invocation: *mut GDBusMethodInvocation,
) {
    g_dbus_method_invocation_return_value(
        invocation,
        g_variant_new(b"()\0" as *const u8 as *const gchar),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_complete_get_id(
    mut object: *mut _GFreedesktopDBus,
    mut invocation: *mut GDBusMethodInvocation,
    mut unique_id: *const gchar,
) {
    g_dbus_method_invocation_return_value(
        invocation,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, unique_id),
    );
}
static mut safe_c2rust__g_freedesktop_dbus_proxy_parent_class: gpointer = NULL;
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_proxy_class_intern_init(mut klass: gpointer) {
    safe_c2rust__g_freedesktop_dbus_proxy_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust__GFreedesktopDBusProxy_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust__GFreedesktopDBusProxy_private_offset,
        );
    }
    safe_c2rust__g_freedesktop_dbus_proxy_class_init(klass as *mut _GFreedesktopDBusProxyClass);
}
static mut safe_c2rust__GFreedesktopDBusProxy_private_offset: gint = 0;
#[inline]
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_proxy_get_instance_private(
    mut self_0: *mut _GFreedesktopDBusProxy,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust__GFreedesktopDBusProxy_private_offset as glong as isize)
        as gpointer;
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_proxy_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_dbus_proxy_get_type(),
        g_intern_static_string(b"_GFreedesktopDBusProxy\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<_GFreedesktopDBusProxyClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust__g_freedesktop_dbus_proxy_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<_GFreedesktopDBusProxy>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut _GFreedesktopDBusProxy) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust__g_freedesktop_dbus_proxy_init
                    as unsafe extern "C" fn(*mut _GFreedesktopDBusProxy) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust__GFreedesktopDBusProxy_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<_GFreedesktopDBusProxyPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut _GFreedesktopDBusIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust__g_freedesktop_dbus_proxy_iface_init
                as unsafe extern "C" fn(*mut _GFreedesktopDBusIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        safe_c2rust__g_freedesktop_dbus_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_proxy_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust__g_freedesktop_dbus_proxy_get_type_once();
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
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_proxy_finalize(mut object: *mut GObject) {
    let mut proxy: *mut _GFreedesktopDBusProxy =
        object as *mut ::core::ffi::c_void as *mut _GFreedesktopDBusProxy;
    g_datalist_clear(&raw mut (*(*proxy).priv_0).qdata);
    (*(safe_c2rust__g_freedesktop_dbus_proxy_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_proxy_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
}
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_proxy_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
}
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_proxy_g_signal(
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
        &raw const safe_c2rust___g_freedesktop_dbus_interface_info.parent_struct
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
        safe_c2rust__g_freedesktop_dbus_get_type(),
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
            let fresh0 = n;
            n = n.wrapping_add(1);
            g_dbus_gvariant_to_gvalue(child, paramv.offset(fresh0 as isize) as *mut GValue);
        }
        g_variant_unref(child);
    }
    signal_id = g_signal_lookup(
        (*info).signal_name,
        safe_c2rust__g_freedesktop_dbus_get_type(),
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
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_proxy_g_properties_changed(
    mut _proxy: *mut GDBusProxy,
    mut changed_properties: *mut GVariant,
    mut invalidated_properties: *const *const gchar,
) {
    let mut proxy: *mut _GFreedesktopDBusProxy =
        _proxy as *mut ::core::ffi::c_void as *mut _GFreedesktopDBusProxy;
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
            &raw const safe_c2rust___g_freedesktop_dbus_interface_info.parent_struct
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
            &raw const safe_c2rust___g_freedesktop_dbus_interface_info.parent_struct
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
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_proxy_init(
    mut proxy: *mut _GFreedesktopDBusProxy,
) {
    (*proxy).priv_0 = safe_c2rust__g_freedesktop_dbus_proxy_get_instance_private(proxy)
        as *mut _GFreedesktopDBusProxyPrivate;
    g_dbus_proxy_set_interface_info(
        proxy as *mut ::core::ffi::c_void as *mut GDBusProxy,
        safe_c2rust__g_freedesktop_dbus_interface_info(),
    );
}
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_proxy_class_init(
    mut klass: *mut _GFreedesktopDBusProxyClass,
) {
    let mut gobject_class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    let mut proxy_class: *mut GDBusProxyClass = ::core::ptr::null_mut::<GDBusProxyClass>();
    gobject_class = klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize = Some(
        safe_c2rust__g_freedesktop_dbus_proxy_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust__g_freedesktop_dbus_proxy_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust__g_freedesktop_dbus_proxy_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    proxy_class = klass as *mut ::core::ffi::c_void as *mut GDBusProxyClass;
    (*proxy_class).g_signal = Some(
        safe_c2rust__g_freedesktop_dbus_proxy_g_signal
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
        safe_c2rust__g_freedesktop_dbus_proxy_g_properties_changed
            as unsafe extern "C" fn(*mut GDBusProxy, *mut GVariant, *const *const gchar) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GDBusProxy, *mut GVariant, *const *const gchar) -> ()>;
}
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_proxy_iface_init(
    mut iface: *mut _GFreedesktopDBusIface,
) {
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_proxy_new(
    mut connection: *mut GDBusConnection,
    mut flags: GDBusProxyFlags,
    mut name: *const gchar,
    mut object_path: *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_async_initable_new_async(
        safe_c2rust__g_freedesktop_dbus_proxy_get_type(),
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
        b"org.freedesktop.DBus\0" as *const u8 as *const ::core::ffi::c_char,
        NULL,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_proxy_new_finish(
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut _GFreedesktopDBus {
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
        return ret as *mut ::core::ffi::c_void as *mut _GFreedesktopDBus;
    } else {
        return ::core::ptr::null_mut::<_GFreedesktopDBus>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_proxy_new_sync(
    mut connection: *mut GDBusConnection,
    mut flags: GDBusProxyFlags,
    mut name: *const gchar,
    mut object_path: *const gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut _GFreedesktopDBus {
    let mut ret: *mut GInitable = ::core::ptr::null_mut::<GInitable>();
    ret = g_initable_new(
        safe_c2rust__g_freedesktop_dbus_proxy_get_type(),
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
        b"org.freedesktop.DBus\0" as *const u8 as *const ::core::ffi::c_char,
        NULL,
    ) as *mut GInitable;
    if !ret.is_null() {
        return ret as *mut ::core::ffi::c_void as *mut _GFreedesktopDBus;
    } else {
        return ::core::ptr::null_mut::<_GFreedesktopDBus>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_proxy_new_for_bus(
    mut bus_type: GBusType,
    mut flags: GDBusProxyFlags,
    mut name: *const gchar,
    mut object_path: *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    g_async_initable_new_async(
        safe_c2rust__g_freedesktop_dbus_proxy_get_type(),
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
        b"org.freedesktop.DBus\0" as *const u8 as *const ::core::ffi::c_char,
        NULL,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_proxy_new_for_bus_finish(
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut _GFreedesktopDBus {
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
        return ret as *mut ::core::ffi::c_void as *mut _GFreedesktopDBus;
    } else {
        return ::core::ptr::null_mut::<_GFreedesktopDBus>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_proxy_new_for_bus_sync(
    mut bus_type: GBusType,
    mut flags: GDBusProxyFlags,
    mut name: *const gchar,
    mut object_path: *const gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut _GFreedesktopDBus {
    let mut ret: *mut GInitable = ::core::ptr::null_mut::<GInitable>();
    ret = g_initable_new(
        safe_c2rust__g_freedesktop_dbus_proxy_get_type(),
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
        b"org.freedesktop.DBus\0" as *const u8 as *const ::core::ffi::c_char,
        NULL,
    ) as *mut GInitable;
    if !ret.is_null() {
        return ret as *mut ::core::ffi::c_void as *mut _GFreedesktopDBus;
    } else {
        return ::core::ptr::null_mut::<_GFreedesktopDBus>();
    };
}
unsafe extern "C" fn safe_c2rust___g_freedesktop_dbus_skeleton_handle_method_call(
    mut connection: *mut GDBusConnection,
    mut sender: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut method_name: *const gchar,
    mut parameters: *mut GVariant,
    mut invocation: *mut GDBusMethodInvocation,
    mut user_data: gpointer,
) {
    let mut skeleton: *mut _GFreedesktopDBusSkeleton = user_data as *mut _GFreedesktopDBusSkeleton;
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
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if !info.is_null() {
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
            b"gio/gdbus-daemon-generated.c\0" as *const u8 as *const ::core::ffi::c_char,
            4561 as ::core::ffi::c_int,
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
        safe_c2rust__g_freedesktop_dbus_get_type(),
    );
    let fresh1 = n;
    n = n.wrapping_add(1);
    g_value_set_object(
        paramv.offset(fresh1 as isize) as *mut GValue,
        skeleton as gpointer,
    );
    g_value_init(
        paramv.offset(n as isize) as *mut GValue,
        g_dbus_method_invocation_get_type(),
    );
    let fresh2 = n;
    n = n.wrapping_add(1);
    g_value_set_object(
        paramv.offset(fresh2 as isize) as *mut GValue,
        invocation as gpointer,
    );
    if (*info).pass_fdlist != 0 {
        g_value_init(
            paramv.offset(n as isize) as *mut GValue,
            g_unix_fd_list_get_type(),
        );
        let fresh3 = n;
        n = n.wrapping_add(1);
        g_value_set_object(
            paramv.offset(fresh3 as isize) as *mut GValue,
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
            let fresh4 = n;
            n = n.wrapping_add(1);
            g_dbus_gvariant_to_gvalue(child, paramv.offset(fresh4 as isize) as *mut GValue);
        }
        g_variant_unref(child);
    }
    signal_id = g_signal_lookup(
        (*info).signal_name,
        safe_c2rust__g_freedesktop_dbus_get_type(),
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
unsafe extern "C" fn safe_c2rust___g_freedesktop_dbus_skeleton_handle_get_property(
    mut connection: *mut GDBusConnection,
    mut sender: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut property_name: *const gchar,
    mut error: *mut *mut GError,
    mut user_data: gpointer,
) -> *mut GVariant {
    let mut skeleton: *mut _GFreedesktopDBusSkeleton = user_data as *mut _GFreedesktopDBusSkeleton;
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
        &raw const safe_c2rust___g_freedesktop_dbus_interface_info.parent_struct
            as *mut GDBusInterfaceInfo,
        property_name,
    ) as *mut _ExtendedGDBusPropertyInfo;
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !info.is_null() {
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
            b"gio/gdbus-daemon-generated.c\0" as *const u8 as *const ::core::ffi::c_char,
            4620 as ::core::ffi::c_int,
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
unsafe extern "C" fn safe_c2rust___g_freedesktop_dbus_skeleton_handle_set_property(
    mut connection: *mut GDBusConnection,
    mut sender: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut property_name: *const gchar,
    mut variant: *mut GVariant,
    mut error: *mut *mut GError,
    mut user_data: gpointer,
) -> gboolean {
    let mut skeleton: *mut _GFreedesktopDBusSkeleton = user_data as *mut _GFreedesktopDBusSkeleton;
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
        &raw const safe_c2rust___g_freedesktop_dbus_interface_info.parent_struct
            as *mut GDBusInterfaceInfo,
        property_name,
    ) as *mut _ExtendedGDBusPropertyInfo;
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if !info.is_null() {
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
            b"gio/gdbus-daemon-generated.c\0" as *const u8 as *const ::core::ffi::c_char,
            4654 as ::core::ffi::c_int,
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
static mut safe_c2rust___g_freedesktop_dbus_skeleton_vtable: GDBusInterfaceVTable = unsafe {
    _GDBusInterfaceVTable {
        method_call: Some(
            safe_c2rust___g_freedesktop_dbus_skeleton_handle_method_call
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
            safe_c2rust___g_freedesktop_dbus_skeleton_handle_get_property
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
            safe_c2rust___g_freedesktop_dbus_skeleton_handle_set_property
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
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_skeleton_dbus_interface_get_info(
    mut skeleton: *mut GDBusInterfaceSkeleton,
) -> *mut GDBusInterfaceInfo {
    return safe_c2rust__g_freedesktop_dbus_interface_info();
}
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_skeleton_dbus_interface_get_vtable(
    mut skeleton: *mut GDBusInterfaceSkeleton,
) -> *mut GDBusInterfaceVTable {
    return &raw const safe_c2rust___g_freedesktop_dbus_skeleton_vtable
        as *mut GDBusInterfaceVTable;
}
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_skeleton_dbus_interface_get_properties(
    mut _skeleton: *mut GDBusInterfaceSkeleton,
) -> *mut GVariant {
    let mut skeleton: *mut _GFreedesktopDBusSkeleton =
        _skeleton as *mut ::core::ffi::c_void as *mut _GFreedesktopDBusSkeleton;
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
    if !safe_c2rust___g_freedesktop_dbus_interface_info
        .parent_struct
        .properties
        .is_null()
    {
        n = 0 as guint;
        while !(*safe_c2rust___g_freedesktop_dbus_interface_info
            .parent_struct
            .properties
            .offset(n as isize))
        .is_null()
        {
            let mut info: *mut GDBusPropertyInfo = *safe_c2rust___g_freedesktop_dbus_interface_info
                .parent_struct
                .properties
                .offset(n as isize);
            if (*info).flags as ::core::ffi::c_uint
                & G_DBUS_PROPERTY_INFO_FLAGS_READABLE as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
            {
                let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
                value = safe_c2rust___g_freedesktop_dbus_skeleton_handle_get_property(
                    g_dbus_interface_skeleton_get_connection(
                        skeleton as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton,
                    ),
                    ::core::ptr::null::<gchar>(),
                    g_dbus_interface_skeleton_get_object_path(
                        skeleton as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton,
                    ),
                    b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
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
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_skeleton_dbus_interface_flush(
    mut _skeleton: *mut GDBusInterfaceSkeleton,
) {
}
unsafe extern "C" fn safe_c2rust___g_freedesktop_dbus_on_signal_name_owner_changed(
    mut object: *mut _GFreedesktopDBus,
    mut arg_name: *const gchar,
    mut arg_old_owner: *const gchar,
    mut arg_new_owner: *const gchar,
) {
    let mut skeleton: *mut _GFreedesktopDBusSkeleton =
        object as *mut ::core::ffi::c_void as *mut _GFreedesktopDBusSkeleton;
    let mut connections: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut signal_variant: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    connections = g_dbus_interface_skeleton_get_connections(
        skeleton as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton,
    );
    signal_variant = g_variant_ref_sink(g_variant_new(
        b"(sss)\0" as *const u8 as *const gchar,
        arg_name,
        arg_old_owner,
        arg_new_owner,
    ));
    l = connections;
    while !l.is_null() {
        let mut connection: *mut GDBusConnection = (*l).data as *mut GDBusConnection;
        g_dbus_connection_emit_signal(
            connection,
            ::core::ptr::null::<gchar>(),
            g_dbus_interface_skeleton_get_object_path(
                skeleton as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton,
            ),
            b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
            b"NameOwnerChanged\0" as *const u8 as *const gchar,
            signal_variant,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        l = (*l).next;
    }
    g_variant_unref(signal_variant);
    g_list_free_full(
        connections,
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
}
unsafe extern "C" fn safe_c2rust___g_freedesktop_dbus_on_signal_name_lost(
    mut object: *mut _GFreedesktopDBus,
    mut arg_name: *const gchar,
) {
    let mut skeleton: *mut _GFreedesktopDBusSkeleton =
        object as *mut ::core::ffi::c_void as *mut _GFreedesktopDBusSkeleton;
    let mut connections: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut signal_variant: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    connections = g_dbus_interface_skeleton_get_connections(
        skeleton as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton,
    );
    signal_variant = g_variant_ref_sink(g_variant_new(
        b"(s)\0" as *const u8 as *const gchar,
        arg_name,
    ));
    l = connections;
    while !l.is_null() {
        let mut connection: *mut GDBusConnection = (*l).data as *mut GDBusConnection;
        g_dbus_connection_emit_signal(
            connection,
            ::core::ptr::null::<gchar>(),
            g_dbus_interface_skeleton_get_object_path(
                skeleton as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton,
            ),
            b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
            b"NameLost\0" as *const u8 as *const gchar,
            signal_variant,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        l = (*l).next;
    }
    g_variant_unref(signal_variant);
    g_list_free_full(
        connections,
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
}
unsafe extern "C" fn safe_c2rust___g_freedesktop_dbus_on_signal_name_acquired(
    mut object: *mut _GFreedesktopDBus,
    mut arg_name: *const gchar,
) {
    let mut skeleton: *mut _GFreedesktopDBusSkeleton =
        object as *mut ::core::ffi::c_void as *mut _GFreedesktopDBusSkeleton;
    let mut connections: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut signal_variant: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    connections = g_dbus_interface_skeleton_get_connections(
        skeleton as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton,
    );
    signal_variant = g_variant_ref_sink(g_variant_new(
        b"(s)\0" as *const u8 as *const gchar,
        arg_name,
    ));
    l = connections;
    while !l.is_null() {
        let mut connection: *mut GDBusConnection = (*l).data as *mut GDBusConnection;
        g_dbus_connection_emit_signal(
            connection,
            ::core::ptr::null::<gchar>(),
            g_dbus_interface_skeleton_get_object_path(
                skeleton as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton,
            ),
            b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
            b"NameAcquired\0" as *const u8 as *const gchar,
            signal_variant,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        l = (*l).next;
    }
    g_variant_unref(signal_variant);
    g_list_free_full(
        connections,
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
}
static mut safe_c2rust__GFreedesktopDBusSkeleton_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_skeleton_class_intern_init(
    mut klass: gpointer,
) {
    safe_c2rust__g_freedesktop_dbus_skeleton_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust__GFreedesktopDBusSkeleton_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust__GFreedesktopDBusSkeleton_private_offset,
        );
    }
    safe_c2rust__g_freedesktop_dbus_skeleton_class_init(
        klass as *mut _GFreedesktopDBusSkeletonClass,
    );
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_skeleton_get_instance_private(
    mut self_0: *mut _GFreedesktopDBusSkeleton,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust__GFreedesktopDBusSkeleton_private_offset as glong as isize)
        as gpointer;
}
static mut safe_c2rust__g_freedesktop_dbus_skeleton_parent_class: gpointer = NULL;
#[inline(never)]
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_skeleton_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_dbus_interface_skeleton_get_type(),
        g_intern_static_string(b"_GFreedesktopDBusSkeleton\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<_GFreedesktopDBusSkeletonClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust__g_freedesktop_dbus_skeleton_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<_GFreedesktopDBusSkeleton>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut _GFreedesktopDBusSkeleton) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust__g_freedesktop_dbus_skeleton_init
                    as unsafe extern "C" fn(*mut _GFreedesktopDBusSkeleton) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust__GFreedesktopDBusSkeleton_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<_GFreedesktopDBusSkeletonPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut _GFreedesktopDBusIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust__g_freedesktop_dbus_skeleton_iface_init
                as unsafe extern "C" fn(*mut _GFreedesktopDBusIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        safe_c2rust__g_freedesktop_dbus_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_skeleton_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust__g_freedesktop_dbus_skeleton_get_type_once();
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
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_skeleton_finalize(mut object: *mut GObject) {
    let mut skeleton: *mut _GFreedesktopDBusSkeleton =
        object as *mut ::core::ffi::c_void as *mut _GFreedesktopDBusSkeleton;
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
    (*(safe_c2rust__g_freedesktop_dbus_skeleton_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_skeleton_init(
    mut skeleton: *mut _GFreedesktopDBusSkeleton,
) {
    (*skeleton).priv_0 = safe_c2rust__g_freedesktop_dbus_skeleton_get_instance_private(skeleton)
        as *mut _GFreedesktopDBusSkeletonPrivate;
    g_mutex_init(&raw mut (*(*skeleton).priv_0).lock);
    (*(*skeleton).priv_0).context = g_main_context_ref_thread_default();
}
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_skeleton_class_init(
    mut klass: *mut _GFreedesktopDBusSkeletonClass,
) {
    let mut gobject_class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    let mut skeleton_class: *mut GDBusInterfaceSkeletonClass =
        ::core::ptr::null_mut::<GDBusInterfaceSkeletonClass>();
    gobject_class = klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize = Some(
        safe_c2rust__g_freedesktop_dbus_skeleton_finalize
            as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    skeleton_class = klass as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeletonClass;
    (*skeleton_class).get_info = Some(
        safe_c2rust__g_freedesktop_dbus_skeleton_dbus_interface_get_info
            as unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GDBusInterfaceInfo,
    )
        as Option<unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GDBusInterfaceInfo>;
    (*skeleton_class).get_properties = Some(
        safe_c2rust__g_freedesktop_dbus_skeleton_dbus_interface_get_properties
            as unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GVariant,
    )
        as Option<unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GVariant>;
    (*skeleton_class).flush = Some(
        safe_c2rust__g_freedesktop_dbus_skeleton_dbus_interface_flush
            as unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> ()>;
    (*skeleton_class).get_vtable = Some(
        safe_c2rust__g_freedesktop_dbus_skeleton_dbus_interface_get_vtable
            as unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GDBusInterfaceVTable,
    )
        as Option<unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GDBusInterfaceVTable>;
}
unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_skeleton_iface_init(
    mut iface: *mut _GFreedesktopDBusIface,
) {
    (*iface).name_owner_changed = Some(
        safe_c2rust___g_freedesktop_dbus_on_signal_name_owner_changed
            as unsafe extern "C" fn(
                *mut _GFreedesktopDBus,
                *const gchar,
                *const gchar,
                *const gchar,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut _GFreedesktopDBus,
                *const gchar,
                *const gchar,
                *const gchar,
            ) -> (),
        >;
    (*iface).name_lost = Some(
        safe_c2rust___g_freedesktop_dbus_on_signal_name_lost
            as unsafe extern "C" fn(*mut _GFreedesktopDBus, *const gchar) -> (),
    )
        as Option<unsafe extern "C" fn(*mut _GFreedesktopDBus, *const gchar) -> ()>;
    (*iface).name_acquired = Some(
        safe_c2rust___g_freedesktop_dbus_on_signal_name_acquired
            as unsafe extern "C" fn(*mut _GFreedesktopDBus, *const gchar) -> (),
    )
        as Option<unsafe extern "C" fn(*mut _GFreedesktopDBus, *const gchar) -> ()>;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_freedesktop_dbus_skeleton_new() -> *mut _GFreedesktopDBus {
    return g_object_new(
        safe_c2rust__g_freedesktop_dbus_skeleton_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut _GFreedesktopDBus;
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
    safe_c2rust___g_freedesktop_dbus_signal_info_pointers = [
        &raw const safe_c2rust___g_freedesktop_dbus_signal_info_name_owner_changed.parent_struct,
        &raw const safe_c2rust___g_freedesktop_dbus_signal_info_name_lost.parent_struct,
        &raw const safe_c2rust___g_freedesktop_dbus_signal_info_name_acquired.parent_struct,
        ::core::ptr::null::<GDBusSignalInfo>(),
    ];
    safe_c2rust___g_freedesktop_dbus_signal_info_name_acquired_ARG_pointers = [
        &raw const safe_c2rust___g_freedesktop_dbus_signal_info_name_acquired_ARG_name
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust___g_freedesktop_dbus_signal_info_name_lost_ARG_pointers = [
        &raw const safe_c2rust___g_freedesktop_dbus_signal_info_name_lost_ARG_name.parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust___g_freedesktop_dbus_signal_info_name_owner_changed_ARG_pointers = [
        &raw const safe_c2rust___g_freedesktop_dbus_signal_info_name_owner_changed_ARG_name
            .parent_struct,
        &raw const safe_c2rust___g_freedesktop_dbus_signal_info_name_owner_changed_ARG_old_owner
            .parent_struct,
        &raw const safe_c2rust___g_freedesktop_dbus_signal_info_name_owner_changed_ARG_new_owner
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust___g_freedesktop_dbus_method_info_pointers = [
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_hello.parent_struct,
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_request_name
            .parent_struct,
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_release_name
            .parent_struct,
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_start_service_by_name
            .parent_struct,
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_name_has_owner
            .parent_struct,
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_list_names.parent_struct,
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_list_activatable_names
            .parent_struct,
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_add_match.parent_struct,
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_remove_match
            .parent_struct,
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_get_name_owner
            .parent_struct,
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_list_queued_owners
            .parent_struct,
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_get_connection_unix_user
            .parent_struct,
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_get_connection_unix_process_id
            .parent_struct,
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_get_connection_selinux_security_context
            .parent_struct,
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_update_activation_environment
            .parent_struct,
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_reload_config
            .parent_struct,
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_get_id.parent_struct,
        ::core::ptr::null::<GDBusMethodInfo>(),
    ];
    safe_c2rust___g_freedesktop_dbus_method_info_get_id_OUT_ARG_pointers = [
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_get_id_OUT_ARG_unique_id
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust___g_freedesktop_dbus_method_info_update_activation_environment_IN_ARG_pointers = [
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_update_activation_environment_IN_ARG_environment
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust___g_freedesktop_dbus_method_info_get_connection_selinux_security_context_OUT_ARG_pointers = [
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_get_connection_selinux_security_context_OUT_ARG_security_context
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust___g_freedesktop_dbus_method_info_get_connection_selinux_security_context_IN_ARG_pointers = [
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_get_connection_selinux_security_context_IN_ARG_name
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust___g_freedesktop_dbus_method_info_get_connection_unix_process_id_OUT_ARG_pointers = [
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_get_connection_unix_process_id_OUT_ARG_pid
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust___g_freedesktop_dbus_method_info_get_connection_unix_process_id_IN_ARG_pointers = [
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_get_connection_unix_process_id_IN_ARG_name
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust___g_freedesktop_dbus_method_info_get_connection_unix_user_OUT_ARG_pointers = [
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_get_connection_unix_user_OUT_ARG_uid
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust___g_freedesktop_dbus_method_info_get_connection_unix_user_IN_ARG_pointers = [
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_get_connection_unix_user_IN_ARG_name
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust___g_freedesktop_dbus_method_info_list_queued_owners_OUT_ARG_pointers = [
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_list_queued_owners_OUT_ARG_queued_owners
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust___g_freedesktop_dbus_method_info_list_queued_owners_IN_ARG_pointers = [
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_list_queued_owners_IN_ARG_name
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust___g_freedesktop_dbus_method_info_get_name_owner_OUT_ARG_pointers = [
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_get_name_owner_OUT_ARG_unique_name
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust___g_freedesktop_dbus_method_info_get_name_owner_IN_ARG_pointers = [
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_get_name_owner_IN_ARG_name
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust___g_freedesktop_dbus_method_info_remove_match_IN_ARG_pointers = [
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_remove_match_IN_ARG_rule
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust___g_freedesktop_dbus_method_info_add_match_IN_ARG_pointers = [
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_add_match_IN_ARG_rule.parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust___g_freedesktop_dbus_method_info_list_activatable_names_OUT_ARG_pointers = [
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_list_activatable_names_OUT_ARG_activatable_names
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust___g_freedesktop_dbus_method_info_list_names_OUT_ARG_pointers = [
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_list_names_OUT_ARG_names
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust___g_freedesktop_dbus_method_info_name_has_owner_OUT_ARG_pointers = [
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_name_has_owner_OUT_ARG_has_owner
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust___g_freedesktop_dbus_method_info_name_has_owner_IN_ARG_pointers = [
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_name_has_owner_IN_ARG_name
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust___g_freedesktop_dbus_method_info_start_service_by_name_OUT_ARG_pointers = [
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_start_service_by_name_OUT_ARG_value
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust___g_freedesktop_dbus_method_info_start_service_by_name_IN_ARG_pointers = [
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_start_service_by_name_IN_ARG_name
            .parent_struct,
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_start_service_by_name_IN_ARG_flags
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust___g_freedesktop_dbus_method_info_release_name_OUT_ARG_pointers = [
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_release_name_OUT_ARG_value
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust___g_freedesktop_dbus_method_info_release_name_IN_ARG_pointers = [
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_release_name_IN_ARG_name
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust___g_freedesktop_dbus_method_info_request_name_OUT_ARG_pointers = [
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_request_name_OUT_ARG_value
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust___g_freedesktop_dbus_method_info_request_name_IN_ARG_pointers = [
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_request_name_IN_ARG_name
            .parent_struct,
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_request_name_IN_ARG_flags
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
    safe_c2rust___g_freedesktop_dbus_method_info_hello_OUT_ARG_pointers = [
        &raw const safe_c2rust___g_freedesktop_dbus_method_info_hello_OUT_ARG_assigned_name
            .parent_struct,
        ::core::ptr::null::<GDBusArgInfo>(),
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
