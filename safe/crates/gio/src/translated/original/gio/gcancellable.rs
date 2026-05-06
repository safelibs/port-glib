use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GDir;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GWakeup;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_cond_wait(cond: *mut GCond, mutex: *mut GMutex);
    fn g_cond_broadcast(cond: *mut GCond);
    fn g_private_get(key: *mut GPrivate) -> gpointer;
    fn g_private_set(key: *mut GPrivate, value: gpointer);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_slist_delete_link(list: *mut GSList, link_: *mut GSList) -> *mut GSList;
    fn g_source_new(source_funcs: *mut GSourceFuncs, struct_size: guint) -> *mut GSource;
    fn g_source_set_dispose_function(source: *mut GSource, dispose: GSourceDisposeFunc);
    fn g_source_ref(source: *mut GSource) -> *mut GSource;
    fn g_source_unref(source: *mut GSource);
    fn g_source_set_static_name(source: *mut GSource, name: *const ::core::ffi::c_char);
    fn g_source_set_ready_time(source: *mut GSource, ready_time: gint64);
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
    fn g_value_init(value: *mut GValue, g_type: GType) -> *mut GValue;
    fn g_value_unset(value: *mut GValue);
    fn g_closure_invoke(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
    );
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
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_value_set_object(value: *mut GValue, v_object: gpointer);
    fn g_value_get_boolean(value: *const GValue) -> gboolean;
    fn g_io_error_quark() -> GQuark;
    fn glib__private__() -> *const GLibPrivateVTable;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GMutex = _GMutex;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCond {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GCond = _GCond;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPrivate {
    pub p: gpointer,
    pub notify: GDestroyNotify,
    pub future: [gpointer; 2],
}
pub type GPrivate = _GPrivate;
pub type GData = _GData;
pub type GDir = _GDir;
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
pub type GSourceDisposeFunc = Option<unsafe extern "C" fn(*mut GSource) -> ()>;
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
pub type GWakeup = _GWakeup;
pub type GDataListUpdateAtomicFunc =
    Option<unsafe extern "C" fn(GQuark, *mut gpointer, *mut GDestroyNotify, gpointer) -> gpointer>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GWin32InvalidParameterHandler {
    pub unused_really: ::core::ffi::c_int,
}
pub type GWin32InvalidParameterHandler = _GWin32InvalidParameterHandler;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GCancellablePrivate {
    pub cancelled: gboolean,
    #[bitfield(name = "cancelled_running", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "cancelled_running_waiting", ty = "guint", bits = "1..=1")]
    pub cancelled_running_cancelled_running_waiting: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub cancelled_emissions: ::core::ffi::c_uint,
    #[bitfield(
        name = "cancelled_emissions_waiting",
        ty = "::core::ffi::c_uint",
        bits = "0..=0"
    )]
    pub cancelled_emissions_waiting: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
    pub fd_refcount: guint,
    pub wakeup: *mut GWakeup,
}
pub type GCancellable = _GCancellable;
pub type GCancellableSourceFunc =
    Option<unsafe extern "C" fn(*mut GCancellable, gpointer) -> gboolean>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellableClass {
    pub parent_class: GObjectClass,
    pub cancelled: Option<unsafe extern "C" fn(*mut GCancellable) -> ()>,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
}
pub type GCancellableClass = _GCancellableClass;
pub const CANCELLED: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GCancellableSource {
    pub source: GSource,
    pub cancellable: *mut GCancellable,
    pub cancelled_handler: gulong,
    pub resurrected_during_cancellation: gboolean,
}
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const LAST_SIGNAL: C2RustUnnamed_1 = 1;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_NONE: GType = ((1 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_BOOLEAN: GType = ((5 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
static mut safe_c2rust_signals: [guint; 1] = [0 as ::core::ffi::c_int as guint];
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_cancellable_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GCancellable\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GCancellableClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_cancellable_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GCancellable>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GCancellable) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_cancellable_init as unsafe extern "C" fn(*mut GCancellable) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GCancellable_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GCancellablePrivate>() as gsize,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_cancellable_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_cancellable_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GCancellable_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut safe_c2rust_GCancellable_private_offset);
    }
    safe_c2rust_g_cancellable_class_init(klass as *mut GCancellableClass);
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_cancellable_get_instance_private(
    mut self_0: *mut GCancellable,
) -> gpointer {
    return (self_0 as *mut guint8).offset(safe_c2rust_GCancellable_private_offset as glong as isize)
        as gpointer;
}
static mut safe_c2rust_g_cancellable_parent_class: gpointer = NULL;
static mut safe_c2rust_GCancellable_private_offset: gint = 0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_cancellable_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_cancellable_get_type_once();
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
static mut safe_c2rust_current_cancellable: GPrivate = _GPrivate {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    notify: None,
    future: [::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void; 2],
};
static mut safe_c2rust_cancellable_mutex: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
static mut safe_c2rust_cancellable_cond: GCond = _GCond {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    i: [0; 2],
};
unsafe extern "C" fn safe_c2rust_g_cancellable_finalize(mut object: *mut GObject) {
    let mut cancellable: *mut GCancellable =
        object as *mut ::core::ffi::c_void as *mut GCancellable;
    if !(*(*cancellable).priv_0).wakeup.is_null() {
        (*glib__private__())
            .g_wakeup_free
            .expect("non-null function pointer")((*(*cancellable).priv_0).wakeup);
    }
    (*(safe_c2rust_g_cancellable_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_cancellable_class_init(mut klass: *mut GCancellableClass) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize =
        Some(safe_c2rust_g_cancellable_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    safe_c2rust_signals[CANCELLED as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"cancelled\0" as *const u8 as *const gchar),
        (*(gobject_class as *mut GTypeClass)).g_type,
        G_SIGNAL_RUN_LAST,
        136 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL,
        None,
        G_TYPE_NONE,
        0 as guint,
    );
}
unsafe extern "C" fn safe_c2rust_g_cancellable_init(mut cancellable: *mut GCancellable) {
    (*cancellable).priv_0 =
        safe_c2rust_g_cancellable_get_instance_private(cancellable) as *mut GCancellablePrivate;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_cancellable_new() -> *mut GCancellable {
    return g_object_new(
        safe_c2rust_g_cancellable_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GCancellable;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_cancellable_push_current(
    mut cancellable: *mut GCancellable,
) {
    let mut l: *mut GSList = ::core::ptr::null_mut::<GSList>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !cancellable.is_null() {
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
            b"cancellable != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    l = g_private_get(&raw mut safe_c2rust_current_cancellable) as *mut GSList;
    l = g_slist_prepend(l, cancellable as gpointer);
    g_private_set(&raw mut safe_c2rust_current_cancellable, l as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_cancellable_pop_current(mut cancellable: *mut GCancellable) {
    let mut l: *mut GSList = ::core::ptr::null_mut::<GSList>();
    l = g_private_get(&raw mut safe_c2rust_current_cancellable) as *mut GSList;
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !l.is_null() {
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
            b"l != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if (*l).data == cancellable as gpointer {
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
            b"l->data == cancellable\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    l = g_slist_delete_link(l, l);
    g_private_set(&raw mut safe_c2rust_current_cancellable, l as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_cancellable_get_current() -> *mut GCancellable {
    let mut l: *mut GSList = ::core::ptr::null_mut::<GSList>();
    l = g_private_get(&raw mut safe_c2rust_current_cancellable) as *mut GSList;
    if l.is_null() {
        return ::core::ptr::null_mut::<GCancellable>();
    }
    return (*l).data as *mut GCancellable;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_cancellable_reset(mut cancellable: *mut GCancellable) {
    let mut priv_0: *mut GCancellablePrivate = ::core::ptr::null_mut::<GCancellablePrivate>();
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = cancellable as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_cancellable_get_type();
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
            b"G_IS_CANCELLABLE (cancellable)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut safe_c2rust_cancellable_mutex);
    priv_0 = (*cancellable).priv_0;
    while (*priv_0).cancelled_running() as ::core::ffi::c_int != 0
        || (*priv_0).cancelled_emissions > 0 as ::core::ffi::c_uint
    {
        if (*priv_0).cancelled_running() != 0 {
            (*priv_0).set_cancelled_running_waiting(TRUE as guint as guint);
        }
        if (*priv_0).cancelled_emissions > 0 as ::core::ffi::c_uint {
            (*priv_0).set_cancelled_emissions_waiting(
                TRUE as ::core::ffi::c_uint as ::core::ffi::c_uint,
            );
        }
        g_cond_wait(
            &raw mut safe_c2rust_cancellable_cond,
            &raw mut safe_c2rust_cancellable_mutex,
        );
    }
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*priv_0).cancelled;
        } else {
        };
        crate::translated::compat::atomic_xchg_seqcst(
            &raw mut (*priv_0).cancelled,
            0 as ::core::ffi::c_int,
        )
    }) != 0
    {
        if !(*priv_0).wakeup.is_null() {
            (*glib__private__())
                .g_wakeup_acknowledge
                .expect("non-null function pointer")((*priv_0).wakeup);
        }
    }
    g_mutex_unlock(&raw mut safe_c2rust_cancellable_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_cancellable_is_cancelled(
    mut cancellable: *mut GCancellable,
) -> gboolean {
    return (!cancellable.is_null()
        && ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*(*cancellable).priv_0).cancelled;
                (*(*cancellable).priv_0).cancelled;
            } else {
            };
            *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
                &raw mut (*(*cancellable).priv_0).cancelled as *mut gint,
            );
            gaig_temp
        }) != 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_cancellable_set_error_if_cancelled(
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    if safe_c2rust_g_cancellable_is_cancelled(cancellable) != 0 {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_CANCELLED as ::core::ffi::c_int as gint,
            glib_gettext(b"Operation was cancelled\0" as *const u8 as *const gchar),
        );
        return TRUE;
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_cancellable_get_fd(
    mut cancellable: *mut GCancellable,
) -> ::core::ffi::c_int {
    let mut pollfd: GPollFD = _GPollFD {
        fd: 0,
        events: 0,
        revents: 0,
    };
    let mut retval: gboolean = 0;
    if cancellable.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    retval = safe_c2rust_g_cancellable_make_pollfd(cancellable, &raw mut pollfd);
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if retval != 0 {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gcancellable.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            369 as ::core::ffi::c_int,
            G_STRFUNC,
            b"retval\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return pollfd.fd as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_cancellable_make_pollfd(
    mut cancellable: *mut GCancellable,
    mut pollfd: *mut GPollFD,
) -> gboolean {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !pollfd.is_null() {
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
            b"pollfd != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if cancellable.is_null() {
        return FALSE;
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = cancellable as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_cancellable_get_type();
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
            b"G_IS_CANCELLABLE (cancellable)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    g_mutex_lock(&raw mut safe_c2rust_cancellable_mutex);
    (*(*cancellable).priv_0).fd_refcount = (*(*cancellable).priv_0).fd_refcount.wrapping_add(1);
    if (*(*cancellable).priv_0).wakeup.is_null() {
        (*(*cancellable).priv_0).wakeup = (*glib__private__())
            .g_wakeup_new
            .expect("non-null function pointer")();
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*(*cancellable).priv_0).cancelled;
                (*(*cancellable).priv_0).cancelled;
            } else {
            };
            *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
                &raw mut (*(*cancellable).priv_0).cancelled as *mut gint,
            );
            gaig_temp
        }) != 0
        {
            (*glib__private__())
                .g_wakeup_signal
                .expect("non-null function pointer")((*(*cancellable).priv_0).wakeup);
        }
    }
    (*glib__private__())
        .g_wakeup_get_pollfd
        .expect("non-null function pointer")((*(*cancellable).priv_0).wakeup, pollfd);
    g_mutex_unlock(&raw mut safe_c2rust_cancellable_mutex);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_cancellable_release_fd(mut cancellable: *mut GCancellable) {
    let mut priv_0: *mut GCancellablePrivate = ::core::ptr::null_mut::<GCancellablePrivate>();
    if cancellable.is_null() {
        return;
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = cancellable as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_cancellable_get_type();
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
            b"G_IS_CANCELLABLE (cancellable)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    priv_0 = (*cancellable).priv_0;
    g_mutex_lock(&raw mut safe_c2rust_cancellable_mutex);
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if (*priv_0).fd_refcount > 0 as guint {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gcancellable.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            460 as ::core::ffi::c_int,
            G_STRFUNC,
            b"priv->fd_refcount > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*priv_0).fd_refcount = (*priv_0).fd_refcount.wrapping_sub(1);
    if (*priv_0).fd_refcount == 0 as guint {
        (*glib__private__())
            .g_wakeup_free
            .expect("non-null function pointer")((*priv_0).wakeup);
        (*priv_0).wakeup = ::core::ptr::null_mut::<GWakeup>();
    }
    g_mutex_unlock(&raw mut safe_c2rust_cancellable_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_cancellable_cancel(mut cancellable: *mut GCancellable) {
    let mut priv_0: *mut GCancellablePrivate = ::core::ptr::null_mut::<GCancellablePrivate>();
    if cancellable.is_null() || safe_c2rust_g_cancellable_is_cancelled(cancellable) != 0 {
        return;
    }
    priv_0 = (*cancellable).priv_0;
    g_mutex_lock(&raw mut safe_c2rust_cancellable_mutex);
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*priv_0).cancelled;
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
        } else {
        };
        crate::translated::compat::atomic_xchg_seqcst(
            &raw mut (*priv_0).cancelled,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
        )
    }) != 0
    {
        g_mutex_unlock(&raw mut safe_c2rust_cancellable_mutex);
        return;
    }
    (*priv_0).set_cancelled_running(TRUE as guint as guint);
    if !(*priv_0).wakeup.is_null() {
        (*glib__private__())
            .g_wakeup_signal
            .expect("non-null function pointer")((*priv_0).wakeup);
    }
    g_mutex_unlock(&raw mut safe_c2rust_cancellable_mutex);
    g_object_ref(cancellable as gpointer);
    g_signal_emit(
        cancellable as gpointer,
        safe_c2rust_signals[CANCELLED as ::core::ffi::c_int as usize],
        0 as GQuark,
    );
    g_mutex_lock(&raw mut safe_c2rust_cancellable_mutex);
    (*priv_0).set_cancelled_running(FALSE as guint as guint);
    if (*priv_0).cancelled_running_waiting() != 0 {
        g_cond_broadcast(&raw mut safe_c2rust_cancellable_cond);
    }
    (*priv_0).set_cancelled_running_waiting(FALSE as guint as guint);
    g_mutex_unlock(&raw mut safe_c2rust_cancellable_mutex);
    g_object_unref(cancellable as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_cancellable_connect(
    mut cancellable: *mut GCancellable,
    mut callback: GCallback,
    mut data: gpointer,
    mut data_destroy_func: GDestroyNotify,
) -> gulong {
    let mut id: gulong = 0;
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = cancellable as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_cancellable_get_type();
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
            b"G_IS_CANCELLABLE (cancellable)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gulong;
    }
    g_mutex_lock(&raw mut safe_c2rust_cancellable_mutex);
    if ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*(*cancellable).priv_0).cancelled;
            (*(*cancellable).priv_0).cancelled;
        } else {
        };
        *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
            &raw mut (*(*cancellable).priv_0).cancelled as *mut gint,
        );
        gaig_temp
    }) != 0
    {
        let mut _callback: Option<unsafe extern "C" fn(*mut GCancellable, gpointer) -> ()> = None;
        _callback =
            ::core::mem::transmute::<
                *mut ::core::ffi::c_void,
                Option<unsafe extern "C" fn(*mut GCancellable, gpointer) -> ()>,
            >(::core::mem::transmute::<GCallback, *mut ::core::ffi::c_void>(callback));
        id = 0 as gulong;
        (*(*cancellable).priv_0).cancelled_emissions =
            (*(*cancellable).priv_0).cancelled_emissions.wrapping_add(1);
        g_mutex_unlock(&raw mut safe_c2rust_cancellable_mutex);
        _callback.expect("non-null function pointer")(cancellable, data);
        if data_destroy_func.is_some() {
            data_destroy_func.expect("non-null function pointer")(data);
        }
        g_mutex_lock(&raw mut safe_c2rust_cancellable_mutex);
        if (*(*cancellable).priv_0).cancelled_emissions_waiting() != 0 {
            g_cond_broadcast(&raw mut safe_c2rust_cancellable_cond);
        }
        (*(*cancellable).priv_0).cancelled_emissions =
            (*(*cancellable).priv_0).cancelled_emissions.wrapping_sub(1);
        g_mutex_unlock(&raw mut safe_c2rust_cancellable_mutex);
    } else {
        id = g_signal_connect_data(
            cancellable as gpointer,
            b"cancelled\0" as *const u8 as *const gchar,
            callback,
            data,
            ::core::mem::transmute::<GDestroyNotify, GClosureNotify>(data_destroy_func),
            G_CONNECT_DEFAULT,
        );
        g_mutex_unlock(&raw mut safe_c2rust_cancellable_mutex);
    }
    return id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_cancellable_disconnect(
    mut cancellable: *mut GCancellable,
    mut handler_id: gulong,
) {
    let mut priv_0: *mut GCancellablePrivate = ::core::ptr::null_mut::<GCancellablePrivate>();
    if handler_id == 0 as gulong || cancellable.is_null() {
        return;
    }
    g_mutex_lock(&raw mut safe_c2rust_cancellable_mutex);
    priv_0 = (*cancellable).priv_0;
    while (*priv_0).cancelled_running() as ::core::ffi::c_int != 0
        || (*priv_0).cancelled_emissions != 0
    {
        if (*priv_0).cancelled_running() != 0 {
            (*priv_0).set_cancelled_running_waiting(TRUE as guint as guint);
        }
        if (*priv_0).cancelled_emissions != 0 {
            (*priv_0).set_cancelled_emissions_waiting(
                TRUE as ::core::ffi::c_uint as ::core::ffi::c_uint,
            );
        }
        g_cond_wait(
            &raw mut safe_c2rust_cancellable_cond,
            &raw mut safe_c2rust_cancellable_mutex,
        );
    }
    g_signal_handler_disconnect(cancellable as gpointer, handler_id);
    g_mutex_unlock(&raw mut safe_c2rust_cancellable_mutex);
}
unsafe extern "C" fn safe_c2rust_cancellable_source_cancelled(
    mut cancellable: *mut GCancellable,
    mut user_data: gpointer,
) {
    let mut source: *mut GSource = user_data as *mut GSource;
    let mut cancellable_source: *mut GCancellableSource = source as *mut GCancellableSource;
    g_mutex_lock(&raw mut safe_c2rust_cancellable_mutex);
    if (*cancellable_source).resurrected_during_cancellation != 0 {
        (*cancellable_source).resurrected_during_cancellation = FALSE as gboolean;
        g_mutex_unlock(&raw mut safe_c2rust_cancellable_mutex);
        g_source_unref(source);
        return;
    }
    g_source_ref(source);
    g_mutex_unlock(&raw mut safe_c2rust_cancellable_mutex);
    g_source_set_ready_time(source, 0 as gint64);
    g_source_unref(source);
}
unsafe extern "C" fn safe_c2rust_cancellable_source_dispatch(
    mut source: *mut GSource,
    mut callback: GSourceFunc,
    mut user_data: gpointer,
) -> gboolean {
    let mut func: GCancellableSourceFunc =
        ::core::mem::transmute::<GSourceFunc, GCancellableSourceFunc>(callback);
    let mut cancellable_source: *mut GCancellableSource = source as *mut GCancellableSource;
    g_source_set_ready_time(source, -(1 as ::core::ffi::c_int) as gint64);
    return Some(func.expect("non-null function pointer")).expect("non-null function pointer")(
        (*cancellable_source).cancellable,
        user_data,
    );
}
unsafe extern "C" fn safe_c2rust_cancellable_source_dispose(mut source: *mut GSource) {
    let mut cancellable_source: *mut GCancellableSource = source as *mut GCancellableSource;
    g_mutex_lock(&raw mut safe_c2rust_cancellable_mutex);
    if !(*cancellable_source).cancellable.is_null() {
        if (*(*(*cancellable_source).cancellable).priv_0).cancelled_running() != 0 {
            g_source_ref(source);
            (*cancellable_source).resurrected_during_cancellation = TRUE as gboolean;
        }
        let _instance: gpointer = (*cancellable_source).cancellable as gpointer;
        let _handler_id_ptr: *mut gulong = &raw mut (*cancellable_source).cancelled_handler;
        let _handler_id: gulong = *_handler_id_ptr;
        if _handler_id > 0 as gulong {
            *_handler_id_ptr = 0 as gulong;
            g_signal_handler_disconnect(_instance, _handler_id);
        }
        let mut _pp: *mut *mut GCancellable = &raw mut (*cancellable_source).cancellable;
        let mut _ptr: *mut GCancellable = *_pp;
        *_pp = ::core::ptr::null_mut::<GCancellable>();
        if !_ptr.is_null() {
            g_object_unref(_ptr as gpointer);
        }
    }
    g_mutex_unlock(&raw mut safe_c2rust_cancellable_mutex);
}
unsafe extern "C" fn safe_c2rust_cancellable_source_closure_callback(
    mut cancellable: *mut GCancellable,
    mut data: gpointer,
) -> gboolean {
    let mut closure: *mut GClosure = data as *mut GClosure;
    let mut params: GValue = _GValue {
        g_type: 0 as GType,
        data: [
            C2RustUnnamed {
                v_int: 0 as ::core::ffi::c_int,
            },
            C2RustUnnamed { v_int: 0 },
        ],
    };
    let mut result_value: GValue = _GValue {
        g_type: 0 as GType,
        data: [
            C2RustUnnamed {
                v_int: 0 as ::core::ffi::c_int,
            },
            C2RustUnnamed { v_int: 0 },
        ],
    };
    let mut result: gboolean = 0;
    g_value_init(&raw mut result_value, G_TYPE_BOOLEAN);
    g_value_init(&raw mut params, safe_c2rust_g_cancellable_get_type());
    g_value_set_object(&raw mut params, cancellable as gpointer);
    g_closure_invoke(
        closure,
        &raw mut result_value,
        1 as guint,
        &raw mut params,
        NULL,
    );
    result = g_value_get_boolean(&raw mut result_value);
    g_value_unset(&raw mut result_value);
    g_value_unset(&raw mut params);
    return result;
}
static mut safe_c2rust_cancellable_source_funcs: GSourceFuncs = unsafe {
    _GSourceFuncs {
        prepare: None,
        check: None,
        dispatch: Some(
            safe_c2rust_cancellable_source_dispatch
                as unsafe extern "C" fn(*mut GSource, GSourceFunc, gpointer) -> gboolean,
        ),
        finalize: None,
        closure_callback: ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GCancellable, gpointer) -> gboolean>,
            GSourceFunc,
        >(Some(
            safe_c2rust_cancellable_source_closure_callback
                as unsafe extern "C" fn(*mut GCancellable, gpointer) -> gboolean,
        )),
        closure_marshal: None,
    }
};
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_cancellable_source_new(
    mut cancellable: *mut GCancellable,
) -> *mut GSource {
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    let mut cancellable_source: *mut GCancellableSource =
        ::core::ptr::null_mut::<GCancellableSource>();
    source = g_source_new(
        &raw mut safe_c2rust_cancellable_source_funcs,
        ::core::mem::size_of::<GCancellableSource>() as guint,
    );
    g_source_set_static_name(
        source,
        b"GCancellable\0" as *const u8 as *const ::core::ffi::c_char,
    );
    g_source_set_dispose_function(
        source,
        Some(safe_c2rust_cancellable_source_dispose as unsafe extern "C" fn(*mut GSource) -> ()),
    );
    cancellable_source = source as *mut GCancellableSource;
    if !cancellable.is_null() {
        (*cancellable_source).cancellable =
            g_object_ref(cancellable as gpointer) as *mut GCancellable as *mut GCancellable;
        (*cancellable_source).cancelled_handler = g_signal_connect_data(
            cancellable as gpointer,
            b"cancelled\0" as *const u8 as *const gchar,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GCancellable, gpointer) -> ()>,
                GCallback,
            >(Some(
                safe_c2rust_cancellable_source_cancelled
                    as unsafe extern "C" fn(*mut GCancellable, gpointer) -> (),
            )),
            source as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
        if safe_c2rust_g_cancellable_is_cancelled(cancellable) != 0 {
            g_source_set_ready_time(source, 0 as gint64);
        }
    }
    return source;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
