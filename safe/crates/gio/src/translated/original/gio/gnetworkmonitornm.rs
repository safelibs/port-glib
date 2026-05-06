use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GVariant;
    pub type _GAsyncResult;
    pub type _GCancellable;
    pub type _GInitable;
    pub type _GIOExtension;
    pub type _GNetworkMonitor;
    pub type _GSocketConnectable;
    pub type _GDBusProxyPrivate;
    pub type _GNetworkMonitorBasePrivate;
    pub type _GNetworkMonitorNetlinkPrivate;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_strfreev(str_array: *mut *mut gchar);
    fn g_strv_contains(strv: *const *const gchar, str: *const gchar) -> gboolean;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_get_uint32(value: *mut GVariant) -> guint32;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_type_name(type_0: GType) -> *const gchar;
    fn g_type_class_peek_parent(g_class: gpointer) -> gpointer;
    fn g_type_interface_peek(instance_class: gpointer, iface_type: GType) -> gpointer;
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
    fn g_signal_connect_data(
        instance: gpointer,
        detailed_signal: *const gchar,
        c_handler: GCallback,
        data: gpointer,
        destroy_data: GClosureNotify,
        connect_flags: GConnectFlags,
    ) -> gulong;
    fn g_signal_handler_disconnect(instance: gpointer, handler_id: gulong);
    fn g_object_class_override_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        name: *const gchar,
    );
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
    fn g_object_unref(object: gpointer);
    fn g_value_set_enum(value: *mut GValue, v_enum: gint);
    fn g_value_set_boolean(value: *mut GValue, v_boolean: gboolean);
    fn _g_network_monitor_netlink_get_type() -> GType;
    fn g_io_error_quark() -> GQuark;
    fn g_initable_get_type() -> GType;
    fn g_io_extension_point_implement(
        extension_point_name: *const ::core::ffi::c_char,
        type_0: GType,
        extension_name: *const ::core::ffi::c_char,
        priority: gint,
    ) -> *mut GIOExtension;
    fn _g_io_modules_ensure_extension_points_registered();
    fn glib_gettext(str: *const gchar) -> *const gchar;
    fn g_network_monitor_get_type() -> GType;
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
    fn g_dbus_proxy_get_cached_property_names(proxy: *mut GDBusProxy) -> *mut *mut gchar;
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
pub type GStrv = *mut *mut gchar;
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
pub type GCancellable = _GCancellable;
pub type GInitable = _GInitable;
pub type GIOExtension = _GIOExtension;
pub type GNetworkMonitor = _GNetworkMonitor;
pub type GSocketConnectable = _GSocketConnectable;
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
pub struct _GNetworkMonitorNetlink {
    pub parent_instance: GNetworkMonitorBase,
    pub priv_0: *mut GNetworkMonitorNetlinkPrivate,
}
pub type GNetworkMonitorNetlinkPrivate = _GNetworkMonitorNetlinkPrivate;
pub type GNetworkMonitorNetlink = _GNetworkMonitorNetlink;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GNetworkMonitorNetlinkClass {
    pub parent_class: GNetworkMonitorBaseClass,
    pub padding: [gpointer; 8],
}
pub type GNetworkMonitorNetlinkClass = _GNetworkMonitorNetlinkClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GNetworkMonitorNM {
    pub parent_instance: GNetworkMonitorNetlink,
    pub priv_0: *mut GNetworkMonitorNMPrivate,
}
pub type GNetworkMonitorNMPrivate = _GNetworkMonitorNMPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GNetworkMonitorNMPrivate {
    pub proxy: *mut GDBusProxy,
    pub signal_id: guint,
    pub connectivity: GNetworkConnectivity,
    pub network_available: gboolean,
    pub network_metered: gboolean,
}
pub type GNetworkMonitorNM = _GNetworkMonitorNM;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GNetworkMonitorNMClass {
    pub parent_class: GNetworkMonitorNetlinkClass,
}
pub type GNetworkMonitorNMClass = _GNetworkMonitorNMClass;
pub const PROP_CONNECTIVITY: C2RustUnnamed_1 = 3;
pub const PROP_NETWORK_METERED: C2RustUnnamed_1 = 2;
pub const PROP_NETWORK_AVAILABLE: C2RustUnnamed_1 = 1;
pub type GInitableIface = _GInitableIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInitableIface {
    pub g_iface: GTypeInterface,
    pub init: Option<
        unsafe extern "C" fn(*mut GInitable, *mut GCancellable, *mut *mut GError) -> gboolean,
    >,
}
pub type NMConnectivityState = ::core::ffi::c_uint;
pub const NM_CONNECTIVITY_FULL: NMConnectivityState = 4;
pub const NM_CONNECTIVITY_LIMITED: NMConnectivityState = 3;
pub const NM_CONNECTIVITY_PORTAL: NMConnectivityState = 2;
pub const NM_CONNECTIVITY_NONE: NMConnectivityState = 1;
pub const NM_CONNECTIVITY_UNKNOWN: NMConnectivityState = 0;
pub const NM_STATE_CONNECTED_SITE: NMState = 60;
pub type NMState = ::core::ffi::c_uint;
pub const NM_STATE_CONNECTED_GLOBAL: NMState = 70;
pub const NM_STATE_CONNECTED_LOCAL: NMState = 50;
pub const NM_STATE_CONNECTING: NMState = 40;
pub const NM_STATE_DISCONNECTING: NMState = 30;
pub const NM_STATE_DISCONNECTED: NMState = 20;
pub const NM_STATE_ASLEEP: NMState = 10;
pub const NM_STATE_UNKNOWN: NMState = 0;
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
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_1 = 0;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut safe_c2rust_g_network_monitor_nm_parent_class: gpointer = NULL;
unsafe extern "C" fn safe_c2rust_g_network_monitor_nm_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_network_monitor_nm_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GNetworkMonitorNM_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GNetworkMonitorNM_private_offset,
        );
    }
    safe_c2rust_g_network_monitor_nm_class_init(klass as *mut GNetworkMonitorNMClass);
}
static mut safe_c2rust_GNetworkMonitorNM_private_offset: gint = 0;
#[inline]
unsafe extern "C" fn safe_c2rust_g_network_monitor_nm_get_instance_private(
    mut self_0: *mut GNetworkMonitorNM,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GNetworkMonitorNM_private_offset as glong as isize)
        as gpointer;
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_network_monitor_nm_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        _g_network_monitor_netlink_get_type(),
        g_intern_static_string(b"GNetworkMonitorNM\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GNetworkMonitorNMClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_network_monitor_nm_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GNetworkMonitorNM>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GNetworkMonitorNM) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_network_monitor_nm_init
                    as unsafe extern "C" fn(*mut GNetworkMonitorNM) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GNetworkMonitorNM_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GNetworkMonitorNMPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GNetworkMonitorInterface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_network_monitor_nm_iface_init
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
            safe_c2rust_g_network_monitor_nm_initable_iface_init
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
        b"networkmanager\0" as *const u8 as *const ::core::ffi::c_char,
        30 as gint,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_network_monitor_nm_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_network_monitor_nm_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_network_monitor_nm_init(mut nm: *mut GNetworkMonitorNM) {
    (*nm).priv_0 =
        safe_c2rust_g_network_monitor_nm_get_instance_private(nm) as *mut GNetworkMonitorNMPrivate;
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_nm_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut nm: *mut GNetworkMonitorNM =
        object as *mut ::core::ffi::c_void as *mut GNetworkMonitorNM;
    match prop_id {
        1 => {
            g_value_set_boolean(value, (*(*nm).priv_0).network_available);
        }
        2 => {
            g_value_set_boolean(value, (*(*nm).priv_0).network_metered);
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gnetworkmonitornm.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                122 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_nm_conn_to_g_conn(
    mut nm_state: ::core::ffi::c_int,
) -> GNetworkConnectivity {
    match nm_state {
        0 => return G_NETWORK_CONNECTIVITY_LOCAL,
        1 => return G_NETWORK_CONNECTIVITY_LOCAL,
        2 => return G_NETWORK_CONNECTIVITY_PORTAL,
        3 => return G_NETWORK_CONNECTIVITY_LIMITED,
        4 => return G_NETWORK_CONNECTIVITY_FULL,
        _ => {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Unknown NM connectivity state %d\0" as *const u8 as *const gchar,
                nm_state,
            );
            return G_NETWORK_CONNECTIVITY_LOCAL;
        }
    };
}
unsafe extern "C" fn safe_c2rust_nm_metered_to_bool(mut nm_metered: guint) -> gboolean {
    match nm_metered {
        1 | 3 => return TRUE,
        0 | 2 | 4 => return FALSE,
        _ => {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Unknown NM metered state %d\0" as *const u8 as *const gchar,
                nm_metered,
            );
            return FALSE;
        }
    };
}
unsafe extern "C" fn safe_c2rust_sync_properties(
    mut nm: *mut GNetworkMonitorNM,
    mut emit_signals: gboolean,
) {
    let mut v: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut nm_state: NMState = NM_STATE_UNKNOWN;
    let mut nm_connectivity: NMConnectivityState = NM_CONNECTIVITY_UNKNOWN;
    let mut new_network_available: gboolean = 0;
    let mut new_network_metered: gboolean = 0;
    let mut new_connectivity: GNetworkConnectivity = 0 as GNetworkConnectivity;
    v = g_dbus_proxy_get_cached_property(
        (*(*nm).priv_0).proxy,
        b"State\0" as *const u8 as *const gchar,
    );
    if v.is_null() {
        return;
    }
    nm_state = g_variant_get_uint32(v) as NMState;
    g_variant_unref(v);
    v = g_dbus_proxy_get_cached_property(
        (*(*nm).priv_0).proxy,
        b"Connectivity\0" as *const u8 as *const gchar,
    );
    if v.is_null() {
        return;
    }
    nm_connectivity = g_variant_get_uint32(v) as NMConnectivityState;
    g_variant_unref(v);
    if nm_state as ::core::ffi::c_uint
        <= NM_STATE_CONNECTED_LOCAL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        new_network_available = FALSE as gboolean;
        new_network_metered = FALSE as gboolean;
        new_connectivity = G_NETWORK_CONNECTIVITY_LOCAL;
    } else if nm_state as ::core::ffi::c_uint
        <= NM_STATE_CONNECTED_SITE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        new_network_available = TRUE as gboolean;
        new_network_metered = FALSE as gboolean;
        if nm_connectivity as ::core::ffi::c_uint
            == NM_CONNECTIVITY_PORTAL as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            new_connectivity = G_NETWORK_CONNECTIVITY_PORTAL;
        } else {
            new_connectivity = G_NETWORK_CONNECTIVITY_LIMITED;
        }
    } else {
        v = g_dbus_proxy_get_cached_property(
            (*(*nm).priv_0).proxy,
            b"Metered\0" as *const u8 as *const gchar,
        );
        if v.is_null() {
            new_network_metered = FALSE as gboolean;
        } else {
            new_network_metered = safe_c2rust_nm_metered_to_bool(g_variant_get_uint32(v) as guint);
            g_variant_unref(v);
        }
        new_network_available = TRUE as gboolean;
        new_connectivity = safe_c2rust_nm_conn_to_g_conn(nm_connectivity as ::core::ffi::c_int);
    }
    if emit_signals == 0 {
        (*(*nm).priv_0).network_metered = new_network_metered;
        (*(*nm).priv_0).network_available = new_network_available;
        (*(*nm).priv_0).connectivity = new_connectivity;
        return;
    }
    if new_network_available != (*(*nm).priv_0).network_available {
        (*(*nm).priv_0).network_available = new_network_available;
        g_object_notify(
            nm as *mut ::core::ffi::c_void as *mut GObject,
            b"network-available\0" as *const u8 as *const gchar,
        );
    }
    if new_network_metered != (*(*nm).priv_0).network_metered {
        (*(*nm).priv_0).network_metered = new_network_metered;
        g_object_notify(
            nm as *mut ::core::ffi::c_void as *mut GObject,
            b"network-metered\0" as *const u8 as *const gchar,
        );
    }
    if new_connectivity as ::core::ffi::c_uint
        != (*(*nm).priv_0).connectivity as ::core::ffi::c_uint
    {
        (*(*nm).priv_0).connectivity = new_connectivity;
        g_object_notify(
            nm as *mut ::core::ffi::c_void as *mut GObject,
            b"connectivity\0" as *const u8 as *const gchar,
        );
    }
}
unsafe extern "C" fn safe_c2rust_proxy_properties_changed_cb(
    mut proxy: *mut GDBusProxy,
    mut changed_properties: *mut GVariant,
    mut invalidated_properties: GStrv,
    mut nm: *mut GNetworkMonitorNM,
) {
    safe_c2rust_sync_properties(nm, TRUE);
}
unsafe extern "C" fn safe_c2rust_has_property(
    mut proxy: *mut GDBusProxy,
    mut property_name: *const ::core::ffi::c_char,
) -> gboolean {
    let mut props: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut prop_found: gboolean = FALSE;
    props = g_dbus_proxy_get_cached_property_names(proxy) as *mut *mut ::core::ffi::c_char;
    if props.is_null() {
        return FALSE;
    }
    prop_found = g_strv_contains(props as *const *const gchar, property_name as *const gchar);
    g_strfreev(props as *mut *mut gchar);
    return prop_found;
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_nm_initable_init(
    mut initable: *mut GInitable,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut nm: *mut GNetworkMonitorNM =
        initable as *mut ::core::ffi::c_void as *mut GNetworkMonitorNM;
    let mut proxy: *mut GDBusProxy = ::core::ptr::null_mut::<GDBusProxy>();
    let mut parent_iface: *mut GInitableIface = ::core::ptr::null_mut::<GInitableIface>();
    let mut name_owner: *mut gchar = ::core::ptr::null_mut::<gchar>();
    parent_iface = g_type_interface_peek_parent(g_type_interface_peek(
        (*(initable as *mut GTypeInstance)).g_class as gpointer,
        g_initable_get_type(),
    ) as *mut GInitable as gpointer) as *mut GInitableIface;
    if (*parent_iface).init.expect("non-null function pointer")(initable, cancellable, error) == 0 {
        return FALSE;
    }
    proxy = g_dbus_proxy_new_for_bus_sync(
        G_BUS_TYPE_SYSTEM,
        (G_DBUS_PROXY_FLAGS_DO_NOT_AUTO_START as ::core::ffi::c_int
            | G_DBUS_PROXY_FLAGS_GET_INVALIDATED_PROPERTIES as ::core::ffi::c_int)
            as GDBusProxyFlags,
        ::core::ptr::null_mut::<GDBusInterfaceInfo>(),
        b"org.freedesktop.NetworkManager\0" as *const u8 as *const gchar,
        b"/org/freedesktop/NetworkManager\0" as *const u8 as *const gchar,
        b"org.freedesktop.NetworkManager\0" as *const u8 as *const gchar,
        cancellable,
        error,
    );
    if proxy.is_null() {
        return FALSE;
    }
    name_owner = g_dbus_proxy_get_name_owner(proxy);
    if name_owner.is_null() {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(b"NetworkManager not running\0" as *const u8 as *const gchar),
        );
        g_object_unref(proxy as gpointer);
        return FALSE;
    }
    g_free(name_owner as gpointer);
    if safe_c2rust_has_property(
        proxy,
        b"Connectivity\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(b"NetworkManager version too old\0" as *const u8 as *const gchar),
        );
        g_object_unref(proxy as gpointer);
        return FALSE;
    }
    (*(*nm).priv_0).signal_id = g_signal_connect_data(
        proxy as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"g-properties-changed\0" as *const u8 as *const gchar,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GDBusProxy,
                    *mut GVariant,
                    GStrv,
                    *mut GNetworkMonitorNM,
                ) -> (),
            >,
            GCallback,
        >(Some(
            safe_c2rust_proxy_properties_changed_cb
                as unsafe extern "C" fn(
                    *mut GDBusProxy,
                    *mut GVariant,
                    GStrv,
                    *mut GNetworkMonitorNM,
                ) -> (),
        )),
        nm as gpointer,
        None,
        G_CONNECT_DEFAULT,
    ) as guint;
    (*(*nm).priv_0).proxy = proxy;
    safe_c2rust_sync_properties(nm, FALSE);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_nm_finalize(mut object: *mut GObject) {
    let mut nm: *mut GNetworkMonitorNM =
        object as *mut ::core::ffi::c_void as *mut GNetworkMonitorNM;
    if !(*(*nm).priv_0).proxy.is_null() && (*(*nm).priv_0).signal_id != 0 as guint {
        g_signal_handler_disconnect(
            (*(*nm).priv_0).proxy as gpointer,
            (*(*nm).priv_0).signal_id as gulong,
        );
        (*(*nm).priv_0).signal_id = 0 as guint;
    }
    let mut _pp: *mut *mut GDBusProxy = &raw mut (*(*nm).priv_0).proxy;
    let mut _ptr: *mut GDBusProxy = *_pp;
    *_pp = ::core::ptr::null_mut::<GDBusProxy>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    (*(safe_c2rust_g_network_monitor_nm_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_nm_class_init(
    mut nl_class: *mut GNetworkMonitorNMClass,
) {
    let mut gobject_class: *mut GObjectClass =
        nl_class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize =
        Some(safe_c2rust_g_network_monitor_nm_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_network_monitor_nm_get_property
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
unsafe extern "C" fn safe_c2rust_g_network_monitor_nm_iface_init(
    mut monitor_iface: *mut GNetworkMonitorInterface,
) {
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_nm_initable_iface_init(
    mut iface: *mut GInitableIface,
) {
    (*iface).init = Some(
        safe_c2rust_g_network_monitor_nm_initable_init
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
