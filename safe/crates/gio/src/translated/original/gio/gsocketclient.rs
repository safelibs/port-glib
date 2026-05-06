use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GHashTable;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _GInetAddressPrivate;
    pub type _GInetSocketAddressPrivate;
    pub type _GNetworkAddressPrivate;
    pub type _GNetworkServicePrivate;
    pub type _GIOStreamPrivate;
    pub type _GSocketPrivate;
    pub type _GProxyResolver;
    pub type _GSocketConnectionPrivate;
    pub type _GSocketConnectable;
    pub type _GTask;
    pub type _GTlsClientConnection;
    pub type _GTlsConnectionPrivate;
    pub type _GProxy;
    pub type _GProxyAddressPrivate;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_propagate_error(dest: *mut *mut GError, src: *mut GError);
    fn g_clear_error(err: *mut *mut GError);
    fn g_prefix_error(err: *mut *mut GError, format: *const gchar, ...);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_hash_table_new_full(
        hash_func: GHashFunc,
        key_equal_func: GEqualFunc,
        key_destroy_func: GDestroyNotify,
        value_destroy_func: GDestroyNotify,
    ) -> *mut GHashTable;
    fn g_hash_table_add(hash_table: *mut GHashTable, key: gpointer) -> gboolean;
    fn g_hash_table_contains(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_unref(hash_table: *mut GHashTable);
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_slist_free(list: *mut GSList);
    fn g_slist_free_full(list: *mut GSList, free_func: GDestroyNotify);
    fn g_slist_append(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_slist_remove(list: *mut GSList, data: gconstpointer) -> *mut GSList;
    fn g_slist_delete_link(list: *mut GSList, link_: *mut GSList) -> *mut GSList;
    fn g_slist_find(list: *mut GSList, data: gconstpointer) -> *mut GSList;
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
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_ref_count_init(rc: *mut grefcount);
    fn g_ref_count_inc(rc: *mut grefcount);
    fn g_ref_count_dec(rc: *mut grefcount) -> gboolean;
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
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
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_set(object: gpointer, first_property_name: *const gchar, ...);
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_value_set_object(value: *mut GValue, v_object: gpointer);
    fn g_value_get_object(value: *const GValue) -> gpointer;
    fn g_value_set_enum(value: *mut GValue, v_enum: gint);
    fn g_value_get_enum(value: *const GValue) -> gint;
    fn g_value_set_flags(value: *mut GValue, v_flags: guint);
    fn g_value_get_flags(value: *const GValue) -> guint;
    fn g_param_spec_boolean(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: gboolean,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_uint(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        minimum: guint,
        maximum: guint,
        default_value: guint,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
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
    fn g_param_spec_object(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        object_type: GType,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_value_set_boolean(value: *mut GValue, v_boolean: gboolean);
    fn g_value_get_boolean(value: *const GValue) -> gboolean;
    fn g_value_set_uint(value: *mut GValue, v_uint: guint);
    fn g_value_get_uint(value: *const GValue) -> guint;
    fn g_socket_family_get_type() -> GType;
    fn g_socket_type_get_type() -> GType;
    fn g_socket_protocol_get_type() -> GType;
    fn g_tls_certificate_flags_get_type() -> GType;
    fn g_socket_client_event_get_type() -> GType;
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
    fn g_socket_new(
        family: GSocketFamily,
        type_0: GSocketType,
        protocol: GSocketProtocol,
        error: *mut *mut GError,
    ) -> *mut GSocket;
    fn g_socket_set_blocking(socket: *mut GSocket, blocking: gboolean);
    fn g_socket_set_timeout(socket: *mut GSocket, timeout: guint);
    fn g_socket_bind(
        socket: *mut GSocket,
        address: *mut GSocketAddress,
        allow_reuse: gboolean,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_socket_set_option(
        socket: *mut GSocket,
        level: gint,
        optname: gint,
        value: gint,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_cancellable_new() -> *mut GCancellable;
    fn g_cancellable_is_cancelled(cancellable: *mut GCancellable) -> gboolean;
    fn g_cancellable_set_error_if_cancelled(
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_cancellable_connect(
        cancellable: *mut GCancellable,
        callback: GCallback,
        data: gpointer,
        data_destroy_func: GDestroyNotify,
    ) -> gulong;
    fn g_cancellable_disconnect(cancellable: *mut GCancellable, handler_id: gulong);
    fn g_cancellable_cancel(cancellable: *mut GCancellable);
    fn g_io_error_quark() -> GQuark;
    fn g_io_stream_get_type() -> GType;
    fn g_socket_connection_get_type() -> GType;
    fn g_socket_connection_connect(
        connection: *mut GSocketConnection,
        address: *mut GSocketAddress,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_socket_connection_connect_async(
        connection: *mut GSocketConnection,
        address: *mut GSocketAddress,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_socket_connection_connect_finish(
        connection: *mut GSocketConnection,
        result: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_socket_connection_factory_create_connection(
        socket: *mut GSocket,
    ) -> *mut GSocketConnection;
    fn g_socket_address_get_type() -> GType;
    fn g_socket_address_get_family(address: *mut GSocketAddress) -> GSocketFamily;
    fn g_socket_connection_set_cached_remote_address(
        connection: *mut GSocketConnection,
        address: *mut GSocketAddress,
    );
    fn g_proxy_address_enumerator_get_type() -> GType;
    fn g_inet_socket_address_get_type() -> GType;
    fn g_inet_socket_address_get_address(address: *mut GInetSocketAddress) -> *mut GInetAddress;
    fn g_proxy_address_get_type() -> GType;
    fn g_proxy_address_get_protocol(proxy: *mut GProxyAddress) -> *const gchar;
    fn g_task_new(
        source_object: gpointer,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        callback_data: gpointer,
    ) -> *mut GTask;
    fn g_task_report_error(
        source_object: gpointer,
        callback: GAsyncReadyCallback,
        callback_data: gpointer,
        source_tag: gpointer,
        error: *mut GError,
    );
    fn g_task_set_task_data(
        task: *mut GTask,
        task_data: gpointer,
        task_data_destroy: GDestroyNotify,
    );
    fn g_task_set_check_cancellable(task: *mut GTask, check_cancellable: gboolean);
    fn g_task_set_source_tag(task: *mut GTask, source_tag: gpointer);
    fn g_task_set_static_name(task: *mut GTask, name: *const gchar);
    fn g_task_get_context(task: *mut GTask) -> *mut GMainContext;
    fn g_task_get_cancellable(task: *mut GTask) -> *mut GCancellable;
    fn g_task_get_name(task: *mut GTask) -> *const gchar;
    fn g_task_is_valid(result: gpointer, source_object: gpointer) -> gboolean;
    fn g_task_return_pointer(task: *mut GTask, result: gpointer, result_destroy: GDestroyNotify);
    fn g_task_return_error(task: *mut GTask, error: *mut GError);
    fn g_task_propagate_pointer(task: *mut GTask, error: *mut *mut GError) -> gpointer;
    fn g_network_address_get_type() -> GType;
    fn g_network_address_parse(
        host_and_port: *const gchar,
        default_port: guint16,
        error: *mut *mut GError,
    ) -> *mut GSocketConnectable;
    fn g_network_address_parse_uri(
        uri: *const gchar,
        default_port: guint16,
        error: *mut *mut GError,
    ) -> *mut GSocketConnectable;
    fn g_network_address_get_hostname(addr: *mut GNetworkAddress) -> *const gchar;
    fn g_network_service_get_type() -> GType;
    fn g_network_service_new(
        service: *const gchar,
        protocol: *const gchar,
        domain: *const gchar,
    ) -> *mut GSocketConnectable;
    fn g_network_service_get_domain(srv: *mut GNetworkService) -> *const gchar;
    fn g_proxy_get_default_for_protocol(protocol: *const gchar) -> *mut GProxy;
    fn g_proxy_connect(
        proxy: *mut GProxy,
        connection: *mut GIOStream,
        proxy_address: *mut GProxyAddress,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GIOStream;
    fn g_proxy_connect_async(
        proxy: *mut GProxy,
        connection: *mut GIOStream,
        proxy_address: *mut GProxyAddress,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_proxy_connect_finish(
        proxy: *mut GProxy,
        result: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> *mut GIOStream;
    fn g_proxy_resolver_get_type() -> GType;
    fn g_proxy_resolver_get_default() -> *mut GProxyResolver;
    fn g_tcp_connection_get_type() -> GType;
    fn g_tcp_wrapper_connection_new(
        base_io_stream: *mut GIOStream,
        socket: *mut GSocket,
    ) -> *mut GSocketConnection;
    fn g_tls_connection_handshake(
        conn: *mut GTlsConnection,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_tls_connection_handshake_async(
        conn: *mut GTlsConnection,
        io_priority: ::core::ffi::c_int,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_tls_connection_handshake_finish(
        conn: *mut GTlsConnection,
        result: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_tls_client_connection_new(
        base_io_stream: *mut GIOStream,
        server_identity: *mut GSocketConnectable,
        error: *mut *mut GError,
    ) -> *mut GIOStream;
    fn g_tls_client_connection_set_validation_flags(
        conn: *mut GTlsClientConnection,
        flags: GTlsCertificateFlags,
    );
    fn g_inet_address_to_string(address: *mut GInetAddress) -> *mut gchar;
    fn glib_gettext(str: *const gchar) -> *const gchar;
    fn _g_cclosure_marshal_VOID__ENUM_OBJECT_OBJECT(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn _g_cclosure_marshal_VOID__ENUM_OBJECT_OBJECTv(
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
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
pub type grefcount = gint;
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
pub type GSocketType = ::core::ffi::c_uint;
pub const G_SOCKET_TYPE_SEQPACKET: GSocketType = 3;
pub const G_SOCKET_TYPE_DATAGRAM: GSocketType = 2;
pub const G_SOCKET_TYPE_STREAM: GSocketType = 1;
pub const G_SOCKET_TYPE_INVALID: GSocketType = 0;
pub type GSocketProtocol = ::core::ffi::c_int;
pub const G_SOCKET_PROTOCOL_SCTP: GSocketProtocol = 132;
pub const G_SOCKET_PROTOCOL_UDP: GSocketProtocol = 17;
pub const G_SOCKET_PROTOCOL_TCP: GSocketProtocol = 6;
pub const G_SOCKET_PROTOCOL_DEFAULT: GSocketProtocol = 0;
pub const G_SOCKET_PROTOCOL_UNKNOWN: GSocketProtocol = -1;
pub type GTlsCertificateFlags = ::core::ffi::c_uint;
pub const G_TLS_CERTIFICATE_VALIDATE_ALL: GTlsCertificateFlags = 127;
pub const G_TLS_CERTIFICATE_GENERIC_ERROR: GTlsCertificateFlags = 64;
pub const G_TLS_CERTIFICATE_INSECURE: GTlsCertificateFlags = 32;
pub const G_TLS_CERTIFICATE_REVOKED: GTlsCertificateFlags = 16;
pub const G_TLS_CERTIFICATE_EXPIRED: GTlsCertificateFlags = 8;
pub const G_TLS_CERTIFICATE_NOT_ACTIVATED: GTlsCertificateFlags = 4;
pub const G_TLS_CERTIFICATE_BAD_IDENTITY: GTlsCertificateFlags = 2;
pub const G_TLS_CERTIFICATE_UNKNOWN_CA: GTlsCertificateFlags = 1;
pub const G_TLS_CERTIFICATE_NO_FLAGS: GTlsCertificateFlags = 0;
pub type GSocketClientEvent = ::core::ffi::c_uint;
pub const G_SOCKET_CLIENT_COMPLETE: GSocketClientEvent = 8;
pub const G_SOCKET_CLIENT_TLS_HANDSHAKED: GSocketClientEvent = 7;
pub const G_SOCKET_CLIENT_TLS_HANDSHAKING: GSocketClientEvent = 6;
pub const G_SOCKET_CLIENT_PROXY_NEGOTIATED: GSocketClientEvent = 5;
pub const G_SOCKET_CLIENT_PROXY_NEGOTIATING: GSocketClientEvent = 4;
pub const G_SOCKET_CLIENT_CONNECTED: GSocketClientEvent = 3;
pub const G_SOCKET_CLIENT_CONNECTING: GSocketClientEvent = 2;
pub const G_SOCKET_CLIENT_RESOLVED: GSocketClientEvent = 1;
pub const G_SOCKET_CLIENT_RESOLVING: GSocketClientEvent = 0;
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
pub type GNetworkAddress = _GNetworkAddress;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GNetworkService {
    pub parent_instance: GObject,
    pub priv_0: *mut GNetworkServicePrivate,
}
pub type GNetworkServicePrivate = _GNetworkServicePrivate;
pub type GNetworkService = _GNetworkService;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GIOStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GIOStreamPrivate,
}
pub type GIOStreamPrivate = _GIOStreamPrivate;
pub type GIOStream = _GIOStream;
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
pub struct _GSocketClient {
    pub parent_instance: GObject,
    pub priv_0: *mut GSocketClientPrivate,
}
pub type GSocketClientPrivate = _GSocketClientPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketClientPrivate {
    pub family: GSocketFamily,
    pub type_0: GSocketType,
    pub protocol: GSocketProtocol,
    pub local_address: *mut GSocketAddress,
    pub timeout: guint,
    pub enable_proxy: gboolean,
    pub app_proxies: *mut GHashTable,
    pub tls: gboolean,
    pub tls_validation_flags: GTlsCertificateFlags,
    pub proxy_resolver: *mut GProxyResolver,
}
pub type GProxyResolver = _GProxyResolver;
pub type GSocketClient = _GSocketClient;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketConnection {
    pub parent_instance: GIOStream,
    pub priv_0: *mut GSocketConnectionPrivate,
}
pub type GSocketConnectionPrivate = _GSocketConnectionPrivate;
pub type GSocketConnection = _GSocketConnection;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketAddressEnumerator {
    pub parent_instance: GObject,
}
pub type GSocketAddressEnumerator = _GSocketAddressEnumerator;
pub type GSocketConnectable = _GSocketConnectable;
pub type GTask = _GTask;
pub type GTlsClientConnection = _GTlsClientConnection;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTlsConnection {
    pub parent_instance: GIOStream,
    pub priv_0: *mut GTlsConnectionPrivate,
}
pub type GTlsConnectionPrivate = _GTlsConnectionPrivate;
pub type GTlsConnection = _GTlsConnection;
pub type GProxy = _GProxy;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GProxyAddress {
    pub parent_instance: GInetSocketAddress,
    pub priv_0: *mut GProxyAddressPrivate,
}
pub type GProxyAddressPrivate = _GProxyAddressPrivate;
pub type GProxyAddress = _GProxyAddress;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketClientClass {
    pub parent_class: GObjectClass,
    pub event: Option<
        unsafe extern "C" fn(
            *mut GSocketClient,
            GSocketClientEvent,
            *mut GSocketConnectable,
            *mut GIOStream,
        ) -> (),
    >,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
}
pub type GSocketClientClass = _GSocketClientClass;
pub const PROP_PROXY_RESOLVER: C2RustUnnamed_3 = 9;
pub const PROP_TLS_VALIDATION_FLAGS: C2RustUnnamed_3 = 8;
pub const PROP_TLS: C2RustUnnamed_3 = 7;
pub const PROP_ENABLE_PROXY: C2RustUnnamed_3 = 6;
pub const PROP_TIMEOUT: C2RustUnnamed_3 = 5;
pub const PROP_LOCAL_ADDRESS: C2RustUnnamed_3 = 4;
pub const PROP_PROTOCOL: C2RustUnnamed_3 = 3;
pub const PROP_TYPE: C2RustUnnamed_3 = 2;
pub const PROP_FAMILY: C2RustUnnamed_3 = 1;
pub const EVENT: C2RustUnnamed_2 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SocketClientErrorInfo {
    pub tmp_error: *mut GError,
    pub best_error: *mut GError,
    pub best_error_event: GSocketClientEvent,
}
pub const IPPROTO_IP: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GSocketClientAsyncConnectData {
    pub task: *mut GTask,
    pub client: *mut GSocketClient,
    pub connectable: *mut GSocketConnectable,
    pub enumerator: *mut GSocketAddressEnumerator,
    pub enumeration_cancellable: *mut GCancellable,
    pub enumeration_parent_cancellable: *mut GCancellable,
    pub enumeration_cancelled_id: gulong,
    pub connection_attempts: *mut GSList,
    pub successful_connections: *mut GSList,
    pub error_info: *mut SocketClientErrorInfo,
    pub n_addresses_enumerated: guint,
    pub enumeration_completed: gboolean,
    pub connection_in_progress: gboolean,
    pub completed: gboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ConnectionAttempt {
    pub address: *mut GSocketAddress,
    pub socket: *mut GSocket,
    pub connection: *mut GIOStream,
    pub proxy_addr: *mut GProxyAddress,
    pub data: *mut GSocketClientAsyncConnectData,
    pub delay_timeout_source: *mut GSource,
    pub delay_reached: gboolean,
    pub cancellable: *mut GCancellable,
    pub task_cancellable: *mut GCancellable,
    pub cancelled_id: gulong,
    pub ref_0: grefcount,
}
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_1 = 263;
pub const IPPROTO_MPTCP: C2RustUnnamed_1 = 262;
pub const IPPROTO_RAW: C2RustUnnamed_1 = 255;
pub const IPPROTO_ETHERNET: C2RustUnnamed_1 = 143;
pub const IPPROTO_MPLS: C2RustUnnamed_1 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_1 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_1 = 132;
pub const IPPROTO_L2TP: C2RustUnnamed_1 = 115;
pub const IPPROTO_COMP: C2RustUnnamed_1 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_1 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_1 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_1 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_1 = 92;
pub const IPPROTO_AH: C2RustUnnamed_1 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_1 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_1 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_1 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_1 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_1 = 33;
pub const IPPROTO_TP: C2RustUnnamed_1 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_1 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_1 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_1 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_1 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_1 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_1 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_1 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_1 = 1;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const LAST_SIGNAL: C2RustUnnamed_2 = 1;
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const PROP_NONE: C2RustUnnamed_3 = 0;
pub const UINT_MAX: ::core::ffi::c_uint = (__INT_MAX__ as ::core::ffi::c_uint)
    .wrapping_mul(2 as ::core::ffi::c_uint)
    .wrapping_add(1 as ::core::ffi::c_uint);
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXUINT: ::core::ffi::c_uint = UINT_MAX;
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
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_NONE: GType = ((1 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const IP_BIND_ADDRESS_NO_PORT: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const HAPPY_EYEBALLS_CONNECTION_ATTEMPT_DELAY_MS: ::core::ffi::c_int =
    250 as ::core::ffi::c_int;
static mut safe_c2rust_signals: [guint; 1] = [0 as ::core::ffi::c_int as guint];
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_socket_client_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GSocketClient\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GSocketClientClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_socket_client_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GSocketClient>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GSocketClient) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_socket_client_init as unsafe extern "C" fn(*mut GSocketClient) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GSocketClient_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GSocketClientPrivate>() as gsize,
    );
    return g_define_type_id;
}
static mut safe_c2rust_GSocketClient_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_socket_client_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_socket_client_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GSocketClient_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GSocketClient_private_offset,
        );
    }
    safe_c2rust_g_socket_client_class_init(klass as *mut GSocketClientClass);
}
static mut safe_c2rust_g_socket_client_parent_class: gpointer = NULL_0;
#[inline]
unsafe extern "C" fn safe_c2rust_g_socket_client_get_instance_private(
    mut self_0: *mut GSocketClient,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GSocketClient_private_offset as glong as isize) as gpointer;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_client_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_socket_client_get_type_once();
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
unsafe extern "C" fn safe_c2rust_create_socket(
    mut client: *mut GSocketClient,
    mut dest_address: *mut GSocketAddress,
    mut error: *mut *mut GError,
) -> *mut GSocket {
    let mut family: GSocketFamily = G_SOCKET_FAMILY_INVALID;
    let mut socket: *mut GSocket = ::core::ptr::null_mut::<GSocket>();
    family = (*(*client).priv_0).family;
    if family as ::core::ffi::c_uint
        == G_SOCKET_FAMILY_INVALID as ::core::ffi::c_int as ::core::ffi::c_uint
        && !(*(*client).priv_0).local_address.is_null()
    {
        family = g_socket_address_get_family((*(*client).priv_0).local_address);
    }
    if family as ::core::ffi::c_uint
        == G_SOCKET_FAMILY_INVALID as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        family = g_socket_address_get_family(dest_address);
    }
    socket = g_socket_new(
        family,
        (*(*client).priv_0).type_0,
        (*(*client).priv_0).protocol,
        error,
    );
    if socket.is_null() {
        return ::core::ptr::null_mut::<GSocket>();
    }
    if !(*(*client).priv_0).local_address.is_null() {
        g_socket_set_option(
            socket,
            IPPROTO_IP as ::core::ffi::c_int as gint,
            IP_BIND_ADDRESS_NO_PORT,
            1 as gint,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        if g_socket_bind(socket, (*(*client).priv_0).local_address, FALSE, error) == 0 {
            g_object_unref(socket as gpointer);
            return ::core::ptr::null_mut::<GSocket>();
        }
    }
    if (*(*client).priv_0).timeout != 0 {
        g_socket_set_timeout(socket, (*(*client).priv_0).timeout);
    }
    return socket;
}
unsafe extern "C" fn safe_c2rust_can_use_proxy(mut client: *mut GSocketClient) -> gboolean {
    let mut priv_0: *mut GSocketClientPrivate = (*client).priv_0;
    return ((*priv_0).enable_proxy != 0
        && (*priv_0).type_0 as ::core::ffi::c_uint
            == G_SOCKET_TYPE_STREAM as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_clarify_connect_error(
    mut error: *mut GError,
    mut connectable: *mut GSocketConnectable,
    mut address: *mut GSocketAddress,
) {
    let mut name: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut tmp_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ({
        let mut __inst: *mut GTypeInstance = address as *mut GTypeInstance;
        let mut __t: GType = g_proxy_address_get_type();
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
        tmp_name = g_inet_address_to_string(g_inet_socket_address_get_address(
            address as *mut ::core::ffi::c_void as *mut GInetSocketAddress,
        )) as *mut ::core::ffi::c_char;
        name = tmp_name;
        g_prefix_error(
            &raw mut error,
            glib_gettext(b"Could not connect to proxy server %s: \0" as *const u8 as *const gchar),
            name,
        );
    } else {
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
        }) != 0
        {
            name = g_network_address_get_hostname(
                connectable as *mut ::core::ffi::c_void as *mut GNetworkAddress,
            ) as *const ::core::ffi::c_char;
        } else if ({
            let mut __inst: *mut GTypeInstance = connectable as *mut GTypeInstance;
            let mut __t: GType = g_network_service_get_type();
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
            name = g_network_service_get_domain(
                connectable as *mut ::core::ffi::c_void as *mut GNetworkService,
            ) as *const ::core::ffi::c_char;
        } else if ({
            let mut __inst: *mut GTypeInstance = connectable as *mut GTypeInstance;
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
            tmp_name = g_inet_address_to_string(g_inet_socket_address_get_address(
                connectable as *mut ::core::ffi::c_void as *mut GInetSocketAddress,
            )) as *mut ::core::ffi::c_char;
            name = tmp_name;
        } else {
            name = ::core::ptr::null::<::core::ffi::c_char>();
        }
        if !name.is_null() {
            g_prefix_error(
                &raw mut error,
                glib_gettext(b"Could not connect to %s: \0" as *const u8 as *const gchar),
                name,
            );
        } else {
            g_prefix_error(
                &raw mut error,
                glib_gettext(b"Could not connect: \0" as *const u8 as *const gchar),
            );
        }
    }
    g_free(tmp_name as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_socket_client_init(mut client: *mut GSocketClient) {
    (*client).priv_0 =
        safe_c2rust_g_socket_client_get_instance_private(client) as *mut GSocketClientPrivate;
    (*(*client).priv_0).type_0 = G_SOCKET_TYPE_STREAM;
    (*(*client).priv_0).app_proxies = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        None,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_client_new() -> *mut GSocketClient {
    return g_object_new(
        safe_c2rust_g_socket_client_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GSocketClient;
}
unsafe extern "C" fn safe_c2rust_g_socket_client_finalize(mut object: *mut GObject) {
    let mut client: *mut GSocketClient = object as *mut ::core::ffi::c_void as *mut GSocketClient;
    let mut _pp: *mut *mut GSocketAddress = &raw mut (*(*client).priv_0).local_address;
    let mut _ptr: *mut GSocketAddress = *_pp;
    *_pp = ::core::ptr::null_mut::<GSocketAddress>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    let mut _pp_0: *mut *mut GProxyResolver = &raw mut (*(*client).priv_0).proxy_resolver;
    let mut _ptr_0: *mut GProxyResolver = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<GProxyResolver>();
    if !_ptr_0.is_null() {
        g_object_unref(_ptr_0 as gpointer);
    }
    (*(safe_c2rust_g_socket_client_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
    g_hash_table_unref((*(*client).priv_0).app_proxies);
}
unsafe extern "C" fn safe_c2rust_g_socket_client_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut client: *mut GSocketClient = object as *mut ::core::ffi::c_void as *mut GSocketClient;
    match prop_id {
        1 => {
            g_value_set_enum(value, (*(*client).priv_0).family as gint);
        }
        2 => {
            g_value_set_enum(value, (*(*client).priv_0).type_0 as gint);
        }
        3 => {
            g_value_set_enum(value, (*(*client).priv_0).protocol as gint);
        }
        4 => {
            g_value_set_object(value, (*(*client).priv_0).local_address as gpointer);
        }
        5 => {
            g_value_set_uint(value, (*(*client).priv_0).timeout);
        }
        6 => {
            g_value_set_boolean(value, (*(*client).priv_0).enable_proxy);
        }
        7 => {
            g_value_set_boolean(value, safe_c2rust_g_socket_client_get_tls(client));
        }
        8 => {
            g_value_set_flags(
                value,
                safe_c2rust_g_socket_client_get_tls_validation_flags(client) as guint,
            );
        }
        9 => {
            g_value_set_object(
                value,
                safe_c2rust_g_socket_client_get_proxy_resolver(client) as gpointer,
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocketclient.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                303 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_socket_client_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut client: *mut GSocketClient = object as *mut ::core::ffi::c_void as *mut GSocketClient;
    match prop_id {
        1 => {
            safe_c2rust_g_socket_client_set_family(
                client,
                g_value_get_enum(value) as GSocketFamily,
            );
        }
        2 => {
            safe_c2rust_g_socket_client_set_socket_type(
                client,
                g_value_get_enum(value) as GSocketType,
            );
        }
        3 => {
            safe_c2rust_g_socket_client_set_protocol(
                client,
                g_value_get_enum(value) as GSocketProtocol,
            );
        }
        4 => {
            safe_c2rust_g_socket_client_set_local_address(
                client,
                g_value_get_object(value) as *mut GSocketAddress,
            );
        }
        5 => {
            safe_c2rust_g_socket_client_set_timeout(client, g_value_get_uint(value));
        }
        6 => {
            safe_c2rust_g_socket_client_set_enable_proxy(client, g_value_get_boolean(value));
        }
        7 => {
            safe_c2rust_g_socket_client_set_tls(client, g_value_get_boolean(value));
        }
        8 => {
            safe_c2rust_g_socket_client_set_tls_validation_flags(
                client,
                g_value_get_flags(value) as GTlsCertificateFlags,
            );
        }
        9 => {
            safe_c2rust_g_socket_client_set_proxy_resolver(
                client,
                g_value_get_object(value) as *mut GProxyResolver,
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocketclient.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                356 as ::core::ffi::c_int,
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
pub unsafe extern "C" fn safe_c2rust_g_socket_client_get_family(
    mut client: *mut GSocketClient,
) -> GSocketFamily {
    return (*(*client).priv_0).family;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_client_set_family(
    mut client: *mut GSocketClient,
    mut family: GSocketFamily,
) {
    if (*(*client).priv_0).family as ::core::ffi::c_uint == family as ::core::ffi::c_uint {
        return;
    }
    (*(*client).priv_0).family = family;
    g_object_notify(
        client as *mut ::core::ffi::c_void as *mut GObject,
        b"family\0" as *const u8 as *const gchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_client_get_socket_type(
    mut client: *mut GSocketClient,
) -> GSocketType {
    return (*(*client).priv_0).type_0;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_client_set_socket_type(
    mut client: *mut GSocketClient,
    mut type_0: GSocketType,
) {
    if (*(*client).priv_0).type_0 as ::core::ffi::c_uint == type_0 as ::core::ffi::c_uint {
        return;
    }
    (*(*client).priv_0).type_0 = type_0;
    g_object_notify(
        client as *mut ::core::ffi::c_void as *mut GObject,
        b"type\0" as *const u8 as *const gchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_client_get_protocol(
    mut client: *mut GSocketClient,
) -> GSocketProtocol {
    return (*(*client).priv_0).protocol;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_client_set_protocol(
    mut client: *mut GSocketClient,
    mut protocol: GSocketProtocol,
) {
    if (*(*client).priv_0).protocol as ::core::ffi::c_int == protocol as ::core::ffi::c_int {
        return;
    }
    (*(*client).priv_0).protocol = protocol;
    g_object_notify(
        client as *mut ::core::ffi::c_void as *mut GObject,
        b"protocol\0" as *const u8 as *const gchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_client_get_local_address(
    mut client: *mut GSocketClient,
) -> *mut GSocketAddress {
    return (*(*client).priv_0).local_address;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_client_set_local_address(
    mut client: *mut GSocketClient,
    mut address: *mut GSocketAddress,
) {
    if !address.is_null() {
        g_object_ref(address as gpointer);
    }
    if !(*(*client).priv_0).local_address.is_null() {
        g_object_unref((*(*client).priv_0).local_address as gpointer);
    }
    (*(*client).priv_0).local_address = address;
    g_object_notify(
        client as *mut ::core::ffi::c_void as *mut GObject,
        b"local-address\0" as *const u8 as *const gchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_client_get_timeout(
    mut client: *mut GSocketClient,
) -> guint {
    return (*(*client).priv_0).timeout;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_client_set_timeout(
    mut client: *mut GSocketClient,
    mut timeout: guint,
) {
    if (*(*client).priv_0).timeout == timeout {
        return;
    }
    (*(*client).priv_0).timeout = timeout;
    g_object_notify(
        client as *mut ::core::ffi::c_void as *mut GObject,
        b"timeout\0" as *const u8 as *const gchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_client_get_enable_proxy(
    mut client: *mut GSocketClient,
) -> gboolean {
    return (*(*client).priv_0).enable_proxy;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_client_set_enable_proxy(
    mut client: *mut GSocketClient,
    mut enable: gboolean,
) {
    enable = (enable != 0) as ::core::ffi::c_int as gboolean;
    if (*(*client).priv_0).enable_proxy == enable {
        return;
    }
    (*(*client).priv_0).enable_proxy = enable;
    g_object_notify(
        client as *mut ::core::ffi::c_void as *mut GObject,
        b"enable-proxy\0" as *const u8 as *const gchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_client_get_tls(
    mut client: *mut GSocketClient,
) -> gboolean {
    return (*(*client).priv_0).tls;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_client_set_tls(
    mut client: *mut GSocketClient,
    mut tls: gboolean,
) {
    tls = (tls != 0) as ::core::ffi::c_int as gboolean;
    if tls == (*(*client).priv_0).tls {
        return;
    }
    (*(*client).priv_0).tls = tls;
    g_object_notify(
        client as *mut ::core::ffi::c_void as *mut GObject,
        b"tls\0" as *const u8 as *const gchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_client_get_tls_validation_flags(
    mut client: *mut GSocketClient,
) -> GTlsCertificateFlags {
    return (*(*client).priv_0).tls_validation_flags;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_client_set_tls_validation_flags(
    mut client: *mut GSocketClient,
    mut flags: GTlsCertificateFlags,
) {
    if (*(*client).priv_0).tls_validation_flags as ::core::ffi::c_uint
        != flags as ::core::ffi::c_uint
    {
        (*(*client).priv_0).tls_validation_flags = flags;
        g_object_notify(
            client as *mut ::core::ffi::c_void as *mut GObject,
            b"tls-validation-flags\0" as *const u8 as *const gchar,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_client_get_proxy_resolver(
    mut client: *mut GSocketClient,
) -> *mut GProxyResolver {
    if !(*(*client).priv_0).proxy_resolver.is_null() {
        return (*(*client).priv_0).proxy_resolver;
    } else {
        return g_proxy_resolver_get_default();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_client_set_proxy_resolver(
    mut client: *mut GSocketClient,
    mut proxy_resolver: *mut GProxyResolver,
) {
    if !(*(*client).priv_0).proxy_resolver.is_null() {
        g_object_unref((*(*client).priv_0).proxy_resolver as gpointer);
    }
    (*(*client).priv_0).proxy_resolver = proxy_resolver;
    if !(*(*client).priv_0).proxy_resolver.is_null() {
        g_object_ref((*(*client).priv_0).proxy_resolver as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_g_socket_client_class_init(mut class: *mut GSocketClientClass) {
    let mut gobject_class: *mut GObjectClass =
        class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize =
        Some(safe_c2rust_g_socket_client_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_socket_client_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_socket_client_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    safe_c2rust_signals[EVENT as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"event\0" as *const u8 as *const gchar),
        (*(gobject_class as *mut GTypeClass)).g_type,
        G_SIGNAL_RUN_LAST,
        136 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL_0,
        Some(
            _g_cclosure_marshal_VOID__ENUM_OBJECT_OBJECT
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
        3 as guint,
        g_socket_client_event_get_type(),
        g_socket_connectable_get_type(),
        g_io_stream_get_type(),
    );
    g_signal_set_va_marshaller(
        safe_c2rust_signals[EVENT as ::core::ffi::c_int as usize],
        (*(class as *mut GTypeClass)).g_type,
        Some(
            _g_cclosure_marshal_VOID__ENUM_OBJECT_OBJECTv
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
    g_object_class_install_property(
        gobject_class,
        PROP_FAMILY as ::core::ffi::c_int as guint,
        g_param_spec_enum(
            b"family\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_socket_family_get_type(),
            G_SOCKET_FAMILY_INVALID as ::core::ffi::c_int as gint,
            (G_PARAM_CONSTRUCT as ::core::ffi::c_int
                | G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_TYPE as ::core::ffi::c_int as guint,
        g_param_spec_enum(
            b"type\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_socket_type_get_type(),
            G_SOCKET_TYPE_STREAM as ::core::ffi::c_int as gint,
            (G_PARAM_CONSTRUCT as ::core::ffi::c_int
                | G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_PROTOCOL as ::core::ffi::c_int as guint,
        g_param_spec_enum(
            b"protocol\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_socket_protocol_get_type(),
            G_SOCKET_PROTOCOL_DEFAULT as ::core::ffi::c_int as gint,
            (G_PARAM_CONSTRUCT as ::core::ffi::c_int
                | G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_LOCAL_ADDRESS as ::core::ffi::c_int as guint,
        g_param_spec_object(
            b"local-address\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_socket_address_get_type(),
            (G_PARAM_CONSTRUCT as ::core::ffi::c_int
                | G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_TIMEOUT as ::core::ffi::c_int as guint,
        g_param_spec_uint(
            b"timeout\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            0 as guint,
            G_MAXUINT,
            0 as guint,
            (G_PARAM_CONSTRUCT as ::core::ffi::c_int
                | G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_ENABLE_PROXY as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"enable-proxy\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            TRUE,
            (G_PARAM_CONSTRUCT as ::core::ffi::c_int
                | G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_TLS as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"tls\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_CONSTRUCT as ::core::ffi::c_int
                | G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_TLS_VALIDATION_FLAGS as ::core::ffi::c_int as guint,
        g_param_spec_flags(
            b"tls-validation-flags\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_tls_certificate_flags_get_type(),
            G_TLS_CERTIFICATE_VALIDATE_ALL as ::core::ffi::c_int as guint,
            (G_PARAM_CONSTRUCT as ::core::ffi::c_int
                | G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)
                | G_PARAM_DEPRECATED as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_PROXY_RESOLVER as ::core::ffi::c_int as guint,
        g_param_spec_object(
            b"proxy-resolver\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_proxy_resolver_get_type(),
            (G_PARAM_CONSTRUCT as ::core::ffi::c_int
                | G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
unsafe extern "C" fn safe_c2rust_g_socket_client_emit_event(
    mut client: *mut GSocketClient,
    mut event: GSocketClientEvent,
    mut connectable: *mut GSocketConnectable,
    mut connection: *mut GIOStream,
) {
    g_signal_emit(
        client as gpointer,
        safe_c2rust_signals[EVENT as ::core::ffi::c_int as usize],
        0 as GQuark,
        event as ::core::ffi::c_uint,
        connectable,
        connection,
    );
}
unsafe extern "C" fn safe_c2rust_socket_client_error_info_new() -> *mut SocketClientErrorInfo {
    return ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<SocketClientErrorInfo>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut SocketClientErrorInfo;
}
unsafe extern "C" fn safe_c2rust_socket_client_error_info_free(
    mut info: *mut SocketClientErrorInfo,
) {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if (*info).tmp_error.is_null() {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocketclient.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1060 as ::core::ffi::c_int,
            G_STRFUNC,
            b"info->tmp_error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_clear_error(&raw mut (*info).best_error);
    g_free(info as gpointer);
}
unsafe extern "C" fn safe_c2rust_consider_tmp_error(
    mut info: *mut SocketClientErrorInfo,
    mut event: GSocketClientEvent,
) {
    if (*info).tmp_error.is_null() {
        return;
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if event as ::core::ffi::c_uint
            <= G_SOCKET_CLIENT_COMPLETE as ::core::ffi::c_int as ::core::ffi::c_uint
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocketclient.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1080 as ::core::ffi::c_int,
            G_STRFUNC,
            b"event <= G_SOCKET_CLIENT_COMPLETE\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if event as ::core::ffi::c_uint >= (*info).best_error_event as ::core::ffi::c_uint {
        g_clear_error(&raw mut (*info).best_error);
        (*info).best_error = (*info).tmp_error;
        (*info).tmp_error = ::core::ptr::null_mut::<GError>();
        (*info).best_error_event = event;
    } else {
        g_clear_error(&raw mut (*info).tmp_error);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_client_connect(
    mut client: *mut GSocketClient,
    mut connectable: *mut GSocketConnectable,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GSocketConnection {
    let mut connection: *mut GIOStream = ::core::ptr::null_mut::<GIOStream>();
    let mut enumerator: *mut GSocketAddressEnumerator =
        ::core::ptr::null_mut::<GSocketAddressEnumerator>();
    let mut error_info: *mut SocketClientErrorInfo =
        ::core::ptr::null_mut::<SocketClientErrorInfo>();
    let mut ever_resolved: gboolean = FALSE;
    error_info = safe_c2rust_socket_client_error_info_new();
    if safe_c2rust_can_use_proxy(client) != 0 {
        enumerator = g_socket_connectable_proxy_enumerate(connectable);
        if !(*(*client).priv_0).proxy_resolver.is_null()
            && ({
                let mut __inst: *mut GTypeInstance = enumerator as *mut GTypeInstance;
                let mut __t: GType = g_proxy_address_enumerator_get_type();
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
            g_object_set(
                enumerator as *mut ::core::ffi::c_void as *mut GObject as gpointer,
                b"proxy-resolver\0" as *const u8 as *const gchar,
                (*(*client).priv_0).proxy_resolver,
                NULL_0,
            );
        }
    } else {
        enumerator = g_socket_connectable_enumerate(connectable);
    }
    while connection.is_null() {
        let mut address: *mut GSocketAddress = ::core::ptr::null_mut::<GSocketAddress>();
        let mut application_proxy: gboolean = FALSE;
        let mut socket: *mut GSocket = ::core::ptr::null_mut::<GSocket>();
        let mut using_proxy: gboolean = 0;
        if g_cancellable_is_cancelled(cancellable) != 0 {
            g_clear_error(&raw mut (*error_info).best_error);
            g_cancellable_set_error_if_cancelled(cancellable, &raw mut (*error_info).best_error);
            break;
        } else {
            if ever_resolved == 0 {
                safe_c2rust_g_socket_client_emit_event(
                    client,
                    G_SOCKET_CLIENT_RESOLVING,
                    connectable,
                    ::core::ptr::null_mut::<GIOStream>(),
                );
            }
            address = g_socket_address_enumerator_next(
                enumerator,
                cancellable,
                &raw mut (*error_info).tmp_error,
            );
            safe_c2rust_consider_tmp_error(error_info, G_SOCKET_CLIENT_RESOLVING);
            if ever_resolved == 0 {
                safe_c2rust_g_socket_client_emit_event(
                    client,
                    G_SOCKET_CLIENT_RESOLVED,
                    connectable,
                    ::core::ptr::null_mut::<GIOStream>(),
                );
                ever_resolved = TRUE as gboolean;
            }
            if address.is_null() {
                if ({
                    let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
                    if !(&raw mut (*error_info).best_error).is_null() {
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
                        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocketclient.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        1183 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"&error_info->best_error != NULL\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                break;
            } else {
                using_proxy = (({
                    let mut __inst: *mut GTypeInstance = address as *mut GTypeInstance;
                    let mut __t: GType = g_proxy_address_get_type();
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
                    && (*(*client).priv_0).enable_proxy != 0)
                    as ::core::ffi::c_int as gboolean;
                socket =
                    safe_c2rust_create_socket(client, address, &raw mut (*error_info).tmp_error);
                safe_c2rust_consider_tmp_error(error_info, G_SOCKET_CLIENT_CONNECTING);
                if socket.is_null() {
                    g_object_unref(address as gpointer);
                } else {
                    connection =
                        g_socket_connection_factory_create_connection(socket) as *mut GIOStream;
                    g_socket_connection_set_cached_remote_address(
                        connection as *mut GSocketConnection,
                        address,
                    );
                    safe_c2rust_g_socket_client_emit_event(
                        client,
                        G_SOCKET_CLIENT_CONNECTING,
                        connectable,
                        connection,
                    );
                    if g_socket_connection_connect(
                        connection as *mut ::core::ffi::c_void as *mut GSocketConnection,
                        address,
                        cancellable,
                        &raw mut (*error_info).tmp_error,
                    ) != 0
                    {
                        g_socket_connection_set_cached_remote_address(
                            connection as *mut GSocketConnection,
                            ::core::ptr::null_mut::<GSocketAddress>(),
                        );
                        safe_c2rust_g_socket_client_emit_event(
                            client,
                            G_SOCKET_CLIENT_CONNECTED,
                            connectable,
                            connection,
                        );
                    } else {
                        safe_c2rust_clarify_connect_error(
                            (*error_info).tmp_error,
                            connectable,
                            address,
                        );
                        safe_c2rust_consider_tmp_error(error_info, G_SOCKET_CLIENT_CONNECTING);
                        g_object_unref(connection as gpointer);
                        connection = ::core::ptr::null_mut::<GIOStream>();
                    }
                    if !connection.is_null() && using_proxy != 0 {
                        let mut proxy_addr: *mut GProxyAddress =
                            address as *mut ::core::ffi::c_void as *mut GProxyAddress;
                        let mut protocol: *const gchar = ::core::ptr::null::<gchar>();
                        let mut proxy: *mut GProxy = ::core::ptr::null_mut::<GProxy>();
                        protocol = g_proxy_address_get_protocol(proxy_addr);
                        if ({
                            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
                            let mut __t: GType = g_tcp_connection_get_type();
                            let mut __r: gboolean = 0;
                            if __inst.is_null() {
                                __r = FALSE as gboolean;
                            } else if !(*__inst).g_class.is_null()
                                && (*(*__inst).g_class).g_type == __t
                            {
                                __r = TRUE as gboolean;
                            } else {
                                __r = g_type_check_instance_is_a(__inst, __t);
                            }
                            __r
                        }) == 0
                        {
                            g_log(
                                G_LOG_DOMAIN.as_ptr() as *const gchar,
                                G_LOG_LEVEL_CRITICAL,
                                b"Trying to proxy over non-TCP connection, this is most likely a bug in GLib IO library.\0"
                                    as *const u8 as *const gchar,
                            );
                            g_set_error_literal(
                                &raw mut (*error_info).tmp_error,
                                g_io_error_quark(),
                                G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
                                glib_gettext(
                                    b"Proxying over a non-TCP connection is not supported.\0"
                                        as *const u8
                                        as *const gchar,
                                ),
                            );
                            safe_c2rust_consider_tmp_error(
                                error_info,
                                G_SOCKET_CLIENT_PROXY_NEGOTIATING,
                            );
                            g_object_unref(connection as gpointer);
                            connection = ::core::ptr::null_mut::<GIOStream>();
                        } else if g_hash_table_contains(
                            (*(*client).priv_0).app_proxies,
                            protocol as gconstpointer,
                        ) != 0
                        {
                            application_proxy = TRUE as gboolean;
                        } else {
                            proxy = g_proxy_get_default_for_protocol(protocol);
                            if !proxy.is_null() {
                                let mut proxy_connection: *mut GIOStream =
                                    ::core::ptr::null_mut::<GIOStream>();
                                safe_c2rust_g_socket_client_emit_event(
                                    client,
                                    G_SOCKET_CLIENT_PROXY_NEGOTIATING,
                                    connectable,
                                    connection,
                                );
                                proxy_connection = g_proxy_connect(
                                    proxy,
                                    connection,
                                    proxy_addr,
                                    cancellable,
                                    &raw mut (*error_info).tmp_error,
                                );
                                safe_c2rust_consider_tmp_error(
                                    error_info,
                                    G_SOCKET_CLIENT_PROXY_NEGOTIATING,
                                );
                                g_object_unref(connection as gpointer);
                                connection = proxy_connection;
                                g_object_unref(proxy as gpointer);
                                if !connection.is_null() {
                                    safe_c2rust_g_socket_client_emit_event(
                                        client,
                                        G_SOCKET_CLIENT_PROXY_NEGOTIATED,
                                        connectable,
                                        connection,
                                    );
                                }
                            } else {
                                g_set_error(
                                    &raw mut (*error_info).tmp_error,
                                    g_io_error_quark(),
                                    G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
                                    glib_gettext(
                                        b"Proxy protocol \xE2\x80\x9C%s\xE2\x80\x9D is not supported.\0"
                                            as *const u8 as *const gchar,
                                    ),
                                    protocol,
                                );
                                safe_c2rust_consider_tmp_error(
                                    error_info,
                                    G_SOCKET_CLIENT_PROXY_NEGOTIATING,
                                );
                                g_object_unref(connection as gpointer);
                                connection = ::core::ptr::null_mut::<GIOStream>();
                            }
                        }
                    }
                    if application_proxy == 0
                        && !connection.is_null()
                        && (*(*client).priv_0).tls != 0
                    {
                        let mut tlsconn: *mut GIOStream = ::core::ptr::null_mut::<GIOStream>();
                        tlsconn = g_tls_client_connection_new(
                            connection,
                            connectable,
                            &raw mut (*error_info).tmp_error,
                        );
                        g_object_unref(connection as gpointer);
                        connection = tlsconn;
                        if !tlsconn.is_null() {
                            g_tls_client_connection_set_validation_flags(
                                tlsconn as *mut ::core::ffi::c_void as *mut GTlsClientConnection,
                                (*(*client).priv_0).tls_validation_flags,
                            );
                            safe_c2rust_g_socket_client_emit_event(
                                client,
                                G_SOCKET_CLIENT_TLS_HANDSHAKING,
                                connectable,
                                connection,
                            );
                            if g_tls_connection_handshake(
                                tlsconn as *mut ::core::ffi::c_void as *mut GTlsConnection,
                                cancellable,
                                &raw mut (*error_info).tmp_error,
                            ) != 0
                            {
                                safe_c2rust_g_socket_client_emit_event(
                                    client,
                                    G_SOCKET_CLIENT_TLS_HANDSHAKED,
                                    connectable,
                                    connection,
                                );
                            } else {
                                safe_c2rust_consider_tmp_error(
                                    error_info,
                                    G_SOCKET_CLIENT_TLS_HANDSHAKING,
                                );
                                g_object_unref(tlsconn as gpointer);
                                connection = ::core::ptr::null_mut::<GIOStream>();
                            }
                        } else {
                            safe_c2rust_consider_tmp_error(
                                error_info,
                                G_SOCKET_CLIENT_TLS_HANDSHAKING,
                            );
                        }
                    }
                    if !connection.is_null()
                        && ({
                            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
                            let mut __t: GType = g_socket_connection_get_type();
                            let mut __r: gboolean = 0;
                            if __inst.is_null() {
                                __r = FALSE as gboolean;
                            } else if !(*__inst).g_class.is_null()
                                && (*(*__inst).g_class).g_type == __t
                            {
                                __r = TRUE as gboolean;
                            } else {
                                __r = g_type_check_instance_is_a(__inst, __t);
                            }
                            __r
                        }) == 0
                    {
                        let mut wrapper_connection: *mut GSocketConnection =
                            ::core::ptr::null_mut::<GSocketConnection>();
                        wrapper_connection = g_tcp_wrapper_connection_new(connection, socket);
                        g_object_unref(connection as gpointer);
                        connection = wrapper_connection as *mut GIOStream;
                    }
                    g_object_unref(socket as gpointer);
                    g_object_unref(address as gpointer);
                }
            }
        }
    }
    g_object_unref(enumerator as gpointer);
    if connection.is_null() {
        g_propagate_error(
            error,
            safe_c2rust_g_steal_pointer(&raw mut (*error_info).best_error as gpointer)
                as *mut GError,
        );
    }
    safe_c2rust_socket_client_error_info_free(error_info);
    safe_c2rust_g_socket_client_emit_event(
        client,
        G_SOCKET_CLIENT_COMPLETE,
        connectable,
        connection,
    );
    return connection as *mut ::core::ffi::c_void as *mut GSocketConnection;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_client_connect_to_host(
    mut client: *mut GSocketClient,
    mut host_and_port: *const gchar,
    mut default_port: guint16,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GSocketConnection {
    let mut connectable: *mut GSocketConnectable = ::core::ptr::null_mut::<GSocketConnectable>();
    let mut connection: *mut GSocketConnection = ::core::ptr::null_mut::<GSocketConnection>();
    connectable = g_network_address_parse(host_and_port, default_port, error);
    if connectable.is_null() {
        return ::core::ptr::null_mut::<GSocketConnection>();
    }
    connection = safe_c2rust_g_socket_client_connect(client, connectable, cancellable, error);
    g_object_unref(connectable as gpointer);
    return connection;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_client_connect_to_service(
    mut client: *mut GSocketClient,
    mut domain: *const gchar,
    mut service: *const gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GSocketConnection {
    let mut connectable: *mut GSocketConnectable = ::core::ptr::null_mut::<GSocketConnectable>();
    let mut connection: *mut GSocketConnection = ::core::ptr::null_mut::<GSocketConnection>();
    connectable = g_network_service_new(service, b"tcp\0" as *const u8 as *const gchar, domain);
    connection = safe_c2rust_g_socket_client_connect(client, connectable, cancellable, error);
    g_object_unref(connectable as gpointer);
    return connection;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_client_connect_to_uri(
    mut client: *mut GSocketClient,
    mut uri: *const gchar,
    mut default_port: guint16,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GSocketConnection {
    let mut connectable: *mut GSocketConnectable = ::core::ptr::null_mut::<GSocketConnectable>();
    let mut connection: *mut GSocketConnection = ::core::ptr::null_mut::<GSocketConnection>();
    connectable = g_network_address_parse_uri(uri, default_port, error);
    if connectable.is_null() {
        return ::core::ptr::null_mut::<GSocketConnection>();
    }
    connection = safe_c2rust_g_socket_client_connect(client, connectable, cancellable, error);
    g_object_unref(connectable as gpointer);
    return connection;
}
unsafe extern "C" fn safe_c2rust_g_socket_client_async_connect_data_free(
    mut data: *mut GSocketClientAsyncConnectData,
) {
    (*data).task = ::core::ptr::null_mut::<GTask>();
    let mut _pp: *mut *mut GSocketConnectable = &raw mut (*data).connectable;
    let mut _ptr: *mut GSocketConnectable = *_pp;
    *_pp = ::core::ptr::null_mut::<GSocketConnectable>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    let mut _pp_0: *mut *mut GSocketAddressEnumerator = &raw mut (*data).enumerator;
    let mut _ptr_0: *mut GSocketAddressEnumerator = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<GSocketAddressEnumerator>();
    if !_ptr_0.is_null() {
        g_object_unref(_ptr_0 as gpointer);
    }
    g_cancellable_disconnect(
        (*data).enumeration_parent_cancellable,
        (*data).enumeration_cancelled_id,
    );
    let mut _pp_1: *mut *mut GCancellable = &raw mut (*data).enumeration_parent_cancellable;
    let mut _ptr_1: *mut GCancellable = *_pp_1;
    *_pp_1 = ::core::ptr::null_mut::<GCancellable>();
    if !_ptr_1.is_null() {
        g_object_unref(_ptr_1 as gpointer);
    }
    (*data).enumeration_cancelled_id = 0 as gulong;
    let mut _pp_2: *mut *mut GCancellable = &raw mut (*data).enumeration_cancellable;
    let mut _ptr_2: *mut GCancellable = *_pp_2;
    *_pp_2 = ::core::ptr::null_mut::<GCancellable>();
    if !_ptr_2.is_null() {
        g_object_unref(_ptr_2 as gpointer);
    }
    g_slist_free_full(
        (*data).connection_attempts,
        Some(safe_c2rust_connection_attempt_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
    g_slist_free_full(
        (*data).successful_connections,
        Some(safe_c2rust_connection_attempt_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
    let mut _pp_3: *mut *mut SocketClientErrorInfo = &raw mut (*data).error_info;
    let mut _ptr_3: *mut SocketClientErrorInfo = *_pp_3;
    *_pp_3 = ::core::ptr::null_mut::<SocketClientErrorInfo>();
    if !_ptr_3.is_null() {
        safe_c2rust_socket_client_error_info_free(_ptr_3 as *mut SocketClientErrorInfo);
    }
    g_slice_free1(
        ::core::mem::size_of::<GSocketClientAsyncConnectData>() as gsize,
        data as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_connection_attempt_new() -> *mut ConnectionAttempt {
    let mut attempt: *mut ConnectionAttempt = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<ConnectionAttempt>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut ConnectionAttempt;
    g_ref_count_init(&raw mut (*attempt).ref_0);
    return attempt;
}
unsafe extern "C" fn safe_c2rust_connection_attempt_ref(
    mut attempt: *mut ConnectionAttempt,
) -> *mut ConnectionAttempt {
    g_ref_count_inc(&raw mut (*attempt).ref_0);
    return attempt;
}
unsafe extern "C" fn safe_c2rust_connection_attempt_unref(mut pointer: gpointer) {
    let mut attempt: *mut ConnectionAttempt = pointer as *mut ConnectionAttempt;
    if g_ref_count_dec(&raw mut (*attempt).ref_0) != 0 {
        let mut _pp: *mut *mut GSocketAddress = &raw mut (*attempt).address;
        let mut _ptr: *mut GSocketAddress = *_pp;
        *_pp = ::core::ptr::null_mut::<GSocketAddress>();
        if !_ptr.is_null() {
            g_object_unref(_ptr as gpointer);
        }
        let mut _pp_0: *mut *mut GSocket = &raw mut (*attempt).socket;
        let mut _ptr_0: *mut GSocket = *_pp_0;
        *_pp_0 = ::core::ptr::null_mut::<GSocket>();
        if !_ptr_0.is_null() {
            g_object_unref(_ptr_0 as gpointer);
        }
        let mut _pp_1: *mut *mut GIOStream = &raw mut (*attempt).connection;
        let mut _ptr_1: *mut GIOStream = *_pp_1;
        *_pp_1 = ::core::ptr::null_mut::<GIOStream>();
        if !_ptr_1.is_null() {
            g_object_unref(_ptr_1 as gpointer);
        }
        g_cancellable_disconnect((*attempt).task_cancellable, (*attempt).cancelled_id);
        let mut _pp_2: *mut *mut GCancellable = &raw mut (*attempt).task_cancellable;
        let mut _ptr_2: *mut GCancellable = *_pp_2;
        *_pp_2 = ::core::ptr::null_mut::<GCancellable>();
        if !_ptr_2.is_null() {
            g_object_unref(_ptr_2 as gpointer);
        }
        (*attempt).cancelled_id = 0 as gulong;
        let mut _pp_3: *mut *mut GCancellable = &raw mut (*attempt).cancellable;
        let mut _ptr_3: *mut GCancellable = *_pp_3;
        *_pp_3 = ::core::ptr::null_mut::<GCancellable>();
        if !_ptr_3.is_null() {
            g_object_unref(_ptr_3 as gpointer);
        }
        let mut _pp_4: *mut *mut GProxyAddress = &raw mut (*attempt).proxy_addr;
        let mut _ptr_4: *mut GProxyAddress = *_pp_4;
        *_pp_4 = ::core::ptr::null_mut::<GProxyAddress>();
        if !_ptr_4.is_null() {
            g_object_unref(_ptr_4 as gpointer);
        }
        if !(*attempt).delay_timeout_source.is_null() {
            g_source_destroy((*attempt).delay_timeout_source);
            g_source_unref((*attempt).delay_timeout_source);
        }
        g_free(attempt as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_connection_attempt_remove(mut attempt: *mut ConnectionAttempt) {
    let mut attempt_link: *mut GSList = g_slist_find(
        (*(*attempt).data).connection_attempts,
        attempt as gconstpointer,
    );
    if !attempt_link.is_null() {
        (*(*attempt).data).connection_attempts =
            g_slist_delete_link((*(*attempt).data).connection_attempts, attempt_link);
        safe_c2rust_connection_attempt_unref(attempt as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_cancel_all_attempts(mut data: *mut GSocketClientAsyncConnectData) {
    let mut l: *mut GSList = ::core::ptr::null_mut::<GSList>();
    l = (*data).connection_attempts;
    while !l.is_null() {
        let mut attempt_entry: *mut ConnectionAttempt = (*l).data as *mut ConnectionAttempt;
        g_cancellable_cancel((*attempt_entry).cancellable);
        safe_c2rust_connection_attempt_unref(attempt_entry as gpointer);
        l = if !l.is_null() {
            (*l).next
        } else {
            ::core::ptr::null_mut::<GSList>()
        };
    }
    g_slist_free((*data).connection_attempts);
    (*data).connection_attempts = ::core::ptr::null_mut::<GSList>();
    g_slist_free_full(
        (*data).successful_connections,
        Some(safe_c2rust_connection_attempt_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
    (*data).successful_connections = ::core::ptr::null_mut::<GSList>();
    g_cancellable_cancel((*data).enumeration_cancellable);
}
unsafe extern "C" fn safe_c2rust_g_socket_client_async_connect_complete(
    mut attempt: *mut ConnectionAttempt,
) {
    let mut data: *mut GSocketClientAsyncConnectData = (*attempt).data;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !(*attempt).connection.is_null() {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocketclient.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1626 as ::core::ffi::c_int,
            G_STRFUNC,
            b"attempt->connection\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if (*data).completed == 0 {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocketclient.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1627 as ::core::ffi::c_int,
            G_STRFUNC,
            b"!data->completed\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut __inst: *mut GTypeInstance = (*attempt).connection as *mut GTypeInstance;
        let mut __t: GType = g_socket_connection_get_type();
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
        let mut wrapper_connection: *mut GSocketConnection =
            ::core::ptr::null_mut::<GSocketConnection>();
        wrapper_connection = g_tcp_wrapper_connection_new((*attempt).connection, (*attempt).socket);
        g_object_unref((*attempt).connection as gpointer);
        (*attempt).connection = wrapper_connection as *mut GIOStream;
    }
    (*data).completed = TRUE as gboolean;
    safe_c2rust_cancel_all_attempts(data);
    if g_cancellable_set_error_if_cancelled(g_task_get_cancellable((*data).task), &raw mut error)
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_DEBUG,
            b"GSocketClient: Connection cancelled!\0" as *const u8 as *const gchar,
        );
        safe_c2rust_g_socket_client_emit_event(
            (*data).client,
            G_SOCKET_CLIENT_COMPLETE,
            (*data).connectable,
            ::core::ptr::null_mut::<GIOStream>(),
        );
        g_task_return_error(
            (*data).task,
            safe_c2rust_g_steal_pointer(&raw mut error as gpointer) as *mut GError,
        );
    } else {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_DEBUG,
            b"GSocketClient: Connection successful!\0" as *const u8 as *const gchar,
        );
        safe_c2rust_g_socket_client_emit_event(
            (*data).client,
            G_SOCKET_CLIENT_COMPLETE,
            (*data).connectable,
            (*attempt).connection,
        );
        g_task_return_pointer(
            (*data).task,
            safe_c2rust_g_steal_pointer(&raw mut (*attempt).connection as gpointer)
                as *mut GIOStream as gpointer,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    }
    safe_c2rust_connection_attempt_unref(attempt as gpointer);
    g_object_unref((*data).task as gpointer);
}
unsafe extern "C" fn safe_c2rust_enumerator_next_async(
    mut data: *mut GSocketClientAsyncConnectData,
    mut add_task_ref: gboolean,
) {
    if add_task_ref != 0 {
        g_object_ref((*data).task as gpointer);
    }
    if (*data).n_addresses_enumerated == 0 as guint {
        safe_c2rust_g_socket_client_emit_event(
            (*data).client,
            G_SOCKET_CLIENT_RESOLVING,
            (*data).connectable,
            ::core::ptr::null_mut::<GIOStream>(),
        );
    }
    g_log(
        G_LOG_DOMAIN.as_ptr() as *const gchar,
        G_LOG_LEVEL_DEBUG,
        b"GSocketClient: Starting new address enumeration\0" as *const u8 as *const gchar,
    );
    g_socket_address_enumerator_next_async(
        (*data).enumerator,
        (*data).enumeration_cancellable,
        Some(
            safe_c2rust_g_socket_client_enumerator_callback
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        data as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_g_socket_client_tls_handshake_callback(
    mut object: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut attempt: *mut ConnectionAttempt = user_data as *mut ConnectionAttempt;
    let mut data: *mut GSocketClientAsyncConnectData = (*attempt).data;
    if g_tls_connection_handshake_finish(
        object as *mut ::core::ffi::c_void as *mut GTlsConnection,
        result,
        &raw mut (*(*data).error_info).tmp_error,
    ) != 0
    {
        g_object_unref((*attempt).connection as gpointer);
        (*attempt).connection = object as *mut ::core::ffi::c_void as *mut GIOStream;
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_DEBUG,
            b"GSocketClient: TLS handshake succeeded\0" as *const u8 as *const gchar,
        );
        safe_c2rust_g_socket_client_emit_event(
            (*data).client,
            G_SOCKET_CLIENT_TLS_HANDSHAKED,
            (*data).connectable,
            (*attempt).connection,
        );
        safe_c2rust_g_socket_client_async_connect_complete(attempt);
    } else {
        g_object_unref(object as gpointer);
        safe_c2rust_connection_attempt_unref(attempt as gpointer);
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_DEBUG,
            b"GSocketClient: TLS handshake failed: %s\0" as *const u8 as *const gchar,
            (*(*(*data).error_info).tmp_error).message,
        );
        safe_c2rust_consider_tmp_error((*data).error_info, G_SOCKET_CLIENT_TLS_HANDSHAKING);
        safe_c2rust_try_next_connection_or_finish(data, TRUE);
    };
}
unsafe extern "C" fn safe_c2rust_g_socket_client_tls_handshake(
    mut attempt: *mut ConnectionAttempt,
) {
    let mut data: *mut GSocketClientAsyncConnectData = (*attempt).data;
    let mut tlsconn: *mut GIOStream = ::core::ptr::null_mut::<GIOStream>();
    if (*(*(*data).client).priv_0).tls == 0 {
        safe_c2rust_g_socket_client_async_connect_complete(attempt);
        return;
    }
    g_log(
        G_LOG_DOMAIN.as_ptr() as *const gchar,
        G_LOG_LEVEL_DEBUG,
        b"GSocketClient: Starting TLS handshake\0" as *const u8 as *const gchar,
    );
    tlsconn = g_tls_client_connection_new(
        (*attempt).connection,
        (*data).connectable,
        &raw mut (*(*data).error_info).tmp_error,
    );
    if !tlsconn.is_null() {
        g_tls_client_connection_set_validation_flags(
            tlsconn as *mut ::core::ffi::c_void as *mut GTlsClientConnection,
            (*(*(*data).client).priv_0).tls_validation_flags,
        );
        safe_c2rust_g_socket_client_emit_event(
            (*data).client,
            G_SOCKET_CLIENT_TLS_HANDSHAKING,
            (*data).connectable,
            tlsconn as *mut ::core::ffi::c_void as *mut GIOStream,
        );
        g_tls_connection_handshake_async(
            tlsconn as *mut ::core::ffi::c_void as *mut GTlsConnection,
            G_PRIORITY_DEFAULT,
            g_task_get_cancellable((*data).task),
            Some(
                safe_c2rust_g_socket_client_tls_handshake_callback
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            attempt as gpointer,
        );
    } else {
        safe_c2rust_connection_attempt_unref(attempt as gpointer);
        safe_c2rust_consider_tmp_error((*data).error_info, G_SOCKET_CLIENT_TLS_HANDSHAKING);
        safe_c2rust_try_next_connection_or_finish(data, TRUE);
    };
}
unsafe extern "C" fn safe_c2rust_g_socket_client_proxy_connect_callback(
    mut object: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut attempt: *mut ConnectionAttempt = user_data as *mut ConnectionAttempt;
    let mut data: *mut GSocketClientAsyncConnectData = (*attempt).data;
    g_object_unref((*attempt).connection as gpointer);
    (*attempt).connection = g_proxy_connect_finish(
        object as *mut ::core::ffi::c_void as *mut GProxy,
        result,
        &raw mut (*(*data).error_info).tmp_error,
    );
    if !(*attempt).connection.is_null() {
        safe_c2rust_g_socket_client_emit_event(
            (*data).client,
            G_SOCKET_CLIENT_PROXY_NEGOTIATED,
            (*data).connectable,
            (*attempt).connection,
        );
        safe_c2rust_g_socket_client_tls_handshake(attempt);
    } else {
        safe_c2rust_connection_attempt_unref(attempt as gpointer);
        safe_c2rust_consider_tmp_error((*data).error_info, G_SOCKET_CLIENT_PROXY_NEGOTIATING);
        safe_c2rust_try_next_connection_or_finish(data, TRUE);
    };
}
unsafe extern "C" fn safe_c2rust_complete_connection_with_error(
    mut data: *mut GSocketClientAsyncConnectData,
    mut error: *mut GError,
) {
    g_log(
        G_LOG_DOMAIN.as_ptr() as *const gchar,
        G_LOG_LEVEL_DEBUG,
        b"GSocketClient: Connection failed: %s\0" as *const u8 as *const gchar,
        (*error).message,
    );
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if (*data).completed == 0 {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocketclient.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1787 as ::core::ffi::c_int,
            G_STRFUNC,
            b"!data->completed\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    safe_c2rust_g_socket_client_emit_event(
        (*data).client,
        G_SOCKET_CLIENT_COMPLETE,
        (*data).connectable,
        ::core::ptr::null_mut::<GIOStream>(),
    );
    (*data).completed = TRUE as gboolean;
    safe_c2rust_cancel_all_attempts(data);
    g_task_return_error((*data).task, error);
}
unsafe extern "C" fn safe_c2rust_task_completed_or_cancelled(
    mut data: *mut GSocketClientAsyncConnectData,
) -> gboolean {
    let mut task: *mut GTask = (*data).task;
    let mut cancellable: *mut GCancellable = g_task_get_cancellable(task);
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if (*data).completed != 0 {
        return TRUE;
    } else if g_cancellable_set_error_if_cancelled(cancellable, &raw mut error) != 0 {
        safe_c2rust_complete_connection_with_error(
            data,
            safe_c2rust_g_steal_pointer(&raw mut error as gpointer) as *mut GError,
        );
        return TRUE;
    } else {
        return FALSE;
    };
}
unsafe extern "C" fn safe_c2rust_try_next_successful_connection(
    mut data: *mut GSocketClientAsyncConnectData,
) -> gboolean {
    let mut attempt: *mut ConnectionAttempt = ::core::ptr::null_mut::<ConnectionAttempt>();
    let mut protocol: *const gchar = ::core::ptr::null::<gchar>();
    let mut proxy: *mut GProxy = ::core::ptr::null_mut::<GProxy>();
    if (*data).connection_in_progress != 0 {
        return FALSE;
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !(*data).successful_connections.is_null() {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocketclient.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1825 as ::core::ffi::c_int,
            G_STRFUNC,
            b"data->successful_connections != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    attempt = (*(*data).successful_connections).data as *mut ConnectionAttempt;
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !attempt.is_null() {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocketclient.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1827 as ::core::ffi::c_int,
            G_STRFUNC,
            b"attempt != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*data).successful_connections =
        g_slist_remove((*data).successful_connections, attempt as gconstpointer);
    (*data).connection_in_progress = TRUE as gboolean;
    g_log(
        G_LOG_DOMAIN.as_ptr() as *const gchar,
        G_LOG_LEVEL_DEBUG,
        b"GSocketClient: Starting application layer connection\0" as *const u8 as *const gchar,
    );
    if (*attempt).proxy_addr.is_null() {
        safe_c2rust_g_socket_client_tls_handshake(safe_c2rust_g_steal_pointer(
            &raw mut attempt as gpointer,
        ) as *mut ConnectionAttempt);
        return TRUE;
    }
    protocol = g_proxy_address_get_protocol((*attempt).proxy_addr);
    if ({
        let mut __inst: *mut GTypeInstance = (*attempt).connection as *mut GTypeInstance;
        let mut __t: GType = g_tcp_connection_get_type();
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
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"Trying to proxy over non-TCP connection, this is most likely a bug in GLib IO library.\0"
                as *const u8 as *const gchar,
        );
        g_set_error_literal(
            &raw mut (*(*data).error_info).tmp_error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Proxying over a non-TCP connection is not supported.\0" as *const u8
                    as *const gchar,
            ),
        );
        safe_c2rust_consider_tmp_error((*data).error_info, G_SOCKET_CLIENT_PROXY_NEGOTIATING);
    } else if g_hash_table_contains(
        (*(*(*data).client).priv_0).app_proxies,
        protocol as gconstpointer,
    ) != 0
    {
        safe_c2rust_g_socket_client_async_connect_complete(safe_c2rust_g_steal_pointer(
            &raw mut attempt as gpointer,
        ) as *mut ConnectionAttempt);
        return TRUE;
    } else {
        proxy = g_proxy_get_default_for_protocol(protocol);
        if !proxy.is_null() {
            let mut connection: *mut GIOStream = (*attempt).connection;
            let mut proxy_addr: *mut GProxyAddress = (*attempt).proxy_addr;
            safe_c2rust_g_socket_client_emit_event(
                (*data).client,
                G_SOCKET_CLIENT_PROXY_NEGOTIATING,
                (*data).connectable,
                (*attempt).connection,
            );
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_DEBUG,
                b"GSocketClient: Starting proxy connection\0" as *const u8 as *const gchar,
            );
            g_proxy_connect_async(
                proxy,
                connection,
                proxy_addr,
                g_task_get_cancellable((*data).task),
                Some(
                    safe_c2rust_g_socket_client_proxy_connect_callback
                        as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
                ),
                safe_c2rust_g_steal_pointer(&raw mut attempt as gpointer) as *mut ConnectionAttempt
                    as gpointer,
            );
            g_object_unref(proxy as gpointer);
            return TRUE;
        } else {
            g_set_error(
                &raw mut (*(*data).error_info).tmp_error,
                g_io_error_quark(),
                G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Proxy protocol \xE2\x80\x9C%s\xE2\x80\x9D is not supported.\0" as *const u8
                        as *const gchar,
                ),
                protocol,
            );
            safe_c2rust_consider_tmp_error((*data).error_info, G_SOCKET_CLIENT_PROXY_NEGOTIATING);
        }
    }
    (*data).connection_in_progress = FALSE as gboolean;
    let mut _pp: *mut *mut ConnectionAttempt = &raw mut attempt;
    let mut _ptr: *mut ConnectionAttempt = *_pp;
    *_pp = ::core::ptr::null_mut::<ConnectionAttempt>();
    if !_ptr.is_null() {
        safe_c2rust_connection_attempt_unref(_ptr as gpointer);
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_try_next_connection_or_finish(
    mut data: *mut GSocketClientAsyncConnectData,
    mut end_current_connection: gboolean,
) {
    if end_current_connection != 0 {
        (*data).connection_in_progress = FALSE as gboolean;
    }
    if (*data).connection_in_progress != 0 {
        return;
    }
    while !(*data).successful_connections.is_null() {
        if safe_c2rust_try_next_successful_connection(data) != 0 {
            return;
        }
    }
    if (*data).enumeration_completed == 0 {
        safe_c2rust_enumerator_next_async(data, FALSE);
        return;
    }
    safe_c2rust_complete_connection_with_error(
        data,
        safe_c2rust_g_steal_pointer(&raw mut (*(*data).error_info).best_error as gpointer)
            as *mut GError,
    );
}
unsafe extern "C" fn safe_c2rust_g_socket_client_connected_callback(
    mut source: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut attempt: *mut ConnectionAttempt =
        safe_c2rust_g_steal_pointer(&raw mut user_data as gpointer) as *mut ConnectionAttempt;
    let mut data: *mut GSocketClientAsyncConnectData = (*attempt).data;
    if safe_c2rust_task_completed_or_cancelled(data) != 0
        || g_cancellable_is_cancelled((*attempt).cancellable) != 0
    {
        safe_c2rust_connection_attempt_remove(attempt);
        safe_c2rust_connection_attempt_unref(attempt as gpointer);
        g_object_unref((*data).task as gpointer);
        return;
    }
    if !(*attempt).delay_timeout_source.is_null() {
        g_source_destroy((*attempt).delay_timeout_source);
        let mut _pp: *mut *mut GSource = &raw mut (*attempt).delay_timeout_source;
        let mut _ptr: *mut GSource = *_pp;
        *_pp = ::core::ptr::null_mut::<GSource>();
        if !_ptr.is_null() {
            g_source_unref(_ptr as *mut GSource);
        }
    }
    if g_socket_connection_connect_finish(
        source as *mut ::core::ffi::c_void as *mut GSocketConnection,
        result,
        &raw mut (*(*data).error_info).tmp_error,
    ) == 0
    {
        if g_cancellable_is_cancelled((*attempt).cancellable) == 0 {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_DEBUG,
                b"GSocketClient: Connection attempt failed: %s\0" as *const u8 as *const gchar,
                (*(*(*data).error_info).tmp_error).message,
            );
            safe_c2rust_clarify_connect_error(
                (*(*data).error_info).tmp_error,
                (*data).connectable,
                (*attempt).address,
            );
            safe_c2rust_consider_tmp_error((*data).error_info, G_SOCKET_CLIENT_CONNECTING);
            safe_c2rust_connection_attempt_remove(attempt);
            safe_c2rust_connection_attempt_unref(attempt as gpointer);
            safe_c2rust_try_next_connection_or_finish(data, FALSE);
        } else {
            g_clear_error(&raw mut (*(*data).error_info).tmp_error);
            g_object_unref((*data).task as gpointer);
            safe_c2rust_connection_attempt_unref(attempt as gpointer);
        }
        return;
    }
    g_socket_connection_set_cached_remote_address(
        (*attempt).connection as *mut GSocketConnection,
        ::core::ptr::null_mut::<GSocketAddress>(),
    );
    g_log(
        G_LOG_DOMAIN.as_ptr() as *const gchar,
        G_LOG_LEVEL_DEBUG,
        b"GSocketClient: TCP connection successful\0" as *const u8 as *const gchar,
    );
    safe_c2rust_g_socket_client_emit_event(
        (*data).client,
        G_SOCKET_CLIENT_CONNECTED,
        (*data).connectable,
        (*attempt).connection,
    );
    g_socket_set_blocking((*attempt).socket, TRUE);
    safe_c2rust_connection_attempt_remove(attempt);
    (*data).successful_connections = g_slist_append(
        (*data).successful_connections,
        safe_c2rust_g_steal_pointer(&raw mut attempt as gpointer) as *mut ConnectionAttempt
            as gpointer,
    );
    safe_c2rust_try_next_connection_or_finish(data, FALSE);
}
unsafe extern "C" fn safe_c2rust_on_connection_attempt_delay_reached(
    mut data: gpointer,
) -> gboolean {
    let mut attempt: *mut ConnectionAttempt = data as *mut ConnectionAttempt;
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if (*attempt).delay_reached == 0 {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocketclient.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1990 as ::core::ffi::c_int,
            G_STRFUNC,
            b"!attempt->delay_reached\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*attempt).delay_reached = TRUE as gboolean;
    if (*(*attempt).data).enumeration_completed == 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_DEBUG,
            b"GSocketClient: Connection attempt delay reached, trying another enumeration\0"
                as *const u8 as *const gchar,
        );
        safe_c2rust_enumerator_next_async((*attempt).data, TRUE);
    }
    let mut _pp: *mut *mut GSource = &raw mut (*attempt).delay_timeout_source;
    let mut _ptr: *mut GSource = *_pp;
    *_pp = ::core::ptr::null_mut::<GSource>();
    if !_ptr.is_null() {
        g_source_unref(_ptr as *mut GSource);
    }
    return G_SOURCE_REMOVE;
}
unsafe extern "C" fn safe_c2rust_on_connection_cancelled(
    mut cancellable: *mut GCancellable,
    mut data: gpointer,
) {
    let mut linked_cancellable: *mut GCancellable = data as *mut GCancellable;
    g_cancellable_cancel(linked_cancellable);
}
unsafe extern "C" fn safe_c2rust_g_socket_client_enumerator_callback(
    mut object: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut data: *mut GSocketClientAsyncConnectData =
        user_data as *mut GSocketClientAsyncConnectData;
    let mut address: *mut GSocketAddress = ::core::ptr::null_mut::<GSocketAddress>();
    let mut socket: *mut GSocket = ::core::ptr::null_mut::<GSocket>();
    let mut attempt: *mut ConnectionAttempt = ::core::ptr::null_mut::<ConnectionAttempt>();
    if safe_c2rust_task_completed_or_cancelled(data) != 0 {
        g_object_unref((*data).task as gpointer);
        return;
    }
    address = g_socket_address_enumerator_next_finish(
        (*data).enumerator,
        result,
        &raw mut (*(*data).error_info).tmp_error,
    );
    if address.is_null() {
        if ({
            let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
            if (*data).enumeration_completed != 0 {
                _g_boolean_var_19 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_19 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_19
        }) as ::core::ffi::c_long
            != 0
        {
            return;
        }
        (*data).enumeration_completed = TRUE as gboolean;
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_DEBUG,
            b"GSocketClient: Address enumeration completed (out of addresses)\0" as *const u8
                as *const gchar,
        );
        if (*data).n_addresses_enumerated > 0 as guint
            && (*data).connection_attempts.is_null()
            && (*data).connection_in_progress == 0
            || (*data).n_addresses_enumerated == 0 as guint
        {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_DEBUG,
                b"GSocketClient: Address enumeration failed: %s\0" as *const u8 as *const gchar,
                if !(*(*data).error_info).tmp_error.is_null() {
                    (*(*(*data).error_info).tmp_error).message
                } else {
                    ::core::ptr::null_mut::<gchar>()
                },
            );
            safe_c2rust_consider_tmp_error((*data).error_info, G_SOCKET_CLIENT_RESOLVING);
            if ({
                let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
                if !(*(*data).error_info).best_error.is_null() {
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
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocketclient.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    2052 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"data->error_info->best_error\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            safe_c2rust_complete_connection_with_error(
                data,
                safe_c2rust_g_steal_pointer(&raw mut (*(*data).error_info).best_error as gpointer)
                    as *mut GError,
            );
        }
        g_object_unref((*data).task as gpointer);
        return;
    }
    g_log(
        G_LOG_DOMAIN.as_ptr() as *const gchar,
        G_LOG_LEVEL_DEBUG,
        b"GSocketClient: Address enumeration succeeded\0" as *const u8 as *const gchar,
    );
    if (*data).n_addresses_enumerated == 0 as guint {
        safe_c2rust_g_socket_client_emit_event(
            (*data).client,
            G_SOCKET_CLIENT_RESOLVED,
            (*data).connectable,
            ::core::ptr::null_mut::<GIOStream>(),
        );
    }
    (*data).n_addresses_enumerated = (*data).n_addresses_enumerated.wrapping_add(1);
    socket = safe_c2rust_create_socket(
        (*data).client,
        address,
        &raw mut (*(*data).error_info).tmp_error,
    );
    if socket.is_null() {
        g_object_unref(address as gpointer);
        safe_c2rust_consider_tmp_error((*data).error_info, G_SOCKET_CLIENT_CONNECTING);
        safe_c2rust_enumerator_next_async(data, FALSE);
        return;
    }
    attempt = safe_c2rust_connection_attempt_new();
    (*attempt).data = data;
    (*attempt).socket = socket;
    (*attempt).address = address;
    (*attempt).cancellable = g_cancellable_new();
    (*attempt).connection = g_socket_connection_factory_create_connection(socket) as *mut GIOStream;
    (*attempt).delay_timeout_source =
        g_timeout_source_new(HAPPY_EYEBALLS_CONNECTION_ATTEMPT_DELAY_MS as guint);
    g_log(
        G_LOG_DOMAIN.as_ptr() as *const gchar,
        G_LOG_LEVEL_DEBUG,
        b"%s: starting connection attempt %p for GSocketClientAsyncConnectData %p\0" as *const u8
            as *const gchar,
        b"g_socket_client_enumerator_callback\0" as *const u8 as *const ::core::ffi::c_char,
        attempt,
        data,
    );
    if ({
        let mut __inst: *mut GTypeInstance = address as *mut GTypeInstance;
        let mut __t: GType = g_proxy_address_get_type();
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
        && (*(*(*data).client).priv_0).enable_proxy != 0
    {
        (*attempt).proxy_addr =
            g_object_ref(address as *mut ::core::ffi::c_void as *mut GProxyAddress as gpointer)
                as *mut GProxyAddress as *mut GProxyAddress;
    }
    g_source_set_callback(
        (*attempt).delay_timeout_source,
        Some(
            safe_c2rust_on_connection_attempt_delay_reached
                as unsafe extern "C" fn(gpointer) -> gboolean,
        ),
        attempt as gpointer,
        None,
    );
    g_source_attach(
        (*attempt).delay_timeout_source,
        g_task_get_context((*data).task),
    );
    (*data).connection_attempts = g_slist_append(
        (*data).connection_attempts,
        safe_c2rust_connection_attempt_ref(attempt) as gpointer,
    );
    if !g_task_get_cancellable((*data).task).is_null() {
        (*attempt).task_cancellable = g_object_ref(g_task_get_cancellable((*data).task) as gpointer)
            as *mut GCancellable as *mut GCancellable;
        (*attempt).cancelled_id = g_cancellable_connect(
            (*attempt).task_cancellable,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GCancellable, gpointer) -> ()>,
                GCallback,
            >(Some(
                safe_c2rust_on_connection_cancelled
                    as unsafe extern "C" fn(*mut GCancellable, gpointer) -> (),
            )),
            g_object_ref((*attempt).cancellable as gpointer) as *mut GCancellable as gpointer,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    }
    g_socket_connection_set_cached_remote_address(
        (*attempt).connection as *mut GSocketConnection,
        address,
    );
    g_log(
        G_LOG_DOMAIN.as_ptr() as *const gchar,
        G_LOG_LEVEL_DEBUG,
        b"GSocketClient: Starting TCP connection attempt\0" as *const u8 as *const gchar,
    );
    safe_c2rust_g_socket_client_emit_event(
        (*data).client,
        G_SOCKET_CLIENT_CONNECTING,
        (*data).connectable,
        (*attempt).connection,
    );
    g_socket_connection_connect_async(
        (*attempt).connection as *mut ::core::ffi::c_void as *mut GSocketConnection,
        address,
        (*attempt).cancellable,
        Some(
            safe_c2rust_g_socket_client_connected_callback
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        attempt as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_client_connect_async(
    mut client: *mut GSocketClient,
    mut connectable: *mut GSocketConnectable,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut data: *mut GSocketClientAsyncConnectData =
        ::core::ptr::null_mut::<GSocketClientAsyncConnectData>();
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = client as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_client_get_type();
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
            b"G_IS_SOCKET_CLIENT (client)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    data = ({
        let mut __s: gsize = ::core::mem::size_of::<GSocketClientAsyncConnectData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut GSocketClientAsyncConnectData;
    (*data).client = client;
    (*data).connectable =
        g_object_ref(connectable as gpointer) as *mut GSocketConnectable as *mut GSocketConnectable;
    (*data).error_info = safe_c2rust_socket_client_error_info_new();
    if safe_c2rust_can_use_proxy(client) != 0 {
        (*data).enumerator = g_socket_connectable_proxy_enumerate(connectable);
        if !(*(*client).priv_0).proxy_resolver.is_null()
            && ({
                let mut __inst: *mut GTypeInstance = (*data).enumerator as *mut GTypeInstance;
                let mut __t: GType = g_proxy_address_enumerator_get_type();
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
            g_object_set(
                (*data).enumerator as *mut ::core::ffi::c_void as *mut GObject as gpointer,
                b"proxy-resolver\0" as *const u8 as *const gchar,
                (*(*client).priv_0).proxy_resolver,
                NULL_0,
            );
        }
    } else {
        (*data).enumerator = g_socket_connectable_enumerate(connectable);
    }
    (*data).task = g_task_new(client as gpointer, cancellable, callback, user_data);
    g_task_set_check_cancellable((*data).task, FALSE);
    let mut _task: *mut GTask = (*data).task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GSocketClient,
                    *mut GSocketConnectable,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_socket_client_connect_async
                as unsafe extern "C" fn(
                    *mut GSocketClient,
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
            b"g_socket_client_connect_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        (*data).task,
        data as gpointer,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GSocketClientAsyncConnectData) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_g_socket_client_async_connect_data_free
                as unsafe extern "C" fn(*mut GSocketClientAsyncConnectData) -> (),
        )),
    );
    (*data).enumeration_cancellable = g_cancellable_new();
    if !cancellable.is_null() {
        (*data).enumeration_parent_cancellable =
            g_object_ref(cancellable as gpointer) as *mut GCancellable as *mut GCancellable;
        (*data).enumeration_cancelled_id = g_cancellable_connect(
            cancellable,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GCancellable, gpointer) -> ()>,
                GCallback,
            >(Some(
                safe_c2rust_on_connection_cancelled
                    as unsafe extern "C" fn(*mut GCancellable, gpointer) -> (),
            )),
            g_object_ref((*data).enumeration_cancellable as gpointer) as *mut GCancellable
                as gpointer,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    }
    g_log(
        G_LOG_DOMAIN.as_ptr() as *const gchar,
        G_LOG_LEVEL_DEBUG,
        b"%s: starting new g_socket_client_connect_async() with GTask %p and GSocketClientAsyncConnectData %p\0"
            as *const u8 as *const gchar,
        b"g_socket_client_connect_async\0" as *const u8 as *const ::core::ffi::c_char,
        (*data).task,
        data,
    );
    safe_c2rust_enumerator_next_async(data, FALSE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_client_connect_to_host_async(
    mut client: *mut GSocketClient,
    mut host_and_port: *const gchar,
    mut default_port: guint16,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut connectable: *mut GSocketConnectable = ::core::ptr::null_mut::<GSocketConnectable>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    error = ::core::ptr::null_mut::<GError>();
    connectable = g_network_address_parse(host_and_port, default_port, &raw mut error);
    if connectable.is_null() {
        g_task_report_error(
            client as gpointer,
            callback,
            user_data,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GSocketClient,
                        *const gchar,
                        guint16,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_socket_client_connect_to_host_async
                    as unsafe extern "C" fn(
                        *mut GSocketClient,
                        *const gchar,
                        guint16,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )),
            error,
        );
    } else {
        safe_c2rust_g_socket_client_connect_async(
            client,
            connectable,
            cancellable,
            callback,
            user_data,
        );
        g_object_unref(connectable as gpointer);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_client_connect_to_service_async(
    mut client: *mut GSocketClient,
    mut domain: *const gchar,
    mut service: *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut connectable: *mut GSocketConnectable = ::core::ptr::null_mut::<GSocketConnectable>();
    connectable = g_network_service_new(service, b"tcp\0" as *const u8 as *const gchar, domain);
    safe_c2rust_g_socket_client_connect_async(
        client,
        connectable,
        cancellable,
        callback,
        user_data,
    );
    g_object_unref(connectable as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_client_connect_to_uri_async(
    mut client: *mut GSocketClient,
    mut uri: *const gchar,
    mut default_port: guint16,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut connectable: *mut GSocketConnectable = ::core::ptr::null_mut::<GSocketConnectable>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    error = ::core::ptr::null_mut::<GError>();
    connectable = g_network_address_parse_uri(uri, default_port, &raw mut error);
    if connectable.is_null() {
        g_task_report_error(
            client as gpointer,
            callback,
            user_data,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GSocketClient,
                        *const gchar,
                        guint16,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_socket_client_connect_to_uri_async
                    as unsafe extern "C" fn(
                        *mut GSocketClient,
                        *const gchar,
                        guint16,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )),
            error,
        );
    } else {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_DEBUG,
            b"g_socket_client_connect_to_uri_async\0" as *const u8 as *const gchar,
        );
        safe_c2rust_g_socket_client_connect_async(
            client,
            connectable,
            cancellable,
            callback,
            user_data,
        );
        g_object_unref(connectable as gpointer);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_client_connect_finish(
    mut client: *mut GSocketClient,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GSocketConnection {
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, client as gpointer) != 0 {
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
            b"g_task_is_valid (result, client)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSocketConnection>();
    }
    return g_task_propagate_pointer(result as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GSocketConnection;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_client_connect_to_host_finish(
    mut client: *mut GSocketClient,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GSocketConnection {
    return safe_c2rust_g_socket_client_connect_finish(client, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_client_connect_to_service_finish(
    mut client: *mut GSocketClient,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GSocketConnection {
    return safe_c2rust_g_socket_client_connect_finish(client, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_client_connect_to_uri_finish(
    mut client: *mut GSocketClient,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GSocketConnection {
    return safe_c2rust_g_socket_client_connect_finish(client, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_client_add_application_proxy(
    mut client: *mut GSocketClient,
    mut protocol: *const gchar,
) {
    g_hash_table_add(
        (*(*client).priv_0).app_proxies,
        safe_c2rust_g_strdup_inline(protocol as *const ::core::ffi::c_char) as gpointer,
    );
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
