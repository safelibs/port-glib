use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GAsyncResult;
    pub type _GInputStreamPrivate;
    pub type _GOutputStreamPrivate;
    pub type _GCancellablePrivate;
    pub type _GPollableOutputStream;
    pub type _GFileDescriptorBased;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn write(__fd: ::core::ffi::c_int, __buf: *const ::core::ffi::c_void, __n: size_t) -> ssize_t;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn writev(
        __fd: ::core::ffi::c_int,
        __iovec: *const iovec,
        __count: ::core::ffi::c_int,
    ) -> ssize_t;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_poll(fds: *mut GPollFD, nfds: guint, timeout: gint) -> gint;
    fn g_source_unref(source: *mut GSource);
    fn g_source_add_child_source(source: *mut GSource, child_source: *mut GSource);
    fn g_strerror(errnum: gint) -> *const gchar;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_unix_fd_source_new(fd: gint, condition: GIOCondition) -> *mut GSource;
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
    fn g_param_spec_boolean(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: gboolean,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_int(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        minimum: gint,
        maximum: gint,
        default_value: gint,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_source_set_dummy_callback(source: *mut GSource);
    fn g_value_set_boolean(value: *mut GValue, v_boolean: gboolean);
    fn g_value_get_boolean(value: *const GValue) -> gboolean;
    fn g_value_set_int(value: *mut GValue, v_int: gint);
    fn g_value_get_int(value: *const GValue) -> gint;
    fn g_io_error_quark() -> GQuark;
    fn g_io_error_from_errno(err_no: gint) -> GIOErrorEnum;
    fn g_output_stream_get_type() -> GType;
    fn g_cancellable_set_error_if_cancelled(
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_cancellable_make_pollfd(cancellable: *mut GCancellable, pollfd: *mut GPollFD) -> gboolean;
    fn g_cancellable_release_fd(cancellable: *mut GCancellable);
    fn g_cancellable_source_new(cancellable: *mut GCancellable) -> *mut GSource;
    fn g_pollable_output_stream_get_type() -> GType;
    fn g_pollable_output_stream_is_writable(stream: *mut GPollableOutputStream) -> gboolean;
    fn g_pollable_source_new(pollable_stream: *mut GObject) -> *mut GSource;
    fn g_file_descriptor_based_get_type() -> GType;
    fn glib_gettext(str: *const gchar) -> *const gchar;
    fn _g_fd_is_pollable(fd: ::core::ffi::c_int) -> gboolean;
}
pub type __ssize_t = ::core::ffi::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut ::core::ffi::c_void,
    pub iov_len: size_t,
}
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
pub type gushort = ::core::ffi::c_ushort;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gfloat = ::core::ffi::c_float;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
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
pub type GPollableOutputStream = _GPollableOutputStream;
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
pub type GPollableOutputStreamInterface = _GPollableOutputStreamInterface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixOutputStream {
    pub parent_instance: GOutputStream,
    pub priv_0: *mut GUnixOutputStreamPrivate,
}
pub type GUnixOutputStreamPrivate = _GUnixOutputStreamPrivate;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GUnixOutputStreamPrivate {
    pub fd: ::core::ffi::c_int,
    #[bitfield(name = "close_fd", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "can_poll", ty = "guint", bits = "1..=1")]
    pub close_fd_can_poll: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type GUnixOutputStream = _GUnixOutputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixOutputStreamClass {
    pub parent_class: GOutputStreamClass,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
}
pub type GUnixOutputStreamClass = _GUnixOutputStreamClass;
pub const PROP_CLOSE_FD: C2RustUnnamed_0 = 2;
pub const PROP_FD: C2RustUnnamed_0 = 1;
pub type GFileDescriptorBasedIface = _GFileDescriptorBasedIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileDescriptorBasedIface {
    pub g_iface: GTypeInterface,
    pub get_fd: Option<unsafe extern "C" fn(*mut GFileDescriptorBased) -> ::core::ffi::c_int>,
}
pub type GFileDescriptorBased = _GFileDescriptorBased;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_0 = 0;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const __IOV_MAX: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const INT_MIN: ::core::ffi::c_int = -__INT_MAX__ - 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const IOV_MAX: ::core::ffi::c_int = __IOV_MAX;
pub const G_MININT: ::core::ffi::c_int = INT_MIN;
pub const G_MAXINT: ::core::ffi::c_int = INT_MAX;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const G_IOV_MAX: ::core::ffi::c_int = IOV_MAX;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_unix_output_stream_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_output_stream_get_type(),
        g_intern_static_string(b"GUnixOutputStream\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GUnixOutputStreamClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_unix_output_stream_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GUnixOutputStream>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GUnixOutputStream) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_unix_output_stream_init
                    as unsafe extern "C" fn(*mut GUnixOutputStream) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GUnixOutputStream_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GUnixOutputStreamPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GPollableOutputStreamInterface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_unix_output_stream_pollable_iface_init
                as unsafe extern "C" fn(*mut GPollableOutputStreamInterface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_pollable_output_stream_get_type(),
        &raw const g_implement_interface_info,
    );
    let g_implement_interface_info_0: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GFileDescriptorBasedIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_unix_output_stream_file_descriptor_based_iface_init
                as unsafe extern "C" fn(*mut GFileDescriptorBasedIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_file_descriptor_based_get_type(),
        &raw const g_implement_interface_info_0,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_output_stream_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_unix_output_stream_get_type_once();
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
#[inline]
unsafe extern "C" fn safe_c2rust_g_unix_output_stream_get_instance_private(
    mut self_0: *mut GUnixOutputStream,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GUnixOutputStream_private_offset as glong as isize)
        as gpointer;
}
static mut safe_c2rust_GUnixOutputStream_private_offset: gint = 0;
static mut safe_c2rust_g_unix_output_stream_parent_class: gpointer = NULL;
unsafe extern "C" fn safe_c2rust_g_unix_output_stream_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_unix_output_stream_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GUnixOutputStream_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GUnixOutputStream_private_offset,
        );
    }
    safe_c2rust_g_unix_output_stream_class_init(klass as *mut GUnixOutputStreamClass);
}
unsafe extern "C" fn safe_c2rust_g_unix_output_stream_class_init(
    mut klass: *mut GUnixOutputStreamClass,
) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    let mut stream_class: *mut GOutputStreamClass =
        klass as *mut ::core::ffi::c_void as *mut GOutputStreamClass;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_unix_output_stream_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_unix_output_stream_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*stream_class).write_fn = Some(
        safe_c2rust_g_unix_output_stream_write
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
    (*stream_class).writev_fn = Some(
        safe_c2rust_g_unix_output_stream_writev
            as unsafe extern "C" fn(
                *mut GOutputStream,
                *const GOutputVector,
                gsize,
                *mut gsize,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GOutputStream,
                *const GOutputVector,
                gsize,
                *mut gsize,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
        >;
    (*stream_class).close_fn = Some(
        safe_c2rust_g_unix_output_stream_close
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
    g_object_class_install_property(
        gobject_class,
        PROP_FD as ::core::ffi::c_int as guint,
        g_param_spec_int(
            b"fd\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            G_MININT,
            G_MAXINT,
            -(1 as gint),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_CLOSE_FD as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"close-fd\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            TRUE,
            (G_PARAM_READABLE as ::core::ffi::c_int
                | G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int) as GParamFlags,
        ),
    );
}
unsafe extern "C" fn safe_c2rust_g_unix_output_stream_pollable_iface_init(
    mut iface: *mut GPollableOutputStreamInterface,
) {
    (*iface).can_poll = Some(
        safe_c2rust_g_unix_output_stream_pollable_can_poll
            as unsafe extern "C" fn(*mut GPollableOutputStream) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GPollableOutputStream) -> gboolean>;
    (*iface).is_writable = Some(
        safe_c2rust_g_unix_output_stream_pollable_is_writable
            as unsafe extern "C" fn(*mut GPollableOutputStream) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GPollableOutputStream) -> gboolean>;
    (*iface).create_source = Some(
        safe_c2rust_g_unix_output_stream_pollable_create_source
            as unsafe extern "C" fn(*mut GPollableOutputStream, *mut GCancellable) -> *mut GSource,
    )
        as Option<
            unsafe extern "C" fn(*mut GPollableOutputStream, *mut GCancellable) -> *mut GSource,
        >;
    (*iface).writev_nonblocking = Some(
        safe_c2rust_g_unix_output_stream_pollable_writev_nonblocking
            as unsafe extern "C" fn(
                *mut GPollableOutputStream,
                *const GOutputVector,
                gsize,
                *mut gsize,
                *mut *mut GError,
            ) -> GPollableReturn,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GPollableOutputStream,
                *const GOutputVector,
                gsize,
                *mut gsize,
                *mut *mut GError,
            ) -> GPollableReturn,
        >;
}
unsafe extern "C" fn safe_c2rust_g_unix_output_stream_file_descriptor_based_iface_init(
    mut iface: *mut GFileDescriptorBasedIface,
) {
    (*iface).get_fd = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut GUnixOutputStream) -> gint>,
        Option<unsafe extern "C" fn(*mut GFileDescriptorBased) -> ::core::ffi::c_int>,
    >(Some(
        safe_c2rust_g_unix_output_stream_get_fd
            as unsafe extern "C" fn(*mut GUnixOutputStream) -> gint,
    ));
}
unsafe extern "C" fn safe_c2rust_g_unix_output_stream_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut unix_stream: *mut GUnixOutputStream = ::core::ptr::null_mut::<GUnixOutputStream>();
    unix_stream = object as *mut ::core::ffi::c_void as *mut GUnixOutputStream;
    match prop_id {
        1 => {
            (*(*unix_stream).priv_0).fd = g_value_get_int(value) as ::core::ffi::c_int;
            (*(*unix_stream).priv_0)
                .set_can_poll(_g_fd_is_pollable((*(*unix_stream).priv_0).fd) as guint as guint);
        }
        2 => {
            (*(*unix_stream).priv_0).set_close_fd(g_value_get_boolean(value) as guint as guint);
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gunixoutputstream.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                188 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_unix_output_stream_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut unix_stream: *mut GUnixOutputStream = ::core::ptr::null_mut::<GUnixOutputStream>();
    unix_stream = object as *mut ::core::ffi::c_void as *mut GUnixOutputStream;
    match prop_id {
        1 => {
            g_value_set_int(value, (*(*unix_stream).priv_0).fd as gint);
        }
        2 => {
            g_value_set_boolean(value, (*(*unix_stream).priv_0).close_fd() as gboolean);
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gunixoutputstream.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                212 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_unix_output_stream_init(
    mut unix_stream: *mut GUnixOutputStream,
) {
    (*unix_stream).priv_0 = safe_c2rust_g_unix_output_stream_get_instance_private(unix_stream)
        as *mut GUnixOutputStreamPrivate;
    (*(*unix_stream).priv_0).fd = -(1 as ::core::ffi::c_int);
    (*(*unix_stream).priv_0).set_close_fd(TRUE as guint as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_output_stream_new(
    mut fd: gint,
    mut close_fd: gboolean,
) -> *mut GOutputStream {
    let mut stream: *mut GUnixOutputStream = ::core::ptr::null_mut::<GUnixOutputStream>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if fd != -(1 as ::core::ffi::c_int) {
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
            b"fd != -1\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GOutputStream>();
    }
    stream = g_object_new(
        safe_c2rust_g_unix_output_stream_get_type(),
        b"fd\0" as *const u8 as *const gchar,
        fd,
        b"close-fd\0" as *const u8 as *const ::core::ffi::c_char,
        close_fd,
        NULL,
    ) as *mut GUnixOutputStream;
    return stream as *mut ::core::ffi::c_void as *mut GOutputStream;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_output_stream_set_close_fd(
    mut stream: *mut GUnixOutputStream,
    mut close_fd: gboolean,
) {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_unix_output_stream_get_type();
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
            b"G_IS_UNIX_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    close_fd = (close_fd != FALSE) as ::core::ffi::c_int as gboolean;
    if (*(*stream).priv_0).close_fd() as ::core::ffi::c_int != close_fd {
        (*(*stream).priv_0).set_close_fd(close_fd as guint as guint);
        g_object_notify(
            stream as *mut ::core::ffi::c_void as *mut GObject,
            b"close-fd\0" as *const u8 as *const gchar,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_output_stream_get_close_fd(
    mut stream: *mut GUnixOutputStream,
) -> gboolean {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_unix_output_stream_get_type();
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
            b"G_IS_UNIX_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*(*stream).priv_0).close_fd() as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_output_stream_get_fd(
    mut stream: *mut GUnixOutputStream,
) -> gint {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_unix_output_stream_get_type();
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
            b"G_IS_UNIX_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    return (*(*stream).priv_0).fd as gint;
}
unsafe extern "C" fn safe_c2rust_g_unix_output_stream_write(
    mut stream: *mut GOutputStream,
    mut buffer: *const ::core::ffi::c_void,
    mut count: gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    let mut unix_stream: *mut GUnixOutputStream = ::core::ptr::null_mut::<GUnixOutputStream>();
    let mut res: gssize = -(1 as ::core::ffi::c_int) as gssize;
    let mut poll_fds: [GPollFD; 2] = [_GPollFD {
        fd: 0,
        events: 0,
        revents: 0,
    }; 2];
    let mut nfds: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut poll_ret: ::core::ffi::c_int = 0;
    unix_stream = stream as *mut ::core::ffi::c_void as *mut GUnixOutputStream;
    poll_fds[0 as ::core::ffi::c_int as usize].fd = (*(*unix_stream).priv_0).fd as gint;
    poll_fds[0 as ::core::ffi::c_int as usize].events = G_IO_OUT as ::core::ffi::c_int as gushort;
    nfds += 1;
    if (*(*unix_stream).priv_0).can_poll() as ::core::ffi::c_int != 0
        && g_cancellable_make_pollfd(
            cancellable,
            (&raw mut poll_fds as *mut GPollFD).offset(1 as ::core::ffi::c_int as isize)
                as *mut GPollFD,
        ) != 0
    {
        nfds += 1;
    }
    loop {
        let mut errsv: ::core::ffi::c_int = 0;
        poll_fds[1 as ::core::ffi::c_int as usize].revents = 0 as gushort;
        poll_fds[0 as ::core::ffi::c_int as usize].revents =
            poll_fds[1 as ::core::ffi::c_int as usize].revents;
        loop {
            poll_ret = g_poll(
                &raw mut poll_fds as *mut GPollFD,
                nfds as guint,
                -(1 as gint),
            ) as ::core::ffi::c_int;
            errsv = *__errno_location();
            if !(poll_ret == -(1 as ::core::ffi::c_int) && errsv == EINTR) {
                break;
            }
        }
        if poll_ret == -(1 as ::core::ffi::c_int) {
            g_set_error(
                error,
                g_io_error_quark(),
                g_io_error_from_errno(errsv as gint) as gint,
                glib_gettext(
                    b"Error writing to file descriptor: %s\0" as *const u8 as *const gchar,
                ),
                g_strerror(errsv as gint),
            );
            break;
        } else {
            if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
                break;
            }
            if poll_fds[0 as ::core::ffi::c_int as usize].revents == 0 {
                continue;
            }
            res = write((*(*unix_stream).priv_0).fd, buffer, count as size_t) as gssize;
            errsv = *__errno_location();
            if !(res == -(1 as ::core::ffi::c_int) as gssize) {
                break;
            }
            if errsv == EINTR || errsv == EAGAIN {
                continue;
            }
            g_set_error(
                error,
                g_io_error_quark(),
                g_io_error_from_errno(errsv as gint) as gint,
                glib_gettext(
                    b"Error writing to file descriptor: %s\0" as *const u8 as *const gchar,
                ),
                g_strerror(errsv as gint),
            );
            break;
        }
    }
    if nfds == 2 as ::core::ffi::c_int {
        g_cancellable_release_fd(cancellable);
    }
    return res;
}
pub const G_OUTPUT_VECTOR_IS_IOVEC: ::core::ffi::c_int = (::core::mem::size_of::<iovec>() as usize
    == ::core::mem::size_of::<GOutputVector>() as usize
    && ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize
        == ::core::mem::size_of::<gconstpointer>() as usize
    && 0 as ::core::ffi::c_ulong as glong == 0 as ::core::ffi::c_ulong as glong
    && ::core::mem::size_of::<size_t>() as usize == ::core::mem::size_of::<gsize>() as usize
    && 8 as ::core::ffi::c_ulong as glong == 8 as ::core::ffi::c_ulong as glong)
    as ::core::ffi::c_int;
unsafe extern "C" fn safe_c2rust_g_unix_output_stream_writev(
    mut stream: *mut GOutputStream,
    mut vectors: *const GOutputVector,
    mut n_vectors: gsize,
    mut bytes_written: *mut gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut unix_stream: *mut GUnixOutputStream = ::core::ptr::null_mut::<GUnixOutputStream>();
    let mut res: gssize = -(1 as ::core::ffi::c_int) as gssize;
    let mut poll_fds: [GPollFD; 2] = [_GPollFD {
        fd: 0,
        events: 0,
        revents: 0,
    }; 2];
    let mut nfds: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut poll_ret: ::core::ffi::c_int = 0;
    let mut iov: *mut iovec = ::core::ptr::null_mut::<iovec>();
    if !bytes_written.is_null() {
        *bytes_written = 0 as gsize;
    }
    if n_vectors > G_IOV_MAX as gsize {
        n_vectors = G_IOV_MAX as gsize;
    }
    unix_stream = stream as *mut ::core::ffi::c_void as *mut GUnixOutputStream;
    if G_OUTPUT_VECTOR_IS_IOVEC != 0 {
        iov = vectors as *mut iovec;
    } else {
        let mut i: gsize = 0;
        alloca_allocations.push(::std::vec::from_elem(
            0,
            (::core::mem::size_of::<iovec>() as usize).wrapping_mul(n_vectors as usize) as usize,
        ));
        iov = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut iovec;
        i = 0 as gsize;
        while i < n_vectors {
            let ref mut fresh0 = (*iov.offset(i as isize)).iov_base;
            *fresh0 = (*vectors.offset(i as isize)).buffer as *mut ::core::ffi::c_void;
            (*iov.offset(i as isize)).iov_len = (*vectors.offset(i as isize)).size as size_t;
            i = i.wrapping_add(1);
        }
    }
    poll_fds[0 as ::core::ffi::c_int as usize].fd = (*(*unix_stream).priv_0).fd as gint;
    poll_fds[0 as ::core::ffi::c_int as usize].events = G_IO_OUT as ::core::ffi::c_int as gushort;
    nfds += 1;
    if (*(*unix_stream).priv_0).can_poll() as ::core::ffi::c_int != 0
        && g_cancellable_make_pollfd(
            cancellable,
            (&raw mut poll_fds as *mut GPollFD).offset(1 as ::core::ffi::c_int as isize)
                as *mut GPollFD,
        ) != 0
    {
        nfds += 1;
    }
    loop {
        let mut errsv: ::core::ffi::c_int = 0;
        poll_fds[1 as ::core::ffi::c_int as usize].revents = 0 as gushort;
        poll_fds[0 as ::core::ffi::c_int as usize].revents =
            poll_fds[1 as ::core::ffi::c_int as usize].revents;
        loop {
            poll_ret = g_poll(
                &raw mut poll_fds as *mut GPollFD,
                nfds as guint,
                -(1 as gint),
            ) as ::core::ffi::c_int;
            errsv = *__errno_location();
            if !(poll_ret == -(1 as ::core::ffi::c_int) && errsv == EINTR) {
                break;
            }
        }
        if poll_ret == -(1 as ::core::ffi::c_int) {
            g_set_error(
                error,
                g_io_error_quark(),
                g_io_error_from_errno(errsv as gint) as gint,
                glib_gettext(
                    b"Error writing to file descriptor: %s\0" as *const u8 as *const gchar,
                ),
                g_strerror(errsv as gint),
            );
            break;
        } else {
            if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
                break;
            }
            if poll_fds[0 as ::core::ffi::c_int as usize].revents == 0 {
                continue;
            }
            res = writev(
                (*(*unix_stream).priv_0).fd,
                iov,
                n_vectors as ::core::ffi::c_int,
            ) as gssize;
            errsv = *__errno_location();
            if res == -(1 as ::core::ffi::c_int) as gssize {
                if errsv == EINTR || errsv == EAGAIN {
                    continue;
                }
                g_set_error(
                    error,
                    g_io_error_quark(),
                    g_io_error_from_errno(errsv as gint) as gint,
                    glib_gettext(
                        b"Error writing to file descriptor: %s\0" as *const u8 as *const gchar,
                    ),
                    g_strerror(errsv as gint),
                );
            }
            if !bytes_written.is_null() {
                *bytes_written = res as gsize;
            }
            break;
        }
    }
    if nfds == 2 as ::core::ffi::c_int {
        g_cancellable_release_fd(cancellable);
    }
    return (res != -(1 as ::core::ffi::c_int) as gssize) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_unix_output_stream_close(
    mut stream: *mut GOutputStream,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut unix_stream: *mut GUnixOutputStream = ::core::ptr::null_mut::<GUnixOutputStream>();
    let mut res: ::core::ffi::c_int = 0;
    unix_stream = stream as *mut ::core::ffi::c_void as *mut GUnixOutputStream;
    if (*(*unix_stream).priv_0).close_fd() == 0 {
        return TRUE;
    }
    res = close((*(*unix_stream).priv_0).fd);
    if res == -(1 as ::core::ffi::c_int) {
        let mut errsv: ::core::ffi::c_int = *__errno_location();
        g_set_error(
            error,
            g_io_error_quark(),
            g_io_error_from_errno(errsv as gint) as gint,
            glib_gettext(b"Error closing file descriptor: %s\0" as *const u8 as *const gchar),
            g_strerror(errsv as gint),
        );
    }
    return (res != -(1 as ::core::ffi::c_int)) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_unix_output_stream_pollable_can_poll(
    mut stream: *mut GPollableOutputStream,
) -> gboolean {
    return (*(*(stream as *mut ::core::ffi::c_void as *mut GUnixOutputStream)).priv_0).can_poll()
        as gboolean;
}
unsafe extern "C" fn safe_c2rust_g_unix_output_stream_pollable_is_writable(
    mut stream: *mut GPollableOutputStream,
) -> gboolean {
    let mut unix_stream: *mut GUnixOutputStream =
        stream as *mut ::core::ffi::c_void as *mut GUnixOutputStream;
    let mut poll_fd: GPollFD = _GPollFD {
        fd: 0,
        events: 0,
        revents: 0,
    };
    let mut result: gint = 0;
    poll_fd.fd = (*(*unix_stream).priv_0).fd as gint;
    poll_fd.events = G_IO_OUT as ::core::ffi::c_int as gushort;
    poll_fd.revents = 0 as gushort;
    loop {
        result = g_poll(&raw mut poll_fd, 1 as guint, 0 as gint);
        if !(result == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
            break;
        }
    }
    return (poll_fd.revents as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_unix_output_stream_pollable_create_source(
    mut stream: *mut GPollableOutputStream,
    mut cancellable: *mut GCancellable,
) -> *mut GSource {
    let mut unix_stream: *mut GUnixOutputStream =
        stream as *mut ::core::ffi::c_void as *mut GUnixOutputStream;
    let mut inner_source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    let mut cancellable_source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    let mut pollable_source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    pollable_source = g_pollable_source_new(stream as *mut ::core::ffi::c_void as *mut GObject);
    inner_source = g_unix_fd_source_new((*(*unix_stream).priv_0).fd as gint, G_IO_OUT);
    g_source_set_dummy_callback(inner_source);
    g_source_add_child_source(pollable_source, inner_source);
    g_source_unref(inner_source);
    if !cancellable.is_null() {
        cancellable_source = g_cancellable_source_new(cancellable);
        g_source_set_dummy_callback(cancellable_source);
        g_source_add_child_source(pollable_source, cancellable_source);
        g_source_unref(cancellable_source);
    }
    return pollable_source;
}
unsafe extern "C" fn safe_c2rust_g_unix_output_stream_pollable_writev_nonblocking(
    mut stream: *mut GPollableOutputStream,
    mut vectors: *const GOutputVector,
    mut n_vectors: gsize,
    mut bytes_written: *mut gsize,
    mut error: *mut *mut GError,
) -> GPollableReturn {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut unix_stream: *mut GUnixOutputStream =
        stream as *mut ::core::ffi::c_void as *mut GUnixOutputStream;
    let mut iov: *mut iovec = ::core::ptr::null_mut::<iovec>();
    let mut res: gssize = -(1 as ::core::ffi::c_int) as gssize;
    if g_pollable_output_stream_is_writable(stream) == 0 {
        *bytes_written = 0 as gsize;
        return G_POLLABLE_RETURN_WOULD_BLOCK;
    }
    if n_vectors > G_IOV_MAX as gsize {
        n_vectors = G_IOV_MAX as gsize;
    }
    if G_OUTPUT_VECTOR_IS_IOVEC != 0 {
        iov = vectors as *mut iovec;
    } else {
        let mut i: gsize = 0;
        alloca_allocations.push(::std::vec::from_elem(
            0,
            (::core::mem::size_of::<iovec>() as usize).wrapping_mul(n_vectors as usize) as usize,
        ));
        iov = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut iovec;
        i = 0 as gsize;
        while i < n_vectors {
            let ref mut fresh1 = (*iov.offset(i as isize)).iov_base;
            *fresh1 = (*vectors.offset(i as isize)).buffer as *mut ::core::ffi::c_void;
            (*iov.offset(i as isize)).iov_len = (*vectors.offset(i as isize)).size as size_t;
            i = i.wrapping_add(1);
        }
    }
    loop {
        let mut errsv: ::core::ffi::c_int = 0;
        res = writev(
            (*(*unix_stream).priv_0).fd,
            iov,
            n_vectors as ::core::ffi::c_int,
        ) as gssize;
        errsv = *__errno_location();
        if res == -(1 as ::core::ffi::c_int) as gssize {
            if errsv == EINTR {
                continue;
            }
            g_set_error(
                error,
                g_io_error_quark(),
                g_io_error_from_errno(errsv as gint) as gint,
                glib_gettext(
                    b"Error writing to file descriptor: %s\0" as *const u8 as *const gchar,
                ),
                g_strerror(errsv as gint),
            );
        }
        if !bytes_written.is_null() {
            *bytes_written = res as gsize;
        }
        break;
    }
    return (if res != -(1 as ::core::ffi::c_int) as gssize {
        G_POLLABLE_RETURN_OK as ::core::ffi::c_int
    } else {
        G_POLLABLE_RETURN_FAILED as ::core::ffi::c_int
    }) as GPollableReturn;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
