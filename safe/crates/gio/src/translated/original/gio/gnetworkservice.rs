extern "C" {
    pub type _GData;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _GResolverPrivate;
    pub type _GSocketConnectable;
    pub type _GSrvTarget;
    pub type _GTask;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_new(domain: GQuark, code: gint, format: *const gchar, ...) -> *mut GError;
    fn g_error_free(error: *mut GError);
    fn g_error_matches(error: *const GError, domain: GQuark, code: gint) -> gboolean;
    fn g_propagate_error(dest: *mut *mut GError, src: *mut GError);
    fn g_clear_error(err: *mut *mut GError);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_list_append(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_hostname_to_ascii(hostname: *const gchar) -> *mut gchar;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_uri_join(
        flags: GUriFlags,
        scheme: *const gchar,
        userinfo: *const gchar,
        host: *const gchar,
        port: gint,
        path: *const gchar,
        query: *const gchar,
        fragment: *const gchar,
    ) -> *mut gchar;
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
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_param_spec_string(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: *const gchar,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_value_set_string(value: *mut GValue, v_string: *const gchar);
    fn g_value_get_string(value: *const GValue) -> *const gchar;
    fn g_value_dup_string(value: *const GValue) -> *mut gchar;
    fn g_io_error_quark() -> GQuark;
    fn g_network_address_new(hostname: *const gchar, port: guint16) -> *mut GSocketConnectable;
    fn g_network_address_parse_uri(
        uri: *const gchar,
        default_port: guint16,
        error: *mut *mut GError,
    ) -> *mut GSocketConnectable;
    fn endservent();
    fn g_resolver_get_default() -> *mut GResolver;
    fn g_resolver_lookup_service(
        resolver: *mut GResolver,
        service: *const gchar,
        protocol: *const gchar,
        domain: *const gchar,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GList;
    fn g_resolver_lookup_service_async(
        resolver: *mut GResolver,
        service: *const gchar,
        protocol: *const gchar,
        domain: *const gchar,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_resolver_lookup_service_finish(
        resolver: *mut GResolver,
        result: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> *mut GList;
    fn g_resolver_free_targets(targets: *mut GList);
    fn g_resolver_error_quark() -> GQuark;
    fn g_getservbyname_ntohs(
        name: *const ::core::ffi::c_char,
        proto: *const ::core::ffi::c_char,
        out_port: *mut guint16,
    ) -> gboolean;
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
    fn g_task_return_pointer(task: *mut GTask, result: gpointer, result_destroy: GDestroyNotify);
    fn g_task_return_error(task: *mut GTask, error: *mut GError);
    fn g_task_propagate_pointer(task: *mut GTask, error: *mut *mut GError) -> gpointer;
    fn g_socket_address_enumerator_get_type() -> GType;
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
    fn g_socket_connectable_get_type() -> GType;
    fn g_socket_connectable_enumerate(
        connectable: *mut GSocketConnectable,
    ) -> *mut GSocketAddressEnumerator;
    fn g_socket_connectable_proxy_enumerate(
        connectable: *mut GSocketConnectable,
    ) -> *mut GSocketAddressEnumerator;
    fn g_srv_target_new(
        hostname: *const gchar,
        port: guint16,
        priority: guint16,
        weight: guint16,
    ) -> *mut GSrvTarget;
    fn g_srv_target_get_hostname(target: *mut GSrvTarget) -> *const gchar;
    fn g_srv_target_get_port(target: *mut GSrvTarget) -> guint16;
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
pub type GUriFlags = ::core::ffi::c_uint;
pub const G_URI_FLAGS_SCHEME_NORMALIZE: GUriFlags = 256;
pub const G_URI_FLAGS_ENCODED_FRAGMENT: GUriFlags = 128;
pub const G_URI_FLAGS_ENCODED_PATH: GUriFlags = 64;
pub const G_URI_FLAGS_ENCODED_QUERY: GUriFlags = 32;
pub const G_URI_FLAGS_NON_DNS: GUriFlags = 16;
pub const G_URI_FLAGS_ENCODED: GUriFlags = 8;
pub const G_URI_FLAGS_HAS_AUTH_PARAMS: GUriFlags = 4;
pub const G_URI_FLAGS_HAS_PASSWORD: GUriFlags = 2;
pub const G_URI_FLAGS_PARSE_RELAXED: GUriFlags = 1;
pub const G_URI_FLAGS_NONE: GUriFlags = 0;
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
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const G_RESOLVER_ERROR_INTERNAL: C2RustUnnamed_1 = 2;
pub const G_RESOLVER_ERROR_TEMPORARY_FAILURE: C2RustUnnamed_1 = 1;
pub const G_RESOLVER_ERROR_NOT_FOUND: C2RustUnnamed_1 = 0;
pub type GAsyncResult = _GAsyncResult;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
pub type GSocketAddress = _GSocketAddress;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketAddress {
    pub parent_instance: GObject,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GNetworkService {
    pub parent_instance: GObject,
    pub priv_0: *mut GNetworkServicePrivate,
}
pub type GNetworkServicePrivate = _GNetworkServicePrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GNetworkServicePrivate {
    pub service: *mut gchar,
    pub protocol: *mut gchar,
    pub domain: *mut gchar,
    pub scheme: *mut gchar,
    pub targets: *mut GList,
}
pub type GNetworkService = _GNetworkService;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GResolver {
    pub parent_instance: GObject,
    pub priv_0: *mut GResolverPrivate,
}
pub type GResolverPrivate = _GResolverPrivate;
pub type GResolver = _GResolver;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketAddressEnumerator {
    pub parent_instance: GObject,
}
pub type GSocketAddressEnumerator = _GSocketAddressEnumerator;
pub type GSocketConnectable = _GSocketConnectable;
pub type GSrvTarget = _GSrvTarget;
pub type GTask = _GTask;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GNetworkServiceClass {
    pub parent_class: GObjectClass,
}
pub type GNetworkServiceClass = _GNetworkServiceClass;
pub const PROP_DOMAIN: C2RustUnnamed_2 = 3;
pub const PROP_PROTOCOL: C2RustUnnamed_2 = 2;
pub const PROP_SERVICE: C2RustUnnamed_2 = 1;
pub const PROP_SCHEME: C2RustUnnamed_2 = 4;
pub type GSocketConnectableIface = _GSocketConnectableIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketConnectableIface {
    pub g_iface: GTypeInterface,
    pub enumerate:
        Option<unsafe extern "C" fn(*mut GSocketConnectable) -> *mut GSocketAddressEnumerator>,
    pub proxy_enumerate:
        Option<unsafe extern "C" fn(*mut GSocketConnectable) -> *mut GSocketAddressEnumerator>,
    pub to_string: Option<unsafe extern "C" fn(*mut GSocketConnectable) -> *mut gchar>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GNetworkServiceAddressEnumerator {
    pub parent_instance: GSocketAddressEnumerator,
    pub resolver: *mut GResolver,
    pub srv: *mut GNetworkService,
    pub addr_enum: *mut GSocketAddressEnumerator,
    pub t: *mut GList,
    pub use_proxy: gboolean,
    pub error: *mut GError,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GNetworkServiceAddressEnumeratorClass {
    pub parent_class: GSocketAddressEnumeratorClass,
}
pub type GSocketAddressEnumeratorClass = _GSocketAddressEnumeratorClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketAddressEnumeratorClass {
    pub parent_class: GObjectClass,
    pub next: Option<
        unsafe extern "C" fn(
            *mut GSocketAddressEnumerator,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GSocketAddress,
    >,
    pub next_async: Option<
        unsafe extern "C" fn(
            *mut GSocketAddressEnumerator,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub next_finish: Option<
        unsafe extern "C" fn(
            *mut GSocketAddressEnumerator,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> *mut GSocketAddress,
    >,
}
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_2 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_network_service_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_network_service_get_type_once();
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
static mut safe_c2rust_g_network_service_parent_class: gpointer = NULL_0;
unsafe extern "C" fn safe_c2rust_g_network_service_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_network_service_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GNetworkService_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GNetworkService_private_offset,
        );
    }
    safe_c2rust_g_network_service_class_init(klass as *mut GNetworkServiceClass);
}
static mut safe_c2rust_GNetworkService_private_offset: gint = 0;
#[inline]
unsafe extern "C" fn safe_c2rust_g_network_service_get_instance_private(
    mut self_0: *mut GNetworkService,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GNetworkService_private_offset as glong as isize)
        as gpointer;
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_network_service_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GNetworkService\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GNetworkServiceClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_network_service_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GNetworkService>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GNetworkService) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_network_service_init
                    as unsafe extern "C" fn(*mut GNetworkService) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GNetworkService_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GNetworkServicePrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GSocketConnectableIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_network_service_connectable_iface_init
                as unsafe extern "C" fn(*mut GSocketConnectableIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_socket_connectable_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_network_service_finalize(mut object: *mut GObject) {
    let mut srv: *mut GNetworkService = object as *mut ::core::ffi::c_void as *mut GNetworkService;
    g_free((*(*srv).priv_0).service as gpointer);
    g_free((*(*srv).priv_0).protocol as gpointer);
    g_free((*(*srv).priv_0).domain as gpointer);
    g_free((*(*srv).priv_0).scheme as gpointer);
    if !(*(*srv).priv_0).targets.is_null() {
        g_resolver_free_targets((*(*srv).priv_0).targets);
    }
    (*(safe_c2rust_g_network_service_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_network_service_class_init(
    mut klass: *mut GNetworkServiceClass,
) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_network_service_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_network_service_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).finalize =
        Some(safe_c2rust_g_network_service_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    g_object_class_install_property(
        gobject_class,
        PROP_SERVICE as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"service\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_PROTOCOL as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"protocol\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_DOMAIN as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"domain\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_DOMAIN as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"scheme\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
unsafe extern "C" fn safe_c2rust_g_network_service_connectable_iface_init(
    mut connectable_iface: *mut GSocketConnectableIface,
) {
    (*connectable_iface).enumerate = Some(
        safe_c2rust_g_network_service_connectable_enumerate
            as unsafe extern "C" fn(*mut GSocketConnectable) -> *mut GSocketAddressEnumerator,
    )
        as Option<unsafe extern "C" fn(*mut GSocketConnectable) -> *mut GSocketAddressEnumerator>;
    (*connectable_iface).proxy_enumerate = Some(
        safe_c2rust_g_network_service_connectable_proxy_enumerate
            as unsafe extern "C" fn(*mut GSocketConnectable) -> *mut GSocketAddressEnumerator,
    )
        as Option<unsafe extern "C" fn(*mut GSocketConnectable) -> *mut GSocketAddressEnumerator>;
    (*connectable_iface).to_string = Some(
        safe_c2rust_g_network_service_connectable_to_string
            as unsafe extern "C" fn(*mut GSocketConnectable) -> *mut gchar,
    )
        as Option<unsafe extern "C" fn(*mut GSocketConnectable) -> *mut gchar>;
}
unsafe extern "C" fn safe_c2rust_g_network_service_init(mut srv: *mut GNetworkService) {
    (*srv).priv_0 =
        safe_c2rust_g_network_service_get_instance_private(srv) as *mut GNetworkServicePrivate;
}
unsafe extern "C" fn safe_c2rust_g_network_service_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut srv: *mut GNetworkService = object as *mut ::core::ffi::c_void as *mut GNetworkService;
    match prop_id {
        1 => {
            (*(*srv).priv_0).service = g_value_dup_string(value);
        }
        2 => {
            (*(*srv).priv_0).protocol = g_value_dup_string(value);
        }
        3 => {
            (*(*srv).priv_0).domain = g_value_dup_string(value);
        }
        4 => {
            safe_c2rust_g_network_service_set_scheme(srv, g_value_get_string(value));
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gnetworkservice.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                215 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_network_service_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut srv: *mut GNetworkService = object as *mut ::core::ffi::c_void as *mut GNetworkService;
    match prop_id {
        1 => {
            g_value_set_string(value, safe_c2rust_g_network_service_get_service(srv));
        }
        2 => {
            g_value_set_string(value, safe_c2rust_g_network_service_get_protocol(srv));
        }
        3 => {
            g_value_set_string(value, safe_c2rust_g_network_service_get_domain(srv));
        }
        4 => {
            g_value_set_string(value, safe_c2rust_g_network_service_get_scheme(srv));
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gnetworkservice.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                247 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_network_service_new(
    mut service: *const gchar,
    mut protocol: *const gchar,
    mut domain: *const gchar,
) -> *mut GSocketConnectable {
    return g_object_new(
        safe_c2rust_g_network_service_get_type(),
        b"service\0" as *const u8 as *const gchar,
        service,
        b"protocol\0" as *const u8 as *const ::core::ffi::c_char,
        protocol,
        b"domain\0" as *const u8 as *const ::core::ffi::c_char,
        domain,
        NULL_0,
    ) as *mut GSocketConnectable;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_network_service_get_service(
    mut srv: *mut GNetworkService,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = srv as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_network_service_get_type();
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
            b"G_IS_NETWORK_SERVICE (srv)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*(*srv).priv_0).service;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_network_service_get_protocol(
    mut srv: *mut GNetworkService,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = srv as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_network_service_get_type();
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
            b"G_IS_NETWORK_SERVICE (srv)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*(*srv).priv_0).protocol;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_network_service_get_domain(
    mut srv: *mut GNetworkService,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = srv as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_network_service_get_type();
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
            b"G_IS_NETWORK_SERVICE (srv)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*(*srv).priv_0).domain;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_network_service_get_scheme(
    mut srv: *mut GNetworkService,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = srv as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_network_service_get_type();
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
            b"G_IS_NETWORK_SERVICE (srv)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    if !(*(*srv).priv_0).scheme.is_null() {
        return (*(*srv).priv_0).scheme;
    } else {
        return (*(*srv).priv_0).service;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_network_service_set_scheme(
    mut srv: *mut GNetworkService,
    mut scheme: *const gchar,
) {
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = srv as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_network_service_get_type();
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
            b"G_IS_NETWORK_SERVICE (srv)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_free((*(*srv).priv_0).scheme as gpointer);
    (*(*srv).priv_0).scheme =
        safe_c2rust_g_strdup_inline(scheme as *const ::core::ffi::c_char) as *mut gchar;
    g_object_notify(
        srv as *mut ::core::ffi::c_void as *mut GObject,
        b"scheme\0" as *const u8 as *const gchar,
    );
}
unsafe extern "C" fn safe_c2rust_g_network_service_fallback_targets(
    mut srv: *mut GNetworkService,
) -> *mut GList {
    let mut target: *mut GSrvTarget = ::core::ptr::null_mut::<GSrvTarget>();
    let mut has_port: gboolean = 0;
    let mut port: guint16 = 0;
    has_port = g_getservbyname_ntohs(
        (*(*srv).priv_0).service,
        b"tcp\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut port,
    );
    endservent();
    if has_port == 0 {
        return ::core::ptr::null_mut::<GList>();
    }
    target = g_srv_target_new((*(*srv).priv_0).domain, port, 0 as guint16, 0 as guint16);
    return g_list_append(::core::ptr::null_mut::<GList>(), target as gpointer);
}
static mut safe_c2rust__g_network_service_address_enumerator_parent_class: gpointer = NULL_0;
static mut safe_c2rust_GNetworkServiceAddressEnumerator_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust__g_network_service_address_enumerator_class_intern_init(
    mut klass: gpointer,
) {
    safe_c2rust__g_network_service_address_enumerator_parent_class =
        g_type_class_peek_parent(klass);
    if safe_c2rust_GNetworkServiceAddressEnumerator_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GNetworkServiceAddressEnumerator_private_offset,
        );
    }
    safe_c2rust__g_network_service_address_enumerator_class_init(
        klass as *mut GNetworkServiceAddressEnumeratorClass,
    );
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust__g_network_service_address_enumerator_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_socket_address_enumerator_get_type(),
        g_intern_static_string(b"GNetworkServiceAddressEnumerator\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GNetworkServiceAddressEnumeratorClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust__g_network_service_address_enumerator_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GNetworkServiceAddressEnumerator>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GNetworkServiceAddressEnumerator) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust__g_network_service_address_enumerator_init
                    as unsafe extern "C" fn(*mut GNetworkServiceAddressEnumerator) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust__g_network_service_address_enumerator_get_type() -> GType {
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
        let mut g_define_type_id: GType =
            safe_c2rust__g_network_service_address_enumerator_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_network_service_address_enumerator_next(
    mut enumerator: *mut GSocketAddressEnumerator,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GSocketAddress {
    let mut srv_enum: *mut GNetworkServiceAddressEnumerator =
        enumerator as *mut ::core::ffi::c_void as *mut GNetworkServiceAddressEnumerator;
    let mut ret: *mut GSocketAddress = ::core::ptr::null_mut::<GSocketAddress>();
    if (*(*(*srv_enum).srv).priv_0).targets.is_null() {
        let mut targets: *mut GList = ::core::ptr::null_mut::<GList>();
        let mut my_error: *mut GError = ::core::ptr::null_mut::<GError>();
        targets = g_resolver_lookup_service(
            (*srv_enum).resolver,
            (*(*(*srv_enum).srv).priv_0).service,
            (*(*(*srv_enum).srv).priv_0).protocol,
            (*(*(*srv_enum).srv).priv_0).domain,
            cancellable,
            &raw mut my_error,
        );
        if targets.is_null()
            && g_error_matches(
                my_error,
                g_resolver_error_quark(),
                G_RESOLVER_ERROR_NOT_FOUND as ::core::ffi::c_int as gint,
            ) != 0
        {
            targets = safe_c2rust_g_network_service_fallback_targets((*srv_enum).srv);
            if !targets.is_null() {
                g_clear_error(&raw mut my_error);
            }
        }
        if !my_error.is_null() {
            g_propagate_error(error, my_error);
            return ::core::ptr::null_mut::<GSocketAddress>();
        }
        (*(*(*srv_enum).srv).priv_0).targets = targets;
        (*srv_enum).t = (*(*(*srv_enum).srv).priv_0).targets;
    }
    let mut current_block_40: u64;
    loop {
        if (*srv_enum).addr_enum.is_null() && !(*srv_enum).t.is_null() {
            let mut my_error_0: *mut GError = ::core::ptr::null_mut::<GError>();
            let mut uri: *mut gchar = ::core::ptr::null_mut::<gchar>();
            let mut hostname: *mut gchar = ::core::ptr::null_mut::<gchar>();
            let mut addr: *mut GSocketConnectable = ::core::ptr::null_mut::<GSocketConnectable>();
            let mut target: *mut GSrvTarget = (*(*srv_enum).t).data as *mut GSrvTarget;
            (*srv_enum).t = if !(*srv_enum).t.is_null() {
                (*(*srv_enum).t).next
            } else {
                ::core::ptr::null_mut::<GList>()
            };
            hostname = g_hostname_to_ascii(g_srv_target_get_hostname(target));
            if hostname.is_null() {
                if (*srv_enum).error.is_null() {
                    (*srv_enum).error = g_error_new(
                        g_io_error_quark(),
                        G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                        b"Received invalid hostname '%s' from GSrvTarget\0" as *const u8
                            as *const gchar,
                        g_srv_target_get_hostname(target),
                    );
                }
                current_block_40 = 7651349459974463963;
            } else {
                uri = g_uri_join(
                    G_URI_FLAGS_NONE,
                    safe_c2rust_g_network_service_get_scheme((*srv_enum).srv),
                    ::core::ptr::null::<gchar>(),
                    hostname,
                    g_srv_target_get_port(target) as gint,
                    b"\0" as *const u8 as *const gchar,
                    ::core::ptr::null::<gchar>(),
                    ::core::ptr::null::<gchar>(),
                );
                g_free(hostname as gpointer);
                addr = g_network_address_parse_uri(
                    uri,
                    g_srv_target_get_port(target),
                    &raw mut my_error_0,
                );
                g_free(uri as gpointer);
                if addr.is_null() {
                    if (*srv_enum).error.is_null() {
                        (*srv_enum).error = my_error_0;
                    } else {
                        g_error_free(my_error_0);
                    }
                    current_block_40 = 7651349459974463963;
                } else {
                    if (*srv_enum).use_proxy != 0 {
                        (*srv_enum).addr_enum = g_socket_connectable_proxy_enumerate(addr);
                    } else {
                        (*srv_enum).addr_enum = g_socket_connectable_enumerate(addr);
                    }
                    g_object_unref(addr as gpointer);
                    current_block_40 = 4775909272756257391;
                }
            }
        } else {
            current_block_40 = 4775909272756257391;
        }
        match current_block_40 {
            4775909272756257391 => {
                if !(*srv_enum).addr_enum.is_null() {
                    let mut my_error_1: *mut GError = ::core::ptr::null_mut::<GError>();
                    ret = g_socket_address_enumerator_next(
                        (*srv_enum).addr_enum,
                        cancellable,
                        &raw mut my_error_1,
                    );
                    if !my_error_1.is_null() {
                        if (*srv_enum).error.is_null() {
                            (*srv_enum).error = my_error_1;
                        } else {
                            g_error_free(my_error_1);
                        }
                    }
                    if ret.is_null() {
                        g_object_unref((*srv_enum).addr_enum as gpointer);
                        (*srv_enum).addr_enum = ::core::ptr::null_mut::<GSocketAddressEnumerator>();
                    }
                }
            }
            _ => {}
        }
        if !((*srv_enum).addr_enum.is_null() && !(*srv_enum).t.is_null()) {
            break;
        }
    }
    if ret.is_null() && !(*srv_enum).error.is_null() {
        g_propagate_error(error, (*srv_enum).error);
        (*srv_enum).error = ::core::ptr::null_mut::<GError>();
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust_g_network_service_address_enumerator_next_async(
    mut enumerator: *mut GSocketAddressEnumerator,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut srv_enum: *mut GNetworkServiceAddressEnumerator =
        enumerator as *mut ::core::ffi::c_void as *mut GNetworkServiceAddressEnumerator;
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    task = g_task_new(enumerator as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GSocketAddressEnumerator,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_network_service_address_enumerator_next_async
                as unsafe extern "C" fn(
                    *mut GSocketAddressEnumerator,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"g_network_service_address_enumerator_next_async\0" as *const u8 as *const gchar,
        );
    }
    if (*(*(*srv_enum).srv).priv_0).targets.is_null() {
        g_resolver_lookup_service_async(
            (*srv_enum).resolver,
            (*(*(*srv_enum).srv).priv_0).service,
            (*(*(*srv_enum).srv).priv_0).protocol,
            (*(*(*srv_enum).srv).priv_0).domain,
            cancellable,
            Some(
                safe_c2rust_next_async_resolved_targets
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            task as gpointer,
        );
    } else {
        safe_c2rust_next_async_have_targets(task);
    };
}
unsafe extern "C" fn safe_c2rust_next_async_resolved_targets(
    mut source_object: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut srv_enum: *mut GNetworkServiceAddressEnumerator =
        g_task_get_source_object(task) as *mut GNetworkServiceAddressEnumerator;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut targets: *mut GList = ::core::ptr::null_mut::<GList>();
    targets = g_resolver_lookup_service_finish((*srv_enum).resolver, result, &raw mut error);
    if targets.is_null()
        && g_error_matches(
            error,
            g_resolver_error_quark(),
            G_RESOLVER_ERROR_NOT_FOUND as ::core::ffi::c_int as gint,
        ) != 0
    {
        targets = safe_c2rust_g_network_service_fallback_targets((*srv_enum).srv);
        if !targets.is_null() {
            g_clear_error(&raw mut error);
        }
    }
    if !error.is_null() {
        g_task_return_error(task, error);
        g_object_unref(task as gpointer);
    } else {
        (*(*(*srv_enum).srv).priv_0).targets = targets;
        (*srv_enum).t = (*(*(*srv_enum).srv).priv_0).targets;
        safe_c2rust_next_async_have_targets(task);
    };
}
unsafe extern "C" fn safe_c2rust_next_async_have_targets(mut task: *mut GTask) {
    let mut srv_enum: *mut GNetworkServiceAddressEnumerator =
        g_task_get_source_object(task) as *mut GNetworkServiceAddressEnumerator;
    if (*srv_enum).addr_enum.is_null() && !(*srv_enum).t.is_null() {
        let mut addr: *mut GSocketConnectable = ::core::ptr::null_mut::<GSocketConnectable>();
        let mut target: *mut GSrvTarget = (*(*srv_enum).t).data as *mut GSrvTarget;
        (*srv_enum).t = if !(*srv_enum).t.is_null() {
            (*(*srv_enum).t).next
        } else {
            ::core::ptr::null_mut::<GList>()
        };
        addr = g_network_address_new(
            g_srv_target_get_hostname(target),
            g_srv_target_get_port(target),
        );
        if (*srv_enum).use_proxy != 0 {
            (*srv_enum).addr_enum = g_socket_connectable_proxy_enumerate(addr);
        } else {
            (*srv_enum).addr_enum = g_socket_connectable_enumerate(addr);
        }
        g_object_unref(addr as gpointer);
    }
    if !(*srv_enum).addr_enum.is_null() {
        g_socket_address_enumerator_next_async(
            (*srv_enum).addr_enum,
            g_task_get_cancellable(task),
            Some(
                safe_c2rust_next_async_have_address
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            task as gpointer,
        );
    } else {
        if !(*srv_enum).error.is_null() {
            g_task_return_error(task, (*srv_enum).error);
            (*srv_enum).error = ::core::ptr::null_mut::<GError>();
        } else {
            g_task_return_pointer(task, NULL_0, None);
        }
        g_object_unref(task as gpointer);
    };
}
unsafe extern "C" fn safe_c2rust_next_async_have_address(
    mut source_object: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut srv_enum: *mut GNetworkServiceAddressEnumerator =
        g_task_get_source_object(task) as *mut GNetworkServiceAddressEnumerator;
    let mut address: *mut GSocketAddress = ::core::ptr::null_mut::<GSocketAddress>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    address =
        g_socket_address_enumerator_next_finish((*srv_enum).addr_enum, result, &raw mut error);
    if !error.is_null() {
        if (*srv_enum).error.is_null() {
            (*srv_enum).error = error;
        } else {
            g_error_free(error);
        }
    }
    if address.is_null() {
        g_object_unref((*srv_enum).addr_enum as gpointer);
        (*srv_enum).addr_enum = ::core::ptr::null_mut::<GSocketAddressEnumerator>();
        safe_c2rust_next_async_have_targets(task);
    } else {
        g_task_return_pointer(
            task,
            address as gpointer,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
        g_object_unref(task as gpointer);
    };
}
unsafe extern "C" fn safe_c2rust_g_network_service_address_enumerator_next_finish(
    mut enumerator: *mut GSocketAddressEnumerator,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GSocketAddress {
    return g_task_propagate_pointer(result as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GSocketAddress;
}
unsafe extern "C" fn safe_c2rust__g_network_service_address_enumerator_init(
    mut enumerator: *mut GNetworkServiceAddressEnumerator,
) {
}
unsafe extern "C" fn safe_c2rust_g_network_service_address_enumerator_finalize(
    mut object: *mut GObject,
) {
    let mut srv_enum: *mut GNetworkServiceAddressEnumerator =
        object as *mut ::core::ffi::c_void as *mut GNetworkServiceAddressEnumerator;
    if !(*srv_enum).srv.is_null() {
        g_object_unref((*srv_enum).srv as gpointer);
    }
    if !(*srv_enum).addr_enum.is_null() {
        g_object_unref((*srv_enum).addr_enum as gpointer);
    }
    if !(*srv_enum).resolver.is_null() {
        g_object_unref((*srv_enum).resolver as gpointer);
    }
    if !(*srv_enum).error.is_null() {
        g_error_free((*srv_enum).error);
    }
    (*(safe_c2rust__g_network_service_address_enumerator_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust__g_network_service_address_enumerator_class_init(
    mut srvenum_class: *mut GNetworkServiceAddressEnumeratorClass,
) {
    let mut object_class: *mut GObjectClass =
        srvenum_class as *mut ::core::ffi::c_void as *mut GObjectClass;
    let mut enumerator_class: *mut GSocketAddressEnumeratorClass =
        srvenum_class as *mut ::core::ffi::c_void as *mut GSocketAddressEnumeratorClass;
    (*enumerator_class).next = Some(
        safe_c2rust_g_network_service_address_enumerator_next
            as unsafe extern "C" fn(
                *mut GSocketAddressEnumerator,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GSocketAddress,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GSocketAddressEnumerator,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GSocketAddress,
        >;
    (*enumerator_class).next_async = Some(
        safe_c2rust_g_network_service_address_enumerator_next_async
            as unsafe extern "C" fn(
                *mut GSocketAddressEnumerator,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GSocketAddressEnumerator,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*enumerator_class).next_finish = Some(
        safe_c2rust_g_network_service_address_enumerator_next_finish
            as unsafe extern "C" fn(
                *mut GSocketAddressEnumerator,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GSocketAddress,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GSocketAddressEnumerator,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GSocketAddress,
        >;
    (*object_class).finalize = Some(
        safe_c2rust_g_network_service_address_enumerator_finalize
            as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
}
unsafe extern "C" fn safe_c2rust_g_network_service_connectable_enumerate(
    mut connectable: *mut GSocketConnectable,
) -> *mut GSocketAddressEnumerator {
    let mut srv_enum: *mut GNetworkServiceAddressEnumerator =
        ::core::ptr::null_mut::<GNetworkServiceAddressEnumerator>();
    srv_enum = g_object_new(
        safe_c2rust__g_network_service_address_enumerator_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GNetworkServiceAddressEnumerator;
    (*srv_enum).srv =
        g_object_ref(connectable as *mut ::core::ffi::c_void as *mut GNetworkService as gpointer)
            as *mut GNetworkService as *mut GNetworkService;
    (*srv_enum).resolver = g_resolver_get_default();
    (*srv_enum).use_proxy = FALSE as gboolean;
    return srv_enum as *mut ::core::ffi::c_void as *mut GSocketAddressEnumerator;
}
unsafe extern "C" fn safe_c2rust_g_network_service_connectable_proxy_enumerate(
    mut connectable: *mut GSocketConnectable,
) -> *mut GSocketAddressEnumerator {
    let mut addr_enum: *mut GSocketAddressEnumerator =
        ::core::ptr::null_mut::<GSocketAddressEnumerator>();
    let mut srv_enum: *mut GNetworkServiceAddressEnumerator =
        ::core::ptr::null_mut::<GNetworkServiceAddressEnumerator>();
    addr_enum = safe_c2rust_g_network_service_connectable_enumerate(connectable);
    srv_enum = addr_enum as *mut ::core::ffi::c_void as *mut GNetworkServiceAddressEnumerator;
    (*srv_enum).use_proxy = TRUE as gboolean;
    return addr_enum;
}
unsafe extern "C" fn safe_c2rust_g_network_service_connectable_to_string(
    mut connectable: *mut GSocketConnectable,
) -> *mut gchar {
    let mut service: *mut GNetworkService = ::core::ptr::null_mut::<GNetworkService>();
    service = connectable as *mut ::core::ffi::c_void as *mut GNetworkService;
    return g_strdup_printf(
        b"(%s, %s, %s, %s)\0" as *const u8 as *const gchar,
        (*(*service).priv_0).service,
        (*(*service).priv_0).protocol,
        (*(*service).priv_0).domain,
        (*(*service).priv_0).scheme,
    );
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
