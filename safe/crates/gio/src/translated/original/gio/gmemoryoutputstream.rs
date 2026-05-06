extern "C" {
    pub type _GBytes;
    pub type _GData;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GAsyncResult;
    pub type _GInputStreamPrivate;
    pub type _GOutputStreamPrivate;
    pub type _GCancellablePrivate;
    pub type _GPollableOutputStream;
    pub type _GSeekable;
    pub type _GTask;
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
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_bytes_new_with_free_func(
        data: gconstpointer,
        size: gsize,
        free_func: GDestroyNotify,
        user_data: gpointer,
    ) -> *mut GBytes;
    fn g_free(mem: gpointer);
    fn g_realloc(mem: gpointer, n_bytes: gsize) -> gpointer;
    fn g_source_unref(source: *mut GSource);
    fn g_timeout_source_new(interval: guint) -> *mut GSource;
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
    fn g_object_unref(object: gpointer);
    fn g_param_spec_ulong(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        minimum: gulong,
        maximum: gulong,
        default_value: gulong,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_pointer(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_value_set_ulong(value: *mut GValue, v_ulong: gulong);
    fn g_value_get_ulong(value: *const GValue) -> gulong;
    fn g_value_set_pointer(value: *mut GValue, v_pointer: gpointer);
    fn g_value_get_pointer(value: *const GValue) -> gpointer;
    fn g_output_stream_get_type() -> GType;
    fn g_output_stream_is_closed(stream: *mut GOutputStream) -> gboolean;
    fn g_io_error_quark() -> GQuark;
    fn g_pollable_source_new_full(
        pollable_stream: gpointer,
        child_source: *mut GSource,
        cancellable: *mut GCancellable,
    ) -> *mut GSource;
    fn g_seekable_get_type() -> GType;
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
    fn g_task_return_boolean(task: *mut GTask, result: gboolean);
    fn g_task_propagate_boolean(task: *mut GTask, error: *mut *mut GError) -> gboolean;
    fn g_pollable_output_stream_get_type() -> GType;
    fn glib_gettext(str: *const gchar) -> *const gchar;
}
pub type size_t = usize;
pub type guint8 = ::core::ffi::c_uchar;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type goffset = gint64;
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
pub type GBytes = _GBytes;
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
pub type GSeekType = ::core::ffi::c_uint;
pub const G_SEEK_END: GSeekType = 2;
pub const G_SEEK_SET: GSeekType = 1;
pub const G_SEEK_CUR: GSeekType = 0;
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
pub type GOutputStreamSpliceFlags = ::core::ffi::c_uint;
pub const G_OUTPUT_STREAM_SPLICE_CLOSE_TARGET: GOutputStreamSpliceFlags = 2;
pub const G_OUTPUT_STREAM_SPLICE_CLOSE_SOURCE: GOutputStreamSpliceFlags = 1;
pub const G_OUTPUT_STREAM_SPLICE_NONE: GOutputStreamSpliceFlags = 0;
pub type GPollableReturn = ::core::ffi::c_int;
pub const G_POLLABLE_RETURN_WOULD_BLOCK: GPollableReturn = -27;
pub const G_POLLABLE_RETURN_OK: GPollableReturn = 1;
pub const G_POLLABLE_RETURN_FAILED: GPollableReturn = 0;
pub type GAsyncResult = _GAsyncResult;
pub type GInputStream = _GInputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInputStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GInputStreamPrivate,
}
pub type GInputStreamPrivate = _GInputStreamPrivate;
pub type GOutputStream = _GOutputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GOutputStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GOutputStreamPrivate,
}
pub type GOutputStreamPrivate = _GOutputStreamPrivate;
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
pub struct _GMemoryOutputStream {
    pub parent_instance: GOutputStream,
    pub priv_0: *mut GMemoryOutputStreamPrivate,
}
pub type GMemoryOutputStreamPrivate = _GMemoryOutputStreamPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMemoryOutputStreamPrivate {
    pub data: gpointer,
    pub len: gsize,
    pub valid_len: gsize,
    pub pos: gsize,
    pub realloc_fn: GReallocFunc,
    pub destroy: GDestroyNotify,
}
pub type GReallocFunc = Option<unsafe extern "C" fn(gpointer, gsize) -> gpointer>;
pub type GMemoryOutputStream = _GMemoryOutputStream;
pub type GPollableOutputStream = _GPollableOutputStream;
pub type GSeekable = _GSeekable;
pub type GTask = _GTask;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GOutputVector {
    pub buffer: gconstpointer,
    pub size: gsize,
}
pub type GOutputVector = _GOutputVector;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GOutputStreamClass {
    pub parent_class: GObjectClass,
    pub write_fn: Option<
        unsafe extern "C" fn(
            *mut GOutputStream,
            *const ::core::ffi::c_void,
            gsize,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gssize,
    >,
    pub splice: Option<
        unsafe extern "C" fn(
            *mut GOutputStream,
            *mut GInputStream,
            GOutputStreamSpliceFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gssize,
    >,
    pub flush: Option<
        unsafe extern "C" fn(*mut GOutputStream, *mut GCancellable, *mut *mut GError) -> gboolean,
    >,
    pub close_fn: Option<
        unsafe extern "C" fn(*mut GOutputStream, *mut GCancellable, *mut *mut GError) -> gboolean,
    >,
    pub write_async: Option<
        unsafe extern "C" fn(
            *mut GOutputStream,
            *const ::core::ffi::c_void,
            gsize,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub write_finish: Option<
        unsafe extern "C" fn(*mut GOutputStream, *mut GAsyncResult, *mut *mut GError) -> gssize,
    >,
    pub splice_async: Option<
        unsafe extern "C" fn(
            *mut GOutputStream,
            *mut GInputStream,
            GOutputStreamSpliceFlags,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub splice_finish: Option<
        unsafe extern "C" fn(*mut GOutputStream, *mut GAsyncResult, *mut *mut GError) -> gssize,
    >,
    pub flush_async: Option<
        unsafe extern "C" fn(
            *mut GOutputStream,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub flush_finish: Option<
        unsafe extern "C" fn(*mut GOutputStream, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    >,
    pub close_async: Option<
        unsafe extern "C" fn(
            *mut GOutputStream,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub close_finish: Option<
        unsafe extern "C" fn(*mut GOutputStream, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    >,
    pub writev_fn: Option<
        unsafe extern "C" fn(
            *mut GOutputStream,
            *const GOutputVector,
            gsize,
            *mut gsize,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub writev_async: Option<
        unsafe extern "C" fn(
            *mut GOutputStream,
            *const GOutputVector,
            gsize,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub writev_finish: Option<
        unsafe extern "C" fn(
            *mut GOutputStream,
            *mut GAsyncResult,
            *mut gsize,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved6: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved7: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved8: Option<unsafe extern "C" fn() -> ()>,
}
pub type GOutputStreamClass = _GOutputStreamClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMemoryOutputStreamClass {
    pub parent_class: GOutputStreamClass,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
}
pub type GMemoryOutputStreamClass = _GMemoryOutputStreamClass;
pub const PROP_DESTROY_FUNCTION: C2RustUnnamed_1 = 5;
pub const PROP_REALLOC_FUNCTION: C2RustUnnamed_1 = 4;
pub const PROP_DATA_SIZE: C2RustUnnamed_1 = 3;
pub const PROP_SIZE: C2RustUnnamed_1 = 2;
pub const PROP_DATA: C2RustUnnamed_1 = 1;
pub type GPollableOutputStreamInterface = _GPollableOutputStreamInterface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPollableOutputStreamInterface {
    pub g_iface: GTypeInterface,
    pub can_poll: Option<unsafe extern "C" fn(*mut GPollableOutputStream) -> gboolean>,
    pub is_writable: Option<unsafe extern "C" fn(*mut GPollableOutputStream) -> gboolean>,
    pub create_source:
        Option<unsafe extern "C" fn(*mut GPollableOutputStream, *mut GCancellable) -> *mut GSource>,
    pub write_nonblocking: Option<
        unsafe extern "C" fn(
            *mut GPollableOutputStream,
            *const ::core::ffi::c_void,
            gsize,
            *mut *mut GError,
        ) -> gssize,
    >,
    pub writev_nonblocking: Option<
        unsafe extern "C" fn(
            *mut GPollableOutputStream,
            *const GOutputVector,
            gsize,
            *mut gsize,
            *mut *mut GError,
        ) -> GPollableReturn,
    >,
}
pub type GSeekableIface = _GSeekableIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSeekableIface {
    pub g_iface: GTypeInterface,
    pub tell: Option<unsafe extern "C" fn(*mut GSeekable) -> goffset>,
    pub can_seek: Option<unsafe extern "C" fn(*mut GSeekable) -> gboolean>,
    pub seek: Option<
        unsafe extern "C" fn(
            *mut GSeekable,
            goffset,
            GSeekType,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub can_truncate: Option<unsafe extern "C" fn(*mut GSeekable) -> gboolean>,
    pub truncate_fn: Option<
        unsafe extern "C" fn(
            *mut GSeekable,
            goffset,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gboolean,
    >,
}
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_1 = 0;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn safe_c2rust_g_nearest_pow(mut num: gsize) -> gsize {
    let mut n: gsize = num.wrapping_sub(1 as gsize);
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if num > 0 as gsize
            && num
                <= (9223372036854775807 as ::core::ffi::c_long as ::core::ffi::c_ulong)
                    .wrapping_mul(2 as ::core::ffi::c_ulong)
                    .wrapping_add(1 as ::core::ffi::c_ulong)
                    .wrapping_div(2 as ::core::ffi::c_ulong)
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
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/glib/gutilsprivate.h\0"
                as *const u8 as *const ::core::ffi::c_char,
            44 as ::core::ffi::c_int,
            G_STRFUNC,
            b"num > 0 && num <= G_MAXSIZE / 2\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    n |= n >> 1 as ::core::ffi::c_int;
    n |= n >> 2 as ::core::ffi::c_int;
    n |= n >> 4 as ::core::ffi::c_int;
    n |= n >> 8 as ::core::ffi::c_int;
    n |= n >> 16 as ::core::ffi::c_int;
    n |= n >> 32 as ::core::ffi::c_int;
    return n.wrapping_add(1 as gsize);
}
static mut safe_c2rust_g_memory_output_stream_parent_class: gpointer = NULL;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_memory_output_stream_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_memory_output_stream_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_memory_output_stream_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_memory_output_stream_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GMemoryOutputStream_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GMemoryOutputStream_private_offset,
        );
    }
    safe_c2rust_g_memory_output_stream_class_init(klass as *mut GMemoryOutputStreamClass);
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_memory_output_stream_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_output_stream_get_type(),
        g_intern_static_string(b"GMemoryOutputStream\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GMemoryOutputStreamClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_memory_output_stream_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GMemoryOutputStream>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GMemoryOutputStream) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_memory_output_stream_init
                    as unsafe extern "C" fn(*mut GMemoryOutputStream) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GMemoryOutputStream_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GMemoryOutputStreamPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GSeekableIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_memory_output_stream_seekable_iface_init
                as unsafe extern "C" fn(*mut GSeekableIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_seekable_get_type(),
        &raw const g_implement_interface_info,
    );
    let g_implement_interface_info_0: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GPollableOutputStreamInterface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_memory_output_stream_pollable_iface_init
                as unsafe extern "C" fn(*mut GPollableOutputStreamInterface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_pollable_output_stream_get_type(),
        &raw const g_implement_interface_info_0,
    );
    return g_define_type_id;
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_memory_output_stream_get_instance_private(
    mut self_0: *mut GMemoryOutputStream,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GMemoryOutputStream_private_offset as glong as isize)
        as gpointer;
}
static mut safe_c2rust_GMemoryOutputStream_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_memory_output_stream_class_init(
    mut klass: *mut GMemoryOutputStreamClass,
) {
    let mut ostream_class: *mut GOutputStreamClass = ::core::ptr::null_mut::<GOutputStreamClass>();
    let mut gobject_class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    gobject_class = klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_memory_output_stream_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_memory_output_stream_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).finalize = Some(
        safe_c2rust_g_memory_output_stream_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    ostream_class = klass as *mut ::core::ffi::c_void as *mut GOutputStreamClass;
    (*ostream_class).write_fn = Some(
        safe_c2rust_g_memory_output_stream_write
            as unsafe extern "C" fn(
                *mut GOutputStream,
                *const ::core::ffi::c_void,
                gsize,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gssize,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GOutputStream,
                *const ::core::ffi::c_void,
                gsize,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gssize,
        >;
    (*ostream_class).close_fn = Some(
        safe_c2rust_g_memory_output_stream_close
            as unsafe extern "C" fn(
                *mut GOutputStream,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GOutputStream,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
        >;
    (*ostream_class).close_async = Some(
        safe_c2rust_g_memory_output_stream_close_async
            as unsafe extern "C" fn(
                *mut GOutputStream,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GOutputStream,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*ostream_class).close_finish = Some(
        safe_c2rust_g_memory_output_stream_close_finish
            as unsafe extern "C" fn(
                *mut GOutputStream,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GOutputStream,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> gboolean,
        >;
    g_object_class_install_property(
        gobject_class,
        PROP_DATA as ::core::ffi::c_int as guint,
        g_param_spec_pointer(
            b"data\0" as *const u8 as *const gchar,
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
        PROP_SIZE as ::core::ffi::c_int as guint,
        g_param_spec_ulong(
            b"size\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            0 as gulong,
            G_MAXULONG,
            0 as gulong,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_DATA_SIZE as ::core::ffi::c_int as guint,
        g_param_spec_ulong(
            b"data-size\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            0 as gulong,
            G_MAXULONG,
            0 as gulong,
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_REALLOC_FUNCTION as ::core::ffi::c_int as guint,
        g_param_spec_pointer(
            b"realloc-function\0" as *const u8 as *const gchar,
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
        PROP_DESTROY_FUNCTION as ::core::ffi::c_int as guint,
        g_param_spec_pointer(
            b"destroy-function\0" as *const u8 as *const gchar,
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
unsafe extern "C" fn safe_c2rust_g_memory_output_stream_pollable_iface_init(
    mut iface: *mut GPollableOutputStreamInterface,
) {
    (*iface).is_writable = Some(
        safe_c2rust_g_memory_output_stream_is_writable
            as unsafe extern "C" fn(*mut GPollableOutputStream) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GPollableOutputStream) -> gboolean>;
    (*iface).create_source = Some(
        safe_c2rust_g_memory_output_stream_create_source
            as unsafe extern "C" fn(*mut GPollableOutputStream, *mut GCancellable) -> *mut GSource,
    )
        as Option<
            unsafe extern "C" fn(*mut GPollableOutputStream, *mut GCancellable) -> *mut GSource,
        >;
}
unsafe extern "C" fn safe_c2rust_g_memory_output_stream_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut stream: *mut GMemoryOutputStream = ::core::ptr::null_mut::<GMemoryOutputStream>();
    let mut priv_0: *mut GMemoryOutputStreamPrivate =
        ::core::ptr::null_mut::<GMemoryOutputStreamPrivate>();
    stream = object as *mut ::core::ffi::c_void as *mut GMemoryOutputStream;
    priv_0 = (*stream).priv_0;
    match prop_id {
        1 => {
            (*priv_0).data = g_value_get_pointer(value);
        }
        2 => {
            (*priv_0).len = g_value_get_ulong(value) as gsize;
        }
        4 => {
            (*priv_0).realloc_fn =
                ::core::mem::transmute::<gpointer, GReallocFunc>(g_value_get_pointer(value));
        }
        5 => {
            (*priv_0).destroy =
                ::core::mem::transmute::<gpointer, GDestroyNotify>(g_value_get_pointer(value));
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gmemoryoutputstream.c\0"
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
unsafe extern "C" fn safe_c2rust_g_memory_output_stream_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut stream: *mut GMemoryOutputStream = ::core::ptr::null_mut::<GMemoryOutputStream>();
    let mut priv_0: *mut GMemoryOutputStreamPrivate =
        ::core::ptr::null_mut::<GMemoryOutputStreamPrivate>();
    stream = object as *mut ::core::ffi::c_void as *mut GMemoryOutputStream;
    priv_0 = (*stream).priv_0;
    match prop_id {
        1 => {
            g_value_set_pointer(value, (*priv_0).data);
        }
        2 => {
            g_value_set_ulong(value, (*priv_0).len as gulong);
        }
        3 => {
            g_value_set_ulong(value, (*priv_0).valid_len as gulong);
        }
        4 => {
            g_value_set_pointer(
                value,
                ::core::mem::transmute::<GReallocFunc, gpointer>((*priv_0).realloc_fn),
            );
        }
        5 => {
            g_value_set_pointer(
                value,
                ::core::mem::transmute::<GDestroyNotify, gpointer>((*priv_0).destroy),
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gmemoryoutputstream.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                282 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_memory_output_stream_finalize(mut object: *mut GObject) {
    let mut stream: *mut GMemoryOutputStream = ::core::ptr::null_mut::<GMemoryOutputStream>();
    let mut priv_0: *mut GMemoryOutputStreamPrivate =
        ::core::ptr::null_mut::<GMemoryOutputStreamPrivate>();
    stream = object as *mut ::core::ffi::c_void as *mut GMemoryOutputStream;
    priv_0 = (*stream).priv_0;
    if (*priv_0).destroy.is_some() {
        (*priv_0).destroy.expect("non-null function pointer")((*priv_0).data);
    }
    (*(safe_c2rust_g_memory_output_stream_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_memory_output_stream_seekable_iface_init(
    mut iface: *mut GSeekableIface,
) {
    (*iface).tell = Some(
        safe_c2rust_g_memory_output_stream_tell as unsafe extern "C" fn(*mut GSeekable) -> goffset,
    ) as Option<unsafe extern "C" fn(*mut GSeekable) -> goffset>;
    (*iface).can_seek = Some(
        safe_c2rust_g_memory_output_stream_can_seek
            as unsafe extern "C" fn(*mut GSeekable) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut GSeekable) -> gboolean>;
    (*iface).seek = Some(
        safe_c2rust_g_memory_output_stream_seek
            as unsafe extern "C" fn(
                *mut GSeekable,
                goffset,
                GSeekType,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GSeekable,
                goffset,
                GSeekType,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
        >;
    (*iface).can_truncate = Some(
        safe_c2rust_g_memory_output_stream_can_truncate
            as unsafe extern "C" fn(*mut GSeekable) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut GSeekable) -> gboolean>;
    (*iface).truncate_fn = Some(
        safe_c2rust_g_memory_output_stream_truncate
            as unsafe extern "C" fn(
                *mut GSeekable,
                goffset,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GSeekable,
                goffset,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
        >;
}
unsafe extern "C" fn safe_c2rust_g_memory_output_stream_init(mut stream: *mut GMemoryOutputStream) {
    (*stream).priv_0 = safe_c2rust_g_memory_output_stream_get_instance_private(stream)
        as *mut GMemoryOutputStreamPrivate;
    (*(*stream).priv_0).pos = 0 as gsize;
    (*(*stream).priv_0).valid_len = 0 as gsize;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_memory_output_stream_new(
    mut data: gpointer,
    mut size: gsize,
    mut realloc_function: GReallocFunc,
    mut destroy_function: GDestroyNotify,
) -> *mut GOutputStream {
    let mut stream: *mut GOutputStream = ::core::ptr::null_mut::<GOutputStream>();
    stream = g_object_new(
        safe_c2rust_g_memory_output_stream_get_type(),
        b"data\0" as *const u8 as *const gchar,
        data,
        b"size\0" as *const u8 as *const ::core::ffi::c_char,
        size,
        b"realloc-function\0" as *const u8 as *const ::core::ffi::c_char,
        realloc_function,
        b"destroy-function\0" as *const u8 as *const ::core::ffi::c_char,
        destroy_function,
        NULL,
    ) as *mut GOutputStream;
    return stream;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_memory_output_stream_new_resizable() -> *mut GOutputStream {
    return safe_c2rust_g_memory_output_stream_new(
        NULL,
        0 as gsize,
        Some(g_realloc as unsafe extern "C" fn(gpointer, gsize) -> gpointer),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_memory_output_stream_get_data(
    mut ostream: *mut GMemoryOutputStream,
) -> gpointer {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = ostream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_memory_output_stream_get_type();
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
            b"G_IS_MEMORY_OUTPUT_STREAM (ostream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return (*(*ostream).priv_0).data;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_memory_output_stream_get_size(
    mut ostream: *mut GMemoryOutputStream,
) -> gsize {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = ostream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_memory_output_stream_get_type();
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
            b"G_IS_MEMORY_OUTPUT_STREAM (ostream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gsize;
    }
    return (*(*ostream).priv_0).len;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_memory_output_stream_get_data_size(
    mut ostream: *mut GMemoryOutputStream,
) -> gsize {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = ostream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_memory_output_stream_get_type();
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
            b"G_IS_MEMORY_OUTPUT_STREAM (ostream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gsize;
    }
    return (*(*ostream).priv_0).valid_len;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_memory_output_stream_steal_data(
    mut ostream: *mut GMemoryOutputStream,
) -> gpointer {
    let mut data: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = ostream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_memory_output_stream_get_type();
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
            b"G_IS_MEMORY_OUTPUT_STREAM (ostream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if g_output_stream_is_closed(ostream as *mut ::core::ffi::c_void as *mut GOutputStream) != 0
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
            b"g_output_stream_is_closed (G_OUTPUT_STREAM (ostream))\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    data = (*(*ostream).priv_0).data;
    (*(*ostream).priv_0).data = NULL as gpointer;
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_memory_output_stream_steal_as_bytes(
    mut ostream: *mut GMemoryOutputStream,
) -> *mut GBytes {
    let mut result: *mut GBytes = ::core::ptr::null_mut::<GBytes>();
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = ostream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_memory_output_stream_get_type();
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
            b"G_IS_MEMORY_OUTPUT_STREAM (ostream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBytes>();
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if g_output_stream_is_closed(ostream as *mut ::core::ffi::c_void as *mut GOutputStream) != 0
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
            b"g_output_stream_is_closed (G_OUTPUT_STREAM (ostream))\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBytes>();
    }
    result = g_bytes_new_with_free_func(
        (*(*ostream).priv_0).data as gconstpointer,
        (*(*ostream).priv_0).valid_len,
        (*(*ostream).priv_0).destroy,
        (*(*ostream).priv_0).data,
    );
    (*(*ostream).priv_0).data = NULL as gpointer;
    return result;
}
unsafe extern "C" fn safe_c2rust_array_resize(
    mut ostream: *mut GMemoryOutputStream,
    mut size: gsize,
    mut allow_partial: gboolean,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut priv_0: *mut GMemoryOutputStreamPrivate =
        ::core::ptr::null_mut::<GMemoryOutputStreamPrivate>();
    let mut data: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut len: gsize = 0;
    priv_0 = (*ostream).priv_0;
    if (*priv_0).len == size {
        return TRUE;
    }
    if (*priv_0).realloc_fn.is_none() {
        if allow_partial != 0 && (*priv_0).pos < (*priv_0).len {
            return TRUE;
        }
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NO_SPACE as ::core::ffi::c_int as gint,
            glib_gettext(b"Memory output stream not resizable\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    len = (*priv_0).len;
    data = (*priv_0).realloc_fn.expect("non-null function pointer")((*priv_0).data, size);
    if size > 0 as gsize && data.is_null() {
        if allow_partial != 0 && (*priv_0).pos < (*priv_0).len {
            return TRUE;
        }
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NO_SPACE as ::core::ffi::c_int as gint,
            glib_gettext(b"Failed to resize memory output stream\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    if size > len {
        memset(
            (data as *mut guint8).offset(len as isize) as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (size as size_t).wrapping_sub(len as size_t),
        );
    }
    (*priv_0).data = data;
    (*priv_0).len = size;
    if (*priv_0).len < (*priv_0).valid_len {
        (*priv_0).valid_len = (*priv_0).len;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_memory_output_stream_write(
    mut stream: *mut GOutputStream,
    mut buffer: *const ::core::ffi::c_void,
    mut count: gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    let mut current_block: u64;
    let mut ostream: *mut GMemoryOutputStream = ::core::ptr::null_mut::<GMemoryOutputStream>();
    let mut priv_0: *mut GMemoryOutputStreamPrivate =
        ::core::ptr::null_mut::<GMemoryOutputStreamPrivate>();
    let mut dest: *mut guint8 = ::core::ptr::null_mut::<guint8>();
    let mut new_size: gsize = 0;
    ostream = stream as *mut ::core::ffi::c_void as *mut GMemoryOutputStream;
    priv_0 = (*ostream).priv_0;
    if count == 0 as gsize {
        return 0 as gssize;
    }
    if !((*priv_0).realloc_fn.is_some() && (*priv_0).pos.wrapping_add(count) < (*priv_0).pos) {
        if (*priv_0).pos.wrapping_add(count) > (*priv_0).len {
            new_size = safe_c2rust_g_nearest_pow((*priv_0).pos.wrapping_add(count));
            if new_size == 0 as gsize {
                current_block = 17639946357473094279;
            } else {
                new_size = if new_size > 16 as gsize {
                    new_size
                } else {
                    16 as gsize
                };
                if safe_c2rust_array_resize(ostream, new_size, TRUE, error) == 0 {
                    return -(1 as ::core::ffi::c_int) as gssize;
                }
                current_block = 11812396948646013369;
            }
        } else {
            current_block = 11812396948646013369;
        }
        match current_block {
            17639946357473094279 => {}
            _ => {
                count = if count < (*priv_0).len.wrapping_sub((*priv_0).pos) {
                    count
                } else {
                    (*priv_0).len.wrapping_sub((*priv_0).pos)
                };
                dest = ((*priv_0).data as *mut guint8).offset((*priv_0).pos as isize);
                memcpy(dest as *mut ::core::ffi::c_void, buffer, count as size_t);
                (*priv_0).pos = (*priv_0).pos.wrapping_add(count);
                if (*priv_0).pos > (*priv_0).valid_len {
                    (*priv_0).valid_len = (*priv_0).pos;
                }
                return count as gssize;
            }
        }
    }
    g_set_error_literal(
        error,
        g_io_error_quark(),
        G_IO_ERROR_NO_SPACE as ::core::ffi::c_int as gint,
        glib_gettext(
            b"Amount of memory required to process the write is larger than available address space\0"
                as *const u8 as *const gchar,
        ),
    );
    return -(1 as ::core::ffi::c_int) as gssize;
}
unsafe extern "C" fn safe_c2rust_g_memory_output_stream_close(
    mut stream: *mut GOutputStream,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_memory_output_stream_close_async(
    mut stream: *mut GOutputStream,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    task = g_task_new(stream as gpointer, cancellable, callback, data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GOutputStream,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_memory_output_stream_close_async
                as unsafe extern "C" fn(
                    *mut GOutputStream,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"g_memory_output_stream_close_async\0" as *const u8 as *const gchar,
        );
    }
    safe_c2rust_g_memory_output_stream_close(
        stream,
        cancellable,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    g_task_return_boolean(task, TRUE);
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_memory_output_stream_close_finish(
    mut stream: *mut GOutputStream,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, stream as gpointer) != 0 {
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
            b"g_task_is_valid (result, stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
}
unsafe extern "C" fn safe_c2rust_g_memory_output_stream_tell(
    mut seekable: *mut GSeekable,
) -> goffset {
    let mut stream: *mut GMemoryOutputStream = ::core::ptr::null_mut::<GMemoryOutputStream>();
    let mut priv_0: *mut GMemoryOutputStreamPrivate =
        ::core::ptr::null_mut::<GMemoryOutputStreamPrivate>();
    stream = seekable as *mut ::core::ffi::c_void as *mut GMemoryOutputStream;
    priv_0 = (*stream).priv_0;
    return (*priv_0).pos as goffset;
}
unsafe extern "C" fn safe_c2rust_g_memory_output_stream_can_seek(
    mut seekable: *mut GSeekable,
) -> gboolean {
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_memory_output_stream_seek(
    mut seekable: *mut GSeekable,
    mut offset: goffset,
    mut type_0: GSeekType,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut stream: *mut GMemoryOutputStream = ::core::ptr::null_mut::<GMemoryOutputStream>();
    let mut priv_0: *mut GMemoryOutputStreamPrivate =
        ::core::ptr::null_mut::<GMemoryOutputStreamPrivate>();
    let mut absolute: goffset = 0;
    stream = seekable as *mut ::core::ffi::c_void as *mut GMemoryOutputStream;
    priv_0 = (*stream).priv_0;
    match type_0 as ::core::ffi::c_uint {
        0 => {
            absolute = (*priv_0).pos.wrapping_add(offset as gsize) as goffset;
        }
        1 => {
            absolute = offset;
        }
        2 => {
            if (*priv_0).realloc_fn.is_some() {
                absolute = (*priv_0).valid_len.wrapping_add(offset as gsize) as goffset;
            } else {
                absolute = (*priv_0).len.wrapping_add(offset as gsize) as goffset;
            }
        }
        _ => {
            g_set_error_literal(
                error,
                g_io_error_quark(),
                G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                glib_gettext(b"Invalid GSeekType supplied\0" as *const u8 as *const gchar),
            );
            return FALSE;
        }
    }
    if absolute < 0 as goffset {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Requested seek before the beginning of the stream\0" as *const u8 as *const gchar,
            ),
        );
        return FALSE;
    }
    if (*priv_0).realloc_fn.is_none() && absolute as gsize > (*priv_0).len {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Requested seek beyond the end of the stream\0" as *const u8 as *const gchar,
            ),
        );
        return FALSE;
    }
    (*priv_0).pos = absolute as gsize;
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_memory_output_stream_can_truncate(
    mut seekable: *mut GSeekable,
) -> gboolean {
    let mut ostream: *mut GMemoryOutputStream = ::core::ptr::null_mut::<GMemoryOutputStream>();
    let mut priv_0: *mut GMemoryOutputStreamPrivate =
        ::core::ptr::null_mut::<GMemoryOutputStreamPrivate>();
    ostream = seekable as *mut ::core::ffi::c_void as *mut GMemoryOutputStream;
    priv_0 = (*ostream).priv_0;
    return (*priv_0).realloc_fn.is_some() as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_memory_output_stream_truncate(
    mut seekable: *mut GSeekable,
    mut offset: goffset,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut ostream: *mut GMemoryOutputStream =
        seekable as *mut ::core::ffi::c_void as *mut GMemoryOutputStream;
    if safe_c2rust_array_resize(ostream, offset as gsize, FALSE, error) == 0 {
        return FALSE;
    }
    (*(*ostream).priv_0).valid_len = offset as gsize;
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_memory_output_stream_is_writable(
    mut stream: *mut GPollableOutputStream,
) -> gboolean {
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_memory_output_stream_create_source(
    mut stream: *mut GPollableOutputStream,
    mut cancellable: *mut GCancellable,
) -> *mut GSource {
    let mut base_source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    let mut pollable_source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    base_source = g_timeout_source_new(0 as guint);
    pollable_source = g_pollable_source_new_full(stream as gpointer, base_source, cancellable);
    g_source_unref(base_source);
    return pollable_source;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
