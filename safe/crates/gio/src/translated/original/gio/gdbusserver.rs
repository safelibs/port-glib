use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GHashTable;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GInputStreamPrivate;
    pub type _GCancellablePrivate;
    pub type _GIOStreamPrivate;
    pub type _GInetAddressPrivate;
    pub type _GInetSocketAddressPrivate;
    pub type _GInitable;
    pub type _GResolverPrivate;
    pub type _GSocketConnectionPrivate;
    pub type _GSocketListenerPrivate;
    pub type _GSocketServicePrivate;
    pub type _GDBusConnection;
    pub type _GDBusAuthObserver;
    pub type _GUnixSocketAddressPrivate;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
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
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
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
    fn write(__fd: ::core::ffi::c_int, __buf: *const ::core::ffi::c_void, __n: size_t) -> ssize_t;
    fn g_file_open_tmp(
        tmpl: *const gchar,
        name_used: *mut *mut gchar,
        error: *mut *mut GError,
    ) -> gint;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_list_free_full(list: *mut GList, free_func: GDestroyNotify);
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_hash_table_unref(hash_table: *mut GHashTable);
    fn g_main_context_unref(context: *mut GMainContext);
    fn g_main_context_ref_thread_default() -> *mut GMainContext;
    fn g_source_unref(source: *mut GSource);
    fn g_source_attach(source: *mut GSource, context: *mut GMainContext) -> guint;
    fn g_source_set_priority(source: *mut GSource, priority: gint);
    fn g_source_set_callback(
        source: *mut GSource,
        func: GSourceFunc,
        data: gpointer,
        notify: GDestroyNotify,
    );
    fn g_source_set_static_name(source: *mut GSource, name: *const ::core::ffi::c_char);
    fn g_idle_source_new() -> *mut GSource;
    fn g_strerror(errnum: gint) -> *const gchar;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_strsplit(
        string: *const gchar,
        delimiter: *const gchar,
        max_tokens: gint,
    ) -> *mut *mut gchar;
    fn g_strfreev(str_array: *mut *mut gchar);
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
    fn g_string_insert_c(string: *mut GString, pos: gssize, c: gchar) -> *mut GString;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_random_int_range(begin: gint32, end: gint32) -> gint32;
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
    fn g_signal_connect_data(
        instance: gpointer,
        detailed_signal: *const gchar,
        c_handler: GCallback,
        data: gpointer,
        destroy_data: GClosureNotify,
        connect_flags: GConnectFlags,
    ) -> gulong;
    fn g_signal_handler_disconnect(instance: gpointer, handler_id: gulong);
    fn g_signal_accumulator_true_handled(
        ihint: *mut GSignalInvocationHint,
        return_accu: *mut GValue,
        handler_return: *const GValue,
        dummy: gpointer,
    ) -> gboolean;
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_value_set_object(value: *mut GValue, v_object: gpointer);
    fn g_value_dup_object(value: *const GValue) -> gpointer;
    fn g_value_set_flags(value: *mut GValue, v_flags: guint);
    fn g_value_get_flags(value: *const GValue) -> guint;
    fn g_param_spec_boolean(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: gboolean,
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
    fn g_value_set_boolean(value: *mut GValue, v_boolean: gboolean);
    fn g_value_set_string(value: *mut GValue, v_string: *const gchar);
    fn g_value_dup_string(value: *const GValue) -> *mut gchar;
    fn g_io_error_quark() -> GQuark;
    fn g_io_error_from_errno(err_no: gint) -> GIOErrorEnum;
    fn g_dbus_address_escape_value(string: *const gchar) -> *mut gchar;
    fn g_dbus_is_supported_address(string: *const gchar, error: *mut *mut GError) -> gboolean;
    fn g_dbus_is_guid(string: *const gchar) -> gboolean;
    fn g_dbus_connection_get_type() -> GType;
    fn g_dbus_connection_new_sync(
        stream: *mut GIOStream,
        guid: *const gchar,
        flags: GDBusConnectionFlags,
        observer: *mut GDBusAuthObserver,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GDBusConnection;
    fn g_dbus_connection_start_message_processing(connection: *mut GDBusConnection);
    fn g_dbus_server_flags_get_type() -> GType;
    fn _g_dbus_address_parse_entry(
        address_entry: *const gchar,
        out_transport_name: *mut *mut gchar,
        out_key_value_pairs: *mut *mut GHashTable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_dbus_auth_observer_get_type() -> GType;
    fn g_initable_get_type() -> GType;
    fn g_initable_new(
        object_type: GType,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
        first_property_name: *const gchar,
        ...
    ) -> gpointer;
    fn g_socket_listener_add_address(
        listener: *mut GSocketListener,
        address: *mut GSocketAddress,
        type_0: GSocketType,
        protocol: GSocketProtocol,
        source_object: *mut GObject,
        effective_address: *mut *mut GSocketAddress,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_socket_listener_close(listener: *mut GSocketListener);
    fn g_socket_service_start(service: *mut GSocketService);
    fn g_socket_service_stop(service: *mut GSocketService);
    fn g_threaded_socket_service_new(max_threads: ::core::ffi::c_int) -> *mut GSocketService;
    fn g_resolver_get_default() -> *mut GResolver;
    fn g_resolver_lookup_by_name(
        resolver: *mut GResolver,
        hostname: *const gchar,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GList;
    fn g_unlink(filename: *const gchar) -> ::core::ffi::c_int;
    fn g_close(fd: gint, error: *mut *mut GError) -> gboolean;
    fn g_inet_socket_address_new(address: *mut GInetAddress, port: guint16) -> *mut GSocketAddress;
    fn g_inet_socket_address_get_port(address: *mut GInetSocketAddress) -> guint16;
    fn g_input_stream_read_all(
        stream: *mut GInputStream,
        buffer: *mut ::core::ffi::c_void,
        count: gsize,
        bytes_read: *mut gsize,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_io_stream_get_input_stream(stream: *mut GIOStream) -> *mut GInputStream;
    fn _g_cclosure_marshal_BOOLEAN__OBJECT(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn _g_cclosure_marshal_BOOLEAN__OBJECTv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
    fn g_unix_socket_address_new(path: *const gchar) -> *mut GSocketAddress;
    fn g_unix_socket_address_new_with_type(
        path: *const gchar,
        path_len: gint,
        type_0: GUnixSocketAddressType,
    ) -> *mut GSocketAddress;
    fn g_unix_socket_address_get_path(
        address: *mut GUnixSocketAddress,
    ) -> *const ::core::ffi::c_char;
    fn g_unix_socket_address_get_address_type(
        address: *mut GUnixSocketAddress,
    ) -> GUnixSocketAddressType;
    fn g_unix_socket_address_abstract_names_supported() -> gboolean;
    fn glib_gettext(str: *const gchar) -> *const gchar;
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
pub type ssize_t = isize;
pub type guint16 = ::core::ffi::c_ushort;
pub type gint32 = ::core::ffi::c_int;
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
pub type GData = _GData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
pub type GList = _GList;
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
pub type GUnixSocketAddressType = ::core::ffi::c_uint;
pub const G_UNIX_SOCKET_ADDRESS_ABSTRACT_PADDED: GUnixSocketAddressType = 4;
pub const G_UNIX_SOCKET_ADDRESS_ABSTRACT: GUnixSocketAddressType = 3;
pub const G_UNIX_SOCKET_ADDRESS_PATH: GUnixSocketAddressType = 2;
pub const G_UNIX_SOCKET_ADDRESS_ANONYMOUS: GUnixSocketAddressType = 1;
pub const G_UNIX_SOCKET_ADDRESS_INVALID: GUnixSocketAddressType = 0;
pub type GDBusConnectionFlags = ::core::ffi::c_uint;
pub const G_DBUS_CONNECTION_FLAGS_CROSS_NAMESPACE: GDBusConnectionFlags = 64;
pub const G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_REQUIRE_SAME_USER: GDBusConnectionFlags = 32;
pub const G_DBUS_CONNECTION_FLAGS_DELAY_MESSAGE_PROCESSING: GDBusConnectionFlags = 16;
pub const G_DBUS_CONNECTION_FLAGS_MESSAGE_BUS_CONNECTION: GDBusConnectionFlags = 8;
pub const G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_ALLOW_ANONYMOUS: GDBusConnectionFlags = 4;
pub const G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_SERVER: GDBusConnectionFlags = 2;
pub const G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_CLIENT: GDBusConnectionFlags = 1;
pub const G_DBUS_CONNECTION_FLAGS_NONE: GDBusConnectionFlags = 0;
pub type GDBusServerFlags = ::core::ffi::c_uint;
pub const G_DBUS_SERVER_FLAGS_AUTHENTICATION_REQUIRE_SAME_USER: GDBusServerFlags = 4;
pub const G_DBUS_SERVER_FLAGS_AUTHENTICATION_ALLOW_ANONYMOUS: GDBusServerFlags = 2;
pub const G_DBUS_SERVER_FLAGS_RUN_IN_THREAD: GDBusServerFlags = 1;
pub const G_DBUS_SERVER_FLAGS_NONE: GDBusServerFlags = 0;
pub type GInputStream = _GInputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInputStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GInputStreamPrivate,
}
pub type GInputStreamPrivate = _GInputStreamPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
pub type GIOStream = _GIOStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GIOStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GIOStreamPrivate,
}
pub type GIOStreamPrivate = _GIOStreamPrivate;
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
pub type GInitable = _GInitable;
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
pub struct _GSocketConnection {
    pub parent_instance: GIOStream,
    pub priv_0: *mut GSocketConnectionPrivate,
}
pub type GSocketConnectionPrivate = _GSocketConnectionPrivate;
pub type GSocketConnection = _GSocketConnection;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketListener {
    pub parent_instance: GObject,
    pub priv_0: *mut GSocketListenerPrivate,
}
pub type GSocketListenerPrivate = _GSocketListenerPrivate;
pub type GSocketListener = _GSocketListener;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketService {
    pub parent_instance: GSocketListener,
    pub priv_0: *mut GSocketServicePrivate,
}
pub type GSocketServicePrivate = _GSocketServicePrivate;
pub type GSocketService = _GSocketService;
pub type GDBusConnection = _GDBusConnection;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusServer {
    pub parent_instance: GObject,
    pub flags: GDBusServerFlags,
    pub address: *mut gchar,
    pub guid: *mut gchar,
    pub nonce: *mut guchar,
    pub nonce_file: *mut gchar,
    pub client_address: *mut gchar,
    pub unix_socket_path: *mut gchar,
    pub listener: *mut GSocketListener,
    pub is_using_listener: gboolean,
    pub run_signal_handler_id: gulong,
    pub main_context_at_construction: *mut GMainContext,
    pub active: gboolean,
    pub authentication_observer: *mut GDBusAuthObserver,
}
pub type GDBusAuthObserver = _GDBusAuthObserver;
pub type GDBusServer = _GDBusServer;
pub type GDBusServerClass = _GDBusServerClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusServerClass {
    pub parent_class: GObjectClass,
    pub new_connection:
        Option<unsafe extern "C" fn(*mut GDBusServer, *mut GDBusConnection) -> gboolean>,
}
pub const NEW_CONNECTION_SIGNAL: C2RustUnnamed_1 = 0;
pub const PROP_AUTHENTICATION_OBSERVER: C2RustUnnamed_0 = 6;
pub const PROP_ACTIVE: C2RustUnnamed_0 = 5;
pub const PROP_CLIENT_ADDRESS: C2RustUnnamed_0 = 2;
pub const PROP_ADDRESS: C2RustUnnamed_0 = 1;
pub const PROP_GUID: C2RustUnnamed_0 = 4;
pub const PROP_FLAGS: C2RustUnnamed_0 = 3;
pub type GInitableIface = _GInitableIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInitableIface {
    pub g_iface: GTypeInterface,
    pub init: Option<
        unsafe extern "C" fn(*mut GInitable, *mut GCancellable, *mut *mut GError) -> gboolean,
    >,
}
pub type GUnixSocketAddress = _GUnixSocketAddress;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixSocketAddress {
    pub parent_instance: GSocketAddress,
    pub priv_0: *mut GUnixSocketAddressPrivate,
}
pub type GUnixSocketAddressPrivate = _GUnixSocketAddressPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EmitIdleData {
    pub server: *mut GDBusServer,
    pub connection: *mut GDBusConnection,
}
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const LAST_SIGNAL: C2RustUnnamed_1 = 1;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const G_PRIORITY_DEFAULT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
unsafe extern "C" fn safe_c2rust_g_string_append_c_inline(
    mut gstring: *mut GString,
    mut c: gchar,
) -> *mut GString {
    if ({
        let mut _g_boolean_var_3: ::core::ffi::c_int = 0;
        if !gstring.is_null() && (*gstring).len.wrapping_add(1 as gsize) < (*gstring).allocated_len
        {
            _g_boolean_var_3 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_3 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_3
    }) as ::core::ffi::c_long
        != 0
    {
        let fresh0 = (*gstring).len;
        (*gstring).len = (*gstring).len.wrapping_add(1);
        *(*gstring).str_0.offset(fresh0 as isize) = c;
        *(*gstring).str_0.offset((*gstring).len as isize) = 0 as gchar;
    } else {
        g_string_insert_c(gstring, -(1 as ::core::ffi::c_int) as gssize, c);
    }
    return gstring;
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
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_BOOLEAN: GType = ((5 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
static mut safe_c2rust__signals: [guint; 1] = [0 as ::core::ffi::c_int as guint];
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_server_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_dbus_server_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_dbus_server_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_dbus_server_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GDBusServer_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut safe_c2rust_GDBusServer_private_offset);
    }
    safe_c2rust_g_dbus_server_class_init(klass as *mut GDBusServerClass);
}
static mut safe_c2rust_GDBusServer_private_offset: gint = 0;
static mut safe_c2rust_g_dbus_server_parent_class: gpointer = NULL_0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_dbus_server_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GDBusServer\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GDBusServerClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dbus_server_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GDBusServer>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusServer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dbus_server_init as unsafe extern "C" fn(*mut GDBusServer) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GInitableIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_initable_iface_init as unsafe extern "C" fn(*mut GInitableIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_initable_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_dbus_server_dispose(mut object: *mut GObject) {
    let mut server: *mut GDBusServer = object as *mut ::core::ffi::c_void as *mut GDBusServer;
    if (*server).active != 0 {
        safe_c2rust_g_dbus_server_stop(server);
    }
    (*(safe_c2rust_g_dbus_server_parent_class as *mut GObjectClass))
        .dispose
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_dbus_server_finalize(mut object: *mut GObject) {
    let mut server: *mut GDBusServer = object as *mut ::core::ffi::c_void as *mut GDBusServer;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if (*server).active == 0 {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusserver.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            182 as ::core::ffi::c_int,
            G_STRFUNC,
            b"!server->active\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !(*server).authentication_observer.is_null() {
        g_object_unref((*server).authentication_observer as gpointer);
    }
    if (*server).run_signal_handler_id > 0 as gulong {
        g_signal_handler_disconnect(
            (*server).listener as gpointer,
            (*server).run_signal_handler_id,
        );
    }
    if !(*server).listener.is_null() {
        g_object_unref((*server).listener as gpointer);
    }
    g_free((*server).address as gpointer);
    g_free((*server).guid as gpointer);
    g_free((*server).client_address as gpointer);
    if !(*server).nonce.is_null() {
        memset(
            (*server).nonce as *mut ::core::ffi::c_void,
            '\0' as i32,
            16 as size_t,
        );
        g_free((*server).nonce as gpointer);
    }
    g_free((*server).unix_socket_path as gpointer);
    g_free((*server).nonce_file as gpointer);
    g_main_context_unref((*server).main_context_at_construction);
    (*(safe_c2rust_g_dbus_server_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_dbus_server_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut server: *mut GDBusServer = object as *mut ::core::ffi::c_void as *mut GDBusServer;
    match prop_id {
        3 => {
            g_value_set_flags(value, (*server).flags as guint);
        }
        4 => {
            g_value_set_string(value, (*server).guid);
        }
        1 => {
            g_value_set_string(value, (*server).address);
        }
        2 => {
            g_value_set_string(value, (*server).client_address);
        }
        5 => {
            g_value_set_boolean(value, (*server).active);
        }
        6 => {
            g_value_set_object(value, (*server).authentication_observer as gpointer);
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusserver.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                245 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_dbus_server_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut server: *mut GDBusServer = object as *mut ::core::ffi::c_void as *mut GDBusServer;
    match prop_id {
        3 => {
            (*server).flags = g_value_get_flags(value) as GDBusServerFlags;
        }
        4 => {
            (*server).guid = g_value_dup_string(value);
        }
        1 => {
            (*server).address = g_value_dup_string(value);
        }
        6 => {
            (*server).authentication_observer = g_value_dup_object(value) as *mut GDBusAuthObserver;
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusserver.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                277 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_dbus_server_class_init(mut klass: *mut GDBusServerClass) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).dispose =
        Some(safe_c2rust_g_dbus_server_dispose as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).finalize =
        Some(safe_c2rust_g_dbus_server_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_dbus_server_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_dbus_server_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    g_object_class_install_property(
        gobject_class,
        PROP_FLAGS as ::core::ffi::c_int as guint,
        g_param_spec_flags(
            b"flags\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_dbus_server_flags_get_type(),
            G_DBUS_SERVER_FLAGS_NONE as ::core::ffi::c_int as guint,
            (G_PARAM_READABLE as ::core::ffi::c_int
                | G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_GUID as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"guid\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_ADDRESS as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"address\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_CLIENT_ADDRESS as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"client-address\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_ACTIVE as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"active\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_READABLE as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_AUTHENTICATION_OBSERVER as ::core::ffi::c_int as guint,
        g_param_spec_object(
            b"authentication-observer\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_dbus_auth_observer_get_type(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    safe_c2rust__signals[NEW_CONNECTION_SIGNAL as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"new-connection\0" as *const u8 as *const gchar),
        safe_c2rust_g_dbus_server_get_type(),
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
        NULL_0,
        Some(
            _g_cclosure_marshal_BOOLEAN__OBJECT
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    guint,
                    *const GValue,
                    gpointer,
                    gpointer,
                ) -> (),
        ),
        G_TYPE_BOOLEAN,
        1 as guint,
        g_dbus_connection_get_type(),
    );
    g_signal_set_va_marshaller(
        safe_c2rust__signals[NEW_CONNECTION_SIGNAL as ::core::ffi::c_int as usize],
        (*(klass as *mut GTypeClass)).g_type,
        Some(
            _g_cclosure_marshal_BOOLEAN__OBJECTv
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
}
unsafe extern "C" fn safe_c2rust_g_dbus_server_init(mut server: *mut GDBusServer) {
    (*server).main_context_at_construction = g_main_context_ref_thread_default();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_server_new_sync(
    mut address: *const gchar,
    mut flags: GDBusServerFlags,
    mut guid: *const gchar,
    mut observer: *mut GDBusAuthObserver,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GDBusServer {
    let mut server: *mut GDBusServer = ::core::ptr::null_mut::<GDBusServer>();
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !address.is_null() {
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
            b"address != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusServer>();
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if g_dbus_is_guid(guid) != 0 {
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
            b"g_dbus_is_guid (guid)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusServer>();
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if flags as ::core::ffi::c_uint
            & !(G_DBUS_SERVER_FLAGS_RUN_IN_THREAD as ::core::ffi::c_int
                | G_DBUS_SERVER_FLAGS_AUTHENTICATION_ALLOW_ANONYMOUS as ::core::ffi::c_int
                | G_DBUS_SERVER_FLAGS_AUTHENTICATION_REQUIRE_SAME_USER as ::core::ffi::c_int)
                as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint
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
            b"(flags & ~G_DBUS_SERVER_FLAGS_ALL) == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusServer>();
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusServer>();
    }
    server = g_initable_new(
        safe_c2rust_g_dbus_server_get_type(),
        cancellable,
        error,
        b"address\0" as *const u8 as *const gchar,
        address,
        b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        flags as ::core::ffi::c_uint,
        b"guid\0" as *const u8 as *const ::core::ffi::c_char,
        guid,
        b"authentication-observer\0" as *const u8 as *const ::core::ffi::c_char,
        observer,
        NULL_0,
    ) as *mut GDBusServer;
    return server;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_server_get_client_address(
    mut server: *mut GDBusServer,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = server as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_server_get_type();
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
            b"G_IS_DBUS_SERVER (server)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*server).client_address;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_server_get_guid(
    mut server: *mut GDBusServer,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = server as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_server_get_type();
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
            b"G_IS_DBUS_SERVER (server)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*server).guid;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_server_get_flags(
    mut server: *mut GDBusServer,
) -> GDBusServerFlags {
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = server as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_server_get_type();
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
            b"G_IS_DBUS_SERVER (server)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_DBUS_SERVER_FLAGS_NONE;
    }
    return (*server).flags;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_server_is_active(
    mut server: *mut GDBusServer,
) -> gboolean {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = server as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_server_get_type();
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
            b"G_IS_DBUS_SERVER (server)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_DBUS_SERVER_FLAGS_NONE as ::core::ffi::c_int as gboolean;
    }
    return (*server).active;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_server_start(mut server: *mut GDBusServer) {
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = server as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_server_get_type();
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
            b"G_IS_DBUS_SERVER (server)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*server).active != 0 {
        return;
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if (*server).is_using_listener != 0 {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusserver.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            608 as ::core::ffi::c_int,
            G_STRFUNC,
            b"server->is_using_listener\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*server).run_signal_handler_id = g_signal_connect_data(
        (*server).listener as *mut ::core::ffi::c_void as *mut GSocketService as gpointer,
        b"run\0" as *const u8 as *const gchar,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GSocketService,
                    *mut GSocketConnection,
                    *mut GObject,
                    gpointer,
                ) -> gboolean,
            >,
            GCallback,
        >(Some(
            safe_c2rust_on_run
                as unsafe extern "C" fn(
                    *mut GSocketService,
                    *mut GSocketConnection,
                    *mut GObject,
                    gpointer,
                ) -> gboolean,
        )),
        g_object_ref(server as gpointer) as *mut GDBusServer as gpointer,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> ()>, GClosureNotify>(
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        ),
        G_CONNECT_DEFAULT,
    );
    g_socket_service_start((*server).listener as *mut ::core::ffi::c_void as *mut GSocketService);
    (*server).active = TRUE as gboolean;
    g_object_notify(
        server as *mut ::core::ffi::c_void as *mut GObject,
        b"active\0" as *const u8 as *const gchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_server_stop(mut server: *mut GDBusServer) {
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = server as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_server_get_type();
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
            b"G_IS_DBUS_SERVER (server)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*server).active == 0 {
        return;
    }
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if (*server).is_using_listener != 0 {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusserver.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            635 as ::core::ffi::c_int,
            G_STRFUNC,
            b"server->is_using_listener\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if (*server).run_signal_handler_id > 0 as gulong {
            _g_boolean_var_23 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_23 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_23
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusserver.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            636 as ::core::ffi::c_int,
            G_STRFUNC,
            b"server->run_signal_handler_id > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let _instance: gpointer = (*server).listener as gpointer;
    let _handler_id_ptr: *mut gulong = &raw mut (*server).run_signal_handler_id;
    let _handler_id: gulong = *_handler_id_ptr;
    if _handler_id > 0 as gulong {
        *_handler_id_ptr = 0 as gulong;
        g_signal_handler_disconnect(_instance, _handler_id);
    }
    g_socket_service_stop((*server).listener as *mut ::core::ffi::c_void as *mut GSocketService);
    (*server).active = FALSE as gboolean;
    g_object_notify(
        server as *mut ::core::ffi::c_void as *mut GObject,
        b"active\0" as *const u8 as *const gchar,
    );
    if !(*server).unix_socket_path.is_null() {
        if g_unlink((*server).unix_socket_path) != 0 as ::core::ffi::c_int {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Failed to delete %s: %s\0" as *const u8 as *const gchar,
                (*server).unix_socket_path,
                g_strerror(*__errno_location()),
            );
        }
    }
    if !(*server).nonce_file.is_null() {
        if g_unlink((*server).nonce_file) != 0 as ::core::ffi::c_int {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Failed to delete %s: %s\0" as *const u8 as *const gchar,
                (*server).nonce_file,
                g_strerror(*__errno_location()),
            );
        }
    }
}
unsafe extern "C" fn safe_c2rust_random_ascii() -> gint {
    let mut ret: gint = 0;
    ret = g_random_int_range(0 as gint32, 60 as gint32) as gint;
    if ret < 25 as ::core::ffi::c_int {
        ret += 'A' as i32;
    } else if ret < 50 as ::core::ffi::c_int {
        ret += 'a' as i32 - 25 as ::core::ffi::c_int;
    } else {
        ret += '0' as i32 - 50 as ::core::ffi::c_int;
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust_try_unix(
    mut server: *mut GDBusServer,
    mut address_entry: *const gchar,
    mut key_value_pairs: *mut GHashTable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut current_block: u64;
    let mut ret: gboolean = 0;
    let mut path: *const gchar = ::core::ptr::null::<gchar>();
    let mut dir: *const gchar = ::core::ptr::null::<gchar>();
    let mut tmpdir: *const gchar = ::core::ptr::null::<gchar>();
    let mut abstract_0: *const gchar = ::core::ptr::null::<gchar>();
    let mut address: *mut GSocketAddress = ::core::ptr::null_mut::<GSocketAddress>();
    ret = FALSE as gboolean;
    address = ::core::ptr::null_mut::<GSocketAddress>();
    path = g_hash_table_lookup(
        key_value_pairs,
        b"path\0" as *const u8 as *const ::core::ffi::c_char as gconstpointer,
    ) as *const gchar;
    dir = g_hash_table_lookup(
        key_value_pairs,
        b"dir\0" as *const u8 as *const ::core::ffi::c_char as gconstpointer,
    ) as *const gchar;
    tmpdir = g_hash_table_lookup(
        key_value_pairs,
        b"tmpdir\0" as *const u8 as *const ::core::ffi::c_char as gconstpointer,
    ) as *const gchar;
    abstract_0 = g_hash_table_lookup(
        key_value_pairs,
        b"abstract\0" as *const u8 as *const ::core::ffi::c_char as gconstpointer,
    ) as *const gchar;
    if !path.is_null() {
        address = g_unix_socket_address_new(path);
        current_block = 1608152415753874203;
    } else if !dir.is_null() || !tmpdir.is_null() {
        let mut n: gint = 0;
        let mut s: *mut GString = ::core::ptr::null_mut::<GString>();
        let mut local_error: *mut GError = ::core::ptr::null_mut::<GError>();
        loop {
            s = g_string_new(if !tmpdir.is_null() { tmpdir } else { dir });
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char =
                        b"/dbus-\0" as *const u8 as *const ::core::ffi::c_char;
                    safe_c2rust_g_string_append_len_inline(
                        s,
                        __val,
                        if ({
                            let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
                            if !__val.is_null() {
                                _g_boolean_var_24 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_24 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_24
                        }) as ::core::ffi::c_long
                            != 0
                        {
                            strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                                as gssize
                        } else {
                            -(1 as ::core::ffi::c_int) as gssize
                        },
                    );
                });
            } else {
                safe_c2rust_g_string_append_len_inline(
                    s,
                    b"/dbus-\0" as *const u8 as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
            n = 0 as ::core::ffi::c_int as gint;
            while n < 8 as ::core::ffi::c_int {
                safe_c2rust_g_string_append_c_inline(s, safe_c2rust_random_ascii() as gchar);
                n += 1;
            }
            address = g_unix_socket_address_new((*s).str_0);
            if 0 != 0 {
                if 0 as ::core::ffi::c_int == 0 {
                    g_string_free(s, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
                } else {
                    g_string_free_and_steal(s);
                };
            } else {
                g_string_free(s, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
            };
            local_error = ::core::ptr::null_mut::<GError>();
            if g_socket_listener_add_address(
                (*server).listener,
                address,
                G_SOCKET_TYPE_STREAM,
                G_SOCKET_PROTOCOL_DEFAULT,
                ::core::ptr::null_mut::<GObject>(),
                ::core::ptr::null_mut::<*mut GSocketAddress>(),
                &raw mut local_error,
            ) == 0
            {
                if (*local_error).domain == g_io_error_quark()
                    && (*local_error).code == G_IO_ERROR_ADDRESS_IN_USE as ::core::ffi::c_int
                {
                    g_error_free(local_error);
                } else {
                    g_propagate_error(error, local_error);
                    break;
                }
            } else {
                ret = TRUE as gboolean;
                break;
            }
        }
        current_block = 8103914185297291777;
    } else if !abstract_0.is_null() {
        if g_unix_socket_address_abstract_names_supported() == 0 {
            g_set_error_literal(
                error,
                g_io_error_quark(),
                G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
                glib_gettext(b"Abstract namespace not supported\0" as *const u8 as *const gchar),
            );
            current_block = 8103914185297291777;
        } else {
            address = g_unix_socket_address_new_with_type(
                abstract_0,
                -(1 as gint),
                G_UNIX_SOCKET_ADDRESS_ABSTRACT,
            );
            current_block = 1608152415753874203;
        }
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusserver.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            748 as ::core::ffi::c_int,
            G_STRFUNC,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    match current_block {
        1608152415753874203 => {
            if !(g_socket_listener_add_address(
                (*server).listener,
                address,
                G_SOCKET_TYPE_STREAM,
                G_SOCKET_PROTOCOL_DEFAULT,
                ::core::ptr::null_mut::<GObject>(),
                ::core::ptr::null_mut::<*mut GSocketAddress>(),
                error,
            ) == 0)
            {
                ret = TRUE as gboolean;
            }
        }
        _ => {}
    }
    if !address.is_null() {
        if ret != 0 {
            let mut address_path: *const ::core::ffi::c_char =
                ::core::ptr::null::<::core::ffi::c_char>();
            let mut escaped_path: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            (*server).is_using_listener = TRUE as gboolean;
            address_path = g_unix_socket_address_get_path(
                address as *mut ::core::ffi::c_void as *mut GUnixSocketAddress,
            );
            escaped_path = g_dbus_address_escape_value(address_path as *const gchar)
                as *mut ::core::ffi::c_char;
            match g_unix_socket_address_get_address_type(
                address as *mut ::core::ffi::c_void as *mut GUnixSocketAddress,
            ) as ::core::ffi::c_uint
            {
                3 => {
                    (*server).client_address = g_strdup_printf(
                        b"unix:abstract=%s\0" as *const u8 as *const gchar,
                        escaped_path,
                    );
                }
                2 => {
                    (*server).client_address = g_strdup_printf(
                        b"unix:path=%s\0" as *const u8 as *const gchar,
                        escaped_path,
                    );
                    (*server).unix_socket_path =
                        safe_c2rust_g_strdup_inline(address_path) as *mut gchar;
                }
                _ => {
                    g_assertion_message_expr(
                        G_LOG_DOMAIN.as_ptr(),
                        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusserver.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        788 as ::core::ffi::c_int,
                        G_STRFUNC,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                    );
                }
            }
            g_free(escaped_path as gpointer);
        }
        g_object_unref(address as gpointer);
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust_try_tcp(
    mut server: *mut GDBusServer,
    mut address_entry: *const gchar,
    mut key_value_pairs: *mut GHashTable,
    mut do_nonce: gboolean,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut current_block: u64;
    let mut ret: gboolean = 0;
    let mut host: *const gchar = ::core::ptr::null::<gchar>();
    let mut port: *const gchar = ::core::ptr::null::<gchar>();
    let mut port_num: gint = 0;
    let mut resolver: *mut GResolver = ::core::ptr::null_mut::<GResolver>();
    let mut resolved_addresses: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    ret = FALSE as gboolean;
    resolver = ::core::ptr::null_mut::<GResolver>();
    resolved_addresses = ::core::ptr::null_mut::<GList>();
    host = g_hash_table_lookup(
        key_value_pairs,
        b"host\0" as *const u8 as *const ::core::ffi::c_char as gconstpointer,
    ) as *const gchar;
    port = g_hash_table_lookup(
        key_value_pairs,
        b"port\0" as *const u8 as *const ::core::ffi::c_char as gconstpointer,
    ) as *const gchar;
    if !g_hash_table_lookup(
        key_value_pairs,
        b"noncefile\0" as *const u8 as *const ::core::ffi::c_char as gconstpointer,
    )
    .is_null()
    {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Cannot specify nonce file when creating a server\0" as *const u8 as *const gchar,
            ),
        );
    } else {
        if host.is_null() {
            host = b"localhost\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
        }
        if port.is_null() {
            port = b"0\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
        }
        port_num = strtol(
            port as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            10 as ::core::ffi::c_int,
        ) as gint;
        resolver = g_resolver_get_default();
        resolved_addresses = g_resolver_lookup_by_name(
            resolver,
            host,
            ::core::ptr::null_mut::<GCancellable>(),
            error,
        );
        if !resolved_addresses.is_null() {
            l = resolved_addresses;
            loop {
                if l.is_null() {
                    current_block = 1109700713171191020;
                    break;
                }
                let mut address: *mut GInetAddress = (*l).data as *mut GInetAddress;
                let mut socket_address: *mut GSocketAddress =
                    ::core::ptr::null_mut::<GSocketAddress>();
                let mut effective_address: *mut GSocketAddress =
                    ::core::ptr::null_mut::<GSocketAddress>();
                socket_address = g_inet_socket_address_new(address, port_num as guint16);
                if g_socket_listener_add_address(
                    (*server).listener,
                    socket_address,
                    G_SOCKET_TYPE_STREAM,
                    G_SOCKET_PROTOCOL_TCP,
                    ::core::ptr::null_mut::<GObject>(),
                    &raw mut effective_address,
                    error,
                ) == 0
                {
                    g_object_unref(socket_address as gpointer);
                    current_block = 16074882702451766239;
                    break;
                } else {
                    if port_num == 0 as ::core::ffi::c_int {
                        port_num = g_inet_socket_address_get_port(
                            effective_address as *mut ::core::ffi::c_void
                                as *mut GInetSocketAddress,
                        ) as gint;
                    }
                    g_object_unref(effective_address as gpointer);
                    g_object_unref(socket_address as gpointer);
                    l = (*l).next;
                }
            }
            match current_block {
                16074882702451766239 => {}
                _ => {
                    if do_nonce != 0 {
                        let mut fd: gint = 0;
                        let mut n: guint = 0;
                        let mut bytes_written: gsize = 0;
                        let mut bytes_remaining: gsize = 0;
                        let mut file_escaped: *mut ::core::ffi::c_char =
                            ::core::ptr::null_mut::<::core::ffi::c_char>();
                        let mut host_escaped: *mut ::core::ffi::c_char =
                            ::core::ptr::null_mut::<::core::ffi::c_char>();
                        (*server).nonce = ({
                            let mut __n: gsize = 16 as ::core::ffi::c_int as gsize;
                            let mut __s: gsize = ::core::mem::size_of::<guchar>() as gsize;
                            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                            if __s == 1 as gsize {
                                __p = g_malloc0(__n);
                            } else if 0 != 0
                                && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s))
                            {
                                __p = g_malloc0(__n.wrapping_mul(__s));
                            } else {
                                __p = g_malloc0_n(__n, __s);
                            }
                            __p
                        }) as *mut guchar;
                        n = 0 as guint;
                        while n < 16 as guint {
                            *(*server).nonce.offset(n as isize) =
                                g_random_int_range(0 as gint32, 256 as gint32) as guchar;
                            n = n.wrapping_add(1);
                        }
                        fd = g_file_open_tmp(
                            b"gdbus-nonce-file-XXXXXX\0" as *const u8 as *const gchar,
                            &raw mut (*server).nonce_file,
                            error,
                        );
                        if fd == -(1 as ::core::ffi::c_int) {
                            g_socket_listener_close((*server).listener);
                            current_block = 16074882702451766239;
                        } else {
                            '_again: loop {
                                bytes_written = 0 as gsize;
                                bytes_remaining = 16 as gsize;
                                while bytes_remaining > 0 as gsize {
                                    let mut size: gssize = 0;
                                    let mut errsv: ::core::ffi::c_int = 0;
                                    size = write(
                                        fd as ::core::ffi::c_int,
                                        (*server).nonce.offset(bytes_written as isize)
                                            as *const ::core::ffi::c_void,
                                        bytes_remaining as size_t,
                                    ) as gssize;
                                    errsv = *__errno_location();
                                    if size == -(1 as ::core::ffi::c_int) as gssize {
                                        if errsv == EINTR {
                                            continue '_again;
                                        }
                                        g_set_error(
                                            error,
                                            g_io_error_quark(),
                                            g_io_error_from_errno(errsv as gint) as gint,
                                            glib_gettext(
                                                b"Error writing nonce file at \xE2\x80\x9C%s\xE2\x80\x9D: %s\0"
                                                    as *const u8 as *const gchar,
                                            ),
                                            (*server).nonce_file,
                                            g_strerror(errsv as gint),
                                        );
                                        current_block = 16074882702451766239;
                                        break '_again;
                                    } else {
                                        bytes_written = bytes_written.wrapping_add(size as gsize);
                                        bytes_remaining =
                                            bytes_remaining.wrapping_sub(size as gsize);
                                    }
                                }
                                if g_close(fd, error) == 0 {
                                    current_block = 16074882702451766239;
                                    break;
                                } else {
                                    current_block = 3222590281903869779;
                                    break;
                                }
                            }
                            match current_block {
                                16074882702451766239 => {}
                                _ => {
                                    host_escaped = g_dbus_address_escape_value(host)
                                        as *mut ::core::ffi::c_char;
                                    file_escaped = g_dbus_address_escape_value((*server).nonce_file)
                                        as *mut ::core::ffi::c_char;
                                    (*server).client_address = g_strdup_printf(
                                        b"nonce-tcp:host=%s,port=%d,noncefile=%s\0" as *const u8
                                            as *const gchar,
                                        host_escaped,
                                        port_num,
                                        file_escaped,
                                    );
                                    g_free(host_escaped as gpointer);
                                    g_free(file_escaped as gpointer);
                                    current_block = 10150597327160359210;
                                }
                            }
                        }
                    } else {
                        (*server).client_address = g_strdup_printf(
                            b"tcp:host=%s,port=%d\0" as *const u8 as *const gchar,
                            host,
                            port_num,
                        );
                        current_block = 10150597327160359210;
                    }
                    match current_block {
                        16074882702451766239 => {}
                        _ => {
                            (*server).is_using_listener = TRUE as gboolean;
                            ret = TRUE as gboolean;
                        }
                    }
                }
            }
        }
    }
    g_list_free_full(
        resolved_addresses,
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
    if !resolver.is_null() {
        g_object_unref(resolver as gpointer);
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust_emit_idle_data_free(mut data: *mut EmitIdleData) {
    g_object_unref((*data).server as gpointer);
    g_object_unref((*data).connection as gpointer);
    g_free(data as gpointer);
}
unsafe extern "C" fn safe_c2rust_emit_new_connection_in_idle(mut user_data: gpointer) -> gboolean {
    let mut data: *mut EmitIdleData = user_data as *mut EmitIdleData;
    let mut claimed: gboolean = 0;
    claimed = FALSE as gboolean;
    g_signal_emit(
        (*data).server as gpointer,
        safe_c2rust__signals[NEW_CONNECTION_SIGNAL as ::core::ffi::c_int as usize],
        0 as GQuark,
        (*data).connection,
        &raw mut claimed,
    );
    if claimed != 0 {
        g_dbus_connection_start_message_processing((*data).connection);
    }
    g_object_unref((*data).connection as gpointer);
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_on_run(
    mut service: *mut GSocketService,
    mut socket_connection: *mut GSocketConnection,
    mut source_object: *mut GObject,
    mut user_data: gpointer,
) -> gboolean {
    let mut current_block: u64;
    let mut server: *mut GDBusServer = user_data as *mut GDBusServer;
    let mut connection: *mut GDBusConnection = ::core::ptr::null_mut::<GDBusConnection>();
    let mut connection_flags: GDBusConnectionFlags = G_DBUS_CONNECTION_FLAGS_NONE;
    if !(*server).nonce.is_null() {
        let mut buf: [gchar; 16] = [0; 16];
        let mut bytes_read: gsize = 0;
        if g_input_stream_read_all(
            g_io_stream_get_input_stream(
                socket_connection as *mut ::core::ffi::c_void as *mut GIOStream,
            ),
            &raw mut buf as *mut gchar as *mut ::core::ffi::c_void,
            16 as gsize,
            &raw mut bytes_read,
            ::core::ptr::null_mut::<GCancellable>(),
            ::core::ptr::null_mut::<*mut GError>(),
        ) == 0
        {
            current_block = 6322464604591062078;
        } else if bytes_read != 16 as gsize {
            current_block = 6322464604591062078;
        } else if memcmp(
            &raw mut buf as *mut gchar as *const ::core::ffi::c_void,
            (*server).nonce as *const ::core::ffi::c_void,
            16 as size_t,
        ) != 0 as ::core::ffi::c_int
        {
            current_block = 6322464604591062078;
        } else {
            current_block = 15619007995458559411;
        }
    } else {
        current_block = 15619007995458559411;
    }
    match current_block {
        15619007995458559411 => {
            connection_flags = (G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_SERVER as ::core::ffi::c_int
                | G_DBUS_CONNECTION_FLAGS_DELAY_MESSAGE_PROCESSING as ::core::ffi::c_int)
                as GDBusConnectionFlags;
            if (*server).flags as ::core::ffi::c_uint
                & G_DBUS_SERVER_FLAGS_AUTHENTICATION_ALLOW_ANONYMOUS as ::core::ffi::c_int
                    as ::core::ffi::c_uint
                != 0
            {
                connection_flags =
                    ::core::mem::transmute::<::core::ffi::c_uint, GDBusConnectionFlags>(
                        connection_flags as ::core::ffi::c_uint
                            | G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_ALLOW_ANONYMOUS
                                as ::core::ffi::c_int
                                as ::core::ffi::c_uint,
                    );
            }
            if (*server).flags as ::core::ffi::c_uint
                & G_DBUS_SERVER_FLAGS_AUTHENTICATION_REQUIRE_SAME_USER as ::core::ffi::c_int
                    as ::core::ffi::c_uint
                != 0
            {
                connection_flags =
                    ::core::mem::transmute::<::core::ffi::c_uint, GDBusConnectionFlags>(
                        connection_flags as ::core::ffi::c_uint
                            | G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_REQUIRE_SAME_USER
                                as ::core::ffi::c_int
                                as ::core::ffi::c_uint,
                    );
            }
            connection = g_dbus_connection_new_sync(
                socket_connection as *mut ::core::ffi::c_void as *mut GIOStream,
                (*server).guid,
                connection_flags,
                (*server).authentication_observer,
                ::core::ptr::null_mut::<GCancellable>(),
                ::core::ptr::null_mut::<*mut GError>(),
            );
            if !connection.is_null() {
                if (*server).flags as ::core::ffi::c_uint
                    & G_DBUS_SERVER_FLAGS_RUN_IN_THREAD as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0
                {
                    let mut claimed: gboolean = 0;
                    claimed = FALSE as gboolean;
                    g_signal_emit(
                        server as gpointer,
                        safe_c2rust__signals[NEW_CONNECTION_SIGNAL as ::core::ffi::c_int as usize],
                        0 as GQuark,
                        connection,
                        &raw mut claimed,
                    );
                    if claimed != 0 {
                        g_dbus_connection_start_message_processing(connection);
                    }
                    g_object_unref(connection as gpointer);
                } else {
                    let mut idle_source: *mut GSource = ::core::ptr::null_mut::<GSource>();
                    let mut data: *mut EmitIdleData = ::core::ptr::null_mut::<EmitIdleData>();
                    data = ({
                        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
                        let mut __s: gsize = ::core::mem::size_of::<EmitIdleData>() as gsize;
                        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                        if __s == 1 as gsize {
                            __p = g_malloc0(__n);
                        } else if 0 != 0
                            && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s))
                        {
                            __p = g_malloc0(__n.wrapping_mul(__s));
                        } else {
                            __p = g_malloc0_n(__n, __s);
                        }
                        __p
                    }) as *mut EmitIdleData;
                    (*data).server =
                        g_object_ref(server as gpointer) as *mut GDBusServer as *mut GDBusServer;
                    (*data).connection = g_object_ref(connection as gpointer)
                        as *mut GDBusConnection
                        as *mut GDBusConnection;
                    idle_source = g_idle_source_new();
                    g_source_set_priority(idle_source, G_PRIORITY_DEFAULT);
                    g_source_set_callback(
                        idle_source,
                        Some(
                            safe_c2rust_emit_new_connection_in_idle
                                as unsafe extern "C" fn(gpointer) -> gboolean,
                        ),
                        data as gpointer,
                        ::core::mem::transmute::<
                            Option<unsafe extern "C" fn(*mut EmitIdleData) -> ()>,
                            GDestroyNotify,
                        >(Some(
                            safe_c2rust_emit_idle_data_free
                                as unsafe extern "C" fn(*mut EmitIdleData) -> (),
                        )),
                    );
                    g_source_set_static_name(
                        idle_source,
                        b"[gio] emit_new_connection_in_idle\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    g_source_attach(idle_source, (*server).main_context_at_construction);
                    g_source_unref(idle_source);
                }
            }
        }
        _ => {}
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_initable_init(
    mut initable: *mut GInitable,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut server: *mut GDBusServer = initable as *mut ::core::ffi::c_void as *mut GDBusServer;
    let mut ret: gboolean = 0;
    let mut n: guint = 0;
    let mut addr_array: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut last_error: *mut GError = ::core::ptr::null_mut::<GError>();
    ret = FALSE as gboolean;
    addr_array = ::core::ptr::null_mut::<*mut gchar>();
    last_error = ::core::ptr::null_mut::<GError>();
    if g_dbus_is_guid((*server).guid) == 0 {
        g_set_error(
            &raw mut last_error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(
                b"The string \xE2\x80\x9C%s\xE2\x80\x9D is not a valid D-Bus GUID\0" as *const u8
                    as *const gchar,
            ),
            (*server).guid,
        );
    } else {
        (*server).listener = g_threaded_socket_service_new(-(1 as ::core::ffi::c_int))
            as *mut ::core::ffi::c_void as *mut GSocketListener;
        addr_array = g_strsplit(
            (*server).address,
            b";\0" as *const u8 as *const gchar,
            0 as gint,
        );
        last_error = ::core::ptr::null_mut::<GError>();
        n = 0 as guint;
        while !addr_array.is_null() && !(*addr_array.offset(n as isize)).is_null() {
            let mut address_entry: *const gchar = *addr_array.offset(n as isize);
            let mut key_value_pairs: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
            let mut transport_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
            let mut this_error: *mut GError = ::core::ptr::null_mut::<GError>();
            this_error = ::core::ptr::null_mut::<GError>();
            if g_dbus_is_supported_address(address_entry, &raw mut this_error) != 0
                && _g_dbus_address_parse_entry(
                    address_entry,
                    &raw mut transport_name,
                    &raw mut key_value_pairs,
                    &raw mut this_error,
                ) != 0
            {
                if g_strcmp0(
                    transport_name,
                    b"unix\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    ret = safe_c2rust_try_unix(
                        server,
                        address_entry,
                        key_value_pairs,
                        &raw mut this_error,
                    );
                } else if g_strcmp0(
                    transport_name,
                    b"tcp\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    ret = safe_c2rust_try_tcp(
                        server,
                        address_entry,
                        key_value_pairs,
                        FALSE,
                        &raw mut this_error,
                    );
                } else if g_strcmp0(
                    transport_name,
                    b"nonce-tcp\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    ret = safe_c2rust_try_tcp(
                        server,
                        address_entry,
                        key_value_pairs,
                        TRUE,
                        &raw mut this_error,
                    );
                } else {
                    g_set_error(
                        &raw mut this_error,
                        g_io_error_quark(),
                        G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                        glib_gettext(
                            b"Cannot listen on unsupported transport \xE2\x80\x9C%s\xE2\x80\x9D\0"
                                as *const u8 as *const gchar,
                        ),
                        transport_name,
                    );
                }
                g_free(transport_name as gpointer);
                if !key_value_pairs.is_null() {
                    g_hash_table_unref(key_value_pairs);
                }
                if ret != 0 {
                    if ({
                        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
                        if this_error.is_null() {
                            _g_boolean_var_25 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_25 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_25
                    }) as ::core::ffi::c_long
                        != 0
                    {
                    } else {
                        g_assertion_message_expr(
                            G_LOG_DOMAIN.as_ptr(),
                            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusserver.c\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            1135 as ::core::ffi::c_int,
                            G_STRFUNC,
                            b"this_error == NULL\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                    break;
                }
            }
            if !this_error.is_null() {
                if !last_error.is_null() {
                    g_error_free(last_error);
                }
                last_error = this_error;
            }
            n = n.wrapping_add(1);
        }
    }
    g_strfreev(addr_array);
    if ret != 0 {
        g_clear_error(&raw mut last_error);
    } else {
        if ({
            let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
            if !last_error.is_null() {
                _g_boolean_var_26 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_26 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_26
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusserver.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1158 as ::core::ffi::c_int,
                G_STRFUNC,
                b"last_error != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        g_propagate_error(error, last_error);
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust_initable_iface_init(mut initable_iface: *mut GInitableIface) {
    (*initable_iface).init = Some(
        safe_c2rust_initable_init
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
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
