use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _GInitable;
    pub type _GIOExtension;
    pub type _GNetworkAddressPrivate;
    pub type _GNetworkMonitor;
    pub type _GSocketConnectable;
    pub type _GTask;
    pub type _GDBusProxyPrivate;
    pub type _GNetworkMonitorBasePrivate;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_matches(error: *const GError, domain: GQuark, code: gint) -> gboolean;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_clear_error(err: *mut *mut GError);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_variant_type_checked_(type_string: *const gchar) -> *const GVariantType;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_is_of_type(value: *mut GVariant, type_0: *const GVariantType) -> gboolean;
    fn g_variant_get_boolean(value: *mut GVariant) -> gboolean;
    fn g_variant_get_uint32(value: *mut GVariant) -> guint32;
    fn g_variant_lookup(
        dictionary: *mut GVariant,
        key: *const gchar,
        format_string: *const gchar,
        ...
    ) -> gboolean;
    fn g_variant_new(format_string: *const gchar, ...) -> *mut GVariant;
    fn g_variant_get(value: *mut GVariant, format_string: *const gchar, ...);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_type_name(type_0: GType) -> *const gchar;
    fn g_type_class_ref(type_0: GType) -> gpointer;
    fn g_type_class_unref(g_class: gpointer);
    fn g_type_class_peek_parent(g_class: gpointer) -> gpointer;
    fn g_type_interface_peek_parent(g_iface: gpointer) -> gpointer;
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
    fn g_signal_emit_by_name(instance: gpointer, detailed_signal: *const gchar, ...);
    fn g_signal_connect_data(
        instance: gpointer,
        detailed_signal: *const gchar,
        c_handler: GCallback,
        data: gpointer,
        destroy_data: GClosureNotify,
        connect_flags: GConnectFlags,
    ) -> gulong;
    fn g_object_class_override_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        name: *const gchar,
    );
    fn g_object_freeze_notify(object: *mut GObject);
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
    fn g_object_thaw_notify(object: *mut GObject);
    fn g_object_unref(object: gpointer);
    fn g_enum_get_value(enum_class: *mut GEnumClass, value: gint) -> *mut GEnumValue;
    fn g_value_set_enum(value: *mut GValue, v_enum: gint);
    fn g_value_set_boolean(value: *mut GValue, v_boolean: gboolean);
    fn g_network_monitor_base_get_type() -> GType;
    fn g_initable_get_type() -> GType;
    fn g_io_extension_point_implement(
        extension_point_name: *const ::core::ffi::c_char,
        type_0: GType,
        extension_name: *const ::core::ffi::c_char,
        priority: gint,
    ) -> *mut GIOExtension;
    fn _g_io_modules_ensure_extension_points_registered();
    fn g_dbus_error_quark() -> GQuark;
    fn g_dbus_proxy_new_for_bus_sync(
        bus_type: GBusType,
        flags: GDBusProxyFlags,
        info: *mut GDBusInterfaceInfo,
        name: *const gchar,
        object_path: *const gchar,
        interface_name: *const gchar,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GDBusProxy;
    fn g_dbus_proxy_get_name_owner(proxy: *mut GDBusProxy) -> *mut gchar;
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
    fn g_io_error_quark() -> GQuark;
    fn g_network_connectivity_get_type() -> GType;
    fn g_network_address_get_type() -> GType;
    fn g_network_address_get_hostname(addr: *mut GNetworkAddress) -> *const gchar;
    fn g_network_address_get_port(addr: *mut GNetworkAddress) -> guint16;
    fn g_network_monitor_get_type() -> GType;
    fn g_task_new(
        source_object: gpointer,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        callback_data: gpointer,
    ) -> *mut GTask;
    fn g_task_get_source_object(task: *mut GTask) -> gpointer;
    fn g_task_return_boolean(task: *mut GTask, result: gboolean);
    fn g_task_return_error(task: *mut GTask, error: *mut GError);
    fn g_task_return_new_error(
        task: *mut GTask,
        domain: GQuark,
        code: gint,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn g_task_return_new_error_literal(
        task: *mut GTask,
        domain: GQuark,
        code: gint,
        message: *const ::core::ffi::c_char,
    );
    fn g_task_propagate_boolean(task: *mut GTask, error: *mut *mut GError) -> gboolean;
    fn glib_should_use_portal() -> gboolean;
    fn glib_network_available_in_sandbox() -> gboolean;
}
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
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GError = _GError;
pub type GData = _GData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type GVariantType = _GVariantType;
pub type GVariant = _GVariant;
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
pub struct _GEnumClass {
    pub g_type_class: GTypeClass,
    pub minimum: gint,
    pub maximum: gint,
    pub n_values: guint,
    pub values: *mut GEnumValue,
}
pub type GEnumValue = _GEnumValue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GEnumValue {
    pub value: gint,
    pub value_name: *const gchar,
    pub value_nick: *const gchar,
}
pub type GEnumClass = _GEnumClass;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const G_IO_ERROR_DESTINATION_UNSET: C2RustUnnamed_0 = 48;
pub const G_IO_ERROR_NO_SUCH_DEVICE: C2RustUnnamed_0 = 47;
pub const G_IO_ERROR_MESSAGE_TOO_LARGE: C2RustUnnamed_0 = 46;
pub const G_IO_ERROR_NOT_CONNECTED: C2RustUnnamed_0 = 45;
pub const G_IO_ERROR_CONNECTION_CLOSED: C2RustUnnamed_0 = 44;
pub const G_IO_ERROR_BROKEN_PIPE: C2RustUnnamed_0 = 44;
pub const G_IO_ERROR_PROXY_NOT_ALLOWED: C2RustUnnamed_0 = 43;
pub const G_IO_ERROR_PROXY_NEED_AUTH: C2RustUnnamed_0 = 42;
pub const G_IO_ERROR_PROXY_AUTH_FAILED: C2RustUnnamed_0 = 41;
pub const G_IO_ERROR_PROXY_FAILED: C2RustUnnamed_0 = 40;
pub const G_IO_ERROR_CONNECTION_REFUSED: C2RustUnnamed_0 = 39;
pub const G_IO_ERROR_NETWORK_UNREACHABLE: C2RustUnnamed_0 = 38;
pub const G_IO_ERROR_HOST_UNREACHABLE: C2RustUnnamed_0 = 37;
pub const G_IO_ERROR_DBUS_ERROR: C2RustUnnamed_0 = 36;
pub const G_IO_ERROR_INVALID_DATA: C2RustUnnamed_0 = 35;
pub const G_IO_ERROR_PARTIAL_INPUT: C2RustUnnamed_0 = 34;
pub const G_IO_ERROR_ADDRESS_IN_USE: C2RustUnnamed_0 = 33;
pub const G_IO_ERROR_NOT_INITIALIZED: C2RustUnnamed_0 = 32;
pub const G_IO_ERROR_TOO_MANY_OPEN_FILES: C2RustUnnamed_0 = 31;
pub const G_IO_ERROR_FAILED_HANDLED: C2RustUnnamed_0 = 30;
pub const G_IO_ERROR_WOULD_MERGE: C2RustUnnamed_0 = 29;
pub const G_IO_ERROR_HOST_NOT_FOUND: C2RustUnnamed_0 = 28;
pub const G_IO_ERROR_WOULD_BLOCK: C2RustUnnamed_0 = 27;
pub const G_IO_ERROR_BUSY: C2RustUnnamed_0 = 26;
pub const G_IO_ERROR_WOULD_RECURSE: C2RustUnnamed_0 = 25;
pub const G_IO_ERROR_TIMED_OUT: C2RustUnnamed_0 = 24;
pub const G_IO_ERROR_WRONG_ETAG: C2RustUnnamed_0 = 23;
pub const G_IO_ERROR_CANT_CREATE_BACKUP: C2RustUnnamed_0 = 22;
pub const G_IO_ERROR_READ_ONLY: C2RustUnnamed_0 = 21;
pub const G_IO_ERROR_PENDING: C2RustUnnamed_0 = 20;
pub const G_IO_ERROR_CANCELLED: C2RustUnnamed_0 = 19;
pub const G_IO_ERROR_CLOSED: C2RustUnnamed_0 = 18;
pub const G_IO_ERROR_ALREADY_MOUNTED: C2RustUnnamed_0 = 17;
pub const G_IO_ERROR_NOT_MOUNTED: C2RustUnnamed_0 = 16;
pub const G_IO_ERROR_NOT_SUPPORTED: C2RustUnnamed_0 = 15;
pub const G_IO_ERROR_PERMISSION_DENIED: C2RustUnnamed_0 = 14;
pub const G_IO_ERROR_INVALID_ARGUMENT: C2RustUnnamed_0 = 13;
pub const G_IO_ERROR_NO_SPACE: C2RustUnnamed_0 = 12;
pub const G_IO_ERROR_TOO_MANY_LINKS: C2RustUnnamed_0 = 11;
pub const G_IO_ERROR_INVALID_FILENAME: C2RustUnnamed_0 = 10;
pub const G_IO_ERROR_FILENAME_TOO_LONG: C2RustUnnamed_0 = 9;
pub const G_IO_ERROR_NOT_MOUNTABLE_FILE: C2RustUnnamed_0 = 8;
pub const G_IO_ERROR_NOT_SYMBOLIC_LINK: C2RustUnnamed_0 = 7;
pub const G_IO_ERROR_NOT_REGULAR_FILE: C2RustUnnamed_0 = 6;
pub const G_IO_ERROR_NOT_EMPTY: C2RustUnnamed_0 = 5;
pub const G_IO_ERROR_NOT_DIRECTORY: C2RustUnnamed_0 = 4;
pub const G_IO_ERROR_IS_DIRECTORY: C2RustUnnamed_0 = 3;
pub const G_IO_ERROR_EXISTS: C2RustUnnamed_0 = 2;
pub const G_IO_ERROR_NOT_FOUND: C2RustUnnamed_0 = 1;
pub const G_IO_ERROR_FAILED: C2RustUnnamed_0 = 0;
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
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const G_DBUS_ERROR_PROPERTY_READ_ONLY: C2RustUnnamed_1 = 44;
pub const G_DBUS_ERROR_UNKNOWN_PROPERTY: C2RustUnnamed_1 = 43;
pub const G_DBUS_ERROR_UNKNOWN_INTERFACE: C2RustUnnamed_1 = 42;
pub const G_DBUS_ERROR_UNKNOWN_OBJECT: C2RustUnnamed_1 = 41;
pub const G_DBUS_ERROR_OBJECT_PATH_IN_USE: C2RustUnnamed_1 = 40;
pub const G_DBUS_ERROR_ADT_AUDIT_DATA_UNKNOWN: C2RustUnnamed_1 = 39;
pub const G_DBUS_ERROR_SELINUX_SECURITY_CONTEXT_UNKNOWN: C2RustUnnamed_1 = 38;
pub const G_DBUS_ERROR_INVALID_FILE_CONTENT: C2RustUnnamed_1 = 37;
pub const G_DBUS_ERROR_INVALID_SIGNATURE: C2RustUnnamed_1 = 36;
pub const G_DBUS_ERROR_UNIX_PROCESS_ID_UNKNOWN: C2RustUnnamed_1 = 35;
pub const G_DBUS_ERROR_SPAWN_NO_MEMORY: C2RustUnnamed_1 = 34;
pub const G_DBUS_ERROR_SPAWN_FILE_INVALID: C2RustUnnamed_1 = 33;
pub const G_DBUS_ERROR_SPAWN_PERMISSIONS_INVALID: C2RustUnnamed_1 = 32;
pub const G_DBUS_ERROR_SPAWN_SERVICE_NOT_FOUND: C2RustUnnamed_1 = 31;
pub const G_DBUS_ERROR_SPAWN_SERVICE_INVALID: C2RustUnnamed_1 = 30;
pub const G_DBUS_ERROR_SPAWN_CONFIG_INVALID: C2RustUnnamed_1 = 29;
pub const G_DBUS_ERROR_SPAWN_SETUP_FAILED: C2RustUnnamed_1 = 28;
pub const G_DBUS_ERROR_SPAWN_FAILED: C2RustUnnamed_1 = 27;
pub const G_DBUS_ERROR_SPAWN_CHILD_SIGNALED: C2RustUnnamed_1 = 26;
pub const G_DBUS_ERROR_SPAWN_CHILD_EXITED: C2RustUnnamed_1 = 25;
pub const G_DBUS_ERROR_SPAWN_FORK_FAILED: C2RustUnnamed_1 = 24;
pub const G_DBUS_ERROR_SPAWN_EXEC_FAILED: C2RustUnnamed_1 = 23;
pub const G_DBUS_ERROR_MATCH_RULE_INVALID: C2RustUnnamed_1 = 22;
pub const G_DBUS_ERROR_MATCH_RULE_NOT_FOUND: C2RustUnnamed_1 = 21;
pub const G_DBUS_ERROR_TIMED_OUT: C2RustUnnamed_1 = 20;
pub const G_DBUS_ERROR_UNKNOWN_METHOD: C2RustUnnamed_1 = 19;
pub const G_DBUS_ERROR_FILE_EXISTS: C2RustUnnamed_1 = 18;
pub const G_DBUS_ERROR_FILE_NOT_FOUND: C2RustUnnamed_1 = 17;
pub const G_DBUS_ERROR_INVALID_ARGS: C2RustUnnamed_1 = 16;
pub const G_DBUS_ERROR_DISCONNECTED: C2RustUnnamed_1 = 15;
pub const G_DBUS_ERROR_ADDRESS_IN_USE: C2RustUnnamed_1 = 14;
pub const G_DBUS_ERROR_NO_NETWORK: C2RustUnnamed_1 = 13;
pub const G_DBUS_ERROR_TIMEOUT: C2RustUnnamed_1 = 12;
pub const G_DBUS_ERROR_NO_SERVER: C2RustUnnamed_1 = 11;
pub const G_DBUS_ERROR_AUTH_FAILED: C2RustUnnamed_1 = 10;
pub const G_DBUS_ERROR_ACCESS_DENIED: C2RustUnnamed_1 = 9;
pub const G_DBUS_ERROR_LIMITS_EXCEEDED: C2RustUnnamed_1 = 8;
pub const G_DBUS_ERROR_NOT_SUPPORTED: C2RustUnnamed_1 = 7;
pub const G_DBUS_ERROR_BAD_ADDRESS: C2RustUnnamed_1 = 6;
pub const G_DBUS_ERROR_IO_ERROR: C2RustUnnamed_1 = 5;
pub const G_DBUS_ERROR_NO_REPLY: C2RustUnnamed_1 = 4;
pub const G_DBUS_ERROR_NAME_HAS_NO_OWNER: C2RustUnnamed_1 = 3;
pub const G_DBUS_ERROR_SERVICE_UNKNOWN: C2RustUnnamed_1 = 2;
pub const G_DBUS_ERROR_NO_MEMORY: C2RustUnnamed_1 = 1;
pub const G_DBUS_ERROR_FAILED: C2RustUnnamed_1 = 0;
pub type GDBusCallFlags = ::core::ffi::c_uint;
pub const G_DBUS_CALL_FLAGS_ALLOW_INTERACTIVE_AUTHORIZATION: GDBusCallFlags = 2;
pub const G_DBUS_CALL_FLAGS_NO_AUTO_START: GDBusCallFlags = 1;
pub const G_DBUS_CALL_FLAGS_NONE: GDBusCallFlags = 0;
pub type GDBusPropertyInfoFlags = ::core::ffi::c_uint;
pub const G_DBUS_PROPERTY_INFO_FLAGS_WRITABLE: GDBusPropertyInfoFlags = 2;
pub const G_DBUS_PROPERTY_INFO_FLAGS_READABLE: GDBusPropertyInfoFlags = 1;
pub const G_DBUS_PROPERTY_INFO_FLAGS_NONE: GDBusPropertyInfoFlags = 0;
pub type GNetworkConnectivity = ::core::ffi::c_uint;
pub const G_NETWORK_CONNECTIVITY_FULL: GNetworkConnectivity = 4;
pub const G_NETWORK_CONNECTIVITY_PORTAL: GNetworkConnectivity = 3;
pub const G_NETWORK_CONNECTIVITY_LIMITED: GNetworkConnectivity = 2;
pub const G_NETWORK_CONNECTIVITY_LOCAL: GNetworkConnectivity = 1;
pub type GAsyncResult = _GAsyncResult;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
pub type GInitable = _GInitable;
pub type GIOExtension = _GIOExtension;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GNetworkAddress {
    pub parent_instance: GObject,
    pub priv_0: *mut GNetworkAddressPrivate,
}
pub type GNetworkAddressPrivate = _GNetworkAddressPrivate;
pub type GNetworkAddress = _GNetworkAddress;
pub type GNetworkMonitor = _GNetworkMonitor;
pub type GSocketConnectable = _GSocketConnectable;
pub type GTask = _GTask;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GNetworkMonitorBase {
    pub parent_instance: GObject,
    pub priv_0: *mut GNetworkMonitorBasePrivate,
}
pub type GNetworkMonitorBasePrivate = _GNetworkMonitorBasePrivate;
pub type GNetworkMonitorBase = _GNetworkMonitorBase;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GNetworkMonitorBaseClass {
    pub parent_class: GObjectClass,
    pub padding: [gpointer; 8],
}
pub type GNetworkMonitorBaseClass = _GNetworkMonitorBaseClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GNetworkMonitorPortal {
    pub parent_instance: GNetworkMonitorBase,
    pub priv_0: *mut GNetworkMonitorPortalPrivate,
}
pub type GNetworkMonitorPortalPrivate = _GNetworkMonitorPortalPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GNetworkMonitorPortalPrivate {
    pub proxy: *mut GDBusProxy,
    pub has_network: gboolean,
    pub available: gboolean,
    pub metered: gboolean,
    pub connectivity: GNetworkConnectivity,
}
pub type GNetworkMonitorPortal = _GNetworkMonitorPortal;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GNetworkMonitorPortalClass {
    pub parent_class: GNetworkMonitorBaseClass,
}
pub type GNetworkMonitorPortalClass = _GNetworkMonitorPortalClass;
pub const PROP_CONNECTIVITY: C2RustUnnamed_2 = 3;
pub const PROP_NETWORK_METERED: C2RustUnnamed_2 = 2;
pub const PROP_NETWORK_AVAILABLE: C2RustUnnamed_2 = 1;
pub type GInitableIface = _GInitableIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInitableIface {
    pub g_iface: GTypeInterface,
    pub init: Option<
        unsafe extern "C" fn(*mut GInitable, *mut GCancellable, *mut *mut GError) -> gboolean,
    >,
}
pub type GNetworkMonitorInterface = _GNetworkMonitorInterface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GNetworkMonitorInterface {
    pub g_iface: GTypeInterface,
    pub network_changed: Option<unsafe extern "C" fn(*mut GNetworkMonitor, gboolean) -> ()>,
    pub can_reach: Option<
        unsafe extern "C" fn(
            *mut GNetworkMonitor,
            *mut GSocketConnectable,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub can_reach_async: Option<
        unsafe extern "C" fn(
            *mut GNetworkMonitor,
            *mut GSocketConnectable,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub can_reach_finish: Option<
        unsafe extern "C" fn(*mut GNetworkMonitor, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    >,
}
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_2 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut safe_c2rust_initable_parent_iface: *mut GInitableIface =
    ::core::ptr::null::<GInitableIface>() as *mut GInitableIface;
unsafe extern "C" fn safe_c2rust_g_network_monitor_portal_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_network_monitor_portal_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GNetworkMonitorPortal_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GNetworkMonitorPortal_private_offset,
        );
    }
    safe_c2rust_g_network_monitor_portal_class_init(klass as *mut GNetworkMonitorPortalClass);
}
static mut safe_c2rust_GNetworkMonitorPortal_private_offset: gint = 0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_network_monitor_portal_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_network_monitor_portal_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_network_monitor_portal_get_instance_private(
    mut self_0: *mut GNetworkMonitorPortal,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GNetworkMonitorPortal_private_offset as glong as isize)
        as gpointer;
}
static mut safe_c2rust_g_network_monitor_portal_parent_class: gpointer = NULL;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_network_monitor_portal_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_network_monitor_base_get_type(),
        g_intern_static_string(b"GNetworkMonitorPortal\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GNetworkMonitorPortalClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_network_monitor_portal_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GNetworkMonitorPortal>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GNetworkMonitorPortal) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_network_monitor_portal_init
                    as unsafe extern "C" fn(*mut GNetworkMonitorPortal) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GNetworkMonitorPortal_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GNetworkMonitorPortalPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GNetworkMonitorInterface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_network_monitor_portal_iface_init
                as unsafe extern "C" fn(*mut GNetworkMonitorInterface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_network_monitor_get_type(),
        &raw const g_implement_interface_info,
    );
    let g_implement_interface_info_0: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GInitableIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_network_monitor_portal_initable_iface_init
                as unsafe extern "C" fn(*mut GInitableIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_initable_get_type(),
        &raw const g_implement_interface_info_0,
    );
    _g_io_modules_ensure_extension_points_registered();
    g_io_extension_point_implement(
        b"gio-network-monitor\0" as *const u8 as *const ::core::ffi::c_char,
        g_define_type_id,
        b"portal\0" as *const u8 as *const ::core::ffi::c_char,
        40 as gint,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_portal_init(mut nm: *mut GNetworkMonitorPortal) {
    (*nm).priv_0 = safe_c2rust_g_network_monitor_portal_get_instance_private(nm)
        as *mut GNetworkMonitorPortalPrivate;
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_portal_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut nm: *mut GNetworkMonitorPortal =
        object as *mut ::core::ffi::c_void as *mut GNetworkMonitorPortal;
    match prop_id {
        1 => {
            g_value_set_boolean(value, (*(*nm).priv_0).available);
        }
        2 => {
            g_value_set_boolean(value, (*(*nm).priv_0).metered);
        }
        3 => {
            g_value_set_enum(value, (*(*nm).priv_0).connectivity as gint);
        }
        _ => {
            let mut _glib__object: *mut GObject = object;
            let mut _glib__pspec: *mut GParamSpec = pspec;
            let mut _glib__property_id: guint = prop_id;
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8
                    as *const gchar,
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gnetworkmonitorportal.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                92 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_is_valid_connectivity(mut value: guint32) -> gboolean {
    let mut enum_value: *mut GEnumValue = ::core::ptr::null_mut::<GEnumValue>();
    let mut enum_klass: *mut GEnumClass = ::core::ptr::null_mut::<GEnumClass>();
    enum_klass = g_type_class_ref(g_network_connectivity_get_type()) as *mut GEnumClass;
    enum_value = g_enum_get_value(enum_klass, value as gint);
    g_type_class_unref(enum_klass as gpointer);
    return (enum_value != NULL as *mut GEnumValue) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_got_available(
    mut source: *mut GObject,
    mut res: *mut GAsyncResult,
    mut data: gpointer,
) {
    let mut proxy: *mut GDBusProxy = source as *mut ::core::ffi::c_void as *mut GDBusProxy;
    let mut nm: *mut GNetworkMonitorPortal = data as *mut GNetworkMonitorPortal;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut available: gboolean = 0;
    ret = g_dbus_proxy_call_finish(proxy, res, &raw mut error);
    if ret.is_null() {
        if g_error_matches(
            error,
            g_dbus_error_quark(),
            G_DBUS_ERROR_UNKNOWN_METHOD as ::core::ffi::c_int as gint,
        ) == 0
        {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"%s\0" as *const u8 as *const gchar,
                (*error).message,
            );
            g_clear_error(&raw mut error);
            return;
        }
        g_clear_error(&raw mut error);
        ret = g_dbus_proxy_get_cached_property(
            (*(*nm).priv_0).proxy,
            b"available\0" as *const u8 as *const gchar,
        );
        if ret.is_null() {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Failed to get the '%s' property\0" as *const u8 as *const gchar,
                b"available\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        available = g_variant_get_boolean(ret);
        g_variant_unref(ret);
    } else {
        g_variant_get(
            ret,
            b"(b)\0" as *const u8 as *const gchar,
            &raw mut available,
        );
        g_variant_unref(ret);
    }
    if (*(*nm).priv_0).available != available {
        (*(*nm).priv_0).available = available;
        g_object_notify(
            nm as *mut ::core::ffi::c_void as *mut GObject,
            b"network-available\0" as *const u8 as *const gchar,
        );
        g_signal_emit_by_name(
            nm as gpointer,
            b"network-changed\0" as *const u8 as *const gchar,
            available,
        );
    }
}
unsafe extern "C" fn safe_c2rust_got_metered(
    mut source: *mut GObject,
    mut res: *mut GAsyncResult,
    mut data: gpointer,
) {
    let mut proxy: *mut GDBusProxy = source as *mut ::core::ffi::c_void as *mut GDBusProxy;
    let mut nm: *mut GNetworkMonitorPortal = data as *mut GNetworkMonitorPortal;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut metered: gboolean = 0;
    ret = g_dbus_proxy_call_finish(proxy, res, &raw mut error);
    if ret.is_null() {
        if g_error_matches(
            error,
            g_dbus_error_quark(),
            G_DBUS_ERROR_UNKNOWN_METHOD as ::core::ffi::c_int as gint,
        ) == 0
        {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"%s\0" as *const u8 as *const gchar,
                (*error).message,
            );
            g_clear_error(&raw mut error);
            return;
        }
        g_clear_error(&raw mut error);
        ret = g_dbus_proxy_get_cached_property(
            (*(*nm).priv_0).proxy,
            b"metered\0" as *const u8 as *const gchar,
        );
        if ret.is_null() {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Failed to get the '%s' property\0" as *const u8 as *const gchar,
                b"metered\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        metered = g_variant_get_boolean(ret);
        g_variant_unref(ret);
    } else {
        g_variant_get(ret, b"(b)\0" as *const u8 as *const gchar, &raw mut metered);
        g_variant_unref(ret);
    }
    if (*(*nm).priv_0).metered != metered {
        (*(*nm).priv_0).metered = metered;
        g_object_notify(
            nm as *mut ::core::ffi::c_void as *mut GObject,
            b"network-metered\0" as *const u8 as *const gchar,
        );
        g_signal_emit_by_name(
            nm as gpointer,
            b"network-changed\0" as *const u8 as *const gchar,
            (*(*nm).priv_0).available,
        );
    }
}
unsafe extern "C" fn safe_c2rust_got_connectivity(
    mut source: *mut GObject,
    mut res: *mut GAsyncResult,
    mut data: gpointer,
) {
    let mut proxy: *mut GDBusProxy = source as *mut ::core::ffi::c_void as *mut GDBusProxy;
    let mut nm: *mut GNetworkMonitorPortal = data as *mut GNetworkMonitorPortal;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut connectivity: GNetworkConnectivity = 0 as GNetworkConnectivity;
    ret = g_dbus_proxy_call_finish(proxy, res, &raw mut error);
    if ret.is_null() {
        if g_error_matches(
            error,
            g_dbus_error_quark(),
            G_DBUS_ERROR_UNKNOWN_METHOD as ::core::ffi::c_int as gint,
        ) == 0
        {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"%s\0" as *const u8 as *const gchar,
                (*error).message,
            );
            g_clear_error(&raw mut error);
            return;
        }
        g_clear_error(&raw mut error);
        ret = g_dbus_proxy_get_cached_property(
            (*(*nm).priv_0).proxy,
            b"connectivity\0" as *const u8 as *const gchar,
        );
        if ret.is_null() {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Failed to get the '%s' property\0" as *const u8 as *const gchar,
                b"connectivity\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        connectivity = g_variant_get_uint32(ret) as GNetworkConnectivity;
        g_variant_unref(ret);
    } else {
        g_variant_get(
            ret,
            b"(u)\0" as *const u8 as *const gchar,
            &raw mut connectivity,
        );
        g_variant_unref(ret);
    }
    if (*(*nm).priv_0).connectivity as ::core::ffi::c_uint != connectivity as ::core::ffi::c_uint
        && safe_c2rust_is_valid_connectivity(connectivity as guint32) != 0
    {
        (*(*nm).priv_0).connectivity = connectivity;
        g_object_notify(
            nm as *mut ::core::ffi::c_void as *mut GObject,
            b"connectivity\0" as *const u8 as *const gchar,
        );
        g_signal_emit_by_name(
            nm as gpointer,
            b"network-changed\0" as *const u8 as *const gchar,
            (*(*nm).priv_0).available,
        );
    }
}
unsafe extern "C" fn safe_c2rust_got_status(
    mut source: *mut GObject,
    mut res: *mut GAsyncResult,
    mut data: gpointer,
) {
    let mut proxy: *mut GDBusProxy = source as *mut ::core::ffi::c_void as *mut GDBusProxy;
    let mut nm: *mut GNetworkMonitorPortal = data as *mut GNetworkMonitorPortal;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut status: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut available: gboolean = 0;
    let mut metered: gboolean = 0;
    let mut connectivity: GNetworkConnectivity = 0 as GNetworkConnectivity;
    ret = g_dbus_proxy_call_finish(proxy, res, &raw mut error);
    if ret.is_null() {
        if g_error_matches(
            error,
            g_dbus_error_quark(),
            G_DBUS_ERROR_UNKNOWN_METHOD as ::core::ffi::c_int as gint,
        ) != 0
        {
            g_dbus_proxy_call(
                proxy,
                b"GetConnectivity\0" as *const u8 as *const gchar,
                ::core::ptr::null_mut::<GVariant>(),
                G_DBUS_CALL_FLAGS_NONE,
                -(1 as gint),
                ::core::ptr::null_mut::<GCancellable>(),
                Some(
                    safe_c2rust_got_connectivity
                        as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
                ),
                nm as gpointer,
            );
            g_dbus_proxy_call(
                proxy,
                b"GetMetered\0" as *const u8 as *const gchar,
                ::core::ptr::null_mut::<GVariant>(),
                G_DBUS_CALL_FLAGS_NONE,
                -(1 as gint),
                ::core::ptr::null_mut::<GCancellable>(),
                Some(
                    safe_c2rust_got_metered
                        as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
                ),
                nm as gpointer,
            );
            g_dbus_proxy_call(
                proxy,
                b"GetAvailable\0" as *const u8 as *const gchar,
                ::core::ptr::null_mut::<GVariant>(),
                G_DBUS_CALL_FLAGS_NONE,
                -(1 as gint),
                ::core::ptr::null_mut::<GCancellable>(),
                Some(
                    safe_c2rust_got_available
                        as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
                ),
                nm as gpointer,
            );
        } else {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"%s\0" as *const u8 as *const gchar,
                (*error).message,
            );
        }
        g_clear_error(&raw mut error);
        return;
    }
    g_variant_get(
        ret,
        b"(@a{sv})\0" as *const u8 as *const gchar,
        &raw mut status,
    );
    g_variant_unref(ret);
    g_variant_lookup(
        status,
        b"available\0" as *const u8 as *const gchar,
        b"b\0" as *const u8 as *const gchar,
        &raw mut available,
    );
    g_variant_lookup(
        status,
        b"metered\0" as *const u8 as *const gchar,
        b"b\0" as *const u8 as *const gchar,
        &raw mut metered,
    );
    g_variant_lookup(
        status,
        b"connectivity\0" as *const u8 as *const gchar,
        b"u\0" as *const u8 as *const gchar,
        &raw mut connectivity,
    );
    g_variant_unref(status);
    g_object_freeze_notify(nm as *mut ::core::ffi::c_void as *mut GObject);
    if (*(*nm).priv_0).available != available {
        (*(*nm).priv_0).available = available;
        g_object_notify(
            nm as *mut ::core::ffi::c_void as *mut GObject,
            b"network-available\0" as *const u8 as *const gchar,
        );
    }
    if (*(*nm).priv_0).metered != metered {
        (*(*nm).priv_0).metered = metered;
        g_object_notify(
            nm as *mut ::core::ffi::c_void as *mut GObject,
            b"network-metered\0" as *const u8 as *const gchar,
        );
    }
    if (*(*nm).priv_0).connectivity as ::core::ffi::c_uint != connectivity as ::core::ffi::c_uint
        && safe_c2rust_is_valid_connectivity(connectivity as guint32) != 0
    {
        (*(*nm).priv_0).connectivity = connectivity;
        g_object_notify(
            nm as *mut ::core::ffi::c_void as *mut GObject,
            b"connectivity\0" as *const u8 as *const gchar,
        );
    }
    g_object_thaw_notify(nm as *mut ::core::ffi::c_void as *mut GObject);
    g_signal_emit_by_name(
        nm as gpointer,
        b"network-changed\0" as *const u8 as *const gchar,
        available,
    );
}
unsafe extern "C" fn safe_c2rust_update_properties(
    mut proxy: *mut GDBusProxy,
    mut nm: *mut GNetworkMonitorPortal,
) {
    g_dbus_proxy_call(
        proxy,
        b"GetStatus\0" as *const u8 as *const gchar,
        ::core::ptr::null_mut::<GVariant>(),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        ::core::ptr::null_mut::<GCancellable>(),
        Some(
            safe_c2rust_got_status
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        nm as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_proxy_signal(
    mut proxy: *mut GDBusProxy,
    mut sender: *const ::core::ffi::c_char,
    mut signal: *const ::core::ffi::c_char,
    mut parameters: *mut GVariant,
    mut nm: *mut GNetworkMonitorPortal,
) {
    if (*(*nm).priv_0).has_network == 0 {
        return;
    }
    if strcmp(
        signal,
        b"changed\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        return;
    }
    if g_variant_is_of_type(
        parameters,
        g_variant_type_checked_(b"(b)\0" as *const u8 as *const gchar),
    ) != 0
    {
        let mut available: gboolean = 0;
        g_variant_get(
            parameters,
            b"(b)\0" as *const u8 as *const gchar,
            &raw mut available,
        );
        if (*(*nm).priv_0).available != available {
            (*(*nm).priv_0).available = available;
            g_object_notify(
                nm as *mut ::core::ffi::c_void as *mut GObject,
                b"available\0" as *const u8 as *const gchar,
            );
        }
        g_signal_emit_by_name(
            nm as gpointer,
            b"network-changed\0" as *const u8 as *const gchar,
            available,
        );
    } else {
        safe_c2rust_update_properties(proxy, nm);
    };
}
unsafe extern "C" fn safe_c2rust_proxy_properties_changed(
    mut proxy: *mut GDBusProxy,
    mut changed: *mut GVariant,
    mut invalidated: *mut GVariant,
    mut nm: *mut GNetworkMonitorPortal,
) {
    let mut should_emit_changed: gboolean = FALSE;
    let mut ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if (*(*nm).priv_0).has_network == 0 {
        return;
    }
    ret = g_dbus_proxy_get_cached_property(proxy, b"connectivity\0" as *const u8 as *const gchar);
    if !ret.is_null() {
        let mut connectivity: GNetworkConnectivity =
            g_variant_get_uint32(ret) as GNetworkConnectivity;
        if (*(*nm).priv_0).connectivity as ::core::ffi::c_uint
            != connectivity as ::core::ffi::c_uint
            && safe_c2rust_is_valid_connectivity(connectivity as guint32) != 0
        {
            (*(*nm).priv_0).connectivity = connectivity;
            g_object_notify(
                nm as *mut ::core::ffi::c_void as *mut GObject,
                b"connectivity\0" as *const u8 as *const gchar,
            );
            should_emit_changed = TRUE as gboolean;
        }
        g_variant_unref(ret);
    }
    ret = g_dbus_proxy_get_cached_property(proxy, b"metered\0" as *const u8 as *const gchar);
    if !ret.is_null() {
        let mut metered: gboolean = g_variant_get_boolean(ret);
        if (*(*nm).priv_0).metered != metered {
            (*(*nm).priv_0).metered = metered;
            g_object_notify(
                nm as *mut ::core::ffi::c_void as *mut GObject,
                b"network-metered\0" as *const u8 as *const gchar,
            );
            should_emit_changed = TRUE as gboolean;
        }
        g_variant_unref(ret);
    }
    ret = g_dbus_proxy_get_cached_property(proxy, b"available\0" as *const u8 as *const gchar);
    if !ret.is_null() {
        let mut available: gboolean = g_variant_get_boolean(ret);
        if (*(*nm).priv_0).available != available {
            (*(*nm).priv_0).available = available;
            g_object_notify(
                nm as *mut ::core::ffi::c_void as *mut GObject,
                b"network-available\0" as *const u8 as *const gchar,
            );
            should_emit_changed = TRUE as gboolean;
        }
        g_variant_unref(ret);
    }
    if should_emit_changed != 0 {
        g_signal_emit_by_name(
            nm as gpointer,
            b"network-changed\0" as *const u8 as *const gchar,
            (*(*nm).priv_0).available,
        );
    }
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_portal_initable_init(
    mut initable: *mut GInitable,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut nm: *mut GNetworkMonitorPortal =
        initable as *mut ::core::ffi::c_void as *mut GNetworkMonitorPortal;
    let mut proxy: *mut GDBusProxy = ::core::ptr::null_mut::<GDBusProxy>();
    let mut name_owner: *mut gchar = ::core::ptr::null_mut::<gchar>();
    (*(*nm).priv_0).available = FALSE as gboolean;
    (*(*nm).priv_0).metered = FALSE as gboolean;
    (*(*nm).priv_0).connectivity = G_NETWORK_CONNECTIVITY_LOCAL;
    if glib_should_use_portal() == 0 {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
            b"Not using portals\0" as *const u8 as *const gchar,
        );
        return FALSE;
    }
    proxy = g_dbus_proxy_new_for_bus_sync(
        G_BUS_TYPE_SESSION,
        G_DBUS_PROXY_FLAGS_DO_NOT_LOAD_PROPERTIES,
        ::core::ptr::null_mut::<GDBusInterfaceInfo>(),
        b"org.freedesktop.portal.Desktop\0" as *const u8 as *const gchar,
        b"/org/freedesktop/portal/desktop\0" as *const u8 as *const gchar,
        b"org.freedesktop.portal.NetworkMonitor\0" as *const u8 as *const gchar,
        cancellable,
        error,
    );
    if proxy.is_null() {
        return FALSE;
    }
    name_owner = g_dbus_proxy_get_name_owner(proxy);
    if name_owner.is_null() {
        g_object_unref(proxy as gpointer);
        g_set_error(
            error,
            g_dbus_error_quark(),
            G_DBUS_ERROR_NAME_HAS_NO_OWNER as ::core::ffi::c_int as gint,
            b"Desktop portal not found\0" as *const u8 as *const gchar,
        );
        return FALSE;
    }
    g_free(name_owner as gpointer);
    g_signal_connect_data(
        proxy as gpointer,
        b"g-signal\0" as *const u8 as *const gchar,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GDBusProxy,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut GVariant,
                    *mut GNetworkMonitorPortal,
                ) -> (),
            >,
            GCallback,
        >(Some(
            safe_c2rust_proxy_signal
                as unsafe extern "C" fn(
                    *mut GDBusProxy,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut GVariant,
                    *mut GNetworkMonitorPortal,
                ) -> (),
        )),
        nm as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        proxy as gpointer,
        b"g-properties-changed\0" as *const u8 as *const gchar,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GDBusProxy,
                    *mut GVariant,
                    *mut GVariant,
                    *mut GNetworkMonitorPortal,
                ) -> (),
            >,
            GCallback,
        >(Some(
            safe_c2rust_proxy_properties_changed
                as unsafe extern "C" fn(
                    *mut GDBusProxy,
                    *mut GVariant,
                    *mut GVariant,
                    *mut GNetworkMonitorPortal,
                ) -> (),
        )),
        nm as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    (*(*nm).priv_0).proxy = proxy;
    (*(*nm).priv_0).has_network = glib_network_available_in_sandbox();
    if (*safe_c2rust_initable_parent_iface)
        .init
        .expect("non-null function pointer")(initable, cancellable, error)
        == 0
    {
        return FALSE;
    }
    if (*(*nm).priv_0).has_network != 0 {
        safe_c2rust_update_properties(proxy, nm);
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_portal_finalize(mut object: *mut GObject) {
    let mut nm: *mut GNetworkMonitorPortal =
        object as *mut ::core::ffi::c_void as *mut GNetworkMonitorPortal;
    let mut _pp: *mut *mut GDBusProxy = &raw mut (*(*nm).priv_0).proxy;
    let mut _ptr: *mut GDBusProxy = *_pp;
    *_pp = ::core::ptr::null_mut::<GDBusProxy>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    (*(safe_c2rust_g_network_monitor_portal_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_portal_class_init(
    mut class: *mut GNetworkMonitorPortalClass,
) {
    let mut gobject_class: *mut GObjectClass =
        class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize = Some(
        safe_c2rust_g_network_monitor_portal_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_network_monitor_portal_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    g_object_class_override_property(
        gobject_class,
        PROP_NETWORK_AVAILABLE as ::core::ffi::c_int as guint,
        b"network-available\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        gobject_class,
        PROP_NETWORK_METERED as ::core::ffi::c_int as guint,
        b"network-metered\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        gobject_class,
        PROP_CONNECTIVITY as ::core::ffi::c_int as guint,
        b"connectivity\0" as *const u8 as *const gchar,
    );
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_portal_can_reach(
    mut monitor: *mut GNetworkMonitor,
    mut connectable: *mut GSocketConnectable,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut nm: *mut GNetworkMonitorPortal =
        monitor as *mut ::core::ffi::c_void as *mut GNetworkMonitorPortal;
    let mut ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut address: *mut GNetworkAddress = ::core::ptr::null_mut::<GNetworkAddress>();
    let mut reachable: gboolean = FALSE;
    if ({
        let mut __inst: *mut GTypeInstance = connectable as *mut GTypeInstance;
        let mut __t: GType = g_network_address_get_type();
        let mut __r: gboolean = 0;
        if __inst.is_null() {
            __r = FALSE as gboolean;
        } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_instance_is_a(__inst, __t);
        }
        __r
    }) == 0
    {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            b"Can't handle this kind of GSocketConnectable (%s)\0" as *const u8 as *const gchar,
            g_type_name((*(*(connectable as *mut GTypeInstance)).g_class).g_type),
        );
        return FALSE;
    }
    address = connectable as *mut ::core::ffi::c_void as *mut GNetworkAddress;
    ret = g_dbus_proxy_call_sync(
        (*(*nm).priv_0).proxy,
        b"CanReach\0" as *const u8 as *const gchar,
        g_variant_new(
            b"(su)\0" as *const u8 as *const gchar,
            g_network_address_get_hostname(address),
            g_network_address_get_port(address) as ::core::ffi::c_int,
        ),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        error,
    );
    if !ret.is_null() {
        g_variant_get(
            ret,
            b"(b)\0" as *const u8 as *const gchar,
            &raw mut reachable,
        );
        g_variant_unref(ret);
    }
    return reachable;
}
unsafe extern "C" fn safe_c2rust_can_reach_done(
    mut source: *mut GObject,
    mut result: *mut GAsyncResult,
    mut data: gpointer,
) {
    let mut task: *mut GTask = data as *mut GTask;
    let mut nm: *mut GNetworkMonitorPortal =
        g_task_get_source_object(task) as *mut GNetworkMonitorPortal;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut reachable: gboolean = 0;
    ret = g_dbus_proxy_call_finish((*(*nm).priv_0).proxy, result, &raw mut error);
    if ret.is_null() {
        g_task_return_error(task, error);
        g_object_unref(task as gpointer);
        return;
    }
    g_variant_get(
        ret,
        b"(b)\0" as *const u8 as *const gchar,
        &raw mut reachable,
    );
    g_variant_unref(ret);
    if reachable != 0 {
        g_task_return_boolean(task, TRUE);
    } else {
        g_task_return_new_error_literal(
            task,
            g_io_error_quark(),
            G_IO_ERROR_HOST_UNREACHABLE as ::core::ffi::c_int as gint,
            b"Can't reach host\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_portal_can_reach_async(
    mut monitor: *mut GNetworkMonitor,
    mut connectable: *mut GSocketConnectable,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut data: gpointer,
) {
    let mut nm: *mut GNetworkMonitorPortal =
        monitor as *mut ::core::ffi::c_void as *mut GNetworkMonitorPortal;
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut address: *mut GNetworkAddress = ::core::ptr::null_mut::<GNetworkAddress>();
    task = g_task_new(monitor as gpointer, cancellable, callback, data);
    if ({
        let mut __inst: *mut GTypeInstance = connectable as *mut GTypeInstance;
        let mut __t: GType = g_network_address_get_type();
        let mut __r: gboolean = 0;
        if __inst.is_null() {
            __r = FALSE as gboolean;
        } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_instance_is_a(__inst, __t);
        }
        __r
    }) == 0
    {
        g_task_return_new_error(
            task,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            b"Can't handle this kind of GSocketConnectable (%s)\0" as *const u8
                as *const ::core::ffi::c_char,
            g_type_name((*(*(connectable as *mut GTypeInstance)).g_class).g_type),
        );
        g_object_unref(task as gpointer);
        return;
    }
    address = connectable as *mut ::core::ffi::c_void as *mut GNetworkAddress;
    g_dbus_proxy_call(
        (*(*nm).priv_0).proxy,
        b"CanReach\0" as *const u8 as *const gchar,
        g_variant_new(
            b"(su)\0" as *const u8 as *const gchar,
            g_network_address_get_hostname(address),
            g_network_address_get_port(address) as ::core::ffi::c_int,
        ),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        Some(
            safe_c2rust_can_reach_done
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        task as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_portal_can_reach_finish(
    mut monitor: *mut GNetworkMonitor,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_portal_iface_init(
    mut monitor_iface: *mut GNetworkMonitorInterface,
) {
    (*monitor_iface).can_reach = Some(
        safe_c2rust_g_network_monitor_portal_can_reach
            as unsafe extern "C" fn(
                *mut GNetworkMonitor,
                *mut GSocketConnectable,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GNetworkMonitor,
                *mut GSocketConnectable,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
        >;
    (*monitor_iface).can_reach_async = Some(
        safe_c2rust_g_network_monitor_portal_can_reach_async
            as unsafe extern "C" fn(
                *mut GNetworkMonitor,
                *mut GSocketConnectable,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GNetworkMonitor,
                *mut GSocketConnectable,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*monitor_iface).can_reach_finish = Some(
        safe_c2rust_g_network_monitor_portal_can_reach_finish
            as unsafe extern "C" fn(
                *mut GNetworkMonitor,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GNetworkMonitor,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> gboolean,
        >;
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_portal_initable_iface_init(
    mut iface: *mut GInitableIface,
) {
    safe_c2rust_initable_parent_iface =
        g_type_interface_peek_parent(iface as gpointer) as *mut GInitableIface;
    (*iface).init = Some(
        safe_c2rust_g_network_monitor_portal_initable_init
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
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
