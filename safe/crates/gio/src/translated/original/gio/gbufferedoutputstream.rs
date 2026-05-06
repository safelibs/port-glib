use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GAsyncResult;
    pub type _GOutputStreamPrivate;
    pub type _GCancellable;
    pub type _GInputStream;
    pub type _GSeekable;
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
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
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
    fn g_object_unref(object: gpointer);
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
    fn g_value_set_boolean(value: *mut GValue, v_boolean: gboolean);
    fn g_value_get_boolean(value: *const GValue) -> gboolean;
    fn g_value_set_uint(value: *mut GValue, v_uint: guint);
    fn g_value_get_uint(value: *const GValue) -> guint;
    fn g_output_stream_get_type() -> GType;
    fn g_output_stream_write_all(
        stream: *mut GOutputStream,
        buffer: *const ::core::ffi::c_void,
        count: gsize,
        bytes_written: *mut gsize,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_output_stream_flush(
        stream: *mut GOutputStream,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_output_stream_close(
        stream: *mut GOutputStream,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_filter_output_stream_get_type() -> GType;
    fn g_filter_output_stream_get_close_base_stream(stream: *mut GFilterOutputStream) -> gboolean;
    fn g_seekable_get_type() -> GType;
    fn g_seekable_tell(seekable: *mut GSeekable) -> goffset;
    fn g_seekable_can_seek(seekable: *mut GSeekable) -> gboolean;
    fn g_seekable_seek(
        seekable: *mut GSeekable,
        offset: goffset,
        type_0: GSeekType,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_seekable_can_truncate(seekable: *mut GSeekable) -> gboolean;
    fn g_seekable_truncate(
        seekable: *mut GSeekable,
        offset: goffset,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
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
    fn g_task_set_priority(task: *mut GTask, priority: gint);
    fn g_task_set_source_tag(task: *mut GTask, source_tag: gpointer);
    fn g_task_set_static_name(task: *mut GTask, name: *const gchar);
    fn g_task_get_name(task: *mut GTask) -> *const gchar;
    fn g_task_is_valid(result: gpointer, source_object: gpointer) -> gboolean;
    fn g_task_run_in_thread(task: *mut GTask, task_func: GTaskThreadFunc);
    fn g_task_return_boolean(task: *mut GTask, result: gboolean);
    fn g_task_return_error(task: *mut GTask, error: *mut GError);
    fn g_task_propagate_boolean(task: *mut GTask, error: *mut *mut GError) -> gboolean;
    fn g_io_error_quark() -> GQuark;
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
pub type GAsyncResult = _GAsyncResult;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GBufferedOutputStream {
    pub parent_instance: GFilterOutputStream,
    pub priv_0: *mut GBufferedOutputStreamPrivate,
}
pub type GBufferedOutputStreamPrivate = _GBufferedOutputStreamPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GBufferedOutputStreamPrivate {
    pub buffer: *mut guint8,
    pub len: gsize,
    pub pos: goffset,
    pub auto_grow: gboolean,
}
pub type GFilterOutputStream = _GFilterOutputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFilterOutputStream {
    pub parent_instance: GOutputStream,
    pub base_stream: *mut GOutputStream,
}
pub type GOutputStream = _GOutputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GOutputStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GOutputStreamPrivate,
}
pub type GOutputStreamPrivate = _GOutputStreamPrivate;
pub type GBufferedOutputStream = _GBufferedOutputStream;
pub type GCancellable = _GCancellable;
pub type GInputStream = _GInputStream;
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
pub struct _GFilterOutputStreamClass {
    pub parent_class: GOutputStreamClass,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
}
pub type GFilterOutputStreamClass = _GFilterOutputStreamClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GBufferedOutputStreamClass {
    pub parent_class: GFilterOutputStreamClass,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
}
pub type GBufferedOutputStreamClass = _GBufferedOutputStreamClass;
pub const PROP_AUTO_GROW: C2RustUnnamed_1 = 2;
pub const PROP_BUFSIZE: C2RustUnnamed_1 = 1;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct FlushData {
    #[bitfield(name = "flush_stream", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "close_stream", ty = "guint", bits = "1..=1")]
    pub flush_stream_close_stream: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type GTaskThreadFunc =
    Option<unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> ()>;
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
pub const UINT_MAX: ::core::ffi::c_uint = (__INT_MAX__ as ::core::ffi::c_uint)
    .wrapping_mul(2 as ::core::ffi::c_uint)
    .wrapping_add(1 as ::core::ffi::c_uint);
pub const G_MAXUINT: ::core::ffi::c_uint = UINT_MAX;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const DEFAULT_BUFFER_SIZE: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
static mut safe_c2rust_GBufferedOutputStream_private_offset: gint = 0;
#[inline]
unsafe extern "C" fn safe_c2rust_g_buffered_output_stream_get_instance_private(
    mut self_0: *mut GBufferedOutputStream,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GBufferedOutputStream_private_offset as glong as isize)
        as gpointer;
}
unsafe extern "C" fn safe_c2rust_g_buffered_output_stream_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_buffered_output_stream_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GBufferedOutputStream_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GBufferedOutputStream_private_offset,
        );
    }
    safe_c2rust_g_buffered_output_stream_class_init(klass as *mut GBufferedOutputStreamClass);
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_buffered_output_stream_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_filter_output_stream_get_type(),
        g_intern_static_string(b"GBufferedOutputStream\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GBufferedOutputStreamClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_buffered_output_stream_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GBufferedOutputStream>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GBufferedOutputStream) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_buffered_output_stream_init
                    as unsafe extern "C" fn(*mut GBufferedOutputStream) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GBufferedOutputStream_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GBufferedOutputStreamPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GSeekableIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_buffered_output_stream_seekable_iface_init
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
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_buffered_output_stream_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_buffered_output_stream_get_type_once();
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
static mut safe_c2rust_g_buffered_output_stream_parent_class: gpointer = NULL;
unsafe extern "C" fn safe_c2rust_g_buffered_output_stream_class_init(
    mut klass: *mut GBufferedOutputStreamClass,
) {
    let mut object_class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    let mut ostream_class: *mut GOutputStreamClass = ::core::ptr::null_mut::<GOutputStreamClass>();
    object_class = klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*object_class).get_property = Some(
        safe_c2rust_g_buffered_output_stream_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*object_class).set_property = Some(
        safe_c2rust_g_buffered_output_stream_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*object_class).finalize = Some(
        safe_c2rust_g_buffered_output_stream_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    ostream_class = klass as *mut ::core::ffi::c_void as *mut GOutputStreamClass;
    (*ostream_class).write_fn = Some(
        safe_c2rust_g_buffered_output_stream_write
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
    (*ostream_class).flush = Some(
        safe_c2rust_g_buffered_output_stream_flush
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
    (*ostream_class).close_fn = Some(
        safe_c2rust_g_buffered_output_stream_close
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
    (*ostream_class).flush_async = Some(
        safe_c2rust_g_buffered_output_stream_flush_async
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
    (*ostream_class).flush_finish = Some(
        safe_c2rust_g_buffered_output_stream_flush_finish
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
    (*ostream_class).close_async = Some(
        safe_c2rust_g_buffered_output_stream_close_async
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
        safe_c2rust_g_buffered_output_stream_close_finish
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
        object_class,
        PROP_BUFSIZE as ::core::ffi::c_int as guint,
        g_param_spec_uint(
            b"buffer-size\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            1 as guint,
            G_MAXUINT,
            DEFAULT_BUFFER_SIZE as guint,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_AUTO_GROW as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"auto-grow\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int) as GParamFlags,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_buffered_output_stream_get_buffer_size(
    mut stream: *mut GBufferedOutputStream,
) -> gsize {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_buffered_output_stream_get_type();
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
            b"G_IS_BUFFERED_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gsize;
    }
    return (*(*stream).priv_0).len;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_buffered_output_stream_set_buffer_size(
    mut stream: *mut GBufferedOutputStream,
    mut size: gsize,
) {
    let mut priv_0: *mut GBufferedOutputStreamPrivate =
        ::core::ptr::null_mut::<GBufferedOutputStreamPrivate>();
    let mut buffer: *mut guint8 = ::core::ptr::null_mut::<guint8>();
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_buffered_output_stream_get_type();
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
            b"G_IS_BUFFERED_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    priv_0 = (*stream).priv_0;
    if size == (*priv_0).len {
        return;
    }
    if !(*priv_0).buffer.is_null() {
        size = if (*priv_0).pos > 0 as goffset {
            if size > (*priv_0).pos as gsize {
                size
            } else {
                (*priv_0).pos as gsize
            }
        } else {
            size
        };
        buffer = g_malloc(size) as *mut guint8;
        memcpy(
            buffer as *mut ::core::ffi::c_void,
            (*priv_0).buffer as *const ::core::ffi::c_void,
            (*priv_0).pos as size_t,
        );
        g_free((*priv_0).buffer as gpointer);
        (*priv_0).buffer = buffer;
        (*priv_0).len = size;
    } else {
        (*priv_0).buffer = g_malloc(size) as *mut guint8;
        (*priv_0).len = size;
        (*priv_0).pos = 0 as goffset;
    }
    g_object_notify(
        stream as *mut ::core::ffi::c_void as *mut GObject,
        b"buffer-size\0" as *const u8 as *const gchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_buffered_output_stream_get_auto_grow(
    mut stream: *mut GBufferedOutputStream,
) -> gboolean {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_buffered_output_stream_get_type();
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
            b"G_IS_BUFFERED_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*(*stream).priv_0).auto_grow;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_buffered_output_stream_set_auto_grow(
    mut stream: *mut GBufferedOutputStream,
    mut auto_grow: gboolean,
) {
    let mut priv_0: *mut GBufferedOutputStreamPrivate =
        ::core::ptr::null_mut::<GBufferedOutputStreamPrivate>();
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_buffered_output_stream_get_type();
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
            b"G_IS_BUFFERED_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    priv_0 = (*stream).priv_0;
    auto_grow = (auto_grow != FALSE) as ::core::ffi::c_int as gboolean;
    if (*priv_0).auto_grow != auto_grow {
        (*priv_0).auto_grow = auto_grow;
        g_object_notify(
            stream as *mut ::core::ffi::c_void as *mut GObject,
            b"auto-grow\0" as *const u8 as *const gchar,
        );
    }
}
unsafe extern "C" fn safe_c2rust_g_buffered_output_stream_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut stream: *mut GBufferedOutputStream = ::core::ptr::null_mut::<GBufferedOutputStream>();
    stream = object as *mut ::core::ffi::c_void as *mut GBufferedOutputStream;
    match prop_id {
        1 => {
            safe_c2rust_g_buffered_output_stream_set_buffer_size(
                stream,
                g_value_get_uint(value) as gsize,
            );
        }
        2 => {
            safe_c2rust_g_buffered_output_stream_set_auto_grow(stream, g_value_get_boolean(value));
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gbufferedoutputstream.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                297 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_buffered_output_stream_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut buffered_stream: *mut GBufferedOutputStream =
        ::core::ptr::null_mut::<GBufferedOutputStream>();
    let mut priv_0: *mut GBufferedOutputStreamPrivate =
        ::core::ptr::null_mut::<GBufferedOutputStreamPrivate>();
    buffered_stream = object as *mut ::core::ffi::c_void as *mut GBufferedOutputStream;
    priv_0 = (*buffered_stream).priv_0;
    match prop_id {
        1 => {
            g_value_set_uint(value, (*priv_0).len as guint);
        }
        2 => {
            g_value_set_boolean(value, (*priv_0).auto_grow);
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gbufferedoutputstream.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                326 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_buffered_output_stream_finalize(mut object: *mut GObject) {
    let mut stream: *mut GBufferedOutputStream = ::core::ptr::null_mut::<GBufferedOutputStream>();
    let mut priv_0: *mut GBufferedOutputStreamPrivate =
        ::core::ptr::null_mut::<GBufferedOutputStreamPrivate>();
    stream = object as *mut ::core::ffi::c_void as *mut GBufferedOutputStream;
    priv_0 = (*stream).priv_0;
    g_free((*priv_0).buffer as gpointer);
    (*(safe_c2rust_g_buffered_output_stream_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_buffered_output_stream_init(
    mut stream: *mut GBufferedOutputStream,
) {
    (*stream).priv_0 = safe_c2rust_g_buffered_output_stream_get_instance_private(stream)
        as *mut GBufferedOutputStreamPrivate;
}
unsafe extern "C" fn safe_c2rust_g_buffered_output_stream_seekable_iface_init(
    mut iface: *mut GSeekableIface,
) {
    (*iface).tell = Some(
        safe_c2rust_g_buffered_output_stream_tell
            as unsafe extern "C" fn(*mut GSeekable) -> goffset,
    ) as Option<unsafe extern "C" fn(*mut GSeekable) -> goffset>;
    (*iface).can_seek = Some(
        safe_c2rust_g_buffered_output_stream_can_seek
            as unsafe extern "C" fn(*mut GSeekable) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut GSeekable) -> gboolean>;
    (*iface).seek = Some(
        safe_c2rust_g_buffered_output_stream_seek
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
        safe_c2rust_g_buffered_output_stream_can_truncate
            as unsafe extern "C" fn(*mut GSeekable) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut GSeekable) -> gboolean>;
    (*iface).truncate_fn = Some(
        safe_c2rust_g_buffered_output_stream_truncate
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_buffered_output_stream_new(
    mut base_stream: *mut GOutputStream,
) -> *mut GOutputStream {
    let mut stream: *mut GOutputStream = ::core::ptr::null_mut::<GOutputStream>();
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = base_stream as *mut GTypeInstance;
            let mut __t: GType = g_output_stream_get_type();
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
            b"G_IS_OUTPUT_STREAM (base_stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GOutputStream>();
    }
    stream = g_object_new(
        safe_c2rust_g_buffered_output_stream_get_type(),
        b"base-stream\0" as *const u8 as *const gchar,
        base_stream,
        NULL,
    ) as *mut GOutputStream;
    return stream;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_buffered_output_stream_new_sized(
    mut base_stream: *mut GOutputStream,
    mut size: gsize,
) -> *mut GOutputStream {
    let mut stream: *mut GOutputStream = ::core::ptr::null_mut::<GOutputStream>();
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = base_stream as *mut GTypeInstance;
            let mut __t: GType = g_output_stream_get_type();
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
            b"G_IS_OUTPUT_STREAM (base_stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GOutputStream>();
    }
    stream = g_object_new(
        safe_c2rust_g_buffered_output_stream_get_type(),
        b"base-stream\0" as *const u8 as *const gchar,
        base_stream,
        b"buffer-size\0" as *const u8 as *const ::core::ffi::c_char,
        size,
        NULL,
    ) as *mut GOutputStream;
    return stream;
}
unsafe extern "C" fn safe_c2rust_flush_buffer(
    mut stream: *mut GBufferedOutputStream,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut priv_0: *mut GBufferedOutputStreamPrivate =
        ::core::ptr::null_mut::<GBufferedOutputStreamPrivate>();
    let mut base_stream: *mut GOutputStream = ::core::ptr::null_mut::<GOutputStream>();
    let mut res: gboolean = 0;
    let mut bytes_written: gsize = 0;
    let mut count: gsize = 0;
    priv_0 = (*stream).priv_0;
    bytes_written = 0 as gsize;
    base_stream = (*(stream as *mut ::core::ffi::c_void as *mut GFilterOutputStream)).base_stream;
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = base_stream as *mut GTypeInstance;
            let mut __t: GType = g_output_stream_get_type();
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
            b"G_IS_OUTPUT_STREAM (base_stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    res = g_output_stream_write_all(
        base_stream,
        (*priv_0).buffer as *const ::core::ffi::c_void,
        (*priv_0).pos as gsize,
        &raw mut bytes_written,
        cancellable,
        error,
    );
    count = ((*priv_0).pos as gsize).wrapping_sub(bytes_written);
    if count > 0 as gsize {
        memmove(
            (*priv_0).buffer as *mut ::core::ffi::c_void,
            (*priv_0).buffer.offset(bytes_written as isize) as *const ::core::ffi::c_void,
            count as size_t,
        );
    }
    (*priv_0).pos = ((*priv_0).pos as gsize).wrapping_sub(bytes_written) as goffset as goffset;
    return res;
}
unsafe extern "C" fn safe_c2rust_g_buffered_output_stream_write(
    mut stream: *mut GOutputStream,
    mut buffer: *const ::core::ffi::c_void,
    mut count: gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    let mut bstream: *mut GBufferedOutputStream = ::core::ptr::null_mut::<GBufferedOutputStream>();
    let mut priv_0: *mut GBufferedOutputStreamPrivate =
        ::core::ptr::null_mut::<GBufferedOutputStreamPrivate>();
    let mut res: gboolean = 0;
    let mut n: gsize = 0;
    let mut new_size: gsize = 0;
    bstream = stream as *mut ::core::ffi::c_void as *mut GBufferedOutputStream;
    priv_0 = (*bstream).priv_0;
    n = (*priv_0).len.wrapping_sub((*priv_0).pos as gsize);
    if (*priv_0).auto_grow != 0 && n < count {
        new_size = if (*priv_0).len.wrapping_mul(2 as gsize) > (*priv_0).len.wrapping_add(count) {
            (*priv_0).len.wrapping_mul(2 as gsize)
        } else {
            (*priv_0).len.wrapping_add(count)
        };
        safe_c2rust_g_buffered_output_stream_set_buffer_size(bstream, new_size);
    } else if n == 0 as gsize {
        res = safe_c2rust_flush_buffer(bstream, cancellable, error);
        if res == FALSE {
            return -(1 as ::core::ffi::c_int) as gssize;
        }
    }
    n = (*priv_0).len.wrapping_sub((*priv_0).pos as gsize);
    count = if count < n { count } else { n };
    memcpy(
        (*priv_0).buffer.offset((*priv_0).pos as isize) as *mut ::core::ffi::c_void,
        buffer,
        count as size_t,
    );
    (*priv_0).pos = ((*priv_0).pos as gsize).wrapping_add(count) as goffset as goffset;
    return count as gssize;
}
unsafe extern "C" fn safe_c2rust_g_buffered_output_stream_flush(
    mut stream: *mut GOutputStream,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut bstream: *mut GBufferedOutputStream = ::core::ptr::null_mut::<GBufferedOutputStream>();
    let mut base_stream: *mut GOutputStream = ::core::ptr::null_mut::<GOutputStream>();
    let mut res: gboolean = 0;
    bstream = stream as *mut ::core::ffi::c_void as *mut GBufferedOutputStream;
    base_stream = (*(stream as *mut ::core::ffi::c_void as *mut GFilterOutputStream)).base_stream;
    res = safe_c2rust_flush_buffer(bstream, cancellable, error);
    if res == FALSE {
        return FALSE;
    }
    res = g_output_stream_flush(base_stream, cancellable, error);
    return res;
}
unsafe extern "C" fn safe_c2rust_g_buffered_output_stream_close(
    mut stream: *mut GOutputStream,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut bstream: *mut GBufferedOutputStream = ::core::ptr::null_mut::<GBufferedOutputStream>();
    let mut base_stream: *mut GOutputStream = ::core::ptr::null_mut::<GOutputStream>();
    let mut res: gboolean = 0;
    bstream = stream as *mut ::core::ffi::c_void as *mut GBufferedOutputStream;
    base_stream = (*(bstream as *mut ::core::ffi::c_void as *mut GFilterOutputStream)).base_stream;
    res = safe_c2rust_flush_buffer(bstream, cancellable, error);
    if g_filter_output_stream_get_close_base_stream(
        stream as *mut ::core::ffi::c_void as *mut GFilterOutputStream,
    ) != 0
    {
        if res != 0 {
            res = g_output_stream_close(base_stream, cancellable, error);
        } else {
            g_output_stream_close(
                base_stream,
                cancellable,
                ::core::ptr::null_mut::<*mut GError>(),
            );
        }
    }
    return res;
}
unsafe extern "C" fn safe_c2rust_g_buffered_output_stream_tell(
    mut seekable: *mut GSeekable,
) -> goffset {
    let mut bstream: *mut GBufferedOutputStream = ::core::ptr::null_mut::<GBufferedOutputStream>();
    let mut priv_0: *mut GBufferedOutputStreamPrivate =
        ::core::ptr::null_mut::<GBufferedOutputStreamPrivate>();
    let mut base_stream: *mut GOutputStream = ::core::ptr::null_mut::<GOutputStream>();
    let mut base_stream_seekable: *mut GSeekable = ::core::ptr::null_mut::<GSeekable>();
    let mut base_offset: goffset = 0;
    bstream = seekable as *mut ::core::ffi::c_void as *mut GBufferedOutputStream;
    priv_0 = (*bstream).priv_0;
    base_stream = (*(seekable as *mut ::core::ffi::c_void as *mut GFilterOutputStream)).base_stream;
    if ({
        let mut __inst: *mut GTypeInstance = base_stream as *mut GTypeInstance;
        let mut __t: GType = g_seekable_get_type();
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
        return 0 as goffset;
    }
    base_stream_seekable = base_stream as *mut ::core::ffi::c_void as *mut GSeekable;
    base_offset = g_seekable_tell(base_stream_seekable);
    return base_offset + (*priv_0).pos;
}
unsafe extern "C" fn safe_c2rust_g_buffered_output_stream_can_seek(
    mut seekable: *mut GSeekable,
) -> gboolean {
    let mut base_stream: *mut GOutputStream = ::core::ptr::null_mut::<GOutputStream>();
    base_stream = (*(seekable as *mut ::core::ffi::c_void as *mut GFilterOutputStream)).base_stream;
    return (({
        let mut __inst: *mut GTypeInstance = base_stream as *mut GTypeInstance;
        let mut __t: GType = g_seekable_get_type();
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
        && g_seekable_can_seek(base_stream as *mut ::core::ffi::c_void as *mut GSeekable) != 0)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_buffered_output_stream_seek(
    mut seekable: *mut GSeekable,
    mut offset: goffset,
    mut type_0: GSeekType,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut bstream: *mut GBufferedOutputStream = ::core::ptr::null_mut::<GBufferedOutputStream>();
    let mut base_stream: *mut GOutputStream = ::core::ptr::null_mut::<GOutputStream>();
    let mut base_stream_seekable: *mut GSeekable = ::core::ptr::null_mut::<GSeekable>();
    let mut flushed: gboolean = 0;
    bstream = seekable as *mut ::core::ffi::c_void as *mut GBufferedOutputStream;
    base_stream = (*(seekable as *mut ::core::ffi::c_void as *mut GFilterOutputStream)).base_stream;
    if ({
        let mut __inst: *mut GTypeInstance = base_stream as *mut GTypeInstance;
        let mut __t: GType = g_seekable_get_type();
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
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(b"Seek not supported on base stream\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    base_stream_seekable = base_stream as *mut ::core::ffi::c_void as *mut GSeekable;
    flushed = safe_c2rust_flush_buffer(bstream, cancellable, error);
    if flushed == 0 {
        return FALSE;
    }
    return g_seekable_seek(base_stream_seekable, offset, type_0, cancellable, error);
}
unsafe extern "C" fn safe_c2rust_g_buffered_output_stream_can_truncate(
    mut seekable: *mut GSeekable,
) -> gboolean {
    let mut base_stream: *mut GOutputStream = ::core::ptr::null_mut::<GOutputStream>();
    base_stream = (*(seekable as *mut ::core::ffi::c_void as *mut GFilterOutputStream)).base_stream;
    return (({
        let mut __inst: *mut GTypeInstance = base_stream as *mut GTypeInstance;
        let mut __t: GType = g_seekable_get_type();
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
        && g_seekable_can_truncate(base_stream as *mut ::core::ffi::c_void as *mut GSeekable) != 0)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_buffered_output_stream_truncate(
    mut seekable: *mut GSeekable,
    mut offset: goffset,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut bstream: *mut GBufferedOutputStream = ::core::ptr::null_mut::<GBufferedOutputStream>();
    let mut base_stream: *mut GOutputStream = ::core::ptr::null_mut::<GOutputStream>();
    let mut base_stream_seekable: *mut GSeekable = ::core::ptr::null_mut::<GSeekable>();
    let mut flushed: gboolean = 0;
    bstream = seekable as *mut ::core::ffi::c_void as *mut GBufferedOutputStream;
    base_stream = (*(seekable as *mut ::core::ffi::c_void as *mut GFilterOutputStream)).base_stream;
    if ({
        let mut __inst: *mut GTypeInstance = base_stream as *mut GTypeInstance;
        let mut __t: GType = g_seekable_get_type();
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
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(b"Truncate not supported on base stream\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    base_stream_seekable = base_stream as *mut ::core::ffi::c_void as *mut GSeekable;
    flushed = safe_c2rust_flush_buffer(bstream, cancellable, error);
    if flushed == 0 {
        return FALSE;
    }
    return g_seekable_truncate(base_stream_seekable, offset, cancellable, error);
}
unsafe extern "C" fn safe_c2rust_free_flush_data(mut data: gpointer) {
    g_slice_free1(::core::mem::size_of::<FlushData>() as gsize, data);
}
unsafe extern "C" fn safe_c2rust_flush_buffer_thread(
    mut task: *mut GTask,
    mut object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut stream: *mut GBufferedOutputStream = ::core::ptr::null_mut::<GBufferedOutputStream>();
    let mut base_stream: *mut GOutputStream = ::core::ptr::null_mut::<GOutputStream>();
    let mut fdata: *mut FlushData = ::core::ptr::null_mut::<FlushData>();
    let mut res: gboolean = 0;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    stream = object as *mut GBufferedOutputStream;
    fdata = task_data as *mut FlushData;
    base_stream = (*(stream as *mut ::core::ffi::c_void as *mut GFilterOutputStream)).base_stream;
    res = safe_c2rust_flush_buffer(stream, cancellable, &raw mut error);
    if res != 0 && (*fdata).flush_stream() as ::core::ffi::c_int != 0 {
        res = g_output_stream_flush(base_stream, cancellable, &raw mut error);
    }
    if (*fdata).close_stream() != 0 {
        if g_filter_output_stream_get_close_base_stream(
            stream as *mut ::core::ffi::c_void as *mut GFilterOutputStream,
        ) != 0
        {
            if res == FALSE {
                g_output_stream_close(
                    base_stream,
                    cancellable,
                    ::core::ptr::null_mut::<*mut GError>(),
                );
            } else {
                res = g_output_stream_close(base_stream, cancellable, &raw mut error);
            }
        }
    }
    if res == FALSE {
        g_task_return_error(task, error);
    } else {
        g_task_return_boolean(task, TRUE);
    };
}
unsafe extern "C" fn safe_c2rust_g_buffered_output_stream_flush_async(
    mut stream: *mut GOutputStream,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut fdata: *mut FlushData = ::core::ptr::null_mut::<FlushData>();
    fdata = ({
        let mut __s: gsize = ::core::mem::size_of::<FlushData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut FlushData;
    (*fdata).set_flush_stream(TRUE as guint as guint);
    (*fdata).set_close_stream(FALSE as guint as guint);
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
            safe_c2rust_g_buffered_output_stream_flush_async
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
            b"g_buffered_output_stream_flush_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        fdata as gpointer,
        Some(safe_c2rust_free_flush_data as unsafe extern "C" fn(gpointer) -> ()),
    );
    g_task_set_priority(task, io_priority as gint);
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_flush_buffer_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_buffered_output_stream_flush_finish(
    mut stream: *mut GOutputStream,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, stream as gpointer) != 0 {
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
            b"g_task_is_valid (result, stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
}
unsafe extern "C" fn safe_c2rust_g_buffered_output_stream_close_async(
    mut stream: *mut GOutputStream,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut fdata: *mut FlushData = ::core::ptr::null_mut::<FlushData>();
    fdata = ({
        let mut __s: gsize = ::core::mem::size_of::<FlushData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut FlushData;
    (*fdata).set_close_stream(TRUE as guint as guint);
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
            safe_c2rust_g_buffered_output_stream_close_async
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
            b"g_buffered_output_stream_close_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        fdata as gpointer,
        Some(safe_c2rust_free_flush_data as unsafe extern "C" fn(gpointer) -> ()),
    );
    g_task_set_priority(task, io_priority as gint);
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_flush_buffer_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_buffered_output_stream_close_finish(
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
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
