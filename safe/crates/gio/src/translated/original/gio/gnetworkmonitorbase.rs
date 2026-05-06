extern "C" {
    pub type _GData;
    pub type _GHashTable;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GAsyncResult;
    pub type _GCancellable;
    pub type _GInetAddressPrivate;
    pub type _GInetAddressMaskPrivate;
    pub type _GInetSocketAddressPrivate;
    pub type _GInitable;
    pub type _GIOExtension;
    pub type _GNetworkMonitor;
    pub type _GSocketConnectable;
    pub type _GTask;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_hash_table_new_full(
        hash_func: GHashFunc,
        key_equal_func: GEqualFunc,
        key_destroy_func: GDestroyNotify,
        value_destroy_func: GDestroyNotify,
    ) -> *mut GHashTable;
    fn g_hash_table_add(hash_table: *mut GHashTable, key: gpointer) -> gboolean;
    fn g_hash_table_remove(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_remove_all(hash_table: *mut GHashTable);
    fn g_hash_table_size(hash_table: *mut GHashTable) -> guint;
    fn g_hash_table_iter_init(iter: *mut GHashTableIter, hash_table: *mut GHashTable);
    fn g_hash_table_iter_next(
        iter: *mut GHashTableIter,
        key: *mut gpointer,
        value: *mut gpointer,
    ) -> gboolean;
    fn g_hash_table_unref(hash_table: *mut GHashTable);
    fn g_main_context_ref(context: *mut GMainContext) -> *mut GMainContext;
    fn g_main_context_unref(context: *mut GMainContext);
    fn g_main_current_source() -> *mut GSource;
    fn g_main_context_get_thread_default() -> *mut GMainContext;
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
    fn g_source_is_destroyed(source: *mut GSource) -> gboolean;
    fn g_source_set_static_name(source: *mut GSource, name: *const ::core::ffi::c_char);
    fn g_idle_source_new() -> *mut GSource;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_type_name(type_0: GType) -> *const gchar;
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
    fn g_signal_emit(instance: gpointer, signal_id: guint, detail: GQuark, ...);
    fn g_signal_lookup(name: *const gchar, itype: GType) -> guint;
    fn g_object_class_override_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        name: *const gchar,
    );
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_value_set_enum(value: *mut GValue, v_enum: gint);
    fn g_value_set_boolean(value: *mut GValue, v_boolean: gboolean);
    fn g_inet_address_to_bytes(address: *mut GInetAddress) -> *const guint8;
    fn g_inet_address_get_native_size(address: *mut GInetAddress) -> gsize;
    fn g_inet_address_get_is_mc_link_local(address: *mut GInetAddress) -> gboolean;
    fn g_inet_address_mask_new_from_string(
        mask_string: *const gchar,
        error: *mut *mut GError,
    ) -> *mut GInetAddressMask;
    fn g_inet_address_mask_get_family(mask: *mut GInetAddressMask) -> GSocketFamily;
    fn g_inet_address_mask_get_address(mask: *mut GInetAddressMask) -> *mut GInetAddress;
    fn g_inet_address_mask_get_length(mask: *mut GInetAddressMask) -> guint;
    fn g_inet_address_mask_matches(
        mask: *mut GInetAddressMask,
        address: *mut GInetAddress,
    ) -> gboolean;
    fn g_inet_address_mask_equal(
        mask: *mut GInetAddressMask,
        mask2: *mut GInetAddressMask,
    ) -> gboolean;
    fn g_inet_socket_address_get_type() -> GType;
    fn g_inet_socket_address_get_address(address: *mut GInetSocketAddress) -> *mut GInetAddress;
    fn g_initable_get_type() -> GType;
    fn g_io_error_quark() -> GQuark;
    fn g_io_extension_point_implement(
        extension_point_name: *const ::core::ffi::c_char,
        type_0: GType,
        extension_name: *const ::core::ffi::c_char,
        priority: gint,
    ) -> *mut GIOExtension;
    fn _g_io_modules_ensure_extension_points_registered();
    fn g_network_monitor_get_type() -> GType;
    fn g_socket_address_enumerator_next(
        enumerator: *mut GSocketAddressEnumerator,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GSocketAddress;
    fn g_socket_address_enumerator_next_async(
        enumerator: *mut GSocketAddressEnumerator,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_socket_address_enumerator_next_finish(
        enumerator: *mut GSocketAddressEnumerator,
        result: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> *mut GSocketAddress;
    fn g_socket_connectable_proxy_enumerate(
        connectable: *mut GSocketConnectable,
    ) -> *mut GSocketAddressEnumerator;
    fn g_task_new(
        source_object: gpointer,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        callback_data: gpointer,
    ) -> *mut GTask;
    fn g_task_set_source_tag(task: *mut GTask, source_tag: gpointer);
    fn g_task_set_static_name(task: *mut GTask, name: *const gchar);
    fn g_task_get_source_object(task: *mut GTask) -> gpointer;
    fn g_task_get_cancellable(task: *mut GTask) -> *mut GCancellable;
    fn g_task_get_name(task: *mut GTask) -> *const gchar;
    fn g_task_is_valid(result: gpointer, source_object: gpointer) -> gboolean;
    fn g_task_return_boolean(task: *mut GTask, result: gboolean);
    fn g_task_return_error(task: *mut GTask, error: *mut GError);
    fn g_task_return_new_error_literal(
        task: *mut GTask,
        domain: GQuark,
        code: gint,
        message: *const ::core::ffi::c_char,
    );
    fn g_task_propagate_boolean(task: *mut GTask, error: *mut *mut GError) -> gboolean;
    fn glib_gettext(str: *const gchar) -> *const gchar;
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
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
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
pub type GHashTable = _GHashTable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GHashTableIter {
    pub dummy1: gpointer,
    pub dummy2: gpointer,
    pub dummy3: gpointer,
    pub dummy4: ::core::ffi::c_int,
    pub dummy5: gboolean,
    pub dummy6: gpointer,
}
pub type GHashTableIter = _GHashTableIter;
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
pub type GSocketFamily = ::core::ffi::c_uint;
pub const G_SOCKET_FAMILY_IPV6: GSocketFamily = 10;
pub const G_SOCKET_FAMILY_IPV4: GSocketFamily = 2;
pub const G_SOCKET_FAMILY_UNIX: GSocketFamily = 1;
pub const G_SOCKET_FAMILY_INVALID: GSocketFamily = 0;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const G_NETWORK_CONNECTIVITY_FULL: C2RustUnnamed_1 = 4;
pub const G_NETWORK_CONNECTIVITY_PORTAL: C2RustUnnamed_1 = 3;
pub const G_NETWORK_CONNECTIVITY_LIMITED: C2RustUnnamed_1 = 2;
pub const G_NETWORK_CONNECTIVITY_LOCAL: C2RustUnnamed_1 = 1;
pub type GAsyncResult = _GAsyncResult;
pub type GCancellable = _GCancellable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInetAddress {
    pub parent_instance: GObject,
    pub priv_0: *mut GInetAddressPrivate,
}
pub type GInetAddressPrivate = _GInetAddressPrivate;
pub type GInetAddress = _GInetAddress;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInetAddressMask {
    pub parent_instance: GObject,
    pub priv_0: *mut GInetAddressMaskPrivate,
}
pub type GInetAddressMaskPrivate = _GInetAddressMaskPrivate;
pub type GInetAddressMask = _GInetAddressMask;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInetSocketAddress {
    pub parent_instance: GSocketAddress,
    pub priv_0: *mut GInetSocketAddressPrivate,
}
pub type GInetSocketAddressPrivate = _GInetSocketAddressPrivate;
pub type GSocketAddress = _GSocketAddress;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketAddress {
    pub parent_instance: GObject,
}
pub type GInetSocketAddress = _GInetSocketAddress;
pub type GInitable = _GInitable;
pub type GIOExtension = _GIOExtension;
pub type GNetworkMonitor = _GNetworkMonitor;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketAddressEnumerator {
    pub parent_instance: GObject,
}
pub type GSocketAddressEnumerator = _GSocketAddressEnumerator;
pub type GSocketConnectable = _GSocketConnectable;
pub type GTask = _GTask;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GNetworkMonitorBase {
    pub parent_instance: GObject,
    pub priv_0: *mut GNetworkMonitorBasePrivate,
}
pub type GNetworkMonitorBasePrivate = _GNetworkMonitorBasePrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GNetworkMonitorBasePrivate {
    pub networks: *mut GHashTable,
    pub have_ipv4_default_route: gboolean,
    pub have_ipv6_default_route: gboolean,
    pub is_available: gboolean,
    pub context: *mut GMainContext,
    pub network_changed_source: *mut GSource,
    pub initializing: gboolean,
}
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
pub union C2RustUnnamed_2 {
    pub bytes: *const guint8,
    pub hash32: *mut guint32,
    pub hash64: *mut guint64,
}
pub const PROP_CONNECTIVITY: C2RustUnnamed_3 = 3;
pub const PROP_NETWORK_METERED: C2RustUnnamed_3 = 2;
pub const PROP_NETWORK_AVAILABLE: C2RustUnnamed_3 = 1;
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
pub type GInitableIface = _GInitableIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInitableIface {
    pub g_iface: GTypeInterface,
    pub init: Option<
        unsafe extern "C" fn(*mut GInitable, *mut GCancellable, *mut *mut GError) -> gboolean,
    >,
}
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_3 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_PRIORITY_HIGH_IDLE: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
static mut safe_c2rust_network_changed_signal: guint = 0 as guint;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_network_monitor_base_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GNetworkMonitorBase\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GNetworkMonitorBaseClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_network_monitor_base_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GNetworkMonitorBase>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GNetworkMonitorBase) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_network_monitor_base_init
                    as unsafe extern "C" fn(*mut GNetworkMonitorBase) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GNetworkMonitorBase_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GNetworkMonitorBasePrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GInitableIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_network_monitor_base_initable_iface_init
                as unsafe extern "C" fn(*mut GInitableIface) -> (),
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
            Option<unsafe extern "C" fn(*mut GNetworkMonitorInterface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_network_monitor_base_iface_init
                as unsafe extern "C" fn(*mut GNetworkMonitorInterface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_network_monitor_get_type(),
        &raw const g_implement_interface_info_0,
    );
    _g_io_modules_ensure_extension_points_registered();
    g_io_extension_point_implement(
        b"gio-network-monitor\0" as *const u8 as *const ::core::ffi::c_char,
        g_define_type_id,
        b"base\0" as *const u8 as *const ::core::ffi::c_char,
        0 as gint,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_network_monitor_base_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_network_monitor_base_get_type_once();
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
static mut safe_c2rust_GNetworkMonitorBase_private_offset: gint = 0;
static mut safe_c2rust_g_network_monitor_base_parent_class: gpointer = NULL;
#[inline]
unsafe extern "C" fn safe_c2rust_g_network_monitor_base_get_instance_private(
    mut self_0: *mut GNetworkMonitorBase,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GNetworkMonitorBase_private_offset as glong as isize)
        as gpointer;
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_base_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_network_monitor_base_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GNetworkMonitorBase_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GNetworkMonitorBase_private_offset,
        );
    }
    safe_c2rust_g_network_monitor_base_class_init(klass as *mut GNetworkMonitorBaseClass);
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_base_init(
    mut monitor: *mut GNetworkMonitorBase,
) {
    (*monitor).priv_0 = safe_c2rust_g_network_monitor_base_get_instance_private(monitor)
        as *mut GNetworkMonitorBasePrivate;
    (*(*monitor).priv_0).networks = g_hash_table_new_full(
        Some(safe_c2rust_inet_address_mask_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(
            safe_c2rust_inet_address_mask_equal
                as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
        ),
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        None,
    );
    (*(*monitor).priv_0).context = g_main_context_get_thread_default();
    if !(*(*monitor).priv_0).context.is_null() {
        g_main_context_ref((*(*monitor).priv_0).context);
    }
    (*(*monitor).priv_0).initializing = TRUE as gboolean;
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_base_constructed(mut object: *mut GObject) {
    let mut monitor: *mut GNetworkMonitorBase =
        object as *mut ::core::ffi::c_void as *mut GNetworkMonitorBase;
    if (*(*(monitor as *mut GTypeInstance)).g_class).g_type
        == safe_c2rust_g_network_monitor_base_get_type()
    {
        let mut mask: *mut GInetAddressMask = ::core::ptr::null_mut::<GInetAddressMask>();
        mask = g_inet_address_mask_new_from_string(
            b"0.0.0.0/0\0" as *const u8 as *const gchar,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        safe_c2rust_g_network_monitor_base_add_network(monitor, mask);
        g_object_unref(mask as gpointer);
        mask = g_inet_address_mask_new_from_string(
            b"::/0\0" as *const u8 as *const gchar,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        if !mask.is_null() {
            safe_c2rust_g_network_monitor_base_add_network(monitor, mask);
            g_object_unref(mask as gpointer);
        }
    }
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_base_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut monitor: *mut GNetworkMonitorBase =
        object as *mut ::core::ffi::c_void as *mut GNetworkMonitorBase;
    match prop_id {
        1 => {
            g_value_set_boolean(value, (*(*monitor).priv_0).is_available);
        }
        2 => {
            g_value_set_boolean(value, FALSE);
        }
        3 => {
            g_value_set_enum(
                value,
                if (*(*monitor).priv_0).is_available != 0 {
                    G_NETWORK_CONNECTIVITY_FULL as ::core::ffi::c_int as gint
                } else {
                    G_NETWORK_CONNECTIVITY_LOCAL as ::core::ffi::c_int as gint
                },
            );
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gnetworkmonitorbase.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                148 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_base_finalize(mut object: *mut GObject) {
    let mut monitor: *mut GNetworkMonitorBase =
        object as *mut ::core::ffi::c_void as *mut GNetworkMonitorBase;
    g_hash_table_unref((*(*monitor).priv_0).networks);
    if !(*(*monitor).priv_0).network_changed_source.is_null() {
        g_source_destroy((*(*monitor).priv_0).network_changed_source);
        g_source_unref((*(*monitor).priv_0).network_changed_source);
    }
    if !(*(*monitor).priv_0).context.is_null() {
        g_main_context_unref((*(*monitor).priv_0).context);
    }
    (*(safe_c2rust_g_network_monitor_base_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_base_class_init(
    mut monitor_class: *mut GNetworkMonitorBaseClass,
) {
    let mut gobject_class: *mut GObjectClass =
        monitor_class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).constructed = Some(
        safe_c2rust_g_network_monitor_base_constructed as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_network_monitor_base_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).finalize = Some(
        safe_c2rust_g_network_monitor_base_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
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
unsafe extern "C" fn safe_c2rust_g_network_monitor_base_can_reach_sockaddr(
    mut base: *mut GNetworkMonitorBase,
    mut sockaddr: *mut GSocketAddress,
) -> gboolean {
    let mut iaddr: *mut GInetAddress = ::core::ptr::null_mut::<GInetAddress>();
    let mut iter: GHashTableIter = _GHashTableIter {
        dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy4: 0,
        dummy5: 0,
        dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut key: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if ({
        let mut __inst: *mut GTypeInstance = sockaddr as *mut GTypeInstance;
        let mut __t: GType = g_inet_socket_address_get_type();
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
        return FALSE;
    }
    iaddr = g_inet_socket_address_get_address(
        sockaddr as *mut ::core::ffi::c_void as *mut GInetSocketAddress,
    );
    g_hash_table_iter_init(&raw mut iter, (*(*base).priv_0).networks);
    while g_hash_table_iter_next(
        &raw mut iter,
        &raw mut key,
        ::core::ptr::null_mut::<gpointer>(),
    ) != 0
    {
        let mut mask: *mut GInetAddressMask = key as *mut GInetAddressMask;
        if g_inet_address_mask_matches(mask, iaddr) != 0 {
            return TRUE;
        }
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_base_can_reach(
    mut monitor: *mut GNetworkMonitor,
    mut connectable: *mut GSocketConnectable,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut base: *mut GNetworkMonitorBase =
        monitor as *mut ::core::ffi::c_void as *mut GNetworkMonitorBase;
    let mut enumerator: *mut GSocketAddressEnumerator =
        ::core::ptr::null_mut::<GSocketAddressEnumerator>();
    let mut addr: *mut GSocketAddress = ::core::ptr::null_mut::<GSocketAddress>();
    if g_hash_table_size((*(*base).priv_0).networks) == 0 as guint {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NETWORK_UNREACHABLE as ::core::ffi::c_int as gint,
            glib_gettext(b"Network unreachable\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    enumerator = g_socket_connectable_proxy_enumerate(connectable);
    addr = g_socket_address_enumerator_next(enumerator, cancellable, error);
    if addr.is_null() {
        g_object_unref(enumerator as gpointer);
        return FALSE;
    }
    if (*(*base).priv_0).have_ipv4_default_route != 0
        && (*(*base).priv_0).have_ipv6_default_route != 0
    {
        g_object_unref(enumerator as gpointer);
        g_object_unref(addr as gpointer);
        return TRUE;
    }
    while !addr.is_null() {
        if safe_c2rust_g_network_monitor_base_can_reach_sockaddr(base, addr) != 0 {
            g_object_unref(addr as gpointer);
            g_object_unref(enumerator as gpointer);
            return TRUE;
        }
        g_object_unref(addr as gpointer);
        addr = g_socket_address_enumerator_next(enumerator, cancellable, error);
    }
    g_object_unref(enumerator as gpointer);
    if !error.is_null() && (*error).is_null() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_HOST_UNREACHABLE as ::core::ffi::c_int as gint,
            glib_gettext(b"Host unreachable\0" as *const u8 as *const gchar),
        );
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_can_reach_async_got_address(
    mut object: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut enumerator: *mut GSocketAddressEnumerator =
        object as *mut ::core::ffi::c_void as *mut GSocketAddressEnumerator;
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut base: *mut GNetworkMonitorBase =
        g_task_get_source_object(task) as *mut GNetworkMonitorBase;
    let mut addr: *mut GSocketAddress = ::core::ptr::null_mut::<GSocketAddress>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    addr = g_socket_address_enumerator_next_finish(enumerator, result, &raw mut error);
    if addr.is_null() {
        if !error.is_null() {
            g_task_return_error(task, error);
            g_object_unref(task as gpointer);
            return;
        } else {
            g_task_return_new_error_literal(
                task,
                g_io_error_quark(),
                G_IO_ERROR_HOST_UNREACHABLE as ::core::ffi::c_int as gint,
                glib_gettext(b"Host unreachable\0" as *const u8 as *const gchar)
                    as *const ::core::ffi::c_char,
            );
            g_object_unref(task as gpointer);
            return;
        }
    }
    if safe_c2rust_g_network_monitor_base_can_reach_sockaddr(base, addr) != 0 {
        g_object_unref(addr as gpointer);
        g_task_return_boolean(task, TRUE);
        g_object_unref(task as gpointer);
        return;
    }
    g_object_unref(addr as gpointer);
    g_socket_address_enumerator_next_async(
        enumerator,
        g_task_get_cancellable(task),
        Some(
            safe_c2rust_can_reach_async_got_address
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        task as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_base_can_reach_async(
    mut monitor: *mut GNetworkMonitor,
    mut connectable: *mut GSocketConnectable,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut enumerator: *mut GSocketAddressEnumerator =
        ::core::ptr::null_mut::<GSocketAddressEnumerator>();
    task = g_task_new(monitor as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GNetworkMonitor,
                    *mut GSocketConnectable,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_network_monitor_base_can_reach_async
                as unsafe extern "C" fn(
                    *mut GNetworkMonitor,
                    *mut GSocketConnectable,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"g_network_monitor_base_can_reach_async\0" as *const u8 as *const gchar,
        );
    }
    if g_hash_table_size(
        (*(*(monitor as *mut ::core::ffi::c_void as *mut GNetworkMonitorBase)).priv_0).networks,
    ) == 0 as guint
    {
        g_task_return_new_error_literal(
            task,
            g_io_error_quark(),
            G_IO_ERROR_NETWORK_UNREACHABLE as ::core::ffi::c_int as gint,
            glib_gettext(b"Network unreachable\0" as *const u8 as *const gchar)
                as *const ::core::ffi::c_char,
        );
        g_object_unref(task as gpointer);
        return;
    }
    enumerator = g_socket_connectable_proxy_enumerate(connectable);
    g_socket_address_enumerator_next_async(
        enumerator,
        cancellable,
        Some(
            safe_c2rust_can_reach_async_got_address
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        task as gpointer,
    );
    g_object_unref(enumerator as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_base_can_reach_finish(
    mut monitor: *mut GNetworkMonitor,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, monitor as gpointer) != 0 {
            _g_boolean_var_10 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_10 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_10
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (result, monitor)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_base_iface_init(
    mut monitor_iface: *mut GNetworkMonitorInterface,
) {
    (*monitor_iface).can_reach = Some(
        safe_c2rust_g_network_monitor_base_can_reach
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
        safe_c2rust_g_network_monitor_base_can_reach_async
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
        safe_c2rust_g_network_monitor_base_can_reach_finish
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
    safe_c2rust_network_changed_signal = g_signal_lookup(
        b"network-changed\0" as *const u8 as *const gchar,
        g_network_monitor_get_type(),
    );
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_base_initable_init(
    mut initable: *mut GInitable,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut base: *mut GNetworkMonitorBase =
        initable as *mut ::core::ffi::c_void as *mut GNetworkMonitorBase;
    (*(*base).priv_0).initializing = FALSE as gboolean;
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_base_initable_iface_init(
    mut iface: *mut GInitableIface,
) {
    (*iface).init = Some(
        safe_c2rust_g_network_monitor_base_initable_init
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
unsafe extern "C" fn safe_c2rust_inet_address_mask_hash(mut key: gconstpointer) -> guint {
    let mut mask: *mut GInetAddressMask = key as *mut ::core::ffi::c_void as *mut GInetAddressMask;
    let mut addr_hash: guint = 0;
    let mut mask_length: guint = g_inet_address_mask_get_length(mask);
    let mut addr: *mut GInetAddress = g_inet_address_mask_get_address(mask);
    let mut bytes: *const guint8 = g_inet_address_to_bytes(addr);
    let mut bytes_length: gsize = g_inet_address_get_native_size(addr);
    let mut integerifier: C2RustUnnamed_2 = C2RustUnnamed_2 {
        bytes: ::core::ptr::null::<guint8>(),
    };
    if bytes_length as usize == ::core::mem::size_of::<guint32>() as usize {
        integerifier.bytes = bytes;
        addr_hash = *integerifier.hash32 as guint;
    } else if bytes_length as usize == ::core::mem::size_of::<guint64>() as usize {
        integerifier.bytes = bytes;
        addr_hash = *integerifier.hash64 as guint;
    } else {
        let mut i: gsize = 0;
        addr_hash = 0 as guint;
        i = 0 as gsize;
        while i < bytes_length {
            addr_hash = addr_hash.wrapping_add(*bytes.offset(i as isize) as guint);
            i = i.wrapping_add(1);
        }
    }
    return addr_hash.wrapping_add(mask_length);
}
unsafe extern "C" fn safe_c2rust_inet_address_mask_equal(
    mut a: gconstpointer,
    mut b: gconstpointer,
) -> gboolean {
    let mut mask_a: *mut GInetAddressMask = a as *mut ::core::ffi::c_void as *mut GInetAddressMask;
    let mut mask_b: *mut GInetAddressMask = b as *mut ::core::ffi::c_void as *mut GInetAddressMask;
    return g_inet_address_mask_equal(mask_a, mask_b);
}
unsafe extern "C" fn safe_c2rust_emit_network_changed(mut user_data: gpointer) -> gboolean {
    let mut monitor: *mut GNetworkMonitorBase = user_data as *mut GNetworkMonitorBase;
    let mut is_available: gboolean = 0;
    if g_source_is_destroyed(g_main_current_source()) != 0 {
        return FALSE;
    }
    g_object_ref(monitor as gpointer);
    is_available = ((*(*monitor).priv_0).have_ipv4_default_route != 0
        || (*(*monitor).priv_0).have_ipv6_default_route != 0)
        as ::core::ffi::c_int as gboolean;
    if (*(*monitor).priv_0).is_available != is_available {
        (*(*monitor).priv_0).is_available = is_available;
        g_object_notify(
            monitor as *mut ::core::ffi::c_void as *mut GObject,
            b"network-available\0" as *const u8 as *const gchar,
        );
    }
    g_signal_emit(
        monitor as gpointer,
        safe_c2rust_network_changed_signal,
        0 as GQuark,
        is_available,
    );
    g_source_unref((*(*monitor).priv_0).network_changed_source);
    (*(*monitor).priv_0).network_changed_source = ::core::ptr::null_mut::<GSource>();
    g_object_unref(monitor as gpointer);
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_queue_network_changed(mut monitor: *mut GNetworkMonitorBase) {
    if (*(*monitor).priv_0).network_changed_source.is_null()
        && (*(*monitor).priv_0).initializing == 0
    {
        let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
        source = g_idle_source_new();
        g_source_set_priority(source, G_PRIORITY_HIGH_IDLE);
        g_source_set_callback(
            source,
            Some(safe_c2rust_emit_network_changed as unsafe extern "C" fn(gpointer) -> gboolean),
            monitor as gpointer,
            None,
        );
        g_source_set_static_name(
            source,
            b"[gio] emit_network_changed\0" as *const u8 as *const ::core::ffi::c_char,
        );
        g_source_attach(source, (*(*monitor).priv_0).context);
        (*(*monitor).priv_0).network_changed_source = source;
    }
    if (*(*monitor).priv_0).initializing != 0 {
        (*(*monitor).priv_0).is_available = ((*(*monitor).priv_0).have_ipv4_default_route != 0
            || (*(*monitor).priv_0).have_ipv6_default_route != 0)
            as ::core::ffi::c_int as gboolean;
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_network_monitor_base_add_network(
    mut monitor: *mut GNetworkMonitorBase,
    mut network: *mut GInetAddressMask,
) {
    if g_hash_table_add(
        (*(*monitor).priv_0).networks,
        g_object_ref(network as gpointer) as *mut GInetAddressMask as gpointer,
    ) == 0
    {
        return;
    }
    if g_inet_address_mask_get_length(network) == 0 as guint {
        match g_inet_address_mask_get_family(network) as ::core::ffi::c_uint {
            2 => {
                (*(*monitor).priv_0).have_ipv4_default_route = TRUE as gboolean;
            }
            10 => {
                (*(*monitor).priv_0).have_ipv6_default_route = TRUE as gboolean;
            }
            _ => {}
        }
    }
    if g_inet_address_get_is_mc_link_local(g_inet_address_mask_get_address(network)) != 0 {
        return;
    }
    safe_c2rust_queue_network_changed(monitor);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_network_monitor_base_remove_network(
    mut monitor: *mut GNetworkMonitorBase,
    mut network: *mut GInetAddressMask,
) {
    if g_hash_table_remove((*(*monitor).priv_0).networks, network as gconstpointer) == 0 {
        return;
    }
    if g_inet_address_mask_get_length(network) == 0 as guint {
        match g_inet_address_mask_get_family(network) as ::core::ffi::c_uint {
            2 => {
                (*(*monitor).priv_0).have_ipv4_default_route = FALSE as gboolean;
            }
            10 => {
                (*(*monitor).priv_0).have_ipv6_default_route = FALSE as gboolean;
            }
            _ => {}
        }
    }
    safe_c2rust_queue_network_changed(monitor);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_network_monitor_base_set_networks(
    mut monitor: *mut GNetworkMonitorBase,
    mut networks: *mut *mut GInetAddressMask,
    mut length: gint,
) {
    let mut i: ::core::ffi::c_int = 0;
    g_hash_table_remove_all((*(*monitor).priv_0).networks);
    (*(*monitor).priv_0).have_ipv4_default_route = FALSE as gboolean;
    (*(*monitor).priv_0).have_ipv6_default_route = FALSE as gboolean;
    i = 0 as ::core::ffi::c_int;
    while i < length {
        safe_c2rust_g_network_monitor_base_add_network(monitor, *networks.offset(i as isize));
        i += 1;
    }
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
