extern "C" {
    pub type _GData;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _GInetAddressPrivate;
    pub type _GInetAddressMaskPrivate;
    pub type _GInitable;
    pub type _GIOExtension;
    pub type _GNetworkMonitor;
    pub type _GSocketPrivate;
    pub type _GSocketControlMessagePrivate;
    pub type _GSocketConnectable;
    pub type _GNetworkMonitorBasePrivate;
    pub type sockaddr_x25;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn g_ptr_array_new_with_free_func(element_free_func: GDestroyNotify) -> *mut GPtrArray;
    fn g_ptr_array_free(array: *mut GPtrArray, free_seg: gboolean) -> *mut gpointer;
    fn g_ptr_array_unref(array: *mut GPtrArray);
    fn g_ptr_array_remove_index_fast(array: *mut GPtrArray, index_: guint) -> gpointer;
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_clear_error(err: *mut *mut GError);
    fn g_prefix_error(err: *mut *mut GError, format: *const gchar, ...);
    fn g_propagate_prefixed_error(
        dest: *mut *mut GError,
        src: *mut GError,
        format: *const gchar,
        ...
    );
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_main_context_unref(context: *mut GMainContext);
    fn g_main_context_ref_thread_default() -> *mut GMainContext;
    fn g_source_unref(source: *mut GSource);
    fn g_source_attach(source: *mut GSource, context: *mut GMainContext) -> guint;
    fn g_source_destroy(source: *mut GSource);
    fn g_source_set_callback(
        source: *mut GSource,
        func: GSourceFunc,
        data: gpointer,
        notify: GDestroyNotify,
    );
    fn g_timeout_source_new_seconds(interval: guint) -> *mut GSource;
    fn g_strerror(errnum: gint) -> *const gchar;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
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
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_network_monitor_base_get_type() -> GType;
    fn g_network_monitor_base_add_network(
        monitor: *mut GNetworkMonitorBase,
        network: *mut GInetAddressMask,
    );
    fn g_network_monitor_base_remove_network(
        monitor: *mut GNetworkMonitorBase,
        network: *mut GInetAddressMask,
    );
    fn g_network_monitor_base_set_networks(
        monitor: *mut GNetworkMonitorBase,
        networks: *mut *mut GInetAddressMask,
        length: gint,
    );
    fn g_inet_address_mask_new(
        addr: *mut GInetAddress,
        length: guint,
        error: *mut *mut GError,
    ) -> *mut GInetAddressMask;
    fn g_inet_address_mask_equal(
        mask: *mut GInetAddressMask,
        mask2: *mut GInetAddressMask,
    ) -> gboolean;
    fn g_initable_get_type() -> GType;
    fn g_io_extension_point_implement(
        extension_point_name: *const ::core::ffi::c_char,
        type_0: GType,
        extension_name: *const ::core::ffi::c_char,
        priority: gint,
    ) -> *mut GIOExtension;
    fn _g_io_modules_ensure_extension_points_registered();
    fn glib_gettext(str: *const gchar) -> *const gchar;
    fn g_close(fd: gint, error: *mut *mut GError) -> gboolean;
    fn bind(
        __fd: ::core::ffi::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> ::core::ffi::c_int;
    fn g_socket(domain: gint, type_0: gint, protocol: gint, error: *mut *mut GError) -> gint;
    fn g_network_monitor_get_type() -> GType;
    fn g_socket_new_from_fd(fd: gint, error: *mut *mut GError) -> *mut GSocket;
    fn g_socket_set_blocking(socket: *mut GSocket, blocking: gboolean);
    fn g_socket_send(
        socket: *mut GSocket,
        buffer: *const gchar,
        size: gsize,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gssize;
    fn g_socket_receive_message(
        socket: *mut GSocket,
        address: *mut *mut GSocketAddress,
        vectors: *mut GInputVector,
        num_vectors: gint,
        messages: *mut *mut *mut GSocketControlMessage,
        num_messages: *mut gint,
        flags: *mut gint,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gssize;
    fn g_socket_close(socket: *mut GSocket, error: *mut *mut GError) -> gboolean;
    fn g_socket_create_source(
        socket: *mut GSocket,
        condition: GIOCondition,
        cancellable: *mut GCancellable,
    ) -> *mut GSource;
    fn g_socket_set_option(
        socket: *mut GSocket,
        level: gint,
        optname: gint,
        value: gint,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_io_error_quark() -> GQuark;
    fn g_io_error_from_errno(err_no: gint) -> GIOErrorEnum;
    fn g_inet_address_new_from_bytes(
        bytes: *const guint8,
        family: GSocketFamily,
    ) -> *mut GInetAddress;
    fn g_inet_address_new_any(family: GSocketFamily) -> *mut GInetAddress;
    fn g_socket_address_to_native(
        address: *mut GSocketAddress,
        dest: gpointer,
        destlen: gsize,
        error: *mut *mut GError,
    ) -> gboolean;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __socklen_t = ::core::ffi::c_uint;
pub type socklen_t = __socklen_t;
pub type guint8 = ::core::ffi::c_uchar;
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
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gfloat = ::core::ffi::c_float;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
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
pub type GData = _GData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type GIOCondition = ::core::ffi::c_uint;
pub const G_IO_NVAL: GIOCondition = 32;
pub const G_IO_HUP: GIOCondition = 16;
pub const G_IO_ERR: GIOCondition = 8;
pub const G_IO_PRI: GIOCondition = 2;
pub const G_IO_OUT: GIOCondition = 4;
pub const G_IO_IN: GIOCondition = 1;
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
pub type GIOErrorEnum = ::core::ffi::c_uint;
pub const G_IO_ERROR_DESTINATION_UNSET: GIOErrorEnum = 48;
pub const G_IO_ERROR_NO_SUCH_DEVICE: GIOErrorEnum = 47;
pub const G_IO_ERROR_MESSAGE_TOO_LARGE: GIOErrorEnum = 46;
pub const G_IO_ERROR_NOT_CONNECTED: GIOErrorEnum = 45;
pub const G_IO_ERROR_CONNECTION_CLOSED: GIOErrorEnum = 44;
pub const G_IO_ERROR_BROKEN_PIPE: GIOErrorEnum = 44;
pub const G_IO_ERROR_PROXY_NOT_ALLOWED: GIOErrorEnum = 43;
pub const G_IO_ERROR_PROXY_NEED_AUTH: GIOErrorEnum = 42;
pub const G_IO_ERROR_PROXY_AUTH_FAILED: GIOErrorEnum = 41;
pub const G_IO_ERROR_PROXY_FAILED: GIOErrorEnum = 40;
pub const G_IO_ERROR_CONNECTION_REFUSED: GIOErrorEnum = 39;
pub const G_IO_ERROR_NETWORK_UNREACHABLE: GIOErrorEnum = 38;
pub const G_IO_ERROR_HOST_UNREACHABLE: GIOErrorEnum = 37;
pub const G_IO_ERROR_DBUS_ERROR: GIOErrorEnum = 36;
pub const G_IO_ERROR_INVALID_DATA: GIOErrorEnum = 35;
pub const G_IO_ERROR_PARTIAL_INPUT: GIOErrorEnum = 34;
pub const G_IO_ERROR_ADDRESS_IN_USE: GIOErrorEnum = 33;
pub const G_IO_ERROR_NOT_INITIALIZED: GIOErrorEnum = 32;
pub const G_IO_ERROR_TOO_MANY_OPEN_FILES: GIOErrorEnum = 31;
pub const G_IO_ERROR_FAILED_HANDLED: GIOErrorEnum = 30;
pub const G_IO_ERROR_WOULD_MERGE: GIOErrorEnum = 29;
pub const G_IO_ERROR_HOST_NOT_FOUND: GIOErrorEnum = 28;
pub const G_IO_ERROR_WOULD_BLOCK: GIOErrorEnum = 27;
pub const G_IO_ERROR_BUSY: GIOErrorEnum = 26;
pub const G_IO_ERROR_WOULD_RECURSE: GIOErrorEnum = 25;
pub const G_IO_ERROR_TIMED_OUT: GIOErrorEnum = 24;
pub const G_IO_ERROR_WRONG_ETAG: GIOErrorEnum = 23;
pub const G_IO_ERROR_CANT_CREATE_BACKUP: GIOErrorEnum = 22;
pub const G_IO_ERROR_READ_ONLY: GIOErrorEnum = 21;
pub const G_IO_ERROR_PENDING: GIOErrorEnum = 20;
pub const G_IO_ERROR_CANCELLED: GIOErrorEnum = 19;
pub const G_IO_ERROR_CLOSED: GIOErrorEnum = 18;
pub const G_IO_ERROR_ALREADY_MOUNTED: GIOErrorEnum = 17;
pub const G_IO_ERROR_NOT_MOUNTED: GIOErrorEnum = 16;
pub const G_IO_ERROR_NOT_SUPPORTED: GIOErrorEnum = 15;
pub const G_IO_ERROR_PERMISSION_DENIED: GIOErrorEnum = 14;
pub const G_IO_ERROR_INVALID_ARGUMENT: GIOErrorEnum = 13;
pub const G_IO_ERROR_NO_SPACE: GIOErrorEnum = 12;
pub const G_IO_ERROR_TOO_MANY_LINKS: GIOErrorEnum = 11;
pub const G_IO_ERROR_INVALID_FILENAME: GIOErrorEnum = 10;
pub const G_IO_ERROR_FILENAME_TOO_LONG: GIOErrorEnum = 9;
pub const G_IO_ERROR_NOT_MOUNTABLE_FILE: GIOErrorEnum = 8;
pub const G_IO_ERROR_NOT_SYMBOLIC_LINK: GIOErrorEnum = 7;
pub const G_IO_ERROR_NOT_REGULAR_FILE: GIOErrorEnum = 6;
pub const G_IO_ERROR_NOT_EMPTY: GIOErrorEnum = 5;
pub const G_IO_ERROR_NOT_DIRECTORY: GIOErrorEnum = 4;
pub const G_IO_ERROR_IS_DIRECTORY: GIOErrorEnum = 3;
pub const G_IO_ERROR_EXISTS: GIOErrorEnum = 2;
pub const G_IO_ERROR_NOT_FOUND: GIOErrorEnum = 1;
pub const G_IO_ERROR_FAILED: GIOErrorEnum = 0;
pub type GSocketFamily = ::core::ffi::c_uint;
pub const G_SOCKET_FAMILY_IPV6: GSocketFamily = 10;
pub const G_SOCKET_FAMILY_IPV4: GSocketFamily = 2;
pub const G_SOCKET_FAMILY_UNIX: GSocketFamily = 1;
pub const G_SOCKET_FAMILY_INVALID: GSocketFamily = 0;
pub type GAsyncResult = _GAsyncResult;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
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
pub type GSocketAddress = _GSocketAddress;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketAddress {
    pub parent_instance: GObject,
}
pub type GInitable = _GInitable;
pub type GIOExtension = _GIOExtension;
pub type GNetworkMonitor = _GNetworkMonitor;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocket {
    pub parent_instance: GObject,
    pub priv_0: *mut GSocketPrivate,
}
pub type GSocketPrivate = _GSocketPrivate;
pub type GSocket = _GSocket;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketControlMessage {
    pub parent_instance: GObject,
    pub priv_0: *mut GSocketControlMessagePrivate,
}
pub type GSocketControlMessagePrivate = _GSocketControlMessagePrivate;
pub type GSocketControlMessage = _GSocketControlMessage;
pub type GSocketConnectable = _GSocketConnectable;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInputVector {
    pub buffer: gpointer,
    pub size: gsize,
}
pub type GInputVector = _GInputVector;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GNetworkMonitorNetlinkPrivate {
    pub sock: *mut GSocket,
    pub source: *mut GSource,
    pub dump_source: *mut GSource,
    pub context: *mut GMainContext,
    pub dump_networks: *mut GPtrArray,
}
pub type GNetworkMonitorNetlink = _GNetworkMonitorNetlink;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GNetworkMonitorNetlinkClass {
    pub parent_class: GNetworkMonitorBaseClass,
    pub padding: [gpointer; 8],
}
pub type GNetworkMonitorNetlinkClass = _GNetworkMonitorNetlinkClass;
pub type GInitableIface = _GInitableIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInitableIface {
    pub g_iface: GTypeInterface,
    pub init: Option<
        unsafe extern "C" fn(*mut GInitable, *mut GCancellable, *mut *mut GError) -> gboolean,
    >,
}
pub type __u16 = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nlmsghdr {
    pub nlmsg_len: __u32,
    pub nlmsg_type: __u16,
    pub nlmsg_flags: __u16,
    pub nlmsg_seq: __u32,
    pub nlmsg_pid: __u32,
}
pub type __u32 = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nlmsgerr {
    pub error: ::core::ffi::c_int,
    pub msg: nlmsghdr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rtgenmsg {
    pub rtgen_family: ::core::ffi::c_uchar,
}
pub const RTM_GETROUTE: C2RustUnnamed_2 = 26;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rtmsg {
    pub rtm_family: ::core::ffi::c_uchar,
    pub rtm_dst_len: ::core::ffi::c_uchar,
    pub rtm_src_len: ::core::ffi::c_uchar,
    pub rtm_tos: ::core::ffi::c_uchar,
    pub rtm_table: ::core::ffi::c_uchar,
    pub rtm_protocol: ::core::ffi::c_uchar,
    pub rtm_scope: ::core::ffi::c_uchar,
    pub rtm_type: ::core::ffi::c_uchar,
    pub rtm_flags: ::core::ffi::c_uint,
}
pub const RTM_NEWROUTE: C2RustUnnamed_2 = 24;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rtattr {
    pub rta_len: ::core::ffi::c_ushort,
    pub rta_type: ::core::ffi::c_ushort,
}
pub const RTA_OIF: rtattr_type_t = 4;
pub const RTA_GATEWAY: rtattr_type_t = 5;
pub const RTA_DST: rtattr_type_t = 1;
pub const RTN_UNREACHABLE: C2RustUnnamed_3 = 7;
pub const RTM_DELROUTE: C2RustUnnamed_2 = 25;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_nl {
    pub nl_family: __kernel_sa_family_t,
    pub nl_pad: ::core::ffi::c_ushort,
    pub nl_pid: __u32,
    pub nl_groups: __u32,
}
pub type __kernel_sa_family_t = ::core::ffi::c_ushort;
pub const MSG_TRUNC: C2RustUnnamed_1 = 32;
pub const MSG_PEEK: C2RustUnnamed_1 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [::core::ffi::c_char; 14],
}
pub type sa_family_t = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __CONST_SOCKADDR_ARG {
    pub __sockaddr__: *const sockaddr,
    pub __sockaddr_at__: *const sockaddr_at,
    pub __sockaddr_ax25__: *const sockaddr_ax25,
    pub __sockaddr_dl__: *const sockaddr_dl,
    pub __sockaddr_eon__: *const sockaddr_eon,
    pub __sockaddr_in__: *const sockaddr_in,
    pub __sockaddr_in6__: *const sockaddr_in6,
    pub __sockaddr_inarp__: *const sockaddr_inarp,
    pub __sockaddr_ipx__: *const sockaddr_ipx,
    pub __sockaddr_iso__: *const sockaddr_iso,
    pub __sockaddr_ns__: *const sockaddr_ns,
    pub __sockaddr_un__: *const sockaddr_un,
    pub __sockaddr_x25__: *const sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [::core::ffi::c_char; 108],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type uint16_t = __uint16_t;
pub type uint8_t = __uint8_t;
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [::core::ffi::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
pub const SOCK_RAW: __socket_type = 3;
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
pub type __socket_type = ::core::ffi::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const MSG_CMSG_CLOEXEC: C2RustUnnamed_1 = 1073741824;
pub const MSG_FASTOPEN: C2RustUnnamed_1 = 536870912;
pub const MSG_ZEROCOPY: C2RustUnnamed_1 = 67108864;
pub const MSG_BATCH: C2RustUnnamed_1 = 262144;
pub const MSG_WAITFORONE: C2RustUnnamed_1 = 65536;
pub const MSG_MORE: C2RustUnnamed_1 = 32768;
pub const MSG_NOSIGNAL: C2RustUnnamed_1 = 16384;
pub const MSG_ERRQUEUE: C2RustUnnamed_1 = 8192;
pub const MSG_RST: C2RustUnnamed_1 = 4096;
pub const MSG_CONFIRM: C2RustUnnamed_1 = 2048;
pub const MSG_SYN: C2RustUnnamed_1 = 1024;
pub const MSG_FIN: C2RustUnnamed_1 = 512;
pub const MSG_WAITALL: C2RustUnnamed_1 = 256;
pub const MSG_EOR: C2RustUnnamed_1 = 128;
pub const MSG_DONTWAIT: C2RustUnnamed_1 = 64;
pub const MSG_PROXY: C2RustUnnamed_1 = 16;
pub const MSG_CTRUNC: C2RustUnnamed_1 = 8;
pub const MSG_TRYHARD: C2RustUnnamed_1 = 4;
pub const MSG_DONTROUTE: C2RustUnnamed_1 = 4;
pub const MSG_OOB: C2RustUnnamed_1 = 1;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const __RTM_MAX: C2RustUnnamed_2 = 123;
pub const RTM_GETTUNNEL: C2RustUnnamed_2 = 122;
pub const RTM_DELTUNNEL: C2RustUnnamed_2 = 121;
pub const RTM_NEWTUNNEL: C2RustUnnamed_2 = 120;
pub const RTM_GETNEXTHOPBUCKET: C2RustUnnamed_2 = 118;
pub const RTM_DELNEXTHOPBUCKET: C2RustUnnamed_2 = 117;
pub const RTM_NEWNEXTHOPBUCKET: C2RustUnnamed_2 = 116;
pub const RTM_GETVLAN: C2RustUnnamed_2 = 114;
pub const RTM_DELVLAN: C2RustUnnamed_2 = 113;
pub const RTM_NEWVLAN: C2RustUnnamed_2 = 112;
pub const RTM_GETLINKPROP: C2RustUnnamed_2 = 110;
pub const RTM_DELLINKPROP: C2RustUnnamed_2 = 109;
pub const RTM_NEWLINKPROP: C2RustUnnamed_2 = 108;
pub const RTM_GETNEXTHOP: C2RustUnnamed_2 = 106;
pub const RTM_DELNEXTHOP: C2RustUnnamed_2 = 105;
pub const RTM_NEWNEXTHOP: C2RustUnnamed_2 = 104;
pub const RTM_GETCHAIN: C2RustUnnamed_2 = 102;
pub const RTM_DELCHAIN: C2RustUnnamed_2 = 101;
pub const RTM_NEWCHAIN: C2RustUnnamed_2 = 100;
pub const RTM_NEWCACHEREPORT: C2RustUnnamed_2 = 96;
pub const RTM_SETSTATS: C2RustUnnamed_2 = 95;
pub const RTM_GETSTATS: C2RustUnnamed_2 = 94;
pub const RTM_NEWSTATS: C2RustUnnamed_2 = 92;
pub const RTM_GETNSID: C2RustUnnamed_2 = 90;
pub const RTM_DELNSID: C2RustUnnamed_2 = 89;
pub const RTM_NEWNSID: C2RustUnnamed_2 = 88;
pub const RTM_GETMDB: C2RustUnnamed_2 = 86;
pub const RTM_DELMDB: C2RustUnnamed_2 = 85;
pub const RTM_NEWMDB: C2RustUnnamed_2 = 84;
pub const RTM_GETNETCONF: C2RustUnnamed_2 = 82;
pub const RTM_DELNETCONF: C2RustUnnamed_2 = 81;
pub const RTM_NEWNETCONF: C2RustUnnamed_2 = 80;
pub const RTM_SETDCB: C2RustUnnamed_2 = 79;
pub const RTM_GETDCB: C2RustUnnamed_2 = 78;
pub const RTM_GETADDRLABEL: C2RustUnnamed_2 = 74;
pub const RTM_DELADDRLABEL: C2RustUnnamed_2 = 73;
pub const RTM_NEWADDRLABEL: C2RustUnnamed_2 = 72;
pub const RTM_NEWNDUSEROPT: C2RustUnnamed_2 = 68;
pub const RTM_SETNEIGHTBL: C2RustUnnamed_2 = 67;
pub const RTM_GETNEIGHTBL: C2RustUnnamed_2 = 66;
pub const RTM_NEWNEIGHTBL: C2RustUnnamed_2 = 64;
pub const RTM_GETANYCAST: C2RustUnnamed_2 = 62;
pub const RTM_GETMULTICAST: C2RustUnnamed_2 = 58;
pub const RTM_NEWPREFIX: C2RustUnnamed_2 = 52;
pub const RTM_GETACTION: C2RustUnnamed_2 = 50;
pub const RTM_DELACTION: C2RustUnnamed_2 = 49;
pub const RTM_NEWACTION: C2RustUnnamed_2 = 48;
pub const RTM_GETTFILTER: C2RustUnnamed_2 = 46;
pub const RTM_DELTFILTER: C2RustUnnamed_2 = 45;
pub const RTM_NEWTFILTER: C2RustUnnamed_2 = 44;
pub const RTM_GETTCLASS: C2RustUnnamed_2 = 42;
pub const RTM_DELTCLASS: C2RustUnnamed_2 = 41;
pub const RTM_NEWTCLASS: C2RustUnnamed_2 = 40;
pub const RTM_GETQDISC: C2RustUnnamed_2 = 38;
pub const RTM_DELQDISC: C2RustUnnamed_2 = 37;
pub const RTM_NEWQDISC: C2RustUnnamed_2 = 36;
pub const RTM_GETRULE: C2RustUnnamed_2 = 34;
pub const RTM_DELRULE: C2RustUnnamed_2 = 33;
pub const RTM_NEWRULE: C2RustUnnamed_2 = 32;
pub const RTM_GETNEIGH: C2RustUnnamed_2 = 30;
pub const RTM_DELNEIGH: C2RustUnnamed_2 = 29;
pub const RTM_NEWNEIGH: C2RustUnnamed_2 = 28;
pub const RTM_GETADDR: C2RustUnnamed_2 = 22;
pub const RTM_DELADDR: C2RustUnnamed_2 = 21;
pub const RTM_NEWADDR: C2RustUnnamed_2 = 20;
pub const RTM_SETLINK: C2RustUnnamed_2 = 19;
pub const RTM_GETLINK: C2RustUnnamed_2 = 18;
pub const RTM_DELLINK: C2RustUnnamed_2 = 17;
pub const RTM_NEWLINK: C2RustUnnamed_2 = 16;
pub const RTM_BASE: C2RustUnnamed_2 = 16;
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const __RTN_MAX: C2RustUnnamed_3 = 12;
pub const RTN_XRESOLVE: C2RustUnnamed_3 = 11;
pub const RTN_NAT: C2RustUnnamed_3 = 10;
pub const RTN_THROW: C2RustUnnamed_3 = 9;
pub const RTN_PROHIBIT: C2RustUnnamed_3 = 8;
pub const RTN_BLACKHOLE: C2RustUnnamed_3 = 6;
pub const RTN_MULTICAST: C2RustUnnamed_3 = 5;
pub const RTN_ANYCAST: C2RustUnnamed_3 = 4;
pub const RTN_BROADCAST: C2RustUnnamed_3 = 3;
pub const RTN_LOCAL: C2RustUnnamed_3 = 2;
pub const RTN_UNICAST: C2RustUnnamed_3 = 1;
pub const RTN_UNSPEC: C2RustUnnamed_3 = 0;
pub type rtattr_type_t = ::core::ffi::c_uint;
pub const __RTA_MAX: rtattr_type_t = 31;
pub const RTA_NH_ID: rtattr_type_t = 30;
pub const RTA_DPORT: rtattr_type_t = 29;
pub const RTA_SPORT: rtattr_type_t = 28;
pub const RTA_IP_PROTO: rtattr_type_t = 27;
pub const RTA_TTL_PROPAGATE: rtattr_type_t = 26;
pub const RTA_UID: rtattr_type_t = 25;
pub const RTA_PAD: rtattr_type_t = 24;
pub const RTA_EXPIRES: rtattr_type_t = 23;
pub const RTA_ENCAP: rtattr_type_t = 22;
pub const RTA_ENCAP_TYPE: rtattr_type_t = 21;
pub const RTA_PREF: rtattr_type_t = 20;
pub const RTA_NEWDST: rtattr_type_t = 19;
pub const RTA_VIA: rtattr_type_t = 18;
pub const RTA_MFC_STATS: rtattr_type_t = 17;
pub const RTA_MARK: rtattr_type_t = 16;
pub const RTA_TABLE: rtattr_type_t = 15;
pub const RTA_MP_ALGO: rtattr_type_t = 14;
pub const RTA_SESSION: rtattr_type_t = 13;
pub const RTA_CACHEINFO: rtattr_type_t = 12;
pub const RTA_FLOW: rtattr_type_t = 11;
pub const RTA_PROTOINFO: rtattr_type_t = 10;
pub const RTA_MULTIPATH: rtattr_type_t = 9;
pub const RTA_METRICS: rtattr_type_t = 8;
pub const RTA_PREFSRC: rtattr_type_t = 7;
pub const RTA_PRIORITY: rtattr_type_t = 6;
pub const RTA_IIF: rtattr_type_t = 3;
pub const RTA_SRC: rtattr_type_t = 2;
pub const RTA_UNSPEC: rtattr_type_t = 0;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SOL_SOCKET: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SO_PASSCRED: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const PF_UNSPEC: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PF_INET6: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const PF_NETLINK: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const AF_UNSPEC: ::core::ffi::c_int = PF_UNSPEC;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const AF_INET6: ::core::ffi::c_int = PF_INET6;
pub const AF_NETLINK: ::core::ffi::c_int = PF_NETLINK;
pub const NETLINK_ROUTE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NLM_F_REQUEST: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const NLM_F_ROOT: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const NLM_F_MATCH: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const NLM_F_DUMP: ::core::ffi::c_int = NLM_F_ROOT | NLM_F_MATCH;
pub const NLMSG_ALIGNTO: ::core::ffi::c_uint = 4 as ::core::ffi::c_uint;
pub const NLMSG_HDRLEN: ::core::ffi::c_int = ((::core::mem::size_of::<nlmsghdr>() as usize)
    .wrapping_add(NLMSG_ALIGNTO as usize)
    .wrapping_sub(1 as usize)
    & !NLMSG_ALIGNTO.wrapping_sub(1 as ::core::ffi::c_uint) as usize)
    as ::core::ffi::c_int;
pub const NLMSG_ERROR: ::core::ffi::c_int = 2;
pub const NLMSG_DONE: ::core::ffi::c_int = 3;
pub const RTM_DELROUTE_0: ::core::ffi::c_int = 25;
pub const RTA_ALIGNTO: ::core::ffi::c_uint = 4 as ::core::ffi::c_uint;
pub const RTMGRP_IPV4_ROUTE: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const RTMGRP_IPV6_ROUTE: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
static mut safe_c2rust_initable_parent_iface: *mut GInitableIface =
    ::core::ptr::null::<GInitableIface>() as *mut GInitableIface;
unsafe extern "C" fn safe_c2rust_g_network_monitor_netlink_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_network_monitor_netlink_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GNetworkMonitorNetlink_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GNetworkMonitorNetlink_private_offset,
        );
    }
    safe_c2rust_g_network_monitor_netlink_class_init(klass as *mut GNetworkMonitorNetlinkClass);
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_network_monitor_netlink_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_network_monitor_base_get_type(),
        g_intern_static_string(b"GNetworkMonitorNetlink\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GNetworkMonitorNetlinkClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_network_monitor_netlink_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GNetworkMonitorNetlink>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GNetworkMonitorNetlink) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_network_monitor_netlink_init
                    as unsafe extern "C" fn(*mut GNetworkMonitorNetlink) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GNetworkMonitorNetlink_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GNetworkMonitorNetlinkPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GNetworkMonitorInterface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_network_monitor_netlink_iface_init
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
            safe_c2rust_g_network_monitor_netlink_initable_iface_init
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
        b"netlink\0" as *const u8 as *const ::core::ffi::c_char,
        20 as gint,
    );
    return g_define_type_id;
}
static mut safe_c2rust_g_network_monitor_netlink_parent_class: gpointer = NULL;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_network_monitor_netlink_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_network_monitor_netlink_get_type_once();
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
static mut safe_c2rust_GNetworkMonitorNetlink_private_offset: gint = 0;
#[inline]
unsafe extern "C" fn safe_c2rust_g_network_monitor_netlink_get_instance_private(
    mut self_0: *mut GNetworkMonitorNetlink,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GNetworkMonitorNetlink_private_offset as glong as isize)
        as gpointer;
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_netlink_init(
    mut nl: *mut GNetworkMonitorNetlink,
) {
    (*nl).priv_0 = safe_c2rust_g_network_monitor_netlink_get_instance_private(nl)
        as *mut GNetworkMonitorNetlinkPrivate;
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_netlink_initable_init(
    mut initable: *mut GInitable,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut nl: *mut GNetworkMonitorNetlink =
        initable as *mut ::core::ffi::c_void as *mut GNetworkMonitorNetlink;
    let mut sockfd: gint = 0;
    let mut snl: sockaddr_nl = sockaddr_nl {
        nl_family: 0,
        nl_pad: 0,
        nl_pid: 0,
        nl_groups: 0,
    };
    sockfd = g_socket(
        PF_NETLINK,
        SOCK_RAW as ::core::ffi::c_int as gint,
        NETLINK_ROUTE,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if sockfd == -(1 as ::core::ffi::c_int) {
        let mut errsv: ::core::ffi::c_int = *__errno_location();
        g_set_error(
            error,
            g_io_error_quark(),
            g_io_error_from_errno(errsv as gint) as gint,
            glib_gettext(b"Could not create network monitor: %s\0" as *const u8 as *const gchar),
            g_strerror(errsv as gint),
        );
        return FALSE;
    }
    snl.nl_family = AF_NETLINK as __kernel_sa_family_t;
    snl.nl_pad = 0 as ::core::ffi::c_ushort;
    snl.nl_pid = snl.nl_pad as __u32;
    snl.nl_groups = (RTMGRP_IPV4_ROUTE | RTMGRP_IPV6_ROUTE) as __u32;
    if bind(
        sockfd as ::core::ffi::c_int,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: &raw mut snl as *mut sockaddr,
        },
        ::core::mem::size_of::<sockaddr_nl>() as socklen_t,
    ) != 0 as ::core::ffi::c_int
    {
        let mut errsv_0: ::core::ffi::c_int = *__errno_location();
        g_set_error(
            error,
            g_io_error_quark(),
            g_io_error_from_errno(errsv_0 as gint) as gint,
            glib_gettext(b"Could not create network monitor: %s\0" as *const u8 as *const gchar),
            g_strerror(errsv_0 as gint),
        );
        g_close(sockfd, ::core::ptr::null_mut::<*mut GError>());
        return FALSE;
    }
    (*(*nl).priv_0).sock = g_socket_new_from_fd(sockfd, error);
    if (*(*nl).priv_0).sock.is_null() {
        g_prefix_error(
            error,
            b"%s\0" as *const u8 as *const gchar,
            glib_gettext(b"Could not create network monitor: \0" as *const u8 as *const gchar),
        );
        g_close(sockfd, ::core::ptr::null_mut::<*mut GError>());
        return FALSE;
    }
    if g_socket_set_option(
        (*(*nl).priv_0).sock,
        SOL_SOCKET,
        SO_PASSCRED,
        TRUE,
        ::core::ptr::null_mut::<*mut GError>(),
    ) == 0
    {
        let mut errsv_1: ::core::ffi::c_int = *__errno_location();
        g_set_error(
            error,
            g_io_error_quark(),
            g_io_error_from_errno(errsv_1 as gint) as gint,
            glib_gettext(b"Could not create network monitor: %s\0" as *const u8 as *const gchar),
            g_strerror(errsv_1 as gint),
        );
        return FALSE;
    }
    if safe_c2rust_request_dump(nl, error) == 0 {
        return FALSE;
    }
    while !(*(*nl).priv_0).dump_networks.is_null() {
        let mut local_error: *mut GError = ::core::ptr::null_mut::<GError>();
        if !(safe_c2rust_read_netlink_messages(nl, &raw mut local_error) == 0) {
            continue;
        }
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"%s\0" as *const u8 as *const gchar,
            (*local_error).message,
        );
        g_clear_error(&raw mut local_error);
        break;
    }
    g_socket_set_blocking((*(*nl).priv_0).sock, FALSE);
    (*(*nl).priv_0).context = g_main_context_ref_thread_default();
    (*(*nl).priv_0).source = g_socket_create_source(
        (*(*nl).priv_0).sock,
        G_IO_IN,
        ::core::ptr::null_mut::<GCancellable>(),
    );
    g_source_set_callback(
        (*(*nl).priv_0).source,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GSocket, GIOCondition, gpointer) -> gboolean>,
            GSourceFunc,
        >(Some(
            safe_c2rust_read_netlink_messages_callback
                as unsafe extern "C" fn(*mut GSocket, GIOCondition, gpointer) -> gboolean,
        )),
        nl as gpointer,
        None,
    );
    g_source_attach((*(*nl).priv_0).source, (*(*nl).priv_0).context);
    return (*safe_c2rust_initable_parent_iface)
        .init
        .expect("non-null function pointer")(initable, cancellable, error);
}
unsafe extern "C" fn safe_c2rust_request_dump(
    mut nl: *mut GNetworkMonitorNetlink,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut n: *mut nlmsghdr = ::core::ptr::null_mut::<nlmsghdr>();
    let mut gen: *mut rtgenmsg = ::core::ptr::null_mut::<rtgenmsg>();
    let mut buf: [gchar; 20] = [0; 20];
    memset(
        &raw mut buf as *mut gchar as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[gchar; 20]>() as size_t,
    );
    n = &raw mut buf as *mut gchar as *mut nlmsghdr;
    (*n).nlmsg_len =
        (::core::mem::size_of::<rtgenmsg>() as usize).wrapping_add(NLMSG_HDRLEN as usize) as __u32;
    (*n).nlmsg_type = RTM_GETROUTE as ::core::ffi::c_int as __u16;
    (*n).nlmsg_flags = (NLM_F_REQUEST | NLM_F_DUMP) as __u16;
    (*n).nlmsg_pid = 0 as __u32;
    gen = (n as *mut ::core::ffi::c_char).offset(NLMSG_HDRLEN as isize) as *mut ::core::ffi::c_void
        as *mut rtgenmsg;
    (*gen).rtgen_family = AF_UNSPEC as ::core::ffi::c_uchar;
    if g_socket_send(
        (*(*nl).priv_0).sock,
        &raw mut buf as *mut gchar,
        ::core::mem::size_of::<[gchar; 20]>() as gsize,
        ::core::ptr::null_mut::<GCancellable>(),
        error,
    ) < 0 as gssize
    {
        g_prefix_error(
            error,
            b"%s\0" as *const u8 as *const gchar,
            glib_gettext(b"Could not get network status: \0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    (*(*nl).priv_0).dump_networks = g_ptr_array_new_with_free_func(Some(
        g_object_unref as unsafe extern "C" fn(gpointer) -> (),
    ));
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_timeout_request_dump(mut user_data: gpointer) -> gboolean {
    let mut nl: *mut GNetworkMonitorNetlink = user_data as *mut GNetworkMonitorNetlink;
    g_source_destroy((*(*nl).priv_0).dump_source);
    g_source_unref((*(*nl).priv_0).dump_source);
    (*(*nl).priv_0).dump_source = ::core::ptr::null_mut::<GSource>();
    safe_c2rust_request_dump(nl, ::core::ptr::null_mut::<*mut GError>());
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_queue_request_dump(mut nl: *mut GNetworkMonitorNetlink) {
    if !(*(*nl).priv_0).dump_networks.is_null() {
        return;
    }
    if !(*(*nl).priv_0).dump_source.is_null() {
        g_source_destroy((*(*nl).priv_0).dump_source);
        g_source_unref((*(*nl).priv_0).dump_source);
    }
    (*(*nl).priv_0).dump_source = g_timeout_source_new_seconds(1 as guint);
    g_source_set_callback(
        (*(*nl).priv_0).dump_source,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> gboolean>, GSourceFunc>(
            Some(safe_c2rust_timeout_request_dump as unsafe extern "C" fn(gpointer) -> gboolean),
        ),
        nl as gpointer,
        None,
    );
    g_source_attach((*(*nl).priv_0).dump_source, (*(*nl).priv_0).context);
}
unsafe extern "C" fn safe_c2rust_create_inet_address_mask(
    mut family: GSocketFamily,
    mut dest: *const guint8,
    mut dest_len: gsize,
) -> *mut GInetAddressMask {
    let mut dest_addr: *mut GInetAddress = ::core::ptr::null_mut::<GInetAddress>();
    let mut network: *mut GInetAddressMask = ::core::ptr::null_mut::<GInetAddressMask>();
    if !dest.is_null() {
        dest_addr = g_inet_address_new_from_bytes(dest, family);
    } else {
        dest_addr = g_inet_address_new_any(family);
    }
    network = g_inet_address_mask_new(
        dest_addr,
        dest_len as guint,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    g_object_unref(dest_addr as gpointer);
    return network;
}
unsafe extern "C" fn safe_c2rust_add_network(
    mut nl: *mut GNetworkMonitorNetlink,
    mut family: GSocketFamily,
    mut dest: *const guint8,
    mut dest_len: gsize,
) {
    let mut network: *mut GInetAddressMask =
        safe_c2rust_create_inet_address_mask(family, dest, dest_len);
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !network.is_null() {
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
            b"network != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !(*(*nl).priv_0).dump_networks.is_null() {
        g_ptr_array_add(
            (*(*nl).priv_0).dump_networks,
            g_object_ref(network as gpointer) as *mut GInetAddressMask as gpointer,
        );
    } else {
        g_network_monitor_base_add_network(
            nl as *mut ::core::ffi::c_void as *mut GNetworkMonitorBase,
            network,
        );
    }
    g_object_unref(network as gpointer);
}
unsafe extern "C" fn safe_c2rust_remove_network(
    mut nl: *mut GNetworkMonitorNetlink,
    mut family: GSocketFamily,
    mut dest: *const guint8,
    mut dest_len: gsize,
) {
    let mut network: *mut GInetAddressMask =
        safe_c2rust_create_inet_address_mask(family, dest, dest_len);
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !network.is_null() {
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
            b"network != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !(*(*nl).priv_0).dump_networks.is_null() {
        let mut dump_networks: *mut *mut GInetAddressMask =
            (*(*(*nl).priv_0).dump_networks).pdata as *mut *mut GInetAddressMask;
        let mut i: guint = 0;
        i = 0 as guint;
        while i < (*(*(*nl).priv_0).dump_networks).len {
            if g_inet_address_mask_equal(network, *dump_networks.offset(i as isize)) != 0 {
                let fresh0 = i;
                i = i.wrapping_sub(1);
                g_ptr_array_remove_index_fast((*(*nl).priv_0).dump_networks, fresh0);
            }
            i = i.wrapping_add(1);
        }
    } else {
        g_network_monitor_base_remove_network(
            nl as *mut ::core::ffi::c_void as *mut GNetworkMonitorBase,
            network,
        );
    }
    g_object_unref(network as gpointer);
}
unsafe extern "C" fn safe_c2rust_finish_dump(mut nl: *mut GNetworkMonitorNetlink) {
    g_network_monitor_base_set_networks(
        nl as *mut ::core::ffi::c_void as *mut GNetworkMonitorBase,
        (*(*(*nl).priv_0).dump_networks).pdata as *mut *mut GInetAddressMask,
        (*(*(*nl).priv_0).dump_networks).len as gint,
    );
    g_ptr_array_free((*(*nl).priv_0).dump_networks, TRUE);
    (*(*nl).priv_0).dump_networks = ::core::ptr::null_mut::<GPtrArray>();
}
unsafe extern "C" fn safe_c2rust_read_netlink_messages(
    mut nl: *mut GNetworkMonitorNetlink,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iv: GInputVector = _GInputVector {
        buffer: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        size: 0,
    };
    let mut len: gssize = 0;
    let mut flags: gint = 0;
    let mut local_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut addr: *mut GSocketAddress = ::core::ptr::null_mut::<GSocketAddress>();
    let mut msg: *mut nlmsghdr = ::core::ptr::null_mut::<nlmsghdr>();
    let mut rtmsg: *mut rtmsg = ::core::ptr::null_mut::<rtmsg>();
    let mut attr: *mut rtattr = ::core::ptr::null_mut::<rtattr>();
    let mut source_sockaddr: sockaddr_nl = sockaddr_nl {
        nl_family: 0,
        nl_pad: 0,
        nl_pid: 0,
        nl_groups: 0,
    };
    let mut attrlen: gsize = 0;
    let mut dest: *mut guint8 = ::core::ptr::null_mut::<guint8>();
    let mut gateway: *mut guint8 = ::core::ptr::null_mut::<guint8>();
    let mut oif: *mut guint8 = ::core::ptr::null_mut::<guint8>();
    let mut retval: gboolean = TRUE;
    iv.buffer = NULL as gpointer;
    iv.size = 0 as gsize;
    flags = (MSG_PEEK as ::core::ffi::c_int | MSG_TRUNC as ::core::ffi::c_int) as gint;
    len = g_socket_receive_message(
        (*(*nl).priv_0).sock,
        ::core::ptr::null_mut::<*mut GSocketAddress>(),
        &raw mut iv,
        1 as gint,
        ::core::ptr::null_mut::<*mut *mut GSocketControlMessage>(),
        ::core::ptr::null_mut::<gint>(),
        &raw mut flags,
        ::core::ptr::null_mut::<GCancellable>(),
        &raw mut local_error,
    );
    if len < 0 as gssize {
        retval = FALSE as gboolean;
    } else {
        iv.buffer = g_malloc(len as gsize);
        iv.size = len as gsize;
        len = g_socket_receive_message(
            (*(*nl).priv_0).sock,
            &raw mut addr,
            &raw mut iv,
            1 as gint,
            ::core::ptr::null_mut::<*mut *mut GSocketControlMessage>(),
            ::core::ptr::null_mut::<gint>(),
            ::core::ptr::null_mut::<gint>(),
            ::core::ptr::null_mut::<GCancellable>(),
            &raw mut local_error,
        );
        if len < 0 as gssize {
            retval = FALSE as gboolean;
        } else if g_socket_address_to_native(
            addr,
            &raw mut source_sockaddr as gpointer,
            ::core::mem::size_of::<sockaddr_nl>() as gsize,
            &raw mut local_error,
        ) == 0
        {
            retval = FALSE as gboolean;
        } else if !(source_sockaddr.nl_pid != 0 as __u32) {
            msg = iv.buffer as *mut nlmsghdr;
            while len > 0 as gssize {
                if !(len as size_t
                    >= ::core::mem::size_of::<nlmsghdr>() as ::core::ffi::c_int as size_t
                    && (*msg).nlmsg_len as usize >= ::core::mem::size_of::<nlmsghdr>() as usize
                    && (*msg).nlmsg_len as size_t <= len as size_t)
                {
                    g_set_error_literal(
                        &raw mut local_error,
                        g_io_error_quark(),
                        G_IO_ERROR_PARTIAL_INPUT as ::core::ffi::c_int as gint,
                        b"netlink message was truncated; shouldn't happen...\0" as *const u8
                            as *const gchar,
                    );
                    retval = FALSE as gboolean;
                    break;
                } else {
                    match (*msg).nlmsg_type as ::core::ffi::c_int {
                        24 | RTM_DELROUTE_0 => {
                            rtmsg = (msg as *mut ::core::ffi::c_char).offset(NLMSG_HDRLEN as isize)
                                as *mut ::core::ffi::c_void
                                as *mut rtmsg;
                            if !((*rtmsg).rtm_family as ::core::ffi::c_int != AF_INET
                                && (*rtmsg).rtm_family as ::core::ffi::c_int != AF_INET6)
                            {
                                if !((*rtmsg).rtm_type as ::core::ffi::c_int
                                    == RTN_UNREACHABLE as ::core::ffi::c_int)
                                {
                                    attrlen = ((*msg).nlmsg_len as usize).wrapping_sub(
                                        (::core::mem::size_of::<rtmsg>() as usize)
                                            .wrapping_add(
                                                ((::core::mem::size_of::<nlmsghdr>() as usize)
                                                    .wrapping_add(4 as usize)
                                                    .wrapping_sub(1 as usize)
                                                    & !(4 as ::core::ffi::c_uint)
                                                        .wrapping_sub(1 as ::core::ffi::c_uint)
                                                        as usize)
                                                    as ::core::ffi::c_int
                                                    as usize,
                                            )
                                            .wrapping_add(NLMSG_ALIGNTO as usize)
                                            .wrapping_sub(1 as usize)
                                            & !NLMSG_ALIGNTO.wrapping_sub(1 as ::core::ffi::c_uint)
                                                as usize,
                                    ) as gsize;
                                    attr = (rtmsg as *mut ::core::ffi::c_char).offset(
                                        ((::core::mem::size_of::<rtmsg>() as usize)
                                            .wrapping_add(NLMSG_ALIGNTO as usize)
                                            .wrapping_sub(1 as usize)
                                            & !NLMSG_ALIGNTO.wrapping_sub(1 as ::core::ffi::c_uint)
                                                as usize)
                                            as isize,
                                    ) as *mut rtattr;
                                    oif = ::core::ptr::null_mut::<guint8>();
                                    gateway = oif;
                                    dest = gateway;
                                    while attrlen
                                        >= ::core::mem::size_of::<rtattr>() as ::core::ffi::c_int
                                            as gsize
                                        && (*attr).rta_len as usize
                                            >= ::core::mem::size_of::<rtattr>() as usize
                                        && (*attr).rta_len as gsize <= attrlen
                                    {
                                        if (*attr).rta_type as ::core::ffi::c_int
                                            == RTA_DST as ::core::ffi::c_int
                                        {
                                            dest = (attr as *mut ::core::ffi::c_char).offset(
                                                ((::core::mem::size_of::<rtattr>() as usize)
                                                    .wrapping_add(RTA_ALIGNTO as usize)
                                                    .wrapping_sub(1 as usize)
                                                    & !RTA_ALIGNTO
                                                        .wrapping_sub(1 as ::core::ffi::c_uint)
                                                        as usize)
                                                    .wrapping_add(0 as usize)
                                                    as isize,
                                            )
                                                as *mut ::core::ffi::c_void
                                                as *mut guint8;
                                        } else if (*attr).rta_type as ::core::ffi::c_int
                                            == RTA_GATEWAY as ::core::ffi::c_int
                                        {
                                            gateway = (attr as *mut ::core::ffi::c_char).offset(
                                                ((::core::mem::size_of::<rtattr>() as usize)
                                                    .wrapping_add(RTA_ALIGNTO as usize)
                                                    .wrapping_sub(1 as usize)
                                                    & !RTA_ALIGNTO
                                                        .wrapping_sub(1 as ::core::ffi::c_uint)
                                                        as usize)
                                                    .wrapping_add(0 as usize)
                                                    as isize,
                                            )
                                                as *mut ::core::ffi::c_void
                                                as *mut guint8;
                                        } else if (*attr).rta_type as ::core::ffi::c_int
                                            == RTA_OIF as ::core::ffi::c_int
                                        {
                                            oif = (attr as *mut ::core::ffi::c_char).offset(
                                                ((::core::mem::size_of::<rtattr>() as usize)
                                                    .wrapping_add(RTA_ALIGNTO as usize)
                                                    .wrapping_sub(1 as usize)
                                                    & !RTA_ALIGNTO
                                                        .wrapping_sub(1 as ::core::ffi::c_uint)
                                                        as usize)
                                                    .wrapping_add(0 as usize)
                                                    as isize,
                                            )
                                                as *mut ::core::ffi::c_void
                                                as *mut guint8;
                                        }
                                        attrlen = attrlen.wrapping_sub(
                                            (((*attr).rta_len as ::core::ffi::c_uint)
                                                .wrapping_add(RTA_ALIGNTO)
                                                .wrapping_sub(1 as ::core::ffi::c_uint)
                                                & !RTA_ALIGNTO
                                                    .wrapping_sub(1 as ::core::ffi::c_uint))
                                                as gsize,
                                        );
                                        attr = (attr as *mut ::core::ffi::c_char).offset(
                                            (((*attr).rta_len as ::core::ffi::c_uint)
                                                .wrapping_add(RTA_ALIGNTO)
                                                .wrapping_sub(1 as ::core::ffi::c_uint)
                                                & !RTA_ALIGNTO
                                                    .wrapping_sub(1 as ::core::ffi::c_uint))
                                                as isize,
                                        )
                                            as *mut rtattr;
                                    }
                                    if !dest.is_null() || !gateway.is_null() || !oif.is_null() {
                                        if !((*(*nl).priv_0).dump_networks.is_null()
                                            && (*rtmsg).rtm_family as ::core::ffi::c_int
                                                == AF_INET6
                                            && (*rtmsg).rtm_dst_len as ::core::ffi::c_int
                                                != 0 as ::core::ffi::c_int
                                            && (!dest.is_null()
                                                && (*dest.offset(0 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_int
                                                    == 0xff as ::core::ffi::c_int
                                                    && *dest
                                                        .offset(1 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int
                                                        & 0xf as ::core::ffi::c_int
                                                        == 0x2 as ::core::ffi::c_int)))
                                        {
                                            if (*msg).nlmsg_type as ::core::ffi::c_int
                                                == RTM_NEWROUTE as ::core::ffi::c_int
                                            {
                                                safe_c2rust_add_network(
                                                    nl,
                                                    (*rtmsg).rtm_family as GSocketFamily,
                                                    dest,
                                                    (*rtmsg).rtm_dst_len as gsize,
                                                );
                                            } else {
                                                safe_c2rust_remove_network(
                                                    nl,
                                                    (*rtmsg).rtm_family as GSocketFamily,
                                                    dest,
                                                    (*rtmsg).rtm_dst_len as gsize,
                                                );
                                            }
                                            safe_c2rust_queue_request_dump(nl);
                                        }
                                    }
                                }
                            }
                            len -= (((*msg).nlmsg_len as ::core::ffi::c_uint)
                                .wrapping_add(NLMSG_ALIGNTO)
                                .wrapping_sub(1 as ::core::ffi::c_uint)
                                & !NLMSG_ALIGNTO.wrapping_sub(1 as ::core::ffi::c_uint))
                                as gssize;
                            msg = (msg as *mut ::core::ffi::c_char).offset(
                                (((*msg).nlmsg_len as ::core::ffi::c_uint)
                                    .wrapping_add(NLMSG_ALIGNTO)
                                    .wrapping_sub(1 as ::core::ffi::c_uint)
                                    & !NLMSG_ALIGNTO.wrapping_sub(1 as ::core::ffi::c_uint))
                                    as isize,
                            ) as *mut nlmsghdr;
                        }
                        NLMSG_DONE => {
                            safe_c2rust_finish_dump(nl);
                            break;
                        }
                        NLMSG_ERROR => {
                            let mut e: *mut nlmsgerr = (msg as *mut ::core::ffi::c_char)
                                .offset(NLMSG_HDRLEN as isize)
                                as *mut ::core::ffi::c_void
                                as *mut nlmsgerr;
                            g_set_error(
                                &raw mut local_error,
                                g_io_error_quark(),
                                g_io_error_from_errno(-((*e).error as gint)) as gint,
                                b"netlink error: %s\0" as *const u8 as *const gchar,
                                g_strerror(-((*e).error as gint)),
                            );
                            retval = FALSE as gboolean;
                            break;
                        }
                        _ => {
                            g_set_error(
                                &raw mut local_error,
                                g_io_error_quark(),
                                G_IO_ERROR_INVALID_DATA as ::core::ffi::c_int as gint,
                                b"unexpected netlink message %d\0" as *const u8 as *const gchar,
                                (*msg).nlmsg_type as ::core::ffi::c_int,
                            );
                            retval = FALSE as gboolean;
                            break;
                        }
                    }
                }
            }
        }
    }
    g_free(iv.buffer);
    let mut _pp: *mut *mut GSocketAddress = &raw mut addr;
    let mut _ptr: *mut GSocketAddress = *_pp;
    *_pp = ::core::ptr::null_mut::<GSocketAddress>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    if retval == 0 && !(*(*nl).priv_0).dump_networks.is_null() {
        safe_c2rust_finish_dump(nl);
    }
    if !local_error.is_null() {
        g_propagate_prefixed_error(
            error,
            local_error,
            b"Error on netlink socket: \0" as *const u8 as *const gchar,
        );
    }
    return retval;
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_netlink_finalize(mut object: *mut GObject) {
    let mut nl: *mut GNetworkMonitorNetlink =
        object as *mut ::core::ffi::c_void as *mut GNetworkMonitorNetlink;
    if !(*(*nl).priv_0).source.is_null() {
        g_source_destroy((*(*nl).priv_0).source);
        g_source_unref((*(*nl).priv_0).source);
    }
    if !(*(*nl).priv_0).dump_source.is_null() {
        g_source_destroy((*(*nl).priv_0).dump_source);
        g_source_unref((*(*nl).priv_0).dump_source);
    }
    if !(*(*nl).priv_0).sock.is_null() {
        g_socket_close((*(*nl).priv_0).sock, ::core::ptr::null_mut::<*mut GError>());
        g_object_unref((*(*nl).priv_0).sock as gpointer);
    }
    let mut _pp: *mut *mut GMainContext = &raw mut (*(*nl).priv_0).context;
    let mut _ptr: *mut GMainContext = *_pp;
    *_pp = ::core::ptr::null_mut::<GMainContext>();
    if !_ptr.is_null() {
        g_main_context_unref(_ptr as *mut GMainContext);
    }
    let mut _pp_0: *mut *mut GPtrArray = &raw mut (*(*nl).priv_0).dump_networks;
    let mut _ptr_0: *mut GPtrArray = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<GPtrArray>();
    if !_ptr_0.is_null() {
        g_ptr_array_unref(_ptr_0 as *mut GPtrArray);
    }
    (*(safe_c2rust_g_network_monitor_netlink_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_read_netlink_messages_callback(
    mut socket: *mut GSocket,
    mut condition: GIOCondition,
    mut user_data: gpointer,
) -> gboolean {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut nl: *mut GNetworkMonitorNetlink = user_data as *mut GNetworkMonitorNetlink;
    if safe_c2rust_read_netlink_messages(nl, &raw mut error) == 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Error reading netlink message: %s\0" as *const u8 as *const gchar,
            (*error).message,
        );
        g_clear_error(&raw mut error);
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_netlink_class_init(
    mut nl_class: *mut GNetworkMonitorNetlinkClass,
) {
    let mut gobject_class: *mut GObjectClass =
        nl_class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize = Some(
        safe_c2rust_g_network_monitor_netlink_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_netlink_iface_init(
    mut monitor_iface: *mut GNetworkMonitorInterface,
) {
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_netlink_initable_iface_init(
    mut iface: *mut GInitableIface,
) {
    safe_c2rust_initable_parent_iface =
        g_type_interface_peek_parent(iface as gpointer) as *mut GInitableIface;
    (*iface).init = Some(
        safe_c2rust_g_network_monitor_netlink_initable_init
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
