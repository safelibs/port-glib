use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GMainContext;
    pub type _GMainLoop;
    pub type _GSourcePrivate;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _GInetAddressPrivate;
    pub type _GInetSocketAddressPrivate;
    pub type _GIOStreamPrivate;
    pub type _GSocketPrivate;
    pub type _GSocketConnectionPrivate;
    pub type _GTask;
    fn g_ptr_array_new_with_free_func(element_free_func: GDestroyNotify) -> *mut GPtrArray;
    fn g_ptr_array_free(array: *mut GPtrArray, free_seg: gboolean) -> *mut gpointer;
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_quark_from_static_string(string: *const gchar) -> GQuark;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_clear_error(err: *mut *mut GError);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_delete_link(list: *mut GList, link_: *mut GList) -> *mut GList;
    fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_slist_delete_link(list: *mut GSList, link_: *mut GSList) -> *mut GSList;
    fn g_main_context_new() -> *mut GMainContext;
    fn g_main_context_unref(context: *mut GMainContext);
    fn g_main_context_get_thread_default() -> *mut GMainContext;
    fn g_main_loop_new(context: *mut GMainContext, is_running: gboolean) -> *mut GMainLoop;
    fn g_main_loop_run(loop_0: *mut GMainLoop);
    fn g_main_loop_quit(loop_0: *mut GMainLoop);
    fn g_main_loop_unref(loop_0: *mut GMainLoop);
    fn g_source_unref(source: *mut GSource);
    fn g_source_attach(source: *mut GSource, context: *mut GMainContext) -> guint;
    fn g_source_destroy(source: *mut GSource);
    fn g_source_set_callback(
        source: *mut GSource,
        func: GSourceFunc,
        data: gpointer,
        notify: GDestroyNotify,
    );
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
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_object_get_qdata(object: *mut GObject, quark: GQuark) -> gpointer;
    fn g_object_set_qdata_full(
        object: *mut GObject,
        quark: GQuark,
        data: gpointer,
        destroy: GDestroyNotify,
    );
    fn g_param_spec_int(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        minimum: gint,
        maximum: gint,
        default_value: gint,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_value_set_int(value: *mut GValue, v_int: gint);
    fn g_value_get_int(value: *const GValue) -> gint;
    fn g_socket_listener_event_get_type() -> GType;
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
    fn g_task_propagate_pointer(task: *mut GTask, error: *mut *mut GError) -> gpointer;
    fn g_socket_address_get_family(address: *mut GSocketAddress) -> GSocketFamily;
    fn g_inet_address_new_any(family: GSocketFamily) -> *mut GInetAddress;
    fn g_io_error_quark() -> GQuark;
    fn g_socket_get_type() -> GType;
    fn g_socket_new(
        family: GSocketFamily,
        type_0: GSocketType,
        protocol: GSocketProtocol,
        error: *mut *mut GError,
    ) -> *mut GSocket;
    fn g_socket_get_local_address(
        socket: *mut GSocket,
        error: *mut *mut GError,
    ) -> *mut GSocketAddress;
    fn g_socket_set_listen_backlog(socket: *mut GSocket, backlog: gint);
    fn g_socket_bind(
        socket: *mut GSocket,
        address: *mut GSocketAddress,
        allow_reuse: gboolean,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_socket_condition_wait(
        socket: *mut GSocket,
        condition: GIOCondition,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_socket_accept(
        socket: *mut GSocket,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GSocket;
    fn g_socket_listen(socket: *mut GSocket, error: *mut *mut GError) -> gboolean;
    fn g_socket_close(socket: *mut GSocket, error: *mut *mut GError) -> gboolean;
    fn g_socket_is_closed(socket: *mut GSocket) -> gboolean;
    fn g_socket_create_source(
        socket: *mut GSocket,
        condition: GIOCondition,
        cancellable: *mut GCancellable,
    ) -> *mut GSource;
    fn g_socket_speaks_ipv4(socket: *mut GSocket) -> gboolean;
    fn g_socket_connection_factory_create_connection(
        socket: *mut GSocket,
    ) -> *mut GSocketConnection;
    fn g_inet_socket_address_get_type() -> GType;
    fn g_inet_socket_address_new(address: *mut GInetAddress, port: guint16) -> *mut GSocketAddress;
    fn g_inet_socket_address_get_port(address: *mut GInetSocketAddress) -> guint16;
    fn glib_gettext(str: *const gchar) -> *const gchar;
    fn _g_cclosure_marshal_VOID__ENUM_OBJECT(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn _g_cclosure_marshal_VOID__ENUM_OBJECTv(
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
pub type GIOCondition = ::core::ffi::c_uint;
pub const G_IO_NVAL: GIOCondition = 32;
pub const G_IO_HUP: GIOCondition = 16;
pub const G_IO_ERR: GIOCondition = 8;
pub const G_IO_PRI: GIOCondition = 2;
pub const G_IO_OUT: GIOCondition = 4;
pub const G_IO_IN: GIOCondition = 1;
pub type GMainContext = _GMainContext;
pub type GMainLoop = _GMainLoop;
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
pub type GSocketListenerEvent = ::core::ffi::c_uint;
pub const G_SOCKET_LISTENER_LISTENED: GSocketListenerEvent = 3;
pub const G_SOCKET_LISTENER_LISTENING: GSocketListenerEvent = 2;
pub const G_SOCKET_LISTENER_BOUND: GSocketListenerEvent = 1;
pub const G_SOCKET_LISTENER_BINDING: GSocketListenerEvent = 0;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GSocketListenerPrivate {
    pub sockets: *mut GPtrArray,
    pub main_context: *mut GMainContext,
    pub listen_backlog: ::core::ffi::c_int,
    #[bitfield(name = "closed", ty = "guint", bits = "0..=0")]
    pub closed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type GSocketListener = _GSocketListener;
pub type GTask = _GTask;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
pub type GSocketSourceFunc =
    Option<unsafe extern "C" fn(*mut GSocket, GIOCondition, gpointer) -> gboolean>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketListenerClass {
    pub parent_class: GObjectClass,
    pub changed: Option<unsafe extern "C" fn(*mut GSocketListener) -> ()>,
    pub event: Option<
        unsafe extern "C" fn(*mut GSocketListener, GSocketListenerEvent, *mut GSocket) -> (),
    >,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved6: Option<unsafe extern "C" fn() -> ()>,
}
pub type GSocketListenerClass = _GSocketListenerClass;
pub const EVENT: C2RustUnnamed_2 = 0;
pub const PROP_LISTEN_BACKLOG: C2RustUnnamed_1 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AcceptData {
    pub loop_0: *mut GMainLoop,
    pub socket: *mut GSocket,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AcceptSocketAsyncData {
    pub sources: *mut GList,
    pub returned_yet: gboolean,
}
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const LAST_SIGNAL: C2RustUnnamed_2 = 1;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
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
pub const G_SOURCE_REMOVE: ::core::ffi::c_int = FALSE;
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_NONE: GType = ((1 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
static mut safe_c2rust_signals: [guint; 1] = [0 as ::core::ffi::c_int as guint];
static mut safe_c2rust_source_quark: GQuark = 0 as GQuark;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_listener_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_socket_listener_get_type_once();
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
static mut safe_c2rust_g_socket_listener_parent_class: gpointer = NULL_0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_socket_listener_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GSocketListener\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GSocketListenerClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_socket_listener_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GSocketListener>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GSocketListener) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_socket_listener_init
                    as unsafe extern "C" fn(*mut GSocketListener) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GSocketListener_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GSocketListenerPrivate>() as gsize,
    );
    return g_define_type_id;
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_socket_listener_get_instance_private(
    mut self_0: *mut GSocketListener,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GSocketListener_private_offset as glong as isize)
        as gpointer;
}
static mut safe_c2rust_GSocketListener_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_socket_listener_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_socket_listener_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GSocketListener_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GSocketListener_private_offset,
        );
    }
    safe_c2rust_g_socket_listener_class_init(klass as *mut GSocketListenerClass);
}
unsafe extern "C" fn safe_c2rust_g_socket_listener_finalize(mut object: *mut GObject) {
    let mut listener: *mut GSocketListener =
        object as *mut ::core::ffi::c_void as *mut GSocketListener;
    if !(*(*listener).priv_0).main_context.is_null() {
        g_main_context_unref((*(*listener).priv_0).main_context);
    }
    g_ptr_array_free((*(*listener).priv_0).sockets, TRUE);
    (*(safe_c2rust_g_socket_listener_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_socket_listener_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut listener: *mut GSocketListener =
        object as *mut ::core::ffi::c_void as *mut GSocketListener;
    match prop_id {
        1 => {
            g_value_set_int(value, (*(*listener).priv_0).listen_backlog as gint);
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocketlistener.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                126 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_socket_listener_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut listener: *mut GSocketListener =
        object as *mut ::core::ffi::c_void as *mut GSocketListener;
    match prop_id {
        1 => {
            safe_c2rust_g_socket_listener_set_backlog(
                listener,
                g_value_get_int(value) as ::core::ffi::c_int,
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocketlistener.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                145 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_socket_listener_class_init(
    mut klass: *mut GSocketListenerClass,
) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize =
        Some(safe_c2rust_g_socket_listener_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_socket_listener_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_socket_listener_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    g_object_class_install_property(
        gobject_class,
        PROP_LISTEN_BACKLOG as ::core::ffi::c_int as guint,
        g_param_spec_int(
            b"listen-backlog\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            0 as gint,
            2000 as gint,
            10 as gint,
            (G_PARAM_CONSTRUCT as ::core::ffi::c_int
                | G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    safe_c2rust_signals[EVENT as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"event\0" as *const u8 as *const gchar),
        (*(gobject_class as *mut GTypeClass)).g_type,
        G_SIGNAL_RUN_LAST,
        144 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL_0,
        Some(
            _g_cclosure_marshal_VOID__ENUM_OBJECT
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
        2 as guint,
        g_socket_listener_event_get_type(),
        g_socket_get_type(),
    );
    g_signal_set_va_marshaller(
        safe_c2rust_signals[EVENT as ::core::ffi::c_int as usize],
        (*(gobject_class as *mut GTypeClass)).g_type,
        Some(
            _g_cclosure_marshal_VOID__ENUM_OBJECTv
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
    safe_c2rust_source_quark =
        g_quark_from_static_string(b"g-socket-listener-source\0" as *const u8 as *const gchar);
}
unsafe extern "C" fn safe_c2rust_g_socket_listener_init(mut listener: *mut GSocketListener) {
    (*listener).priv_0 =
        safe_c2rust_g_socket_listener_get_instance_private(listener) as *mut GSocketListenerPrivate;
    (*(*listener).priv_0).sockets = g_ptr_array_new_with_free_func(::core::mem::transmute::<
        Option<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(
        g_object_unref as unsafe extern "C" fn(gpointer) -> (),
    )));
    (*(*listener).priv_0).listen_backlog = 10 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_listener_new() -> *mut GSocketListener {
    return g_object_new(
        safe_c2rust_g_socket_listener_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GSocketListener;
}
unsafe extern "C" fn safe_c2rust_check_listener(
    mut listener: *mut GSocketListener,
    mut error: *mut *mut GError,
) -> gboolean {
    if (*(*listener).priv_0).closed() != 0 {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_CLOSED as ::core::ffi::c_int as gint,
            glib_gettext(b"Listener is already closed\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_listener_add_socket(
    mut listener: *mut GSocketListener,
    mut socket: *mut GSocket,
    mut source_object: *mut GObject,
    mut error: *mut *mut GError,
) -> gboolean {
    if safe_c2rust_check_listener(listener, error) == 0 {
        return FALSE;
    }
    if g_socket_is_closed(socket) != 0 {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(b"Added socket is closed\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    g_object_ref(socket as gpointer);
    g_ptr_array_add((*(*listener).priv_0).sockets, socket as gpointer);
    if !source_object.is_null() {
        g_object_set_qdata_full(
            socket as *mut ::core::ffi::c_void as *mut GObject,
            safe_c2rust_source_quark,
            g_object_ref(source_object as gpointer) as *mut GObject as gpointer,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    }
    if (*((*(listener as *mut GTypeInstance)).g_class as *mut GSocketListenerClass))
        .changed
        .is_some()
    {
        (*((*(listener as *mut GTypeInstance)).g_class as *mut GSocketListenerClass))
            .changed
            .expect("non-null function pointer")(listener);
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_listener_add_address(
    mut listener: *mut GSocketListener,
    mut address: *mut GSocketAddress,
    mut type_0: GSocketType,
    mut protocol: GSocketProtocol,
    mut source_object: *mut GObject,
    mut effective_address: *mut *mut GSocketAddress,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut local_address: *mut GSocketAddress = ::core::ptr::null_mut::<GSocketAddress>();
    let mut family: GSocketFamily = G_SOCKET_FAMILY_INVALID;
    let mut socket: *mut GSocket = ::core::ptr::null_mut::<GSocket>();
    if safe_c2rust_check_listener(listener, error) == 0 {
        return FALSE;
    }
    family = g_socket_address_get_family(address);
    socket = g_socket_new(family, type_0, protocol, error);
    if socket.is_null() {
        return FALSE;
    }
    g_socket_set_listen_backlog(socket, (*(*listener).priv_0).listen_backlog as gint);
    g_signal_emit(
        listener as gpointer,
        safe_c2rust_signals[EVENT as ::core::ffi::c_int as usize],
        0 as GQuark,
        G_SOCKET_LISTENER_BINDING as ::core::ffi::c_int,
        socket,
    );
    if g_socket_bind(socket, address, TRUE, error) == 0 {
        g_object_unref(socket as gpointer);
        return FALSE;
    }
    g_signal_emit(
        listener as gpointer,
        safe_c2rust_signals[EVENT as ::core::ffi::c_int as usize],
        0 as GQuark,
        G_SOCKET_LISTENER_BOUND as ::core::ffi::c_int,
        socket,
    );
    g_signal_emit(
        listener as gpointer,
        safe_c2rust_signals[EVENT as ::core::ffi::c_int as usize],
        0 as GQuark,
        G_SOCKET_LISTENER_LISTENING as ::core::ffi::c_int,
        socket,
    );
    if g_socket_listen(socket, error) == 0 {
        g_object_unref(socket as gpointer);
        return FALSE;
    }
    g_signal_emit(
        listener as gpointer,
        safe_c2rust_signals[EVENT as ::core::ffi::c_int as usize],
        0 as GQuark,
        G_SOCKET_LISTENER_LISTENED as ::core::ffi::c_int,
        socket,
    );
    local_address = ::core::ptr::null_mut::<GSocketAddress>();
    if !effective_address.is_null() {
        local_address = g_socket_get_local_address(socket, error);
        if local_address.is_null() {
            g_object_unref(socket as gpointer);
            return FALSE;
        }
    }
    if safe_c2rust_g_socket_listener_add_socket(listener, socket, source_object, error) == 0 {
        if !local_address.is_null() {
            g_object_unref(local_address as gpointer);
        }
        g_object_unref(socket as gpointer);
        return FALSE;
    }
    if !effective_address.is_null() {
        *effective_address = local_address;
    }
    g_object_unref(socket as gpointer);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_listener_add_inet_port(
    mut listener: *mut GSocketListener,
    mut port: guint16,
    mut source_object: *mut GObject,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut need_ipv4_socket: gboolean = TRUE;
    let mut socket4: *mut GSocket = ::core::ptr::null_mut::<GSocket>();
    let mut socket6: *mut GSocket = ::core::ptr::null_mut::<GSocket>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !listener.is_null() {
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
            b"listener != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if port as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
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
            b"port != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if safe_c2rust_check_listener(listener, error) == 0 {
        return FALSE;
    }
    socket6 = g_socket_new(
        G_SOCKET_FAMILY_IPV6,
        G_SOCKET_TYPE_STREAM,
        G_SOCKET_PROTOCOL_DEFAULT,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if !socket6.is_null() {
        let mut inet_address: *mut GInetAddress = ::core::ptr::null_mut::<GInetAddress>();
        let mut address: *mut GSocketAddress = ::core::ptr::null_mut::<GSocketAddress>();
        inet_address = g_inet_address_new_any(G_SOCKET_FAMILY_IPV6);
        address = g_inet_socket_address_new(inet_address, port);
        g_object_unref(inet_address as gpointer);
        g_socket_set_listen_backlog(socket6, (*(*listener).priv_0).listen_backlog as gint);
        g_signal_emit(
            listener as gpointer,
            safe_c2rust_signals[EVENT as ::core::ffi::c_int as usize],
            0 as GQuark,
            G_SOCKET_LISTENER_BINDING as ::core::ffi::c_int,
            socket6,
        );
        if g_socket_bind(socket6, address, TRUE, error) == 0 {
            g_object_unref(address as gpointer);
            g_object_unref(socket6 as gpointer);
            return FALSE;
        }
        g_object_unref(address as gpointer);
        g_signal_emit(
            listener as gpointer,
            safe_c2rust_signals[EVENT as ::core::ffi::c_int as usize],
            0 as GQuark,
            G_SOCKET_LISTENER_BOUND as ::core::ffi::c_int,
            socket6,
        );
        g_signal_emit(
            listener as gpointer,
            safe_c2rust_signals[EVENT as ::core::ffi::c_int as usize],
            0 as GQuark,
            G_SOCKET_LISTENER_LISTENING as ::core::ffi::c_int,
            socket6,
        );
        if g_socket_listen(socket6, error) == 0 {
            g_object_unref(socket6 as gpointer);
            return FALSE;
        }
        g_signal_emit(
            listener as gpointer,
            safe_c2rust_signals[EVENT as ::core::ffi::c_int as usize],
            0 as GQuark,
            G_SOCKET_LISTENER_LISTENED as ::core::ffi::c_int,
            socket6,
        );
        if !source_object.is_null() {
            g_object_set_qdata_full(
                socket6 as *mut ::core::ffi::c_void as *mut GObject,
                safe_c2rust_source_quark,
                g_object_ref(source_object as gpointer) as *mut GObject as gpointer,
                Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
            );
        }
        if g_socket_speaks_ipv4(socket6) != 0 {
            need_ipv4_socket = FALSE as gboolean;
        }
    }
    if need_ipv4_socket != 0 {
        socket4 = g_socket_new(
            G_SOCKET_FAMILY_IPV4,
            G_SOCKET_TYPE_STREAM,
            G_SOCKET_PROTOCOL_DEFAULT,
            error,
        );
        if !socket4.is_null() {
            let mut inet_address_0: *mut GInetAddress = ::core::ptr::null_mut::<GInetAddress>();
            let mut address_0: *mut GSocketAddress = ::core::ptr::null_mut::<GSocketAddress>();
            inet_address_0 = g_inet_address_new_any(G_SOCKET_FAMILY_IPV4);
            address_0 = g_inet_socket_address_new(inet_address_0, port);
            g_object_unref(inet_address_0 as gpointer);
            g_socket_set_listen_backlog(socket4, (*(*listener).priv_0).listen_backlog as gint);
            g_signal_emit(
                listener as gpointer,
                safe_c2rust_signals[EVENT as ::core::ffi::c_int as usize],
                0 as GQuark,
                G_SOCKET_LISTENER_BINDING as ::core::ffi::c_int,
                socket4,
            );
            if g_socket_bind(socket4, address_0, TRUE, error) == 0 {
                g_object_unref(address_0 as gpointer);
                g_object_unref(socket4 as gpointer);
                if !socket6.is_null() {
                    g_object_unref(socket6 as gpointer);
                }
                return FALSE;
            }
            g_object_unref(address_0 as gpointer);
            g_signal_emit(
                listener as gpointer,
                safe_c2rust_signals[EVENT as ::core::ffi::c_int as usize],
                0 as GQuark,
                G_SOCKET_LISTENER_BOUND as ::core::ffi::c_int,
                socket4,
            );
            g_signal_emit(
                listener as gpointer,
                safe_c2rust_signals[EVENT as ::core::ffi::c_int as usize],
                0 as GQuark,
                G_SOCKET_LISTENER_LISTENING as ::core::ffi::c_int,
                socket4,
            );
            if g_socket_listen(socket4, error) == 0 {
                g_object_unref(socket4 as gpointer);
                if !socket6.is_null() {
                    g_object_unref(socket6 as gpointer);
                }
                return FALSE;
            }
            g_signal_emit(
                listener as gpointer,
                safe_c2rust_signals[EVENT as ::core::ffi::c_int as usize],
                0 as GQuark,
                G_SOCKET_LISTENER_LISTENED as ::core::ffi::c_int,
                socket4,
            );
            if !source_object.is_null() {
                g_object_set_qdata_full(
                    socket4 as *mut ::core::ffi::c_void as *mut GObject,
                    safe_c2rust_source_quark,
                    g_object_ref(source_object as gpointer) as *mut GObject as gpointer,
                    Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
                );
            }
        } else if !socket6.is_null() {
            g_clear_error(error);
        } else {
            return FALSE;
        }
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !socket6.is_null() || !socket4.is_null() {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocketlistener.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            590 as ::core::ffi::c_int,
            G_STRFUNC,
            b"socket6 != NULL || socket4 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !socket6.is_null() {
        g_ptr_array_add((*(*listener).priv_0).sockets, socket6 as gpointer);
    }
    if !socket4.is_null() {
        g_ptr_array_add((*(*listener).priv_0).sockets, socket4 as gpointer);
    }
    if (*((*(listener as *mut GTypeInstance)).g_class as *mut GSocketListenerClass))
        .changed
        .is_some()
    {
        (*((*(listener as *mut GTypeInstance)).g_class as *mut GSocketListenerClass))
            .changed
            .expect("non-null function pointer")(listener);
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_add_sources(
    mut listener: *mut GSocketListener,
    mut callback: GSocketSourceFunc,
    mut callback_data: gpointer,
    mut cancellable: *mut GCancellable,
    mut context: *mut GMainContext,
) -> *mut GList {
    let mut socket: *mut GSocket = ::core::ptr::null_mut::<GSocket>();
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    let mut sources: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut i: guint = 0;
    sources = ::core::ptr::null_mut::<GList>();
    i = 0 as guint;
    while i < (*(*(*listener).priv_0).sockets).len {
        socket = *(*(*(*listener).priv_0).sockets).pdata.offset(i as isize) as *mut GSocket;
        source = g_socket_create_source(socket, G_IO_IN, cancellable);
        g_source_set_callback(
            source,
            ::core::mem::transmute::<GSocketSourceFunc, GSourceFunc>(callback),
            callback_data,
            None,
        );
        g_source_attach(source, context);
        sources = g_list_prepend(sources, source as gpointer);
        i = i.wrapping_add(1);
    }
    return sources;
}
unsafe extern "C" fn safe_c2rust_free_sources(mut sources: *mut GList) {
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    while !sources.is_null() {
        source = (*sources).data as *mut GSource;
        sources = g_list_delete_link(sources, sources);
        g_source_destroy(source);
        g_source_unref(source);
    }
}
unsafe extern "C" fn safe_c2rust_accept_callback(
    mut socket: *mut GSocket,
    mut condition: GIOCondition,
    mut user_data: gpointer,
) -> gboolean {
    let mut data: *mut AcceptData = user_data as *mut AcceptData;
    (*data).socket = socket;
    g_main_loop_quit((*data).loop_0);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_listener_accept_socket(
    mut listener: *mut GSocketListener,
    mut source_object: *mut *mut GObject,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GSocket {
    let mut accept_socket: *mut GSocket = ::core::ptr::null_mut::<GSocket>();
    let mut socket: *mut GSocket = ::core::ptr::null_mut::<GSocket>();
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = listener as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_listener_get_type();
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
            b"G_IS_SOCKET_LISTENER (listener)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSocket>();
    }
    if safe_c2rust_check_listener(listener, error) == 0 {
        return ::core::ptr::null_mut::<GSocket>();
    }
    if (*(*(*listener).priv_0).sockets).len == 1 as guint {
        accept_socket = *(*(*(*listener).priv_0).sockets)
            .pdata
            .offset(0 as ::core::ffi::c_int as isize) as *mut GSocket;
        if g_socket_condition_wait(accept_socket, G_IO_IN, cancellable, error) == 0 {
            return ::core::ptr::null_mut::<GSocket>();
        }
    } else {
        let mut sources: *mut GList = ::core::ptr::null_mut::<GList>();
        let mut data: AcceptData = AcceptData {
            loop_0: ::core::ptr::null_mut::<GMainLoop>(),
            socket: ::core::ptr::null_mut::<GSocket>(),
        };
        let mut loop_0: *mut GMainLoop = ::core::ptr::null_mut::<GMainLoop>();
        if (*(*listener).priv_0).main_context.is_null() {
            (*(*listener).priv_0).main_context = g_main_context_new();
        }
        loop_0 = g_main_loop_new((*(*listener).priv_0).main_context, FALSE);
        data.loop_0 = loop_0;
        sources = safe_c2rust_add_sources(
            listener,
            Some(
                safe_c2rust_accept_callback
                    as unsafe extern "C" fn(*mut GSocket, GIOCondition, gpointer) -> gboolean,
            ),
            &raw mut data as gpointer,
            cancellable,
            (*(*listener).priv_0).main_context,
        );
        g_main_loop_run(loop_0);
        accept_socket = data.socket;
        safe_c2rust_free_sources(sources);
        g_main_loop_unref(loop_0);
    }
    socket = g_socket_accept(accept_socket, cancellable, error);
    if socket.is_null() {
        return ::core::ptr::null_mut::<GSocket>();
    }
    if !source_object.is_null() {
        *source_object = g_object_get_qdata(
            accept_socket as *mut ::core::ffi::c_void as *mut GObject,
            safe_c2rust_source_quark,
        ) as *mut GObject;
    }
    return socket;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_listener_accept(
    mut listener: *mut GSocketListener,
    mut source_object: *mut *mut GObject,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GSocketConnection {
    let mut connection: *mut GSocketConnection = ::core::ptr::null_mut::<GSocketConnection>();
    let mut socket: *mut GSocket = ::core::ptr::null_mut::<GSocket>();
    socket =
        safe_c2rust_g_socket_listener_accept_socket(listener, source_object, cancellable, error);
    if socket.is_null() {
        return ::core::ptr::null_mut::<GSocketConnection>();
    }
    connection = g_socket_connection_factory_create_connection(socket);
    g_object_unref(socket as gpointer);
    return connection;
}
unsafe extern "C" fn safe_c2rust_accept_socket_async_data_free(
    mut data: *mut AcceptSocketAsyncData,
) {
    safe_c2rust_free_sources((*data).sources);
    g_free(data as gpointer);
}
unsafe extern "C" fn safe_c2rust_accept_ready(
    mut accept_socket: *mut GSocket,
    mut condition: GIOCondition,
    mut user_data: gpointer,
) -> gboolean {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut socket: *mut GSocket = ::core::ptr::null_mut::<GSocket>();
    let mut source_object: *mut GObject = ::core::ptr::null_mut::<GObject>();
    let mut data: *mut AcceptSocketAsyncData =
        g_task_get_task_data(task) as *mut AcceptSocketAsyncData;
    if (*data).returned_yet != 0 {
        return G_SOURCE_REMOVE;
    }
    socket = g_socket_accept(accept_socket, g_task_get_cancellable(task), &raw mut error);
    if !socket.is_null() {
        source_object = g_object_get_qdata(
            accept_socket as *mut ::core::ffi::c_void as *mut GObject,
            safe_c2rust_source_quark,
        ) as *mut GObject;
        if !source_object.is_null() {
            g_object_set_qdata_full(
                task as *mut ::core::ffi::c_void as *mut GObject,
                safe_c2rust_source_quark,
                g_object_ref(source_object as gpointer) as *mut GObject as gpointer,
                Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
            );
        }
        g_task_return_pointer(
            task,
            socket as gpointer,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    } else {
        g_task_return_error(task, error);
    }
    (*data).returned_yet = TRUE as gboolean;
    g_object_unref(task as gpointer);
    return G_SOURCE_REMOVE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_listener_accept_socket_async(
    mut listener: *mut GSocketListener,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut data: *mut AcceptSocketAsyncData = ::core::ptr::null_mut::<AcceptSocketAsyncData>();
    task = g_task_new(listener as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GSocketListener,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_socket_listener_accept_socket_async
                as unsafe extern "C" fn(
                    *mut GSocketListener,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"g_socket_listener_accept_socket_async\0" as *const u8 as *const gchar,
        );
    }
    if safe_c2rust_check_listener(listener, &raw mut error) == 0 {
        g_task_return_error(task, error);
        g_object_unref(task as gpointer);
        return;
    }
    data = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<AcceptSocketAsyncData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut AcceptSocketAsyncData;
    (*data).returned_yet = FALSE as gboolean;
    (*data).sources = safe_c2rust_add_sources(
        listener,
        Some(
            safe_c2rust_accept_ready
                as unsafe extern "C" fn(*mut GSocket, GIOCondition, gpointer) -> gboolean,
        ),
        task as gpointer,
        cancellable,
        g_main_context_get_thread_default(),
    );
    g_task_set_task_data(
        task,
        safe_c2rust_g_steal_pointer(&raw mut data as gpointer) as *mut AcceptSocketAsyncData
            as gpointer,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut AcceptSocketAsyncData) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_accept_socket_async_data_free
                as unsafe extern "C" fn(*mut AcceptSocketAsyncData) -> (),
        )),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_listener_accept_socket_finish(
    mut listener: *mut GSocketListener,
    mut result: *mut GAsyncResult,
    mut source_object: *mut *mut GObject,
    mut error: *mut *mut GError,
) -> *mut GSocket {
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = listener as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_listener_get_type();
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
            b"G_IS_SOCKET_LISTENER (listener)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSocket>();
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, listener as gpointer) != 0 {
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
            b"g_task_is_valid (result, listener)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSocket>();
    }
    if !source_object.is_null() {
        *source_object = g_object_get_qdata(
            result as *mut ::core::ffi::c_void as *mut GObject,
            safe_c2rust_source_quark,
        ) as *mut GObject;
    }
    return g_task_propagate_pointer(result as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GSocket;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_listener_accept_async(
    mut listener: *mut GSocketListener,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    safe_c2rust_g_socket_listener_accept_socket_async(listener, cancellable, callback, user_data);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_listener_accept_finish(
    mut listener: *mut GSocketListener,
    mut result: *mut GAsyncResult,
    mut source_object: *mut *mut GObject,
    mut error: *mut *mut GError,
) -> *mut GSocketConnection {
    let mut socket: *mut GSocket = ::core::ptr::null_mut::<GSocket>();
    let mut connection: *mut GSocketConnection = ::core::ptr::null_mut::<GSocketConnection>();
    socket =
        safe_c2rust_g_socket_listener_accept_socket_finish(listener, result, source_object, error);
    if socket.is_null() {
        return ::core::ptr::null_mut::<GSocketConnection>();
    }
    connection = g_socket_connection_factory_create_connection(socket);
    g_object_unref(socket as gpointer);
    return connection;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_listener_set_backlog(
    mut listener: *mut GSocketListener,
    mut listen_backlog: ::core::ffi::c_int,
) {
    let mut socket: *mut GSocket = ::core::ptr::null_mut::<GSocket>();
    let mut i: guint = 0;
    if (*(*listener).priv_0).closed() != 0 {
        return;
    }
    (*(*listener).priv_0).listen_backlog = listen_backlog;
    i = 0 as guint;
    while i < (*(*(*listener).priv_0).sockets).len {
        socket = *(*(*(*listener).priv_0).sockets).pdata.offset(i as isize) as *mut GSocket;
        g_socket_set_listen_backlog(socket, listen_backlog as gint);
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_listener_close(mut listener: *mut GSocketListener) {
    let mut socket: *mut GSocket = ::core::ptr::null_mut::<GSocket>();
    let mut i: guint = 0;
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = listener as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_listener_get_type();
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
            b"G_IS_SOCKET_LISTENER (listener)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*(*listener).priv_0).closed() != 0 {
        return;
    }
    i = 0 as guint;
    while i < (*(*(*listener).priv_0).sockets).len {
        socket = *(*(*(*listener).priv_0).sockets).pdata.offset(i as isize) as *mut GSocket;
        g_socket_close(socket, ::core::ptr::null_mut::<*mut GError>());
        i = i.wrapping_add(1);
    }
    (*(*listener).priv_0).set_closed(TRUE as guint as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_listener_add_any_inet_port(
    mut listener: *mut GSocketListener,
    mut source_object: *mut GObject,
    mut error: *mut *mut GError,
) -> guint16 {
    let mut sockets_to_close: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut candidate_port: guint16 = 0 as guint16;
    let mut socket6: *mut GSocket = ::core::ptr::null_mut::<GSocket>();
    let mut socket4: *mut GSocket = ::core::ptr::null_mut::<GSocket>();
    let mut attempts: gint = 37 as gint;
    loop {
        let fresh0 = attempts;
        attempts = attempts - 1;
        if !(fresh0 != 0) {
            break;
        }
        let mut inet_address: *mut GInetAddress = ::core::ptr::null_mut::<GInetAddress>();
        let mut address: *mut GSocketAddress = ::core::ptr::null_mut::<GSocketAddress>();
        let mut result: gboolean = 0;
        if ({
            let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
            if socket6.is_null() {
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocketlistener.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1084 as ::core::ffi::c_int,
                G_STRFUNC,
                b"socket6 == NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        socket6 = g_socket_new(
            G_SOCKET_FAMILY_IPV6,
            G_SOCKET_TYPE_STREAM,
            G_SOCKET_PROTOCOL_DEFAULT,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        if !socket6.is_null() {
            inet_address = g_inet_address_new_any(G_SOCKET_FAMILY_IPV6);
            address = g_inet_socket_address_new(inet_address, 0 as guint16);
            g_object_unref(inet_address as gpointer);
            g_signal_emit(
                listener as gpointer,
                safe_c2rust_signals[EVENT as ::core::ffi::c_int as usize],
                0 as GQuark,
                G_SOCKET_LISTENER_BINDING as ::core::ffi::c_int,
                socket6,
            );
            result = g_socket_bind(socket6, address, TRUE, error);
            g_object_unref(address as gpointer);
            if result == 0 || {
                address = g_socket_get_local_address(socket6, error);
                address.is_null()
            } {
                g_object_unref(socket6 as gpointer);
                socket6 = ::core::ptr::null_mut::<GSocket>();
                break;
            } else {
                g_signal_emit(
                    listener as gpointer,
                    safe_c2rust_signals[EVENT as ::core::ffi::c_int as usize],
                    0 as GQuark,
                    G_SOCKET_LISTENER_BOUND as ::core::ffi::c_int,
                    socket6,
                );
                if ({
                    let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
                    if ({
                        let mut __inst: *mut GTypeInstance = address as *mut GTypeInstance;
                        let mut __t: GType = g_inet_socket_address_get_type();
                        let mut __r: gboolean = 0;
                        if __inst.is_null() {
                            __r = 0 as ::core::ffi::c_int as gboolean;
                        } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t
                        {
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
                    g_assertion_message_expr(
                        G_LOG_DOMAIN.as_ptr(),
                        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocketlistener.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        1113 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"G_IS_INET_SOCKET_ADDRESS (address)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                candidate_port = g_inet_socket_address_get_port(
                    address as *mut ::core::ffi::c_void as *mut GInetSocketAddress,
                );
                if ({
                    let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
                    if candidate_port as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
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
                        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocketlistener.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        1116 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"candidate_port != 0\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                g_object_unref(address as gpointer);
                if g_socket_speaks_ipv4(socket6) != 0 {
                    break;
                }
            }
        }
        if ({
            let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
            if socket4.is_null() {
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocketlistener.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1123 as ::core::ffi::c_int,
                G_STRFUNC,
                b"socket4 == NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        socket4 = g_socket_new(
            G_SOCKET_FAMILY_IPV4,
            G_SOCKET_TYPE_STREAM,
            G_SOCKET_PROTOCOL_DEFAULT,
            if !socket6.is_null() {
                ::core::ptr::null_mut::<*mut GError>()
            } else {
                error
            },
        );
        if socket4.is_null() {
            break;
        }
        inet_address = g_inet_address_new_any(G_SOCKET_FAMILY_IPV4);
        address = g_inet_socket_address_new(inet_address, candidate_port);
        g_object_unref(inet_address as gpointer);
        g_signal_emit(
            listener as gpointer,
            safe_c2rust_signals[EVENT as ::core::ffi::c_int as usize],
            0 as GQuark,
            G_SOCKET_LISTENER_BINDING as ::core::ffi::c_int,
            socket4,
        );
        result = g_socket_bind(
            socket4,
            address,
            TRUE,
            if candidate_port as ::core::ffi::c_int != 0 && attempts != 0 {
                ::core::ptr::null_mut::<*mut GError>()
            } else {
                error
            },
        );
        g_object_unref(address as gpointer);
        if candidate_port != 0 {
            if ({
                let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
                if !socket6.is_null() {
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
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocketlistener.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    1166 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"socket6 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if result != 0 {
                g_signal_emit(
                    listener as gpointer,
                    safe_c2rust_signals[EVENT as ::core::ffi::c_int as usize],
                    0 as GQuark,
                    G_SOCKET_LISTENER_BOUND as ::core::ffi::c_int,
                    socket4,
                );
                break;
            } else {
                g_object_unref(socket4 as gpointer);
                socket4 = ::core::ptr::null_mut::<GSocket>();
                sockets_to_close = g_slist_prepend(sockets_to_close, socket6 as gpointer);
                candidate_port = 0 as guint16;
                socket6 = ::core::ptr::null_mut::<GSocket>();
            }
        } else {
            if ({
                let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
                if socket6.is_null() {
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
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocketlistener.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    1194 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"socket6 == NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if result == 0 || {
                address = g_socket_get_local_address(socket4, error);
                address.is_null()
            } {
                g_object_unref(socket4 as gpointer);
                socket4 = ::core::ptr::null_mut::<GSocket>();
                break;
            } else {
                g_signal_emit(
                    listener as gpointer,
                    safe_c2rust_signals[EVENT as ::core::ffi::c_int as usize],
                    0 as GQuark,
                    G_SOCKET_LISTENER_BOUND as ::core::ffi::c_int,
                    socket4,
                );
                if ({
                    let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
                    if ({
                        let mut __inst: *mut GTypeInstance = address as *mut GTypeInstance;
                        let mut __t: GType = g_inet_socket_address_get_type();
                        let mut __r: gboolean = 0;
                        if __inst.is_null() {
                            __r = 0 as ::core::ffi::c_int as gboolean;
                        } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t
                        {
                            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
                        } else {
                            __r = g_type_check_instance_is_a(__inst, __t);
                        }
                        __r
                    }) != 0
                    {
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
                        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocketlistener.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        1207 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"G_IS_INET_SOCKET_ADDRESS (address)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                candidate_port = g_inet_socket_address_get_port(
                    address as *mut ::core::ffi::c_void as *mut GInetSocketAddress,
                );
                if ({
                    let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
                    if candidate_port as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                        _g_boolean_var_24 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_24 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_24
                }) as ::core::ffi::c_long
                    != 0
                {
                } else {
                    g_assertion_message_expr(
                        G_LOG_DOMAIN.as_ptr(),
                        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocketlistener.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        1210 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"candidate_port != 0\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                g_object_unref(address as gpointer);
                break;
            }
        }
    }
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if (candidate_port as ::core::ffi::c_int != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
            == (!socket4.is_null() || !socket6.is_null()) as ::core::ffi::c_int
        {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocketlistener.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1217 as ::core::ffi::c_int,
            G_STRFUNC,
            b"(candidate_port != 0) == (socket4 || socket6)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    while !sockets_to_close.is_null() {
        g_object_unref((*sockets_to_close).data);
        sockets_to_close = g_slist_delete_link(sockets_to_close, sockets_to_close);
    }
    if !socket6.is_null() {
        g_socket_set_listen_backlog(socket6, (*(*listener).priv_0).listen_backlog as gint);
        g_signal_emit(
            listener as gpointer,
            safe_c2rust_signals[EVENT as ::core::ffi::c_int as usize],
            0 as GQuark,
            G_SOCKET_LISTENER_LISTENING as ::core::ffi::c_int,
            socket6,
        );
        if g_socket_listen(socket6, error) == 0 {
            g_object_unref(socket6 as gpointer);
            if !socket4.is_null() {
                g_object_unref(socket4 as gpointer);
            }
            return 0 as guint16;
        }
        g_signal_emit(
            listener as gpointer,
            safe_c2rust_signals[EVENT as ::core::ffi::c_int as usize],
            0 as GQuark,
            G_SOCKET_LISTENER_LISTENED as ::core::ffi::c_int,
            socket6,
        );
        if !source_object.is_null() {
            g_object_set_qdata_full(
                socket6 as *mut ::core::ffi::c_void as *mut GObject,
                safe_c2rust_source_quark,
                g_object_ref(source_object as gpointer) as *mut GObject as gpointer,
                Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
            );
        }
        g_ptr_array_add((*(*listener).priv_0).sockets, socket6 as gpointer);
    }
    if !socket4.is_null() {
        g_socket_set_listen_backlog(socket4, (*(*listener).priv_0).listen_backlog as gint);
        g_signal_emit(
            listener as gpointer,
            safe_c2rust_signals[EVENT as ::core::ffi::c_int as usize],
            0 as GQuark,
            G_SOCKET_LISTENER_LISTENING as ::core::ffi::c_int,
            socket4,
        );
        if g_socket_listen(socket4, error) == 0 {
            g_object_unref(socket4 as gpointer);
            if !socket6.is_null() {
                g_object_unref(socket6 as gpointer);
            }
            return 0 as guint16;
        }
        g_signal_emit(
            listener as gpointer,
            safe_c2rust_signals[EVENT as ::core::ffi::c_int as usize],
            0 as GQuark,
            G_SOCKET_LISTENER_LISTENED as ::core::ffi::c_int,
            socket4,
        );
        if !source_object.is_null() {
            g_object_set_qdata_full(
                socket4 as *mut ::core::ffi::c_void as *mut GObject,
                safe_c2rust_source_quark,
                g_object_ref(source_object as gpointer) as *mut GObject as gpointer,
                Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
            );
        }
        g_ptr_array_add((*(*listener).priv_0).sockets, socket4 as gpointer);
    }
    if (!socket4.is_null() || !socket6.is_null())
        && (*((*(listener as *mut GTypeInstance)).g_class as *mut GSocketListenerClass))
            .changed
            .is_some()
    {
        (*((*(listener as *mut GTypeInstance)).g_class as *mut GSocketListenerClass))
            .changed
            .expect("non-null function pointer")(listener);
    }
    return candidate_port;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
