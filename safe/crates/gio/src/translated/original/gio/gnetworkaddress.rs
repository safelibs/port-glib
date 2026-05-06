extern "C" {
    pub type _GData;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GAsyncResult;
    pub type _GCancellable;
    pub type _GInetAddressPrivate;
    pub type _GInetSocketAddressPrivate;
    pub type _GResolverPrivate;
    pub type _GSocketConnectable;
    pub type _GTask;
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
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_clear_error(err: *mut *mut GError);
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_list_free(list: *mut GList);
    fn g_list_free_full(list: *mut GList, free_func: GDestroyNotify);
    fn g_list_append(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_concat(list1: *mut GList, list2: *mut GList) -> *mut GList;
    fn g_list_delete_link(list: *mut GList, link_: *mut GList) -> *mut GList;
    fn g_list_reverse(list: *mut GList) -> *mut GList;
    fn g_list_copy_deep(list: *mut GList, func: GCopyFunc, user_data: gpointer) -> *mut GList;
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
    fn g_timeout_source_new(interval: guint) -> *mut GSource;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn g_string_new(init: *const gchar) -> *mut GString;
    fn g_string_free(string: *mut GString, free_segment: gboolean) -> *mut gchar;
    fn g_string_free_and_steal(string: *mut GString) -> *mut gchar;
    fn g_string_insert_len(
        string: *mut GString,
        pos: gssize,
        val: *const gchar,
        len: gssize,
    ) -> *mut GString;
    fn g_string_append_len(string: *mut GString, val: *const gchar, len: gssize) -> *mut GString;
    fn g_string_append_printf(string: *mut GString, format: *const gchar, ...);
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
    fn g_uri_split_network(
        uri_string: *const gchar,
        flags: GUriFlags,
        scheme: *mut *mut gchar,
        host: *mut *mut gchar,
        port: *mut gint,
        error: *mut *mut GError,
    ) -> gboolean;
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
    fn glib_gettext(str: *const gchar) -> *const gchar;
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
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
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
    fn g_value_set_uint(value: *mut GValue, v_uint: guint);
    fn g_value_get_uint(value: *const GValue) -> guint;
    fn g_value_set_string(value: *mut GValue, v_string: *const gchar);
    fn g_value_dup_string(value: *const GValue) -> *mut gchar;
    fn g_inet_address_new_loopback(family: GSocketFamily) -> *mut GInetAddress;
    fn g_inet_address_get_family(address: *mut GInetAddress) -> GSocketFamily;
    fn g_inet_socket_address_get_type() -> GType;
    fn g_inet_socket_address_new(address: *mut GInetAddress, port: guint16) -> *mut GSocketAddress;
    fn g_inet_socket_address_new_from_string(
        address: *const ::core::ffi::c_char,
        port: guint,
    ) -> *mut GSocketAddress;
    fn g_inet_socket_address_get_address(address: *mut GInetSocketAddress) -> *mut GInetAddress;
    fn endservent();
    fn g_resolver_get_default() -> *mut GResolver;
    fn g_resolver_lookup_by_name(
        resolver: *mut GResolver,
        hostname: *const gchar,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GList;
    fn g_resolver_lookup_by_name_with_flags_async(
        resolver: *mut GResolver,
        hostname: *const gchar,
        flags: GResolverNameLookupFlags,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_resolver_lookup_by_name_with_flags_finish(
        resolver: *mut GResolver,
        result: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> *mut GList;
    fn g_resolver_get_serial(resolver: *mut GResolver) -> guint64;
    fn g_getservbyname_ntohs(
        name: *const ::core::ffi::c_char,
        proto: *const ::core::ffi::c_char,
        out_port: *mut guint16,
    ) -> gboolean;
    fn g_socket_address_enumerator_get_type() -> GType;
    fn g_proxy_address_enumerator_get_type() -> GType;
    fn g_task_new(
        source_object: gpointer,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        callback_data: gpointer,
    ) -> *mut GTask;
    fn g_task_set_source_tag(task: *mut GTask, source_tag: gpointer);
    fn g_task_set_static_name(task: *mut GTask, name: *const gchar);
    fn g_task_get_name(task: *mut GTask) -> *const gchar;
    fn g_task_is_valid(result: gpointer, source_object: gpointer) -> gboolean;
    fn g_task_return_pointer(task: *mut GTask, result: gpointer, result_destroy: GDestroyNotify);
    fn g_task_return_error(task: *mut GTask, error: *mut GError);
    fn g_task_propagate_pointer(task: *mut GTask, error: *mut *mut GError) -> gpointer;
    fn g_io_error_quark() -> GQuark;
    fn g_socket_connectable_get_type() -> GType;
}
pub type size_t = usize;
pub type guint8 = ::core::ffi::c_uchar;
pub type guint16 = ::core::ffi::c_ushort;
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
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GCopyFunc = Option<unsafe extern "C" fn(gconstpointer, gpointer) -> gpointer>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub type GString = _GString;
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
pub type GSocketFamily = ::core::ffi::c_uint;
pub const G_SOCKET_FAMILY_IPV6: GSocketFamily = 10;
pub const G_SOCKET_FAMILY_IPV4: GSocketFamily = 2;
pub const G_SOCKET_FAMILY_UNIX: GSocketFamily = 1;
pub const G_SOCKET_FAMILY_INVALID: GSocketFamily = 0;
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
pub struct _GNetworkAddress {
    pub parent_instance: GObject,
    pub priv_0: *mut GNetworkAddressPrivate,
}
pub type GNetworkAddressPrivate = _GNetworkAddressPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GNetworkAddressPrivate {
    pub hostname: *mut gchar,
    pub port: guint16,
    pub cached_sockaddrs: *mut GList,
    pub scheme: *mut gchar,
    pub resolver_serial: gint64,
}
pub type GNetworkAddress = _GNetworkAddress;
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
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GNetworkAddressClass {
    pub parent_class: GObjectClass,
}
pub type GNetworkAddressClass = _GNetworkAddressClass;
pub const PROP_SCHEME: C2RustUnnamed_1 = 3;
pub const PROP_PORT: C2RustUnnamed_1 = 2;
pub const PROP_HOSTNAME: C2RustUnnamed_1 = 1;
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
pub struct GNetworkAddressAddressEnumerator {
    pub parent_instance: GSocketAddressEnumerator,
    pub addr: *mut GNetworkAddress,
    pub addresses: *mut GList,
    pub current_item: *mut GList,
    pub queued_task: *mut GTask,
    pub waiting_task: *mut GTask,
    pub last_error: *mut GError,
    pub wait_source: *mut GSource,
    pub context: *mut GMainContext,
    pub state: ResolveState,
}
pub type ResolveState = ::core::ffi::c_uint;
pub const RESOLVE_STATE_WAITING_ON_IPV6: ResolveState = 2;
pub const RESOLVE_STATE_WAITING_ON_IPV4: ResolveState = 1;
pub const RESOLVE_STATE_NONE: ResolveState = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GNetworkAddressAddressEnumeratorClass {
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
pub type GResolverNameLookupFlags = ::core::ffi::c_uint;
pub const G_RESOLVER_NAME_LOOKUP_FLAGS_IPV6_ONLY: GResolverNameLookupFlags = 2;
pub const G_RESOLVER_NAME_LOOKUP_FLAGS_IPV4_ONLY: GResolverNameLookupFlags = 1;
pub const G_RESOLVER_NAME_LOOKUP_FLAGS_DEFAULT: GResolverNameLookupFlags = 0;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_1 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_MAXUINT16: guint16 = 0xffff as ::core::ffi::c_int as guint16;
#[inline]
unsafe extern "C" fn safe_c2rust_g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    ref_0 = *ptr;
    *ptr = NULL as gpointer;
    return ref_0;
}
pub const G_SOURCE_REMOVE: ::core::ffi::c_int = FALSE;
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
#[inline(always)]
unsafe extern "C" fn safe_c2rust_g_string_append_len_inline(
    mut gstring: *mut GString,
    mut val: *const ::core::ffi::c_char,
    mut len: gssize,
) -> *mut GString {
    let mut len_unsigned: gsize = 0;
    if ({
        let mut _g_boolean_var_4: ::core::ffi::c_int = 0;
        if gstring.is_null() {
            _g_boolean_var_4 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_4 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_4
    }) as ::core::ffi::c_long
        != 0
    {
        return g_string_append_len(gstring, val as *const gchar, len);
    }
    if ({
        let mut _g_boolean_var_5: ::core::ffi::c_int = 0;
        if val.is_null() {
            _g_boolean_var_5 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_5 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_5
    }) as ::core::ffi::c_long
        != 0
    {
        return if len != 0 as gssize {
            g_string_append_len(gstring, val as *const gchar, len)
        } else {
            gstring
        };
    }
    if len < 0 as gssize {
        len_unsigned = strlen(val) as gsize;
    } else {
        len_unsigned = len as gsize;
    }
    if ({
        let mut _g_boolean_var_6: ::core::ffi::c_int = 0;
        if (*gstring).len.wrapping_add(len_unsigned) < (*gstring).allocated_len {
            _g_boolean_var_6 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_6 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_6
    }) as ::core::ffi::c_long
        != 0
    {
        let mut end: *mut ::core::ffi::c_char = (*gstring).str_0.offset((*gstring).len as isize);
        if ({
            let mut _g_boolean_var_7: ::core::ffi::c_int = 0;
            if val.offset(len_unsigned as isize) <= end as *const ::core::ffi::c_char
                || val > end.offset(len_unsigned as isize) as *const ::core::ffi::c_char
            {
                _g_boolean_var_7 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_7 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_7
        }) as ::core::ffi::c_long
            != 0
        {
            memcpy(
                end as *mut ::core::ffi::c_void,
                val as *const ::core::ffi::c_void,
                len_unsigned as size_t,
            );
        } else {
            memmove(
                end as *mut ::core::ffi::c_void,
                val as *const ::core::ffi::c_void,
                len_unsigned as size_t,
            );
        }
        (*gstring).len = (*gstring).len.wrapping_add(len_unsigned);
        *(*gstring).str_0.offset((*gstring).len as isize) = 0 as gchar;
        return gstring;
    } else {
        return g_string_insert_len(
            gstring,
            -(1 as ::core::ffi::c_int) as gssize,
            val as *const gchar,
            len,
        );
    };
}
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PF_INET6: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const AF_INET6: ::core::ffi::c_int = PF_INET6;
pub const HAPPY_EYEBALLS_RESOLUTION_DELAY_MS: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_network_address_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GNetworkAddress\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GNetworkAddressClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_network_address_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GNetworkAddress>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GNetworkAddress) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_network_address_init
                    as unsafe extern "C" fn(*mut GNetworkAddress) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GNetworkAddress_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GNetworkAddressPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GSocketConnectableIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_network_address_connectable_iface_init
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
#[inline]
unsafe extern "C" fn safe_c2rust_g_network_address_get_instance_private(
    mut self_0: *mut GNetworkAddress,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GNetworkAddress_private_offset as glong as isize)
        as gpointer;
}
static mut safe_c2rust_GNetworkAddress_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_network_address_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_network_address_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GNetworkAddress_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GNetworkAddress_private_offset,
        );
    }
    safe_c2rust_g_network_address_class_init(klass as *mut GNetworkAddressClass);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_network_address_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_network_address_get_type_once();
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
static mut safe_c2rust_g_network_address_parent_class: gpointer = NULL_0;
unsafe extern "C" fn safe_c2rust_g_network_address_finalize(mut object: *mut GObject) {
    let mut addr: *mut GNetworkAddress = object as *mut ::core::ffi::c_void as *mut GNetworkAddress;
    g_free((*(*addr).priv_0).hostname as gpointer);
    g_free((*(*addr).priv_0).scheme as gpointer);
    g_list_free_full(
        (*(*addr).priv_0).cached_sockaddrs,
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
    (*(safe_c2rust_g_network_address_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_network_address_class_init(
    mut klass: *mut GNetworkAddressClass,
) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_network_address_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_network_address_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).finalize =
        Some(safe_c2rust_g_network_address_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    g_object_class_install_property(
        gobject_class,
        PROP_HOSTNAME as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"hostname\0" as *const u8 as *const gchar,
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
        PROP_PORT as ::core::ffi::c_int as guint,
        g_param_spec_uint(
            b"port\0" as *const u8 as *const gchar,
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
        gobject_class,
        PROP_SCHEME as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"scheme\0" as *const u8 as *const gchar,
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
}
unsafe extern "C" fn safe_c2rust_g_network_address_connectable_iface_init(
    mut connectable_iface: *mut GSocketConnectableIface,
) {
    (*connectable_iface).enumerate = Some(
        safe_c2rust_g_network_address_connectable_enumerate
            as unsafe extern "C" fn(*mut GSocketConnectable) -> *mut GSocketAddressEnumerator,
    )
        as Option<unsafe extern "C" fn(*mut GSocketConnectable) -> *mut GSocketAddressEnumerator>;
    (*connectable_iface).proxy_enumerate = Some(
        safe_c2rust_g_network_address_connectable_proxy_enumerate
            as unsafe extern "C" fn(*mut GSocketConnectable) -> *mut GSocketAddressEnumerator,
    )
        as Option<unsafe extern "C" fn(*mut GSocketConnectable) -> *mut GSocketAddressEnumerator>;
    (*connectable_iface).to_string = Some(
        safe_c2rust_g_network_address_connectable_to_string
            as unsafe extern "C" fn(*mut GSocketConnectable) -> *mut gchar,
    )
        as Option<unsafe extern "C" fn(*mut GSocketConnectable) -> *mut gchar>;
}
unsafe extern "C" fn safe_c2rust_g_network_address_init(mut addr: *mut GNetworkAddress) {
    (*addr).priv_0 =
        safe_c2rust_g_network_address_get_instance_private(addr) as *mut GNetworkAddressPrivate;
}
unsafe extern "C" fn safe_c2rust_g_network_address_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut addr: *mut GNetworkAddress = object as *mut ::core::ffi::c_void as *mut GNetworkAddress;
    match prop_id {
        1 => {
            g_free((*(*addr).priv_0).hostname as gpointer);
            (*(*addr).priv_0).hostname = g_value_dup_string(value);
        }
        2 => {
            (*(*addr).priv_0).port = g_value_get_uint(value) as guint16;
        }
        3 => {
            g_free((*(*addr).priv_0).scheme as gpointer);
            (*(*addr).priv_0).scheme = g_value_dup_string(value);
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gnetworkaddress.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                201 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_network_address_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut addr: *mut GNetworkAddress = object as *mut ::core::ffi::c_void as *mut GNetworkAddress;
    match prop_id {
        1 => {
            g_value_set_string(value, (*(*addr).priv_0).hostname);
        }
        2 => {
            g_value_set_uint(value, (*(*addr).priv_0).port as guint);
        }
        3 => {
            g_value_set_string(value, (*(*addr).priv_0).scheme);
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gnetworkaddress.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                230 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_inet_addresses_to_inet_socket_addresses(
    mut addr: *mut GNetworkAddress,
    mut addresses: *mut GList,
) -> *mut GList {
    let mut a: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut socket_addresses: *mut GList = ::core::ptr::null_mut::<GList>();
    a = addresses;
    while !a.is_null() {
        let mut sockaddr: *mut GSocketAddress =
            g_inet_socket_address_new((*a).data as *mut GInetAddress, (*(*addr).priv_0).port);
        socket_addresses = g_list_append(
            socket_addresses,
            safe_c2rust_g_steal_pointer(&raw mut sockaddr as gpointer) as *mut GSocketAddress
                as gpointer,
        );
        g_object_unref((*a).data);
        a = (*a).next;
    }
    g_list_free(addresses);
    return socket_addresses;
}
unsafe extern "C" fn safe_c2rust_g_network_address_set_cached_addresses(
    mut addr: *mut GNetworkAddress,
    mut addresses: *mut GList,
    mut resolver_serial: guint64,
) {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !addresses.is_null() {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gnetworkaddress.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            272 as ::core::ffi::c_int,
            G_STRFUNC,
            b"addresses != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !(*(*addr).priv_0).cached_sockaddrs.is_null() {
        g_list_free_full(
            (*(*addr).priv_0).cached_sockaddrs,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    }
    if ({
        let mut __inst: *mut GTypeInstance = (*addresses).data as *mut GTypeInstance;
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
    }) != 0
    {
        (*(*addr).priv_0).cached_sockaddrs =
            safe_c2rust_g_steal_pointer(&raw mut addresses as gpointer) as *mut GList as *mut GList;
    } else {
        (*(*addr).priv_0).cached_sockaddrs = safe_c2rust_inet_addresses_to_inet_socket_addresses(
            addr,
            safe_c2rust_g_steal_pointer(&raw mut addresses as gpointer) as *mut GList,
        );
    }
    (*(*addr).priv_0).resolver_serial = resolver_serial as gint64;
}
unsafe extern "C" fn safe_c2rust_g_network_address_parse_sockaddr(
    mut addr: *mut GNetworkAddress,
) -> gboolean {
    let mut sockaddr: *mut GSocketAddress = ::core::ptr::null_mut::<GSocketAddress>();
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if (*(*addr).priv_0).cached_sockaddrs.is_null() {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gnetworkaddress.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            289 as ::core::ffi::c_int,
            G_STRFUNC,
            b"addr->priv->cached_sockaddrs == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    sockaddr = g_inet_socket_address_new_from_string(
        (*(*addr).priv_0).hostname,
        (*(*addr).priv_0).port as guint,
    );
    if !sockaddr.is_null() {
        (*(*addr).priv_0).cached_sockaddrs =
            g_list_append((*(*addr).priv_0).cached_sockaddrs, sockaddr as gpointer);
        return TRUE;
    } else {
        return FALSE;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_network_address_new(
    mut hostname: *const gchar,
    mut port: guint16,
) -> *mut GSocketConnectable {
    return g_object_new(
        safe_c2rust_g_network_address_get_type(),
        b"hostname\0" as *const u8 as *const gchar,
        hostname,
        b"port\0" as *const u8 as *const ::core::ffi::c_char,
        port as ::core::ffi::c_int,
        NULL_0,
    ) as *mut GSocketConnectable;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_network_address_new_loopback(
    mut port: guint16,
) -> *mut GSocketConnectable {
    let mut addr: *mut GNetworkAddress = ::core::ptr::null_mut::<GNetworkAddress>();
    let mut addrs: *mut GList = ::core::ptr::null_mut::<GList>();
    addr = g_object_new(
        safe_c2rust_g_network_address_get_type(),
        b"hostname\0" as *const u8 as *const gchar,
        b"localhost\0" as *const u8 as *const ::core::ffi::c_char,
        b"port\0" as *const u8 as *const ::core::ffi::c_char,
        port as ::core::ffi::c_int,
        NULL_0,
    ) as *mut GNetworkAddress;
    addrs = g_list_append(
        addrs,
        g_inet_address_new_loopback(G_SOCKET_FAMILY_IPV6) as gpointer,
    );
    addrs = g_list_append(
        addrs,
        g_inet_address_new_loopback(G_SOCKET_FAMILY_IPV4) as gpointer,
    );
    safe_c2rust_g_network_address_set_cached_addresses(
        addr,
        safe_c2rust_g_steal_pointer(&raw mut addrs as gpointer) as *mut GList,
        0 as guint64,
    );
    return addr as *mut ::core::ffi::c_void as *mut GSocketConnectable;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_network_address_parse(
    mut host_and_port: *const gchar,
    mut default_port: guint16,
    mut error: *mut *mut GError,
) -> *mut GSocketConnectable {
    let mut connectable: *mut GSocketConnectable = ::core::ptr::null_mut::<GSocketConnectable>();
    let mut port: *const gchar = ::core::ptr::null::<gchar>();
    let mut portnum: guint16 = 0;
    let mut name: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !host_and_port.is_null() {
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
            b"host_and_port != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSocketConnectable>();
    }
    port = ::core::ptr::null::<gchar>();
    if *host_and_port.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '[' as i32 {
        let mut end: *const gchar = ::core::ptr::null::<gchar>();
        end = strchr(host_and_port as *const ::core::ffi::c_char, ']' as i32);
        if end.is_null() {
            g_set_error(
                error,
                g_io_error_quark(),
                G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Hostname \xE2\x80\x9C%s\xE2\x80\x9D contains \xE2\x80\x9C[\xE2\x80\x9D but not \xE2\x80\x9C]\xE2\x80\x9D\0"
                        as *const u8 as *const gchar,
                ),
                host_and_port,
            );
            return ::core::ptr::null_mut::<GSocketConnectable>();
        }
        if *end.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32 {
            port = ::core::ptr::null::<gchar>();
        } else if *end.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ':' as i32
        {
            port = end.offset(2 as ::core::ffi::c_int as isize) as *const gchar;
        } else {
            g_set_error(
                error,
                g_io_error_quark(),
                G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                b"The ']' character (in hostname '%s') must come at the end or be immediately followed by ':' and a port\0"
                    as *const u8 as *const gchar,
                host_and_port,
            );
            return ::core::ptr::null_mut::<GSocketConnectable>();
        }
        name = g_strndup(
            host_and_port.offset(1 as ::core::ffi::c_int as isize),
            (end.offset_from(host_and_port) as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                as gsize,
        );
    } else {
        port = strchr(host_and_port as *const ::core::ffi::c_char, ':' as i32);
        if !port.is_null() {
            port = port.offset(1);
            if !strchr(port as *const ::core::ffi::c_char, ':' as i32).is_null() {
                name = safe_c2rust_g_strdup_inline(host_and_port as *const ::core::ffi::c_char)
                    as *mut gchar;
                port = ::core::ptr::null::<gchar>();
            } else {
                name = g_strndup(
                    host_and_port,
                    (port.offset_from(host_and_port) as ::core::ffi::c_long
                        - 1 as ::core::ffi::c_long) as gsize,
                );
            }
        } else {
            name = safe_c2rust_g_strdup_inline(host_and_port as *const ::core::ffi::c_char)
                as *mut gchar;
        }
    }
    if !port.is_null() {
        if *port.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32 {
            g_set_error(
                error,
                g_io_error_quark(),
                G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                b"If a ':' character is given, it must be followed by a port (in hostname '%s').\0"
                    as *const u8 as *const gchar,
                host_and_port,
            );
            g_free(name as gpointer);
            return ::core::ptr::null_mut::<GSocketConnectable>();
        } else if '0' as i32 <= *port.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            && *port.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int <= '9' as i32
        {
            let mut end_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut value: ::core::ffi::c_long = 0;
            value = strtol(
                port as *const ::core::ffi::c_char,
                &raw mut end_0,
                10 as ::core::ffi::c_int,
            );
            if *end_0 as ::core::ffi::c_int != '\0' as i32
                || value < 0 as ::core::ffi::c_long
                || value > G_MAXUINT16 as ::core::ffi::c_long
            {
                g_set_error(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                    b"Invalid numeric port '%s' specified in hostname '%s'\0" as *const u8
                        as *const gchar,
                    port,
                    host_and_port,
                );
                g_free(name as gpointer);
                return ::core::ptr::null_mut::<GSocketConnectable>();
            }
            portnum = value as guint16;
        } else {
            if g_getservbyname_ntohs(
                port as *const ::core::ffi::c_char,
                b"tcp\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut portnum,
            ) == 0
            {
                g_set_error(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                    b"Unknown service '%s' specified in hostname '%s'\0" as *const u8
                        as *const gchar,
                    port,
                    host_and_port,
                );
                endservent();
                g_free(name as gpointer);
                return ::core::ptr::null_mut::<GSocketConnectable>();
            }
            endservent();
        }
    } else {
        portnum = default_port;
    }
    connectable = safe_c2rust_g_network_address_new(name, portnum);
    g_free(name as gpointer);
    return connectable;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_network_address_parse_uri(
    mut uri: *const gchar,
    mut default_port: guint16,
    mut error: *mut *mut GError,
) -> *mut GSocketConnectable {
    let mut conn: *mut GSocketConnectable = ::core::ptr::null_mut::<GSocketConnectable>();
    let mut scheme: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut hostname: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut port: gint = 0;
    if g_uri_split_network(
        uri,
        G_URI_FLAGS_NONE,
        &raw mut scheme,
        &raw mut hostname,
        &raw mut port,
        ::core::ptr::null_mut::<*mut GError>(),
    ) == 0
    {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            b"Invalid URI \xE2\x80\x98%s\xE2\x80\x99\0" as *const u8 as *const gchar,
            uri,
        );
        return ::core::ptr::null_mut::<GSocketConnectable>();
    }
    if port <= 0 as ::core::ffi::c_int {
        port = default_port as gint;
    }
    conn = g_object_new(
        safe_c2rust_g_network_address_get_type(),
        b"hostname\0" as *const u8 as *const gchar,
        hostname,
        b"port\0" as *const u8 as *const ::core::ffi::c_char,
        port as guint,
        b"scheme\0" as *const u8 as *const ::core::ffi::c_char,
        scheme,
        NULL_0,
    ) as *mut GSocketConnectable;
    g_free(scheme as gpointer);
    g_free(hostname as gpointer);
    return conn;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_network_address_get_hostname(
    mut addr: *mut GNetworkAddress,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = addr as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_network_address_get_type();
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
            b"G_IS_NETWORK_ADDRESS (addr)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*(*addr).priv_0).hostname;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_network_address_get_port(
    mut addr: *mut GNetworkAddress,
) -> guint16 {
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = addr as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_network_address_get_type();
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
            b"G_IS_NETWORK_ADDRESS (addr)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint16;
    }
    return (*(*addr).priv_0).port;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_network_address_get_scheme(
    mut addr: *mut GNetworkAddress,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = addr as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_network_address_get_type();
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
            b"G_IS_NETWORK_ADDRESS (addr)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*(*addr).priv_0).scheme;
}
static mut safe_c2rust_GNetworkAddressAddressEnumerator_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust__g_network_address_address_enumerator_get_type() -> GType {
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
            safe_c2rust__g_network_address_address_enumerator_get_type_once();
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
unsafe extern "C" fn safe_c2rust__g_network_address_address_enumerator_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_socket_address_enumerator_get_type(),
        g_intern_static_string(b"GNetworkAddressAddressEnumerator\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GNetworkAddressAddressEnumeratorClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust__g_network_address_address_enumerator_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GNetworkAddressAddressEnumerator>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GNetworkAddressAddressEnumerator) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust__g_network_address_address_enumerator_init
                    as unsafe extern "C" fn(*mut GNetworkAddressAddressEnumerator) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
static mut safe_c2rust__g_network_address_address_enumerator_parent_class: gpointer = NULL_0;
unsafe extern "C" fn safe_c2rust__g_network_address_address_enumerator_class_intern_init(
    mut klass: gpointer,
) {
    safe_c2rust__g_network_address_address_enumerator_parent_class =
        g_type_class_peek_parent(klass);
    if safe_c2rust_GNetworkAddressAddressEnumerator_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GNetworkAddressAddressEnumerator_private_offset,
        );
    }
    safe_c2rust__g_network_address_address_enumerator_class_init(
        klass as *mut GNetworkAddressAddressEnumeratorClass,
    );
}
unsafe extern "C" fn safe_c2rust_g_network_address_address_enumerator_finalize(
    mut object: *mut GObject,
) {
    let mut addr_enum: *mut GNetworkAddressAddressEnumerator =
        object as *mut ::core::ffi::c_void as *mut GNetworkAddressAddressEnumerator;
    if !(*addr_enum).wait_source.is_null() {
        g_source_destroy((*addr_enum).wait_source);
        let mut _pp: *mut *mut GSource = &raw mut (*addr_enum).wait_source;
        let mut _ptr: *mut GSource = *_pp;
        *_pp = ::core::ptr::null_mut::<GSource>();
        if !_ptr.is_null() {
            g_source_unref(_ptr as *mut GSource);
        }
    }
    let mut _pp_0: *mut *mut GTask = &raw mut (*addr_enum).queued_task;
    let mut _ptr_0: *mut GTask = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<GTask>();
    if !_ptr_0.is_null() {
        g_object_unref(_ptr_0 as gpointer);
    }
    let mut _pp_1: *mut *mut GTask = &raw mut (*addr_enum).waiting_task;
    let mut _ptr_1: *mut GTask = *_pp_1;
    *_pp_1 = ::core::ptr::null_mut::<GTask>();
    if !_ptr_1.is_null() {
        g_object_unref(_ptr_1 as gpointer);
    }
    g_clear_error(&raw mut (*addr_enum).last_error);
    g_object_unref((*addr_enum).addr as gpointer);
    let mut _pp_2: *mut *mut GMainContext = &raw mut (*addr_enum).context;
    let mut _ptr_2: *mut GMainContext = *_pp_2;
    *_pp_2 = ::core::ptr::null_mut::<GMainContext>();
    if !_ptr_2.is_null() {
        g_main_context_unref(_ptr_2 as *mut GMainContext);
    }
    g_list_free_full(
        (*addr_enum).addresses,
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
    (*(safe_c2rust__g_network_address_address_enumerator_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
#[inline]
unsafe extern "C" fn safe_c2rust_get_address_family(
    mut address: *mut GInetSocketAddress,
) -> GSocketFamily {
    return g_inet_address_get_family(g_inet_socket_address_get_address(address));
}
unsafe extern "C" fn safe_c2rust_list_split_families(
    mut list: *mut GList,
    mut out_ipv4: *mut *mut GList,
    mut out_ipv6: *mut *mut GList,
) {
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !out_ipv4.is_null() {
            _g_boolean_var_16 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_16 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_16
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gnetworkaddress.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            693 as ::core::ffi::c_int,
            G_STRFUNC,
            b"out_ipv4\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !out_ipv6.is_null() {
            _g_boolean_var_17 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_17 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_17
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gnetworkaddress.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            694 as ::core::ffi::c_int,
            G_STRFUNC,
            b"out_ipv6\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    while !list.is_null() {
        let mut family: GSocketFamily =
            safe_c2rust_get_address_family((*list).data as *mut GInetSocketAddress);
        match family as ::core::ffi::c_uint {
            2 => {
                *out_ipv4 = g_list_prepend(*out_ipv4, (*list).data);
            }
            10 => {
                *out_ipv6 = g_list_prepend(*out_ipv6, (*list).data);
            }
            0 | 1 => {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gnetworkaddress.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    709 as ::core::ffi::c_int,
                    G_STRFUNC,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
            }
            _ => {}
        }
        list = if !list.is_null() {
            (*list).next
        } else {
            ::core::ptr::null_mut::<GList>()
        };
    }
    *out_ipv4 = g_list_reverse(*out_ipv4);
    *out_ipv6 = g_list_reverse(*out_ipv6);
}
unsafe extern "C" fn safe_c2rust_list_interleave_families(
    mut list1: *mut GList,
    mut list2: *mut GList,
) -> *mut GList {
    let mut interleaved: *mut GList = ::core::ptr::null_mut::<GList>();
    while !list1.is_null() || !list2.is_null() {
        if !list1.is_null() {
            interleaved = g_list_append(interleaved, (*list1).data);
            list1 = g_list_delete_link(list1, list1);
        }
        if !list2.is_null() {
            interleaved = g_list_append(interleaved, (*list2).data);
            list2 = g_list_delete_link(list2, list2);
        }
    }
    return interleaved;
}
unsafe extern "C" fn safe_c2rust_list_copy_interleaved(mut list: *mut GList) -> *mut GList {
    let mut ipv4: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut ipv6: *mut GList = ::core::ptr::null_mut::<GList>();
    safe_c2rust_list_split_families(list, &raw mut ipv4, &raw mut ipv6);
    return safe_c2rust_list_interleave_families(ipv6, ipv4);
}
unsafe extern "C" fn safe_c2rust_list_concat_interleaved(
    mut parent_list: *mut GList,
    mut current_item: *mut GList,
    mut new_list: *mut GList,
) -> *mut GList {
    let mut ipv4: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut ipv6: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut interleaved: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut trailing: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut last_family: GSocketFamily = G_SOCKET_FAMILY_IPV4;
    if !current_item.is_null() {
        last_family =
            safe_c2rust_get_address_family((*current_item).data as *mut GInetSocketAddress);
        trailing = if !current_item.is_null() {
            (*current_item).next
        } else {
            ::core::ptr::null_mut::<GList>()
        };
        (*current_item).next = ::core::ptr::null_mut::<GList>();
    }
    safe_c2rust_list_split_families(trailing, &raw mut ipv4, &raw mut ipv6);
    safe_c2rust_list_split_families(new_list, &raw mut ipv4, &raw mut ipv6);
    g_list_free(new_list);
    if !trailing.is_null() {
        g_list_free(trailing);
    }
    if last_family as ::core::ffi::c_uint
        == G_SOCKET_FAMILY_IPV4 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        interleaved = safe_c2rust_list_interleave_families(ipv6, ipv4);
    } else {
        interleaved = safe_c2rust_list_interleave_families(ipv4, ipv6);
    }
    return g_list_concat(parent_list, interleaved);
}
unsafe extern "C" fn safe_c2rust_maybe_update_address_cache(
    mut addr_enum: *mut GNetworkAddressAddressEnumerator,
    mut resolver: *mut GResolver,
) {
    let mut addresses: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut p: *mut GList = ::core::ptr::null_mut::<GList>();
    if (*addr_enum).state as ::core::ffi::c_uint
        & RESOLVE_STATE_WAITING_ON_IPV4 as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
        || (*addr_enum).state as ::core::ffi::c_uint
            & RESOLVE_STATE_WAITING_ON_IPV6 as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
    {
        return;
    }
    addresses = safe_c2rust_list_copy_interleaved((*addr_enum).addresses);
    p = addresses;
    while !p.is_null() {
        g_object_ref((*p).data);
        p = (*p).next;
    }
    safe_c2rust_g_network_address_set_cached_addresses(
        (*addr_enum).addr,
        safe_c2rust_g_steal_pointer(&raw mut addresses as gpointer) as *mut GList,
        g_resolver_get_serial(resolver),
    );
}
unsafe extern "C" fn safe_c2rust_g_network_address_address_enumerator_add_addresses(
    mut addr_enum: *mut GNetworkAddressAddressEnumerator,
    mut addresses: *mut GList,
    mut resolver: *mut GResolver,
) {
    let mut new_addresses: *mut GList =
        safe_c2rust_inet_addresses_to_inet_socket_addresses((*addr_enum).addr, addresses);
    if (*addr_enum).addresses.is_null() {
        (*addr_enum).addresses = safe_c2rust_g_steal_pointer(&raw mut new_addresses as gpointer)
            as *mut GList as *mut GList;
    } else {
        (*addr_enum).addresses = safe_c2rust_list_concat_interleaved(
            (*addr_enum).addresses,
            (*addr_enum).current_item,
            safe_c2rust_g_steal_pointer(&raw mut new_addresses as gpointer) as *mut GList,
        );
    }
    safe_c2rust_maybe_update_address_cache(addr_enum, resolver);
}
unsafe extern "C" fn safe_c2rust_copy_object(
    mut src: gconstpointer,
    mut user_data: gpointer,
) -> gpointer {
    return g_object_ref(src as *mut ::core::ffi::c_void as *mut GObject as gpointer) as *mut GObject
        as gpointer;
}
unsafe extern "C" fn safe_c2rust_init_and_query_next_address(
    mut addr_enum: *mut GNetworkAddressAddressEnumerator,
) -> *mut GSocketAddress {
    let mut next_item: *mut GList = ::core::ptr::null_mut::<GList>();
    if (*addr_enum).addresses.is_null() {
        (*addr_enum).addresses = g_list_copy_deep(
            (*(*(*addr_enum).addr).priv_0).cached_sockaddrs,
            Some(
                safe_c2rust_copy_object
                    as unsafe extern "C" fn(gconstpointer, gpointer) -> gpointer,
            ),
            NULL_0,
        );
    }
    if (*addr_enum).current_item.is_null() {
        (*addr_enum).current_item = (*addr_enum).addresses;
        next_item = (*addr_enum).current_item;
    } else {
        next_item = if !(*addr_enum).current_item.is_null() {
            (*(*addr_enum).current_item).next
        } else {
            ::core::ptr::null_mut::<GList>()
        };
    }
    if !next_item.is_null() {
        (*addr_enum).current_item = next_item;
        return g_object_ref((*(*addr_enum).current_item).data) as *mut GSocketAddress;
    } else {
        return ::core::ptr::null_mut::<GSocketAddress>();
    };
}
unsafe extern "C" fn safe_c2rust_g_network_address_address_enumerator_next(
    mut enumerator: *mut GSocketAddressEnumerator,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GSocketAddress {
    let mut addr_enum: *mut GNetworkAddressAddressEnumerator =
        enumerator as *mut ::core::ffi::c_void as *mut GNetworkAddressAddressEnumerator;
    if (*addr_enum).addresses.is_null() {
        let mut addr: *mut GNetworkAddress = (*addr_enum).addr;
        let mut resolver: *mut GResolver = g_resolver_get_default();
        let mut serial: gint64 = g_resolver_get_serial(resolver) as gint64;
        if (*(*addr).priv_0).resolver_serial != 0 as gint64
            && (*(*addr).priv_0).resolver_serial != serial
        {
            g_list_free_full(
                (*(*addr).priv_0).cached_sockaddrs,
                Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
            );
            (*(*addr).priv_0).cached_sockaddrs = ::core::ptr::null_mut::<GList>();
        }
        if (*(*addr).priv_0).cached_sockaddrs.is_null() {
            safe_c2rust_g_network_address_parse_sockaddr(addr);
        }
        if (*(*addr).priv_0).cached_sockaddrs.is_null() {
            let mut addresses: *mut GList = ::core::ptr::null_mut::<GList>();
            addresses =
                g_resolver_lookup_by_name(resolver, (*(*addr).priv_0).hostname, cancellable, error);
            if addresses.is_null() {
                g_object_unref(resolver as gpointer);
                return ::core::ptr::null_mut::<GSocketAddress>();
            }
            safe_c2rust_g_network_address_set_cached_addresses(
                addr,
                safe_c2rust_g_steal_pointer(&raw mut addresses as gpointer) as *mut GList,
                serial as guint64,
            );
        }
        g_object_unref(resolver as gpointer);
    }
    return safe_c2rust_init_and_query_next_address(addr_enum);
}
unsafe extern "C" fn safe_c2rust_complete_queued_task(
    mut addr_enum: *mut GNetworkAddressAddressEnumerator,
    mut task: *mut GTask,
    mut error: *mut GError,
) {
    if !error.is_null() {
        g_task_return_error(task, error);
    } else {
        let mut sockaddr: *mut GSocketAddress = safe_c2rust_init_and_query_next_address(addr_enum);
        g_task_return_pointer(
            task,
            safe_c2rust_g_steal_pointer(&raw mut sockaddr as gpointer) as *mut GSocketAddress
                as gpointer,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    }
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_on_address_timeout(mut user_data: gpointer) -> ::core::ffi::c_int {
    let mut addr_enum: *mut GNetworkAddressAddressEnumerator =
        user_data as *mut GNetworkAddressAddressEnumerator;
    g_object_ref(addr_enum as gpointer);
    if !(*addr_enum).queued_task.is_null() {
        safe_c2rust_complete_queued_task(
            addr_enum,
            safe_c2rust_g_steal_pointer(&raw mut (*addr_enum).queued_task as gpointer)
                as *mut GTask,
            safe_c2rust_g_steal_pointer(&raw mut (*addr_enum).last_error as gpointer)
                as *mut GError,
        );
    } else if !(*addr_enum).waiting_task.is_null() {
        safe_c2rust_complete_queued_task(
            addr_enum,
            safe_c2rust_g_steal_pointer(&raw mut (*addr_enum).waiting_task as gpointer)
                as *mut GTask,
            ::core::ptr::null_mut::<GError>(),
        );
    }
    let mut _pp: *mut *mut GSource = &raw mut (*addr_enum).wait_source;
    let mut _ptr: *mut GSource = *_pp;
    *_pp = ::core::ptr::null_mut::<GSource>();
    if !_ptr.is_null() {
        g_source_unref(_ptr as *mut GSource);
    }
    g_object_unref(addr_enum as gpointer);
    return G_SOURCE_REMOVE;
}
unsafe extern "C" fn safe_c2rust_got_ipv6_addresses(
    mut source_object: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut addr_enum: *mut GNetworkAddressAddressEnumerator =
        user_data as *mut GNetworkAddressAddressEnumerator;
    let mut resolver: *mut GResolver = source_object as *mut ::core::ffi::c_void as *mut GResolver;
    let mut addresses: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    (*addr_enum).state = ::core::mem::transmute::<::core::ffi::c_uint, ResolveState>(
        (*addr_enum).state as ::core::ffi::c_uint
            ^ RESOLVE_STATE_WAITING_ON_IPV6 as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    addresses = g_resolver_lookup_by_name_with_flags_finish(resolver, result, &raw mut error);
    if error.is_null() {
        safe_c2rust_g_network_address_address_enumerator_add_addresses(
            addr_enum,
            safe_c2rust_g_steal_pointer(&raw mut addresses as gpointer) as *mut GList,
            resolver,
        );
    } else {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_DEBUG,
            b"IPv6 DNS error: %s\0" as *const u8 as *const gchar,
            (*error).message,
        );
    }
    if !(*addr_enum).wait_source.is_null() {
        g_source_destroy((*addr_enum).wait_source);
        let mut _pp: *mut *mut GSource = &raw mut (*addr_enum).wait_source;
        let mut _ptr: *mut GSource = *_pp;
        *_pp = ::core::ptr::null_mut::<GSource>();
        if !_ptr.is_null() {
            g_source_unref(_ptr as *mut GSource);
        }
    }
    if !error.is_null()
        && (*addr_enum).last_error.is_null()
        && (*addr_enum).state as ::core::ffi::c_uint
            & RESOLVE_STATE_WAITING_ON_IPV4 as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
    {
        (*addr_enum).last_error =
            safe_c2rust_g_steal_pointer(&raw mut error as gpointer) as *mut GError as *mut GError;
    } else if !(*addr_enum).waiting_task.is_null() {
        safe_c2rust_complete_queued_task(
            addr_enum,
            safe_c2rust_g_steal_pointer(&raw mut (*addr_enum).waiting_task as gpointer)
                as *mut GTask,
            ::core::ptr::null_mut::<GError>(),
        );
    } else if !(*addr_enum).queued_task.is_null() {
        let mut task_error: *mut GError = ::core::ptr::null_mut::<GError>();
        if !error.is_null() && !(*addr_enum).last_error.is_null() {
            task_error = safe_c2rust_g_steal_pointer(&raw mut error as gpointer) as *mut GError
                as *mut GError;
        }
        g_clear_error(&raw mut (*addr_enum).last_error);
        safe_c2rust_complete_queued_task(
            addr_enum,
            safe_c2rust_g_steal_pointer(&raw mut (*addr_enum).queued_task as gpointer)
                as *mut GTask,
            safe_c2rust_g_steal_pointer(&raw mut task_error as gpointer) as *mut GError,
        );
    }
    g_clear_error(&raw mut error);
    g_object_unref(addr_enum as gpointer);
}
unsafe extern "C" fn safe_c2rust_got_ipv4_addresses(
    mut source_object: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut addr_enum: *mut GNetworkAddressAddressEnumerator =
        user_data as *mut GNetworkAddressAddressEnumerator;
    let mut resolver: *mut GResolver = source_object as *mut ::core::ffi::c_void as *mut GResolver;
    let mut addresses: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    (*addr_enum).state = ::core::mem::transmute::<::core::ffi::c_uint, ResolveState>(
        (*addr_enum).state as ::core::ffi::c_uint
            ^ RESOLVE_STATE_WAITING_ON_IPV4 as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    addresses = g_resolver_lookup_by_name_with_flags_finish(resolver, result, &raw mut error);
    if error.is_null() {
        safe_c2rust_g_network_address_address_enumerator_add_addresses(
            addr_enum,
            safe_c2rust_g_steal_pointer(&raw mut addresses as gpointer) as *mut GList,
            resolver,
        );
    } else {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_DEBUG,
            b"IPv4 DNS error: %s\0" as *const u8 as *const gchar,
            (*error).message,
        );
    }
    if !(*addr_enum).wait_source.is_null() {
        g_source_destroy((*addr_enum).wait_source);
        let mut _pp: *mut *mut GSource = &raw mut (*addr_enum).wait_source;
        let mut _ptr: *mut GSource = *_pp;
        *_pp = ::core::ptr::null_mut::<GSource>();
        if !_ptr.is_null() {
            g_source_unref(_ptr as *mut GSource);
        }
    }
    if !(*addr_enum).last_error.is_null() {
        if ({
            let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
            if !(*addr_enum).queued_task.is_null() {
                _g_boolean_var_18 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_18 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_18
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gnetworkaddress.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1040 as ::core::ffi::c_int,
                G_STRFUNC,
                b"addr_enum->queued_task\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        g_clear_error(&raw mut (*addr_enum).last_error);
        safe_c2rust_complete_queued_task(
            addr_enum,
            safe_c2rust_g_steal_pointer(&raw mut (*addr_enum).queued_task as gpointer)
                as *mut GTask,
            safe_c2rust_g_steal_pointer(&raw mut error as gpointer) as *mut GError,
        );
    } else if !(*addr_enum).waiting_task.is_null() {
        safe_c2rust_complete_queued_task(
            addr_enum,
            safe_c2rust_g_steal_pointer(&raw mut (*addr_enum).waiting_task as gpointer)
                as *mut GTask,
            ::core::ptr::null_mut::<GError>(),
        );
    } else if !(*addr_enum).queued_task.is_null() {
        (*addr_enum).last_error =
            safe_c2rust_g_steal_pointer(&raw mut error as gpointer) as *mut GError as *mut GError;
        (*addr_enum).wait_source =
            g_timeout_source_new(HAPPY_EYEBALLS_RESOLUTION_DELAY_MS as guint);
        g_source_set_callback(
            (*addr_enum).wait_source,
            Some(
                safe_c2rust_on_address_timeout
                    as unsafe extern "C" fn(gpointer) -> ::core::ffi::c_int,
            ),
            addr_enum as gpointer,
            None,
        );
        g_source_attach((*addr_enum).wait_source, (*addr_enum).context);
    }
    g_clear_error(&raw mut error);
    g_object_unref(addr_enum as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_network_address_address_enumerator_next_async(
    mut enumerator: *mut GSocketAddressEnumerator,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut addr_enum: *mut GNetworkAddressAddressEnumerator =
        enumerator as *mut ::core::ffi::c_void as *mut GNetworkAddressAddressEnumerator;
    let mut sockaddr: *mut GSocketAddress = ::core::ptr::null_mut::<GSocketAddress>();
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    task = g_task_new(addr_enum as gpointer, cancellable, callback, user_data);
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
            safe_c2rust_g_network_address_address_enumerator_next_async
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
            b"g_network_address_address_enumerator_next_async\0" as *const u8 as *const gchar,
        );
    }
    if (*addr_enum).addresses.is_null()
        && (*addr_enum).state as ::core::ffi::c_uint
            == RESOLVE_STATE_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut addr: *mut GNetworkAddress = (*addr_enum).addr;
        let mut resolver: *mut GResolver = g_resolver_get_default();
        let mut serial: gint64 = g_resolver_get_serial(resolver) as gint64;
        if (*(*addr).priv_0).resolver_serial != 0 as gint64
            && (*(*addr).priv_0).resolver_serial != serial
        {
            g_list_free_full(
                (*(*addr).priv_0).cached_sockaddrs,
                Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
            );
            (*(*addr).priv_0).cached_sockaddrs = ::core::ptr::null_mut::<GList>();
        }
        if (*(*addr).priv_0).cached_sockaddrs.is_null() {
            if safe_c2rust_g_network_address_parse_sockaddr(addr) != 0 {
                safe_c2rust_complete_queued_task(
                    addr_enum,
                    task,
                    ::core::ptr::null_mut::<GError>(),
                );
            } else {
                if ({
                    let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
                    if (*addr_enum).queued_task.is_null() {
                        _g_boolean_var_19 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_19 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_19
                }) as ::core::ffi::c_long
                    != 0
                {
                } else {
                    g_assertion_message_expr(
                        G_LOG_DOMAIN.as_ptr(),
                        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gnetworkaddress.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        1099 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"addr_enum->queued_task == NULL\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                (*addr_enum).state = (RESOLVE_STATE_WAITING_ON_IPV4 as ::core::ffi::c_int
                    | RESOLVE_STATE_WAITING_ON_IPV6 as ::core::ffi::c_int)
                    as ResolveState;
                (*addr_enum).queued_task = safe_c2rust_g_steal_pointer(&raw mut task as gpointer)
                    as *mut GTask as *mut GTask;
                g_resolver_lookup_by_name_with_flags_async(
                    resolver,
                    (*(*addr).priv_0).hostname,
                    G_RESOLVER_NAME_LOOKUP_FLAGS_IPV6_ONLY,
                    cancellable,
                    Some(
                        safe_c2rust_got_ipv6_addresses
                            as unsafe extern "C" fn(
                                *mut GObject,
                                *mut GAsyncResult,
                                gpointer,
                            ) -> (),
                    ),
                    g_object_ref(addr_enum as gpointer) as *mut GNetworkAddressAddressEnumerator
                        as gpointer,
                );
                g_resolver_lookup_by_name_with_flags_async(
                    resolver,
                    (*(*addr).priv_0).hostname,
                    G_RESOLVER_NAME_LOOKUP_FLAGS_IPV4_ONLY,
                    cancellable,
                    Some(
                        safe_c2rust_got_ipv4_addresses
                            as unsafe extern "C" fn(
                                *mut GObject,
                                *mut GAsyncResult,
                                gpointer,
                            ) -> (),
                    ),
                    g_object_ref(addr_enum as gpointer) as *mut GNetworkAddressAddressEnumerator
                        as gpointer,
                );
            }
            g_object_unref(resolver as gpointer);
            return;
        }
        g_object_unref(resolver as gpointer);
    }
    sockaddr = safe_c2rust_init_and_query_next_address(addr_enum);
    if sockaddr.is_null()
        && ((*addr_enum).state as ::core::ffi::c_uint
            & RESOLVE_STATE_WAITING_ON_IPV4 as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
            || (*addr_enum).state as ::core::ffi::c_uint
                & RESOLVE_STATE_WAITING_ON_IPV6 as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0)
    {
        (*addr_enum).waiting_task = task;
    } else {
        g_task_return_pointer(
            task,
            sockaddr as gpointer,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
        g_object_unref(task as gpointer);
    };
}
unsafe extern "C" fn safe_c2rust_g_network_address_address_enumerator_next_finish(
    mut enumerator: *mut GSocketAddressEnumerator,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GSocketAddress {
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, enumerator as gpointer) != 0 {
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
            b"g_task_is_valid (result, enumerator)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSocketAddress>();
    }
    return g_task_propagate_pointer(result as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GSocketAddress;
}
unsafe extern "C" fn safe_c2rust__g_network_address_address_enumerator_init(
    mut enumerator: *mut GNetworkAddressAddressEnumerator,
) {
    (*enumerator).context = g_main_context_ref_thread_default();
}
unsafe extern "C" fn safe_c2rust__g_network_address_address_enumerator_class_init(
    mut addrenum_class: *mut GNetworkAddressAddressEnumeratorClass,
) {
    let mut object_class: *mut GObjectClass =
        addrenum_class as *mut ::core::ffi::c_void as *mut GObjectClass;
    let mut enumerator_class: *mut GSocketAddressEnumeratorClass =
        addrenum_class as *mut ::core::ffi::c_void as *mut GSocketAddressEnumeratorClass;
    (*enumerator_class).next = Some(
        safe_c2rust_g_network_address_address_enumerator_next
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
        safe_c2rust_g_network_address_address_enumerator_next_async
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
        safe_c2rust_g_network_address_address_enumerator_next_finish
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
        safe_c2rust_g_network_address_address_enumerator_finalize
            as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
}
unsafe extern "C" fn safe_c2rust_g_network_address_connectable_enumerate(
    mut connectable: *mut GSocketConnectable,
) -> *mut GSocketAddressEnumerator {
    let mut addr_enum: *mut GNetworkAddressAddressEnumerator =
        ::core::ptr::null_mut::<GNetworkAddressAddressEnumerator>();
    addr_enum = g_object_new(
        safe_c2rust__g_network_address_address_enumerator_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GNetworkAddressAddressEnumerator;
    (*addr_enum).addr =
        g_object_ref(connectable as *mut ::core::ffi::c_void as *mut GNetworkAddress as gpointer)
            as *mut GNetworkAddress as *mut GNetworkAddress;
    return addr_enum as *mut GSocketAddressEnumerator;
}
unsafe extern "C" fn safe_c2rust_g_network_address_connectable_proxy_enumerate(
    mut connectable: *mut GSocketConnectable,
) -> *mut GSocketAddressEnumerator {
    let mut self_0: *mut GNetworkAddress =
        connectable as *mut ::core::ffi::c_void as *mut GNetworkAddress;
    let mut proxy_enum: *mut GSocketAddressEnumerator =
        ::core::ptr::null_mut::<GSocketAddressEnumerator>();
    let mut uri: *mut gchar = ::core::ptr::null_mut::<gchar>();
    uri = g_uri_join(
        G_URI_FLAGS_NONE,
        if !(*(*self_0).priv_0).scheme.is_null() {
            (*(*self_0).priv_0).scheme as *const gchar
        } else {
            b"none\0" as *const u8 as *const gchar
        },
        ::core::ptr::null::<gchar>(),
        (*(*self_0).priv_0).hostname,
        (*(*self_0).priv_0).port as gint,
        b"\0" as *const u8 as *const gchar,
        ::core::ptr::null::<gchar>(),
        ::core::ptr::null::<gchar>(),
    );
    proxy_enum = g_object_new(
        g_proxy_address_enumerator_get_type(),
        b"connectable\0" as *const u8 as *const gchar,
        connectable,
        b"uri\0" as *const u8 as *const ::core::ffi::c_char,
        uri,
        NULL_0,
    ) as *mut GSocketAddressEnumerator;
    g_free(uri as gpointer);
    return proxy_enum;
}
unsafe extern "C" fn safe_c2rust_g_network_address_connectable_to_string(
    mut connectable: *mut GSocketConnectable,
) -> *mut gchar {
    let mut addr: *mut GNetworkAddress = ::core::ptr::null_mut::<GNetworkAddress>();
    let mut scheme: *const gchar = ::core::ptr::null::<gchar>();
    let mut port: guint16 = 0;
    let mut out: *mut GString = ::core::ptr::null_mut::<GString>();
    addr = connectable as *mut ::core::ffi::c_void as *mut GNetworkAddress;
    out = g_string_new(b"\0" as *const u8 as *const gchar);
    scheme = safe_c2rust_g_network_address_get_scheme(addr);
    if !scheme.is_null() {
        g_string_append_printf(out, b"%s:\0" as *const u8 as *const gchar, scheme);
    }
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char =
                safe_c2rust_g_network_address_get_hostname(addr) as *const ::core::ffi::c_char;
            safe_c2rust_g_string_append_len_inline(
                out,
                __val,
                if ({
                    let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
                    if !__val.is_null() {
                        _g_boolean_var_21 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_21 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_21
                }) as ::core::ffi::c_long
                    != 0
                {
                    strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize)) as gssize
                } else {
                    -(1 as ::core::ffi::c_int) as gssize
                },
            );
        });
    } else {
        safe_c2rust_g_string_append_len_inline(
            out,
            safe_c2rust_g_network_address_get_hostname(addr) as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
    port = safe_c2rust_g_network_address_get_port(addr);
    if port as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        g_string_append_printf(
            out,
            b":%u\0" as *const u8 as *const gchar,
            port as ::core::ffi::c_int,
        );
    }
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(out, 0 as gboolean)
        } else {
            g_string_free_and_steal(out)
        }
    } else {
        g_string_free(out, 0 as gboolean)
    };
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
