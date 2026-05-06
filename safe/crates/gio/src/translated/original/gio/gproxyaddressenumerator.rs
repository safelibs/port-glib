extern "C" {
    pub type _GData;
    pub type _GDir;
    pub type _GMainContext;
    pub type _GAsyncResult;
    pub type _GCancellable;
    pub type _GInetAddressPrivate;
    pub type _GInetSocketAddressPrivate;
    pub type _GResolverPrivate;
    pub type _GSocketConnectable;
    pub type _GTask;
    pub type _GProxyResolver;
    pub type _GProxy;
    pub type _GWakeup;
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
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_error_matches(error: *const GError, domain: GQuark, code: gint) -> gboolean;
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_propagate_error(dest: *mut *mut GError, src: *mut GError);
    fn g_clear_error(err: *mut *mut GError);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strfreev(str_array: *mut *mut gchar);
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
    fn g_uri_split_with_user(
        uri_ref: *const gchar,
        flags: GUriFlags,
        scheme: *mut *mut gchar,
        user: *mut *mut gchar,
        password: *mut *mut gchar,
        auth_params: *mut *mut gchar,
        host: *mut *mut gchar,
        port: *mut gint,
        path: *mut *mut gchar,
        query: *mut *mut gchar,
        fragment: *mut *mut gchar,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_uri_parse_scheme(uri: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
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
    fn g_type_add_instance_private(class_type: GType, private_size: gsize) -> gint;
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_get(object: gpointer, first_property_name: *const gchar, ...);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_value_set_object(value: *mut GValue, v_object: gpointer);
    fn g_value_get_object(value: *const GValue) -> gpointer;
    fn g_value_dup_object(value: *const GValue) -> gpointer;
    fn g_param_spec_uint(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        minimum: guint,
        maximum: guint,
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
    fn g_param_spec_object(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        object_type: GType,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_value_set_uint(value: *mut GValue, v_uint: guint);
    fn g_value_get_uint(value: *const GValue) -> guint;
    fn g_value_set_string(value: *mut GValue, v_string: *const gchar);
    fn g_value_dup_string(value: *const GValue) -> *mut gchar;
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
    fn g_inet_address_to_string(address: *mut GInetAddress) -> *mut gchar;
    fn g_io_error_quark() -> GQuark;
    fn glib_gettext(str: *const gchar) -> *const gchar;
    fn glib__private__() -> *const GLibPrivateVTable;
    fn g_network_address_new(hostname: *const gchar, port: guint16) -> *mut GSocketConnectable;
    fn g_network_address_parse_uri(
        uri: *const gchar,
        default_port: guint16,
        error: *mut *mut GError,
    ) -> *mut GSocketConnectable;
    fn g_resolver_get_default() -> *mut GResolver;
    fn g_resolver_lookup_by_name(
        resolver: *mut GResolver,
        hostname: *const gchar,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GList;
    fn g_resolver_lookup_by_name_async(
        resolver: *mut GResolver,
        hostname: *const gchar,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_resolver_lookup_by_name_finish(
        resolver: *mut GResolver,
        result: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> *mut GList;
    fn g_resolver_free_addresses(addresses: *mut GList);
    fn g_proxy_get_default_for_protocol(protocol: *const gchar) -> *mut GProxy;
    fn g_proxy_supports_hostname(proxy: *mut GProxy) -> gboolean;
    fn g_inet_socket_address_get_type() -> GType;
    fn g_inet_socket_address_get_address(address: *mut GInetSocketAddress) -> *mut GInetAddress;
    fn g_inet_socket_address_get_port(address: *mut GInetSocketAddress) -> guint16;
    fn g_proxy_address_get_type() -> GType;
    fn g_proxy_resolver_get_type() -> GType;
    fn g_proxy_resolver_get_default() -> *mut GProxyResolver;
    fn g_proxy_resolver_lookup(
        resolver: *mut GProxyResolver,
        uri: *const gchar,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut *mut gchar;
    fn g_proxy_resolver_lookup_async(
        resolver: *mut GProxyResolver,
        uri: *const gchar,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_proxy_resolver_lookup_finish(
        resolver: *mut GProxyResolver,
        result: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> *mut *mut gchar;
    fn g_task_new(
        source_object: gpointer,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        callback_data: gpointer,
    ) -> *mut GTask;
    fn g_task_set_task_data(
        task: *mut GTask,
        task_data: gpointer,
        task_data_destroy: GDestroyNotify,
    );
    fn g_task_set_source_tag(task: *mut GTask, source_tag: gpointer);
    fn g_task_set_static_name(task: *mut GTask, name: *const gchar);
    fn g_task_get_task_data(task: *mut GTask) -> gpointer;
    fn g_task_get_cancellable(task: *mut GTask) -> *mut GCancellable;
    fn g_task_get_name(task: *mut GTask) -> *const gchar;
    fn g_task_is_valid(result: gpointer, source_object: gpointer) -> gboolean;
    fn g_task_return_pointer(task: *mut GTask, result: gpointer, result_destroy: GDestroyNotify);
    fn g_task_return_error(task: *mut GTask, error: *mut GError);
    fn g_task_return_new_error_literal(
        task: *mut GTask,
        domain: GQuark,
        code: gint,
        message: *const ::core::ffi::c_char,
    );
    fn g_task_propagate_pointer(task: *mut GTask, error: *mut *mut GError) -> gpointer;
    fn g_socket_connectable_get_type() -> GType;
    fn g_socket_connectable_enumerate(
        connectable: *mut GSocketConnectable,
    ) -> *mut GSocketAddressEnumerator;
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
pub type gushort = ::core::ffi::c_ushort;
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
pub type GDir = _GDir;
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
pub struct _GTypeInstance {
    pub g_class: *mut GTypeClass,
}
pub type GTypeInstance = _GTypeInstance;
pub type GInstanceInitFunc = Option<unsafe extern "C" fn(*mut GTypeInstance, gpointer) -> ()>;
pub type GClassInitFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
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
pub type GTask = _GTask;
pub type GProxyResolver = _GProxyResolver;
pub type GProxy = _GProxy;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GProxyAddressEnumerator {
    pub parent_instance: GSocketAddressEnumerator,
    pub priv_0: *mut GProxyAddressEnumeratorPrivate,
}
pub type GProxyAddressEnumeratorPrivate = _GProxyAddressEnumeratorPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GProxyAddressEnumeratorPrivate {
    pub connectable: *mut GSocketConnectable,
    pub dest_uri: *mut gchar,
    pub default_port: guint16,
    pub dest_hostname: *mut gchar,
    pub dest_port: guint16,
    pub dest_ips: *mut GList,
    pub proxy_resolver: *mut GProxyResolver,
    pub proxies: *mut *mut gchar,
    pub next_proxy: *mut *mut gchar,
    pub addr_enum: *mut GSocketAddressEnumerator,
    pub proxy_address: *mut GSocketAddress,
    pub proxy_uri: *const gchar,
    pub proxy_type: *mut gchar,
    pub proxy_username: *mut gchar,
    pub proxy_password: *mut gchar,
    pub supports_hostname: gboolean,
    pub next_dest_ip: *mut GList,
    pub last_error: *mut GError,
    pub ever_enumerated: gboolean,
}
pub type GProxyAddressEnumerator = _GProxyAddressEnumerator;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
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
pub type GSocketAddressEnumeratorClass = _GSocketAddressEnumeratorClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GProxyAddressEnumeratorClass {
    pub parent_class: GSocketAddressEnumeratorClass,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved6: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved7: Option<unsafe extern "C" fn() -> ()>,
}
pub type GProxyAddressEnumeratorClass = _GProxyAddressEnumeratorClass;
pub const PROP_PROXY_RESOLVER: C2RustUnnamed_1 = 4;
pub const PROP_CONNECTABLE: C2RustUnnamed_1 = 3;
pub const PROP_DEFAULT_PORT: C2RustUnnamed_1 = 2;
pub const PROP_URI: C2RustUnnamed_1 = 1;
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
pub type GDataListUpdateAtomicFunc =
    Option<unsafe extern "C" fn(GQuark, *mut gpointer, *mut GDestroyNotify, gpointer) -> gpointer>;
pub type GWin32InvalidParameterHandler = _GWin32InvalidParameterHandler;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GWin32InvalidParameterHandler {
    pub unused_really: ::core::ffi::c_int,
}
pub type GWakeup = _GWakeup;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_1 = 0;
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
unsafe extern "C" fn safe_c2rust_g_proxy_address_enumerator_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_proxy_address_enumerator_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GProxyAddressEnumerator_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GProxyAddressEnumerator_private_offset,
        );
    }
    safe_c2rust_g_proxy_address_enumerator_class_init(klass as *mut GProxyAddressEnumeratorClass);
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_proxy_address_enumerator_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_socket_address_enumerator_get_type(),
        g_intern_static_string(b"GProxyAddressEnumerator\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GProxyAddressEnumeratorClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_proxy_address_enumerator_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GProxyAddressEnumerator>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GProxyAddressEnumerator) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_proxy_address_enumerator_init
                    as unsafe extern "C" fn(*mut GProxyAddressEnumerator) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GProxyAddressEnumerator_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GProxyAddressEnumeratorPrivate>() as gsize,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_proxy_address_enumerator_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_proxy_address_enumerator_get_type_once();
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
static mut safe_c2rust_g_proxy_address_enumerator_parent_class: gpointer = NULL_0;
static mut safe_c2rust_GProxyAddressEnumerator_private_offset: gint = 0;
#[inline]
unsafe extern "C" fn safe_c2rust_g_proxy_address_enumerator_get_instance_private(
    mut self_0: *mut GProxyAddressEnumerator,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GProxyAddressEnumerator_private_offset as glong as isize)
        as gpointer;
}
unsafe extern "C" fn safe_c2rust_save_userinfo(
    mut priv_0: *mut GProxyAddressEnumeratorPrivate,
    mut proxy: *const gchar,
) {
    let mut _pp: *mut *mut gchar = &raw mut (*priv_0).proxy_username;
    let mut _ptr: *mut gchar = *_pp;
    *_pp = ::core::ptr::null_mut::<gchar>();
    if !_ptr.is_null() {
        g_free(_ptr as gpointer);
    }
    let mut _pp_0: *mut *mut gchar = &raw mut (*priv_0).proxy_password;
    let mut _ptr_0: *mut gchar = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<gchar>();
    if !_ptr_0.is_null() {
        g_free(_ptr_0 as gpointer);
    }
    g_uri_split_with_user(
        proxy,
        G_URI_FLAGS_HAS_PASSWORD,
        ::core::ptr::null_mut::<*mut gchar>(),
        &raw mut (*priv_0).proxy_username,
        &raw mut (*priv_0).proxy_password,
        ::core::ptr::null_mut::<*mut gchar>(),
        ::core::ptr::null_mut::<*mut gchar>(),
        ::core::ptr::null_mut::<gint>(),
        ::core::ptr::null_mut::<*mut gchar>(),
        ::core::ptr::null_mut::<*mut gchar>(),
        ::core::ptr::null_mut::<*mut gchar>(),
        ::core::ptr::null_mut::<*mut GError>(),
    );
}
unsafe extern "C" fn safe_c2rust_next_enumerator(mut priv_0: *mut GProxyAddressEnumeratorPrivate) {
    if !(*priv_0).proxy_address.is_null() {
        return;
    }
    while (*priv_0).addr_enum.is_null() && !(*(*priv_0).next_proxy).is_null() {
        let mut connectable: *mut GSocketConnectable =
            ::core::ptr::null_mut::<GSocketConnectable>();
        let mut proxy: *mut GProxy = ::core::ptr::null_mut::<GProxy>();
        let fresh0 = (*priv_0).next_proxy;
        (*priv_0).next_proxy = (*priv_0).next_proxy.offset(1);
        (*priv_0).proxy_uri = *fresh0;
        g_free((*priv_0).proxy_type as gpointer);
        (*priv_0).proxy_type =
            g_uri_parse_scheme((*priv_0).proxy_uri as *const ::core::ffi::c_char) as *mut gchar;
        if (*priv_0).proxy_type.is_null() {
            continue;
        }
        (*priv_0).supports_hostname = TRUE as gboolean;
        proxy = g_proxy_get_default_for_protocol((*priv_0).proxy_type);
        if !proxy.is_null() {
            (*priv_0).supports_hostname = g_proxy_supports_hostname(proxy);
            g_object_unref(proxy as gpointer);
        }
        if strcmp(
            b"direct\0" as *const u8 as *const ::core::ffi::c_char,
            (*priv_0).proxy_type,
        ) == 0 as ::core::ffi::c_int
        {
            if !(*priv_0).connectable.is_null() {
                connectable = g_object_ref((*priv_0).connectable as gpointer)
                    as *mut GSocketConnectable
                    as *mut GSocketConnectable;
            } else {
                connectable = g_network_address_new((*priv_0).dest_hostname, (*priv_0).dest_port);
            }
        } else {
            let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
            let mut default_port: ::core::ffi::c_int = 0;
            default_port =
                (*glib__private__())
                    .g_uri_get_default_scheme_port
                    .expect("non-null function pointer")((*priv_0).proxy_type);
            if default_port == -(1 as ::core::ffi::c_int) {
                default_port = 0 as ::core::ffi::c_int;
            }
            connectable = g_network_address_parse_uri(
                (*priv_0).proxy_uri,
                default_port as guint16,
                &raw mut error,
            );
            if !error.is_null() {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_WARNING,
                    b"Invalid proxy URI '%s': %s\0" as *const u8 as *const gchar,
                    (*priv_0).proxy_uri,
                    (*error).message,
                );
                g_error_free(error);
            }
            safe_c2rust_save_userinfo(priv_0, (*priv_0).proxy_uri);
        }
        if !connectable.is_null() {
            (*priv_0).addr_enum = g_socket_connectable_enumerate(connectable);
            g_object_unref(connectable as gpointer);
        }
    }
}
unsafe extern "C" fn safe_c2rust_g_proxy_address_enumerator_next(
    mut enumerator: *mut GSocketAddressEnumerator,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GSocketAddress {
    let mut priv_0: *mut GProxyAddressEnumeratorPrivate =
        (*(enumerator as *mut ::core::ffi::c_void as *mut GProxyAddressEnumerator)).priv_0;
    let mut result: *mut GSocketAddress = ::core::ptr::null_mut::<GSocketAddress>();
    let mut first_error: *mut GError = ::core::ptr::null_mut::<GError>();
    if (*priv_0).ever_enumerated == 0 {
        if ({
            let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
            if (*priv_0).proxies.is_null() {
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gproxyaddressenumerator.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                197 as ::core::ffi::c_int,
                G_STRFUNC,
                b"priv->proxies == NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        (*priv_0).proxies = g_proxy_resolver_lookup(
            (*priv_0).proxy_resolver,
            (*priv_0).dest_uri,
            cancellable,
            error,
        );
        (*priv_0).next_proxy = (*priv_0).proxies;
        if (*priv_0).proxies.is_null() {
            (*priv_0).ever_enumerated = TRUE as gboolean;
            return ::core::ptr::null_mut::<GSocketAddress>();
        }
    }
    while result.is_null() && (!(*(*priv_0).next_proxy).is_null() || !(*priv_0).addr_enum.is_null())
    {
        let mut dest_hostname: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut dest_protocol: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut inetsaddr: *mut GInetSocketAddress = ::core::ptr::null_mut::<GInetSocketAddress>();
        let mut inetaddr: *mut GInetAddress = ::core::ptr::null_mut::<GInetAddress>();
        let mut port: guint16 = 0;
        safe_c2rust_next_enumerator(priv_0);
        if (*priv_0).addr_enum.is_null() {
            continue;
        }
        if (*priv_0).proxy_address.is_null() {
            (*priv_0).proxy_address = g_socket_address_enumerator_next(
                (*priv_0).addr_enum,
                cancellable,
                if !first_error.is_null() {
                    ::core::ptr::null_mut::<*mut GError>()
                } else {
                    &raw mut first_error
                },
            );
        }
        if (*priv_0).proxy_address.is_null() {
            g_object_unref((*priv_0).addr_enum as gpointer);
            (*priv_0).addr_enum = ::core::ptr::null_mut::<GSocketAddressEnumerator>();
            if !(*priv_0).dest_ips.is_null() {
                g_resolver_free_addresses((*priv_0).dest_ips);
                (*priv_0).dest_ips = ::core::ptr::null_mut::<GList>();
            }
        } else if strcmp(
            b"direct\0" as *const u8 as *const ::core::ffi::c_char,
            (*priv_0).proxy_type,
        ) == 0 as ::core::ffi::c_int
        {
            result = (*priv_0).proxy_address;
            (*priv_0).proxy_address = ::core::ptr::null_mut::<GSocketAddress>();
        } else {
            if (*priv_0).supports_hostname == 0 {
                let mut dest_ip: *mut GInetAddress = ::core::ptr::null_mut::<GInetAddress>();
                if (*priv_0).dest_ips.is_null() {
                    let mut resolver: *mut GResolver = ::core::ptr::null_mut::<GResolver>();
                    resolver = g_resolver_get_default();
                    (*priv_0).dest_ips = g_resolver_lookup_by_name(
                        resolver,
                        (*priv_0).dest_hostname,
                        cancellable,
                        if !first_error.is_null() {
                            ::core::ptr::null_mut::<*mut GError>()
                        } else {
                            &raw mut first_error
                        },
                    );
                    g_object_unref(resolver as gpointer);
                    if (*priv_0).dest_ips.is_null() {
                        g_object_unref((*priv_0).proxy_address as gpointer);
                        (*priv_0).proxy_address = ::core::ptr::null_mut::<GSocketAddress>();
                        continue;
                    }
                }
                if (*priv_0).next_dest_ip.is_null() {
                    (*priv_0).next_dest_ip = (*priv_0).dest_ips;
                }
                dest_ip = (*(*priv_0).next_dest_ip).data as *mut GInetAddress;
                dest_hostname = g_inet_address_to_string(dest_ip);
                (*priv_0).next_dest_ip = if !(*priv_0).next_dest_ip.is_null() {
                    (*(*priv_0).next_dest_ip).next
                } else {
                    ::core::ptr::null_mut::<GList>()
                };
            } else {
                dest_hostname = safe_c2rust_g_strdup_inline((*priv_0).dest_hostname) as *mut gchar;
            }
            dest_protocol = g_uri_parse_scheme((*priv_0).dest_uri) as *mut gchar;
            if ({
                let mut __inst: *mut GTypeInstance = (*priv_0).proxy_address as *mut GTypeInstance;
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
                g_free(dest_hostname as gpointer);
                g_free(dest_protocol as gpointer);
            }
            if ({
                let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
                if ({
                    let mut __inst: *mut GTypeInstance =
                        (*priv_0).proxy_address as *mut GTypeInstance;
                    let mut __t: GType = g_inet_socket_address_get_type();
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
                    b"G_IS_INET_SOCKET_ADDRESS (priv->proxy_address)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return ::core::ptr::null_mut::<GSocketAddress>();
            }
            inetsaddr =
                (*priv_0).proxy_address as *mut ::core::ffi::c_void as *mut GInetSocketAddress;
            inetaddr = g_inet_socket_address_get_address(inetsaddr);
            port = g_inet_socket_address_get_port(inetsaddr);
            result = g_object_new(
                g_proxy_address_get_type(),
                b"address\0" as *const u8 as *const gchar,
                inetaddr,
                b"port\0" as *const u8 as *const ::core::ffi::c_char,
                port as ::core::ffi::c_int,
                b"protocol\0" as *const u8 as *const ::core::ffi::c_char,
                (*priv_0).proxy_type,
                b"destination-protocol\0" as *const u8 as *const ::core::ffi::c_char,
                dest_protocol,
                b"destination-hostname\0" as *const u8 as *const ::core::ffi::c_char,
                dest_hostname,
                b"destination-port\0" as *const u8 as *const ::core::ffi::c_char,
                (*priv_0).dest_port as ::core::ffi::c_int,
                b"username\0" as *const u8 as *const ::core::ffi::c_char,
                (*priv_0).proxy_username,
                b"password\0" as *const u8 as *const ::core::ffi::c_char,
                (*priv_0).proxy_password,
                b"uri\0" as *const u8 as *const ::core::ffi::c_char,
                (*priv_0).proxy_uri,
                NULL_0,
            ) as *mut GSocketAddress;
            g_free(dest_hostname as gpointer);
            g_free(dest_protocol as gpointer);
            if (*priv_0).supports_hostname != 0 || (*priv_0).next_dest_ip.is_null() {
                g_object_unref((*priv_0).proxy_address as gpointer);
                (*priv_0).proxy_address = ::core::ptr::null_mut::<GSocketAddress>();
            }
        }
    }
    if result.is_null()
        && !first_error.is_null()
        && ((*priv_0).ever_enumerated == 0
            || g_error_matches(
                first_error,
                g_io_error_quark(),
                G_IO_ERROR_CANCELLED as ::core::ffi::c_int as gint,
            ) != 0)
    {
        g_propagate_error(error, first_error);
    } else if !first_error.is_null() {
        g_error_free(first_error);
    }
    if result.is_null() && !error.is_null() && (*error).is_null() && (*priv_0).ever_enumerated == 0
    {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(b"Unspecified proxy lookup failure\0" as *const u8 as *const gchar),
        );
    }
    (*priv_0).ever_enumerated = TRUE as gboolean;
    return result;
}
unsafe extern "C" fn safe_c2rust_complete_async(mut task: *mut GTask) {
    let mut priv_0: *mut GProxyAddressEnumeratorPrivate =
        g_task_get_task_data(task) as *mut GProxyAddressEnumeratorPrivate;
    if !(*priv_0).last_error.is_null()
        && ((*priv_0).ever_enumerated == 0
            || g_error_matches(
                (*priv_0).last_error,
                g_io_error_quark(),
                G_IO_ERROR_CANCELLED as ::core::ffi::c_int as gint,
            ) != 0)
    {
        g_task_return_error(task, (*priv_0).last_error);
        (*priv_0).last_error = ::core::ptr::null_mut::<GError>();
    } else if (*priv_0).ever_enumerated == 0 {
        g_task_return_new_error_literal(
            task,
            g_io_error_quark(),
            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(b"Unspecified proxy lookup failure\0" as *const u8 as *const gchar)
                as *const ::core::ffi::c_char,
        );
    } else {
        g_task_return_pointer(task, NULL_0, None);
    }
    (*priv_0).ever_enumerated = TRUE as gboolean;
    g_clear_error(&raw mut (*priv_0).last_error);
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_return_result(mut task: *mut GTask) {
    let mut priv_0: *mut GProxyAddressEnumeratorPrivate =
        g_task_get_task_data(task) as *mut GProxyAddressEnumeratorPrivate;
    let mut result: *mut GSocketAddress = ::core::ptr::null_mut::<GSocketAddress>();
    let mut is_inet_socket_address: gboolean = 0;
    if strcmp(
        b"direct\0" as *const u8 as *const ::core::ffi::c_char,
        (*priv_0).proxy_type,
    ) == 0 as ::core::ffi::c_int
    {
        result = (*priv_0).proxy_address;
        (*priv_0).proxy_address = ::core::ptr::null_mut::<GSocketAddress>();
    } else {
        let mut dest_hostname: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut dest_protocol: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut inetsaddr: *mut GInetSocketAddress = ::core::ptr::null_mut::<GInetSocketAddress>();
        let mut inetaddr: *mut GInetAddress = ::core::ptr::null_mut::<GInetAddress>();
        let mut port: guint16 = 0;
        if (*priv_0).supports_hostname == 0 {
            let mut dest_ip: *mut GInetAddress = ::core::ptr::null_mut::<GInetAddress>();
            if (*priv_0).next_dest_ip.is_null() {
                (*priv_0).next_dest_ip = (*priv_0).dest_ips;
            }
            dest_ip = (*(*priv_0).next_dest_ip).data as *mut GInetAddress;
            dest_hostname = g_inet_address_to_string(dest_ip);
            (*priv_0).next_dest_ip = if !(*priv_0).next_dest_ip.is_null() {
                (*(*priv_0).next_dest_ip).next
            } else {
                ::core::ptr::null_mut::<GList>()
            };
        } else {
            dest_hostname = safe_c2rust_g_strdup_inline((*priv_0).dest_hostname) as *mut gchar;
        }
        dest_protocol = g_uri_parse_scheme((*priv_0).dest_uri) as *mut gchar;
        is_inet_socket_address = ({
            let mut __inst: *mut GTypeInstance = (*priv_0).proxy_address as *mut GTypeInstance;
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
        });
        if is_inet_socket_address == 0 {
            g_free(dest_hostname as gpointer);
            g_free(dest_protocol as gpointer);
        }
        if ({
            let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
            if is_inet_socket_address != 0 {
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
                b"is_inet_socket_address\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        inetsaddr = (*priv_0).proxy_address as *mut ::core::ffi::c_void as *mut GInetSocketAddress;
        inetaddr = g_inet_socket_address_get_address(inetsaddr);
        port = g_inet_socket_address_get_port(inetsaddr);
        result = g_object_new(
            g_proxy_address_get_type(),
            b"address\0" as *const u8 as *const gchar,
            inetaddr,
            b"port\0" as *const u8 as *const ::core::ffi::c_char,
            port as ::core::ffi::c_int,
            b"protocol\0" as *const u8 as *const ::core::ffi::c_char,
            (*priv_0).proxy_type,
            b"destination-protocol\0" as *const u8 as *const ::core::ffi::c_char,
            dest_protocol,
            b"destination-hostname\0" as *const u8 as *const ::core::ffi::c_char,
            dest_hostname,
            b"destination-port\0" as *const u8 as *const ::core::ffi::c_char,
            (*priv_0).dest_port as ::core::ffi::c_int,
            b"username\0" as *const u8 as *const ::core::ffi::c_char,
            (*priv_0).proxy_username,
            b"password\0" as *const u8 as *const ::core::ffi::c_char,
            (*priv_0).proxy_password,
            b"uri\0" as *const u8 as *const ::core::ffi::c_char,
            (*priv_0).proxy_uri,
            NULL_0,
        ) as *mut GSocketAddress;
        g_free(dest_hostname as gpointer);
        g_free(dest_protocol as gpointer);
        if (*priv_0).supports_hostname != 0 || (*priv_0).next_dest_ip.is_null() {
            g_object_unref((*priv_0).proxy_address as gpointer);
            (*priv_0).proxy_address = ::core::ptr::null_mut::<GSocketAddress>();
        }
    }
    (*priv_0).ever_enumerated = TRUE as gboolean;
    g_task_return_pointer(
        task,
        result as gpointer,
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_next_proxy(mut task: *mut GTask) {
    let mut priv_0: *mut GProxyAddressEnumeratorPrivate =
        g_task_get_task_data(task) as *mut GProxyAddressEnumeratorPrivate;
    if !(*(*priv_0).next_proxy).is_null() {
        g_object_unref((*priv_0).addr_enum as gpointer);
        (*priv_0).addr_enum = ::core::ptr::null_mut::<GSocketAddressEnumerator>();
        if !(*priv_0).dest_ips.is_null() {
            g_resolver_free_addresses((*priv_0).dest_ips);
            (*priv_0).dest_ips = ::core::ptr::null_mut::<GList>();
        }
        safe_c2rust_next_enumerator(priv_0);
        if !(*priv_0).addr_enum.is_null() {
            g_socket_address_enumerator_next_async(
                (*priv_0).addr_enum,
                g_task_get_cancellable(task),
                Some(
                    safe_c2rust_address_enumerate_cb
                        as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
                ),
                task as gpointer,
            );
            return;
        }
    }
    safe_c2rust_complete_async(task);
}
unsafe extern "C" fn safe_c2rust_dest_hostname_lookup_cb(
    mut object: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut priv_0: *mut GProxyAddressEnumeratorPrivate =
        g_task_get_task_data(task) as *mut GProxyAddressEnumeratorPrivate;
    g_clear_error(&raw mut (*priv_0).last_error);
    (*priv_0).dest_ips = g_resolver_lookup_by_name_finish(
        object as *mut ::core::ffi::c_void as *mut GResolver,
        result,
        &raw mut (*priv_0).last_error,
    );
    if !(*priv_0).dest_ips.is_null() {
        safe_c2rust_return_result(task);
    } else {
        let mut _pp: *mut *mut GSocketAddress = &raw mut (*priv_0).proxy_address;
        let mut _ptr: *mut GSocketAddress = *_pp;
        *_pp = ::core::ptr::null_mut::<GSocketAddress>();
        if !_ptr.is_null() {
            g_object_unref(_ptr as gpointer);
        }
        safe_c2rust_next_proxy(task);
    };
}
unsafe extern "C" fn safe_c2rust_address_enumerate_cb(
    mut object: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut priv_0: *mut GProxyAddressEnumeratorPrivate =
        g_task_get_task_data(task) as *mut GProxyAddressEnumeratorPrivate;
    g_clear_error(&raw mut (*priv_0).last_error);
    (*priv_0).proxy_address = g_socket_address_enumerator_next_finish(
        (*priv_0).addr_enum,
        result,
        &raw mut (*priv_0).last_error,
    );
    if !(*priv_0).proxy_address.is_null() {
        if (*priv_0).supports_hostname == 0 && (*priv_0).dest_ips.is_null() {
            let mut resolver: *mut GResolver = ::core::ptr::null_mut::<GResolver>();
            resolver = g_resolver_get_default();
            g_resolver_lookup_by_name_async(
                resolver,
                (*priv_0).dest_hostname,
                g_task_get_cancellable(task),
                Some(
                    safe_c2rust_dest_hostname_lookup_cb
                        as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
                ),
                task as gpointer,
            );
            g_object_unref(resolver as gpointer);
            return;
        }
        safe_c2rust_return_result(task);
    } else {
        safe_c2rust_next_proxy(task);
    };
}
unsafe extern "C" fn safe_c2rust_proxy_lookup_cb(
    mut object: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut priv_0: *mut GProxyAddressEnumeratorPrivate =
        g_task_get_task_data(task) as *mut GProxyAddressEnumeratorPrivate;
    g_clear_error(&raw mut (*priv_0).last_error);
    (*priv_0).proxies = g_proxy_resolver_lookup_finish(
        object as *mut ::core::ffi::c_void as *mut GProxyResolver,
        result,
        &raw mut (*priv_0).last_error,
    );
    (*priv_0).next_proxy = (*priv_0).proxies;
    if !(*priv_0).last_error.is_null() {
        safe_c2rust_complete_async(task);
        return;
    } else {
        safe_c2rust_next_enumerator(priv_0);
        if !(*priv_0).addr_enum.is_null() {
            g_socket_address_enumerator_next_async(
                (*priv_0).addr_enum,
                g_task_get_cancellable(task),
                Some(
                    safe_c2rust_address_enumerate_cb
                        as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
                ),
                task as gpointer,
            );
            return;
        }
    }
    safe_c2rust_complete_async(task);
}
unsafe extern "C" fn safe_c2rust_g_proxy_address_enumerator_next_async(
    mut enumerator: *mut GSocketAddressEnumerator,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut priv_0: *mut GProxyAddressEnumeratorPrivate =
        (*(enumerator as *mut ::core::ffi::c_void as *mut GProxyAddressEnumerator)).priv_0;
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
            safe_c2rust_g_proxy_address_enumerator_next_async
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
            b"g_proxy_address_enumerator_next_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(task, priv_0 as gpointer, None);
    if (*priv_0).proxies.is_null() {
        g_proxy_resolver_lookup_async(
            (*priv_0).proxy_resolver,
            (*priv_0).dest_uri,
            cancellable,
            Some(
                safe_c2rust_proxy_lookup_cb
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            task as gpointer,
        );
        return;
    }
    if !(*priv_0).addr_enum.is_null() {
        if !(*priv_0).proxy_address.is_null() {
            safe_c2rust_return_result(task);
            return;
        } else {
            g_socket_address_enumerator_next_async(
                (*priv_0).addr_enum,
                cancellable,
                Some(
                    safe_c2rust_address_enumerate_cb
                        as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
                ),
                task as gpointer,
            );
            return;
        }
    }
    safe_c2rust_complete_async(task);
}
unsafe extern "C" fn safe_c2rust_g_proxy_address_enumerator_next_finish(
    mut enumerator: *mut GSocketAddressEnumerator,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GSocketAddress {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, enumerator as gpointer) != 0 {
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
            b"g_task_is_valid (result, enumerator)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSocketAddress>();
    }
    return g_task_propagate_pointer(result as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GSocketAddress;
}
unsafe extern "C" fn safe_c2rust_g_proxy_address_enumerator_constructed(mut object: *mut GObject) {
    let mut priv_0: *mut GProxyAddressEnumeratorPrivate =
        (*(object as *mut ::core::ffi::c_void as *mut GProxyAddressEnumerator)).priv_0;
    let mut conn: *mut GSocketConnectable = ::core::ptr::null_mut::<GSocketConnectable>();
    let mut port: guint = 0;
    if !(*priv_0).dest_uri.is_null() {
        conn = g_network_address_parse_uri(
            (*priv_0).dest_uri,
            (*priv_0).default_port,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        if !conn.is_null() {
            g_object_get(
                conn as gpointer,
                b"hostname\0" as *const u8 as *const gchar,
                &raw mut (*priv_0).dest_hostname,
                b"port\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut port,
                NULL_0,
            );
            (*priv_0).dest_port = port as guint16;
            g_object_unref(conn as gpointer);
        } else {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Invalid URI '%s'\0" as *const u8 as *const gchar,
                (*priv_0).dest_uri,
            );
        }
    }
    (*(safe_c2rust_g_proxy_address_enumerator_parent_class as *mut GObjectClass))
        .constructed
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_proxy_address_enumerator_get_property(
    mut object: *mut GObject,
    mut property_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut priv_0: *mut GProxyAddressEnumeratorPrivate =
        (*(object as *mut ::core::ffi::c_void as *mut GProxyAddressEnumerator)).priv_0;
    match property_id {
        1 => {
            g_value_set_string(value, (*priv_0).dest_uri);
        }
        2 => {
            g_value_set_uint(value, (*priv_0).default_port as guint);
        }
        3 => {
            g_value_set_object(value, (*priv_0).connectable as gpointer);
        }
        4 => {
            g_value_set_object(value, (*priv_0).proxy_resolver as gpointer);
        }
        _ => {
            let mut _glib__object: *mut GObject = object;
            let mut _glib__pspec: *mut GParamSpec = pspec;
            let mut _glib__property_id: guint = property_id;
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8
                    as *const gchar,
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gproxyaddressenumerator.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                667 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_proxy_address_enumerator_set_property(
    mut object: *mut GObject,
    mut property_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut priv_0: *mut GProxyAddressEnumeratorPrivate =
        (*(object as *mut ::core::ffi::c_void as *mut GProxyAddressEnumerator)).priv_0;
    match property_id {
        1 => {
            (*priv_0).dest_uri = g_value_dup_string(value);
        }
        2 => {
            (*priv_0).default_port = g_value_get_uint(value) as guint16;
        }
        3 => {
            (*priv_0).connectable = g_value_dup_object(value) as *mut GSocketConnectable;
        }
        4 => {
            if !(*priv_0).proxy_resolver.is_null() {
                g_object_unref((*priv_0).proxy_resolver as gpointer);
            }
            (*priv_0).proxy_resolver = g_value_get_object(value) as *mut GProxyResolver;
            if (*priv_0).proxy_resolver.is_null() {
                (*priv_0).proxy_resolver = g_proxy_resolver_get_default();
            }
            g_object_ref((*priv_0).proxy_resolver as gpointer);
        }
        _ => {
            let mut _glib__object: *mut GObject = object;
            let mut _glib__pspec: *mut GParamSpec = pspec;
            let mut _glib__property_id: guint = property_id;
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8
                    as *const gchar,
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gproxyaddressenumerator.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                702 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_proxy_address_enumerator_finalize(mut object: *mut GObject) {
    let mut priv_0: *mut GProxyAddressEnumeratorPrivate =
        (*(object as *mut ::core::ffi::c_void as *mut GProxyAddressEnumerator)).priv_0;
    if !(*priv_0).connectable.is_null() {
        g_object_unref((*priv_0).connectable as gpointer);
    }
    if !(*priv_0).proxy_resolver.is_null() {
        g_object_unref((*priv_0).proxy_resolver as gpointer);
    }
    g_free((*priv_0).dest_uri as gpointer);
    g_free((*priv_0).dest_hostname as gpointer);
    if !(*priv_0).dest_ips.is_null() {
        g_resolver_free_addresses((*priv_0).dest_ips);
    }
    g_strfreev((*priv_0).proxies);
    if !(*priv_0).addr_enum.is_null() {
        g_object_unref((*priv_0).addr_enum as gpointer);
    }
    g_free((*priv_0).proxy_type as gpointer);
    g_free((*priv_0).proxy_username as gpointer);
    g_free((*priv_0).proxy_password as gpointer);
    g_clear_error(&raw mut (*priv_0).last_error);
    (*(safe_c2rust_g_proxy_address_enumerator_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_proxy_address_enumerator_init(
    mut self_0: *mut GProxyAddressEnumerator,
) {
    (*self_0).priv_0 = safe_c2rust_g_proxy_address_enumerator_get_instance_private(self_0)
        as *mut GProxyAddressEnumeratorPrivate;
}
unsafe extern "C" fn safe_c2rust_g_proxy_address_enumerator_class_init(
    mut proxy_enumerator_class: *mut GProxyAddressEnumeratorClass,
) {
    let mut object_class: *mut GObjectClass =
        proxy_enumerator_class as *mut ::core::ffi::c_void as *mut GObjectClass;
    let mut enumerator_class: *mut GSocketAddressEnumeratorClass =
        proxy_enumerator_class as *mut ::core::ffi::c_void as *mut GSocketAddressEnumeratorClass;
    (*object_class).constructed = Some(
        safe_c2rust_g_proxy_address_enumerator_constructed
            as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*object_class).set_property = Some(
        safe_c2rust_g_proxy_address_enumerator_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*object_class).get_property = Some(
        safe_c2rust_g_proxy_address_enumerator_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*object_class).finalize = Some(
        safe_c2rust_g_proxy_address_enumerator_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*enumerator_class).next = Some(
        safe_c2rust_g_proxy_address_enumerator_next
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
        safe_c2rust_g_proxy_address_enumerator_next_async
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
        safe_c2rust_g_proxy_address_enumerator_next_finish
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
    g_object_class_install_property(
        object_class,
        PROP_URI as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"uri\0" as *const u8 as *const gchar,
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
        object_class,
        PROP_DEFAULT_PORT as ::core::ffi::c_int as guint,
        g_param_spec_uint(
            b"default-port\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            0 as guint,
            65535 as guint,
            0 as guint,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_CONNECTABLE as ::core::ffi::c_int as guint,
        g_param_spec_object(
            b"connectable\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_socket_connectable_get_type(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_PROXY_RESOLVER as ::core::ffi::c_int as guint,
        g_param_spec_object(
            b"proxy-resolver\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_proxy_resolver_get_type(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
