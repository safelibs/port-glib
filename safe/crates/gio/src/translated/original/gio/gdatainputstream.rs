use ::c2rust_asm_casts;
use ::c2rust_asm_casts::AsmCastTrait;
use ::core::arch::asm;
extern "C" {
    pub type _GData;
    pub type _GAsyncResult;
    pub type _GBufferedInputStreamPrivate;
    pub type _GInputStreamPrivate;
    pub type _GCancellablePrivate;
    pub type _GTask;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_convert_error_quark() -> GQuark;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_utf8_validate(str: *const gchar, max_len: gssize, end: *mut *const gchar) -> gboolean;
    fn g_memdup2(mem: gconstpointer, byte_size: gsize) -> gpointer;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_warn_message(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        warnexpr: *const ::core::ffi::c_char,
    );
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn g_assertion_message_cmpint(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
        arg1: guint64,
        cmp: *const ::core::ffi::c_char,
        arg2: guint64,
        numtype: ::core::ffi::c_char,
    );
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
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
    fn g_object_unref(object: gpointer);
    fn g_value_set_enum(value: *mut GValue, v_enum: gint);
    fn g_value_get_enum(value: *const GValue) -> gint;
    fn g_param_spec_enum(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        enum_type: GType,
        default_value: gint,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_input_stream_get_type() -> GType;
    fn g_input_stream_read(
        stream: *mut GInputStream,
        buffer: *mut ::core::ffi::c_void,
        count: gsize,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gssize;
    fn g_input_stream_skip(
        stream: *mut GInputStream,
        count: gsize,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gssize;
    fn g_buffered_input_stream_get_type() -> GType;
    fn g_buffered_input_stream_get_buffer_size(stream: *mut GBufferedInputStream) -> gsize;
    fn g_buffered_input_stream_set_buffer_size(stream: *mut GBufferedInputStream, size: gsize);
    fn g_buffered_input_stream_get_available(stream: *mut GBufferedInputStream) -> gsize;
    fn g_buffered_input_stream_peek_buffer(
        stream: *mut GBufferedInputStream,
        count: *mut gsize,
    ) -> *const ::core::ffi::c_void;
    fn g_buffered_input_stream_fill(
        stream: *mut GBufferedInputStream,
        count: gssize,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gssize;
    fn g_buffered_input_stream_fill_async(
        stream: *mut GBufferedInputStream,
        count: gssize,
        io_priority: ::core::ffi::c_int,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_buffered_input_stream_fill_finish(
        stream: *mut GBufferedInputStream,
        result: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gssize;
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
    fn g_task_get_source_object(task: *mut GTask) -> gpointer;
    fn g_task_get_task_data(task: *mut GTask) -> gpointer;
    fn g_task_get_priority(task: *mut GTask) -> gint;
    fn g_task_get_cancellable(task: *mut GTask) -> *mut GCancellable;
    fn g_task_get_name(task: *mut GTask) -> *const gchar;
    fn g_task_is_valid(result: gpointer, source_object: gpointer) -> gboolean;
    fn g_task_return_pointer(task: *mut GTask, result: gpointer, result_destroy: GDestroyNotify);
    fn g_task_return_error(task: *mut GTask, error: *mut GError);
    fn g_task_propagate_pointer(task: *mut GTask, error: *mut *mut GError) -> gpointer;
    fn g_cancellable_get_type() -> GType;
    fn g_data_stream_byte_order_get_type() -> GType;
    fn g_data_stream_newline_type_get_type() -> GType;
    fn g_io_error_quark() -> GQuark;
    fn glib_gettext(str: *const gchar) -> *const gchar;
}
pub type size_t = usize;
pub type guint8 = ::core::ffi::c_uchar;
pub type gint16 = ::core::ffi::c_short;
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
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const G_CONVERT_ERROR_EMBEDDED_NUL: C2RustUnnamed = 7;
pub const G_CONVERT_ERROR_NO_MEMORY: C2RustUnnamed = 6;
pub const G_CONVERT_ERROR_NOT_ABSOLUTE_PATH: C2RustUnnamed = 5;
pub const G_CONVERT_ERROR_BAD_URI: C2RustUnnamed = 4;
pub const G_CONVERT_ERROR_PARTIAL_INPUT: C2RustUnnamed = 3;
pub const G_CONVERT_ERROR_FAILED: C2RustUnnamed = 2;
pub const G_CONVERT_ERROR_ILLEGAL_SEQUENCE: C2RustUnnamed = 1;
pub const G_CONVERT_ERROR_NO_CONVERSION: C2RustUnnamed = 0;
pub type GData = _GData;
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
pub type GType = gsize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GValue {
    pub g_type: GType,
    pub data: [C2RustUnnamed_0; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
pub type GDataStreamByteOrder = ::core::ffi::c_uint;
pub const G_DATA_STREAM_BYTE_ORDER_HOST_ENDIAN: GDataStreamByteOrder = 2;
pub const G_DATA_STREAM_BYTE_ORDER_LITTLE_ENDIAN: GDataStreamByteOrder = 1;
pub const G_DATA_STREAM_BYTE_ORDER_BIG_ENDIAN: GDataStreamByteOrder = 0;
pub type GDataStreamNewlineType = ::core::ffi::c_uint;
pub const G_DATA_STREAM_NEWLINE_TYPE_ANY: GDataStreamNewlineType = 3;
pub const G_DATA_STREAM_NEWLINE_TYPE_CR_LF: GDataStreamNewlineType = 2;
pub const G_DATA_STREAM_NEWLINE_TYPE_CR: GDataStreamNewlineType = 1;
pub const G_DATA_STREAM_NEWLINE_TYPE_LF: GDataStreamNewlineType = 0;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const G_IO_ERROR_DESTINATION_UNSET: C2RustUnnamed_1 = 48;
pub const G_IO_ERROR_NO_SUCH_DEVICE: C2RustUnnamed_1 = 47;
pub const G_IO_ERROR_MESSAGE_TOO_LARGE: C2RustUnnamed_1 = 46;
pub const G_IO_ERROR_NOT_CONNECTED: C2RustUnnamed_1 = 45;
pub const G_IO_ERROR_CONNECTION_CLOSED: C2RustUnnamed_1 = 44;
pub const G_IO_ERROR_BROKEN_PIPE: C2RustUnnamed_1 = 44;
pub const G_IO_ERROR_PROXY_NOT_ALLOWED: C2RustUnnamed_1 = 43;
pub const G_IO_ERROR_PROXY_NEED_AUTH: C2RustUnnamed_1 = 42;
pub const G_IO_ERROR_PROXY_AUTH_FAILED: C2RustUnnamed_1 = 41;
pub const G_IO_ERROR_PROXY_FAILED: C2RustUnnamed_1 = 40;
pub const G_IO_ERROR_CONNECTION_REFUSED: C2RustUnnamed_1 = 39;
pub const G_IO_ERROR_NETWORK_UNREACHABLE: C2RustUnnamed_1 = 38;
pub const G_IO_ERROR_HOST_UNREACHABLE: C2RustUnnamed_1 = 37;
pub const G_IO_ERROR_DBUS_ERROR: C2RustUnnamed_1 = 36;
pub const G_IO_ERROR_INVALID_DATA: C2RustUnnamed_1 = 35;
pub const G_IO_ERROR_PARTIAL_INPUT: C2RustUnnamed_1 = 34;
pub const G_IO_ERROR_ADDRESS_IN_USE: C2RustUnnamed_1 = 33;
pub const G_IO_ERROR_NOT_INITIALIZED: C2RustUnnamed_1 = 32;
pub const G_IO_ERROR_TOO_MANY_OPEN_FILES: C2RustUnnamed_1 = 31;
pub const G_IO_ERROR_FAILED_HANDLED: C2RustUnnamed_1 = 30;
pub const G_IO_ERROR_WOULD_MERGE: C2RustUnnamed_1 = 29;
pub const G_IO_ERROR_HOST_NOT_FOUND: C2RustUnnamed_1 = 28;
pub const G_IO_ERROR_WOULD_BLOCK: C2RustUnnamed_1 = 27;
pub const G_IO_ERROR_BUSY: C2RustUnnamed_1 = 26;
pub const G_IO_ERROR_WOULD_RECURSE: C2RustUnnamed_1 = 25;
pub const G_IO_ERROR_TIMED_OUT: C2RustUnnamed_1 = 24;
pub const G_IO_ERROR_WRONG_ETAG: C2RustUnnamed_1 = 23;
pub const G_IO_ERROR_CANT_CREATE_BACKUP: C2RustUnnamed_1 = 22;
pub const G_IO_ERROR_READ_ONLY: C2RustUnnamed_1 = 21;
pub const G_IO_ERROR_PENDING: C2RustUnnamed_1 = 20;
pub const G_IO_ERROR_CANCELLED: C2RustUnnamed_1 = 19;
pub const G_IO_ERROR_CLOSED: C2RustUnnamed_1 = 18;
pub const G_IO_ERROR_ALREADY_MOUNTED: C2RustUnnamed_1 = 17;
pub const G_IO_ERROR_NOT_MOUNTED: C2RustUnnamed_1 = 16;
pub const G_IO_ERROR_NOT_SUPPORTED: C2RustUnnamed_1 = 15;
pub const G_IO_ERROR_PERMISSION_DENIED: C2RustUnnamed_1 = 14;
pub const G_IO_ERROR_INVALID_ARGUMENT: C2RustUnnamed_1 = 13;
pub const G_IO_ERROR_NO_SPACE: C2RustUnnamed_1 = 12;
pub const G_IO_ERROR_TOO_MANY_LINKS: C2RustUnnamed_1 = 11;
pub const G_IO_ERROR_INVALID_FILENAME: C2RustUnnamed_1 = 10;
pub const G_IO_ERROR_FILENAME_TOO_LONG: C2RustUnnamed_1 = 9;
pub const G_IO_ERROR_NOT_MOUNTABLE_FILE: C2RustUnnamed_1 = 8;
pub const G_IO_ERROR_NOT_SYMBOLIC_LINK: C2RustUnnamed_1 = 7;
pub const G_IO_ERROR_NOT_REGULAR_FILE: C2RustUnnamed_1 = 6;
pub const G_IO_ERROR_NOT_EMPTY: C2RustUnnamed_1 = 5;
pub const G_IO_ERROR_NOT_DIRECTORY: C2RustUnnamed_1 = 4;
pub const G_IO_ERROR_IS_DIRECTORY: C2RustUnnamed_1 = 3;
pub const G_IO_ERROR_EXISTS: C2RustUnnamed_1 = 2;
pub const G_IO_ERROR_NOT_FOUND: C2RustUnnamed_1 = 1;
pub const G_IO_ERROR_FAILED: C2RustUnnamed_1 = 0;
pub type GAsyncResult = _GAsyncResult;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GBufferedInputStream {
    pub parent_instance: GFilterInputStream,
    pub priv_0: *mut GBufferedInputStreamPrivate,
}
pub type GBufferedInputStreamPrivate = _GBufferedInputStreamPrivate;
pub type GFilterInputStream = _GFilterInputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFilterInputStream {
    pub parent_instance: GInputStream,
    pub base_stream: *mut GInputStream,
}
pub type GInputStream = _GInputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInputStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GInputStreamPrivate,
}
pub type GInputStreamPrivate = _GInputStreamPrivate;
pub type GBufferedInputStream = _GBufferedInputStream;
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
pub struct _GDataInputStream {
    pub parent_instance: GBufferedInputStream,
    pub priv_0: *mut GDataInputStreamPrivate,
}
pub type GDataInputStreamPrivate = _GDataInputStreamPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDataInputStreamPrivate {
    pub byte_order: GDataStreamByteOrder,
    pub newline_type: GDataStreamNewlineType,
}
pub type GDataInputStream = _GDataInputStream;
pub type GTask = _GTask;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInputStreamClass {
    pub parent_class: GObjectClass,
    pub read_fn: Option<
        unsafe extern "C" fn(
            *mut GInputStream,
            *mut ::core::ffi::c_void,
            gsize,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gssize,
    >,
    pub skip: Option<
        unsafe extern "C" fn(
            *mut GInputStream,
            gsize,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gssize,
    >,
    pub close_fn: Option<
        unsafe extern "C" fn(*mut GInputStream, *mut GCancellable, *mut *mut GError) -> gboolean,
    >,
    pub read_async: Option<
        unsafe extern "C" fn(
            *mut GInputStream,
            *mut ::core::ffi::c_void,
            gsize,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub read_finish: Option<
        unsafe extern "C" fn(*mut GInputStream, *mut GAsyncResult, *mut *mut GError) -> gssize,
    >,
    pub skip_async: Option<
        unsafe extern "C" fn(
            *mut GInputStream,
            gsize,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub skip_finish: Option<
        unsafe extern "C" fn(*mut GInputStream, *mut GAsyncResult, *mut *mut GError) -> gssize,
    >,
    pub close_async: Option<
        unsafe extern "C" fn(
            *mut GInputStream,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub close_finish: Option<
        unsafe extern "C" fn(*mut GInputStream, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    >,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
}
pub type GInputStreamClass = _GInputStreamClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFilterInputStreamClass {
    pub parent_class: GInputStreamClass,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
}
pub type GFilterInputStreamClass = _GFilterInputStreamClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GBufferedInputStreamClass {
    pub parent_class: GFilterInputStreamClass,
    pub fill: Option<
        unsafe extern "C" fn(
            *mut GBufferedInputStream,
            gssize,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gssize,
    >,
    pub fill_async: Option<
        unsafe extern "C" fn(
            *mut GBufferedInputStream,
            gssize,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub fill_finish: Option<
        unsafe extern "C" fn(
            *mut GBufferedInputStream,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> gssize,
    >,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
}
pub type GBufferedInputStreamClass = _GBufferedInputStreamClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDataInputStreamClass {
    pub parent_class: GBufferedInputStreamClass,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
}
pub type GDataInputStreamClass = _GDataInputStreamClass;
pub const PROP_NEWLINE_TYPE: C2RustUnnamed_2 = 2;
pub const PROP_BYTE_ORDER: C2RustUnnamed_2 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GDataInputStreamReadData {
    pub last_saw_cr: gboolean,
    pub checked: gsize,
    pub stop_chars: *mut gchar,
    pub stop_chars_len: gsize,
    pub length: gsize,
}
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_2 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_input_stream_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_data_input_stream_get_type_once();
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
static mut safe_c2rust_g_data_input_stream_parent_class: gpointer = NULL;
unsafe extern "C" fn safe_c2rust_g_data_input_stream_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_data_input_stream_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GDataInputStream_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GDataInputStream_private_offset,
        );
    }
    safe_c2rust_g_data_input_stream_class_init(klass as *mut GDataInputStreamClass);
}
static mut safe_c2rust_GDataInputStream_private_offset: gint = 0;
#[inline]
unsafe extern "C" fn safe_c2rust_g_data_input_stream_get_instance_private(
    mut self_0: *mut GDataInputStream,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GDataInputStream_private_offset as glong as isize)
        as gpointer;
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_data_input_stream_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_buffered_input_stream_get_type(),
        g_intern_static_string(b"GDataInputStream\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GDataInputStreamClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_data_input_stream_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GDataInputStream>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDataInputStream) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_data_input_stream_init
                    as unsafe extern "C" fn(*mut GDataInputStream) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GDataInputStream_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GDataInputStreamPrivate>() as gsize,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_data_input_stream_class_init(
    mut klass: *mut GDataInputStreamClass,
) {
    let mut object_class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    object_class = klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*object_class).get_property = Some(
        safe_c2rust_g_data_input_stream_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*object_class).set_property = Some(
        safe_c2rust_g_data_input_stream_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    g_object_class_install_property(
        object_class,
        PROP_BYTE_ORDER as ::core::ffi::c_int as guint,
        g_param_spec_enum(
            b"byte-order\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_data_stream_byte_order_get_type(),
            G_DATA_STREAM_BYTE_ORDER_BIG_ENDIAN as ::core::ffi::c_int as gint,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_NEWLINE_TYPE as ::core::ffi::c_int as guint,
        g_param_spec_enum(
            b"newline-type\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_data_stream_newline_type_get_type(),
            G_DATA_STREAM_NEWLINE_TYPE_LF as ::core::ffi::c_int as gint,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int) as GParamFlags,
        ),
    );
}
unsafe extern "C" fn safe_c2rust_g_data_input_stream_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut dstream: *mut GDataInputStream = ::core::ptr::null_mut::<GDataInputStream>();
    dstream = object as *mut ::core::ffi::c_void as *mut GDataInputStream;
    match prop_id {
        1 => {
            safe_c2rust_g_data_input_stream_set_byte_order(
                dstream,
                g_value_get_enum(value) as GDataStreamByteOrder,
            );
        }
        2 => {
            safe_c2rust_g_data_input_stream_set_newline_type(
                dstream,
                g_value_get_enum(value) as GDataStreamNewlineType,
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdatainputstream.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                125 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_data_input_stream_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut priv_0: *mut GDataInputStreamPrivate =
        ::core::ptr::null_mut::<GDataInputStreamPrivate>();
    let mut dstream: *mut GDataInputStream = ::core::ptr::null_mut::<GDataInputStream>();
    dstream = object as *mut ::core::ffi::c_void as *mut GDataInputStream;
    priv_0 = (*dstream).priv_0;
    match prop_id {
        1 => {
            g_value_set_enum(value, (*priv_0).byte_order as gint);
        }
        2 => {
            g_value_set_enum(value, (*priv_0).newline_type as gint);
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdatainputstream.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                154 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_data_input_stream_init(mut stream: *mut GDataInputStream) {
    (*stream).priv_0 = safe_c2rust_g_data_input_stream_get_instance_private(stream)
        as *mut GDataInputStreamPrivate;
    (*(*stream).priv_0).byte_order = G_DATA_STREAM_BYTE_ORDER_BIG_ENDIAN;
    (*(*stream).priv_0).newline_type = G_DATA_STREAM_NEWLINE_TYPE_LF;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_input_stream_new(
    mut base_stream: *mut GInputStream,
) -> *mut GDataInputStream {
    let mut stream: *mut GDataInputStream = ::core::ptr::null_mut::<GDataInputStream>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = base_stream as *mut GTypeInstance;
            let mut __t: GType = g_input_stream_get_type();
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
            b"G_IS_INPUT_STREAM (base_stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDataInputStream>();
    }
    stream = g_object_new(
        safe_c2rust_g_data_input_stream_get_type(),
        b"base-stream\0" as *const u8 as *const gchar,
        base_stream,
        NULL,
    ) as *mut GDataInputStream;
    return stream;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_input_stream_set_byte_order(
    mut stream: *mut GDataInputStream,
    mut order: GDataStreamByteOrder,
) {
    let mut priv_0: *mut GDataInputStreamPrivate =
        ::core::ptr::null_mut::<GDataInputStreamPrivate>();
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_data_input_stream_get_type();
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
            b"G_IS_DATA_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    priv_0 = (*stream).priv_0;
    if (*priv_0).byte_order as ::core::ffi::c_uint != order as ::core::ffi::c_uint {
        (*priv_0).byte_order = order;
        g_object_notify(
            stream as *mut ::core::ffi::c_void as *mut GObject,
            b"byte-order\0" as *const u8 as *const gchar,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_input_stream_get_byte_order(
    mut stream: *mut GDataInputStream,
) -> GDataStreamByteOrder {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_data_input_stream_get_type();
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
            b"G_IS_DATA_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_DATA_STREAM_BYTE_ORDER_HOST_ENDIAN;
    }
    return (*(*stream).priv_0).byte_order;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_input_stream_set_newline_type(
    mut stream: *mut GDataInputStream,
    mut type_0: GDataStreamNewlineType,
) {
    let mut priv_0: *mut GDataInputStreamPrivate =
        ::core::ptr::null_mut::<GDataInputStreamPrivate>();
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_data_input_stream_get_type();
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
            b"G_IS_DATA_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    priv_0 = (*stream).priv_0;
    if (*priv_0).newline_type as ::core::ffi::c_uint != type_0 as ::core::ffi::c_uint {
        (*priv_0).newline_type = type_0;
        g_object_notify(
            stream as *mut ::core::ffi::c_void as *mut GObject,
            b"newline-type\0" as *const u8 as *const gchar,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_input_stream_get_newline_type(
    mut stream: *mut GDataInputStream,
) -> GDataStreamNewlineType {
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_data_input_stream_get_type();
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
            b"G_IS_DATA_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_DATA_STREAM_NEWLINE_TYPE_ANY;
    }
    return (*(*stream).priv_0).newline_type;
}
unsafe extern "C" fn safe_c2rust_read_data(
    mut stream: *mut GDataInputStream,
    mut buffer: *mut ::core::ffi::c_void,
    mut size: gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut available: gsize = 0;
    let mut res: gssize = 0;
    loop {
        available = g_buffered_input_stream_get_available(
            stream as *mut ::core::ffi::c_void as *mut GBufferedInputStream,
        );
        if !(available < size) {
            break;
        }
        res = g_buffered_input_stream_fill(
            stream as *mut ::core::ffi::c_void as *mut GBufferedInputStream,
            size.wrapping_sub(available) as gssize,
            cancellable,
            error,
        );
        if res < 0 as gssize {
            return FALSE;
        }
        if res == 0 as gssize {
            g_set_error_literal(
                error,
                g_io_error_quark(),
                G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                glib_gettext(b"Unexpected early end-of-stream\0" as *const u8 as *const gchar),
            );
            return FALSE;
        }
    }
    res = g_input_stream_read(
        stream as *mut ::core::ffi::c_void as *mut GInputStream,
        buffer,
        size,
        ::core::ptr::null_mut::<GCancellable>(),
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if !(({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if res >= 0 as gssize && res as gsize == size {
            _g_boolean_var_15 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_15 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_15
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdatainputstream.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            307 as ::core::ffi::c_int,
            G_STRFUNC,
            b"res >= 0 && (gsize) res == size\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_input_stream_read_byte(
    mut stream: *mut GDataInputStream,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> guchar {
    let mut c: guchar = 0;
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_data_input_stream_get_type();
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
            b"G_IS_DATA_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return '\0' as i32 as guchar;
    }
    if safe_c2rust_read_data(
        stream,
        &raw mut c as *mut ::core::ffi::c_void,
        1 as gsize,
        cancellable,
        error,
    ) != 0
    {
        return c;
    }
    return 0 as guchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_input_stream_read_int16(
    mut stream: *mut GDataInputStream,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gint16 {
    let mut v: gint16 = 0;
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_data_input_stream_get_type();
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
            b"G_IS_DATA_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint16;
    }
    if safe_c2rust_read_data(
        stream,
        &raw mut v as *mut ::core::ffi::c_void,
        2 as gsize,
        cancellable,
        error,
    ) != 0
    {
        match (*(*stream).priv_0).byte_order as ::core::ffi::c_uint {
            0 => {
                v = ((v as guint16 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as guint16
                    as ::core::ffi::c_int
                    | ((v as guint16 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as guint16
                        as ::core::ffi::c_int) as guint16 as gint16;
            }
            1 => {
                v = v;
            }
            2 | _ => {}
        }
        return v;
    }
    return 0 as gint16;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_input_stream_read_uint16(
    mut stream: *mut GDataInputStream,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> guint16 {
    let mut v: guint16 = 0;
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_data_input_stream_get_type();
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
            b"G_IS_DATA_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint16;
    }
    if safe_c2rust_read_data(
        stream,
        &raw mut v as *mut ::core::ffi::c_void,
        2 as gsize,
        cancellable,
        error,
    ) != 0
    {
        match (*(*stream).priv_0).byte_order as ::core::ffi::c_uint {
            0 => {
                v = ((v as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as guint16
                    as ::core::ffi::c_int
                    | ((v as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as guint16
                        as ::core::ffi::c_int) as guint16;
            }
            1 => {
                v = v;
            }
            2 | _ => {}
        }
        return v;
    }
    return 0 as guint16;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_input_stream_read_int32(
    mut stream: *mut GDataInputStream,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gint32 {
    let mut v: gint32 = 0;
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_data_input_stream_get_type();
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
            b"G_IS_DATA_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint32;
    }
    if safe_c2rust_read_data(
        stream,
        &raw mut v as *mut ::core::ffi::c_void,
        4 as gsize,
        cancellable,
        error,
    ) != 0
    {
        match (*(*stream).priv_0).byte_order as ::core::ffi::c_uint {
            0 => {
                v = ({
                    let mut __v: guint32 = 0;
                    let mut __x: guint32 = v as guint32;
                    if 0 != 0 {
                        __v = (__x & 0xff as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int
                            | (__x & 0xff00 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int
                            | (__x & 0xff0000 as ::core::ffi::c_uint) >> 8 as ::core::ffi::c_int
                            | (__x & 0xff000000 as ::core::ffi::c_uint) >> 24 as ::core::ffi::c_int;
                    } else {
                        let fresh0 = &mut __v;
                        let fresh1;
                        let fresh2 = __x;
                        asm!(
                            "bswapl {0:e}\n", inlateout(reg)
                            c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2) => fresh1,
                            options(preserves_flags, pure, readonly, att_syntax)
                        );
                        c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
                    }
                    __v
                }) as gint32;
            }
            1 => {
                v = v;
            }
            2 | _ => {}
        }
        return v;
    }
    return 0 as gint32;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_input_stream_read_uint32(
    mut stream: *mut GDataInputStream,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> guint32 {
    let mut v: guint32 = 0;
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_data_input_stream_get_type();
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
            b"G_IS_DATA_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint32;
    }
    if safe_c2rust_read_data(
        stream,
        &raw mut v as *mut ::core::ffi::c_void,
        4 as gsize,
        cancellable,
        error,
    ) != 0
    {
        match (*(*stream).priv_0).byte_order as ::core::ffi::c_uint {
            0 => {
                v = ({
                    let mut __v: guint32 = 0;
                    let mut __x: guint32 = v;
                    if 0 != 0 {
                        __v = (__x & 0xff as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int
                            | (__x & 0xff00 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int
                            | (__x & 0xff0000 as ::core::ffi::c_uint) >> 8 as ::core::ffi::c_int
                            | (__x & 0xff000000 as ::core::ffi::c_uint) >> 24 as ::core::ffi::c_int;
                    } else {
                        let fresh3 = &mut __v;
                        let fresh4;
                        let fresh5 = __x;
                        asm!(
                            "bswapl {0:e}\n", inlateout(reg)
                            c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5) => fresh4,
                            options(preserves_flags, pure, readonly, att_syntax)
                        );
                        c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
                    }
                    __v
                });
            }
            1 => {
                v = v;
            }
            2 | _ => {}
        }
        return v;
    }
    return 0 as guint32;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_input_stream_read_int64(
    mut stream: *mut GDataInputStream,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gint64 {
    let mut v: gint64 = 0;
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_data_input_stream_get_type();
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
            b"G_IS_DATA_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint64;
    }
    if safe_c2rust_read_data(
        stream,
        &raw mut v as *mut ::core::ffi::c_void,
        8 as gsize,
        cancellable,
        error,
    ) != 0
    {
        match (*(*stream).priv_0).byte_order as ::core::ffi::c_uint {
            0 => {
                v = ({
                    let mut __v: guint64 = 0;
                    let mut __x: guint64 = v as guint64;
                    if 0 != 0 {
                        __v = (__x & 0xff as ::core::ffi::c_ulong) << 56 as ::core::ffi::c_int
                            | (__x & 0xff00 as ::core::ffi::c_ulong) << 40 as ::core::ffi::c_int
                            | (__x & 0xff0000 as ::core::ffi::c_ulong) << 24 as ::core::ffi::c_int
                            | (__x & 0xff000000 as ::core::ffi::c_ulong) << 8 as ::core::ffi::c_int
                            | (__x & 0xff00000000 as ::core::ffi::c_ulong)
                                >> 8 as ::core::ffi::c_int
                            | (__x & 0xff0000000000 as ::core::ffi::c_ulong)
                                >> 24 as ::core::ffi::c_int
                            | (__x & 0xff000000000000 as ::core::ffi::c_ulong)
                                >> 40 as ::core::ffi::c_int
                            | (__x & 0xff00000000000000 as ::core::ffi::c_ulong)
                                >> 56 as ::core::ffi::c_int;
                    } else {
                        let fresh6 = &mut __v;
                        let fresh7;
                        let fresh8 = __x;
                        asm!(
                            "bswapq {0}\n", inlateout(reg)
                            c2rust_asm_casts::AsmCast::cast_in(fresh6, fresh8) => fresh7,
                            options(preserves_flags, pure, readonly, att_syntax)
                        );
                        c2rust_asm_casts::AsmCast::cast_out(fresh6, fresh8, fresh7);
                    }
                    __v
                }) as gint64;
            }
            1 => {
                v = v;
            }
            2 | _ => {}
        }
        return v;
    }
    return 0 as gint64;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_input_stream_read_uint64(
    mut stream: *mut GDataInputStream,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> guint64 {
    let mut v: guint64 = 0;
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_data_input_stream_get_type();
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
            b"G_IS_DATA_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint64;
    }
    if safe_c2rust_read_data(
        stream,
        &raw mut v as *mut ::core::ffi::c_void,
        8 as gsize,
        cancellable,
        error,
    ) != 0
    {
        match (*(*stream).priv_0).byte_order as ::core::ffi::c_uint {
            0 => {
                v = ({
                    let mut __v: guint64 = 0;
                    let mut __x: guint64 = v;
                    if 0 != 0 {
                        __v = (__x & 0xff as ::core::ffi::c_ulong) << 56 as ::core::ffi::c_int
                            | (__x & 0xff00 as ::core::ffi::c_ulong) << 40 as ::core::ffi::c_int
                            | (__x & 0xff0000 as ::core::ffi::c_ulong) << 24 as ::core::ffi::c_int
                            | (__x & 0xff000000 as ::core::ffi::c_ulong) << 8 as ::core::ffi::c_int
                            | (__x & 0xff00000000 as ::core::ffi::c_ulong)
                                >> 8 as ::core::ffi::c_int
                            | (__x & 0xff0000000000 as ::core::ffi::c_ulong)
                                >> 24 as ::core::ffi::c_int
                            | (__x & 0xff000000000000 as ::core::ffi::c_ulong)
                                >> 40 as ::core::ffi::c_int
                            | (__x & 0xff00000000000000 as ::core::ffi::c_ulong)
                                >> 56 as ::core::ffi::c_int;
                    } else {
                        let fresh9 = &mut __v;
                        let fresh10;
                        let fresh11 = __x;
                        asm!(
                            "bswapq {0}\n", inlateout(reg)
                            c2rust_asm_casts::AsmCast::cast_in(fresh9, fresh11) =>
                            fresh10, options(preserves_flags, pure, readonly, att_syntax)
                        );
                        c2rust_asm_casts::AsmCast::cast_out(fresh9, fresh11, fresh10);
                    }
                    __v
                });
            }
            1 => {
                v = v;
            }
            2 | _ => {}
        }
        return v;
    }
    return 0 as guint64;
}
unsafe extern "C" fn safe_c2rust_scan_for_newline(
    mut stream: *mut GDataInputStream,
    mut checked_out: *mut gsize,
    mut last_saw_cr_out: *mut gboolean,
    mut newline_len_out: *mut ::core::ffi::c_int,
) -> gssize {
    let mut bstream: *mut GBufferedInputStream = ::core::ptr::null_mut::<GBufferedInputStream>();
    let mut priv_0: *mut GDataInputStreamPrivate =
        ::core::ptr::null_mut::<GDataInputStreamPrivate>();
    let mut buffer: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut start: gsize = 0;
    let mut end: gsize = 0;
    let mut peeked: gsize = 0;
    let mut i: gsize = 0;
    let mut found_pos: gssize = 0;
    let mut newline_len: ::core::ffi::c_int = 0;
    let mut available: gsize = 0;
    let mut checked: gsize = 0;
    let mut last_saw_cr: gboolean = 0;
    priv_0 = (*stream).priv_0;
    bstream = stream as *mut ::core::ffi::c_void as *mut GBufferedInputStream;
    checked = *checked_out;
    last_saw_cr = *last_saw_cr_out;
    found_pos = -(1 as ::core::ffi::c_int) as gssize;
    newline_len = 0 as ::core::ffi::c_int;
    start = checked;
    buffer = (g_buffered_input_stream_peek_buffer(bstream, &raw mut available)
        as *const ::core::ffi::c_char)
        .offset(start as isize);
    end = available;
    peeked = end.wrapping_sub(start);
    i = 0 as gsize;
    while checked < available && i < peeked {
        match (*priv_0).newline_type as ::core::ffi::c_uint {
            0 => {
                if *buffer.offset(i as isize) as ::core::ffi::c_int == 10 as ::core::ffi::c_int {
                    found_pos = start.wrapping_add(i) as gssize;
                    newline_len = 1 as ::core::ffi::c_int;
                }
            }
            1 => {
                if *buffer.offset(i as isize) as ::core::ffi::c_int == 13 as ::core::ffi::c_int {
                    found_pos = start.wrapping_add(i) as gssize;
                    newline_len = 1 as ::core::ffi::c_int;
                }
            }
            2 => {
                if last_saw_cr != 0
                    && *buffer.offset(i as isize) as ::core::ffi::c_int == 10 as ::core::ffi::c_int
                {
                    found_pos = start.wrapping_add(i).wrapping_sub(1 as gsize) as gssize;
                    newline_len = 2 as ::core::ffi::c_int;
                }
            }
            3 | _ => {
                if *buffer.offset(i as isize) as ::core::ffi::c_int == 10 as ::core::ffi::c_int {
                    if last_saw_cr != 0 {
                        found_pos = start.wrapping_add(i).wrapping_sub(1 as gsize) as gssize;
                        newline_len = 2 as ::core::ffi::c_int;
                    } else {
                        found_pos = start.wrapping_add(i) as gssize;
                        newline_len = 1 as ::core::ffi::c_int;
                    }
                } else if last_saw_cr != 0 {
                    found_pos = start.wrapping_add(i).wrapping_sub(1 as gsize) as gssize;
                    newline_len = 1 as ::core::ffi::c_int;
                }
            }
        }
        last_saw_cr = (*buffer.offset(i as isize) as ::core::ffi::c_int == 13 as ::core::ffi::c_int)
            as ::core::ffi::c_int as gboolean;
        if found_pos != -(1 as ::core::ffi::c_int) as gssize {
            *newline_len_out = newline_len;
            return found_pos;
        }
        i = i.wrapping_add(1);
    }
    checked = end;
    *checked_out = checked;
    *last_saw_cr_out = last_saw_cr;
    return -(1 as ::core::ffi::c_int) as gssize;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_input_stream_read_line(
    mut stream: *mut GDataInputStream,
    mut length: *mut gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut ::core::ffi::c_char {
    let mut bstream: *mut GBufferedInputStream = ::core::ptr::null_mut::<GBufferedInputStream>();
    let mut checked: gsize = 0;
    let mut last_saw_cr: gboolean = 0;
    let mut found_pos: gssize = 0;
    let mut res: gssize = 0;
    let mut newline_len: ::core::ffi::c_int = 0;
    let mut line: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_data_input_stream_get_type();
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
            _g_boolean_var_23 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_23 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_23
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DATA_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    bstream = stream as *mut ::core::ffi::c_void as *mut GBufferedInputStream;
    newline_len = 0 as ::core::ffi::c_int;
    checked = 0 as gsize;
    last_saw_cr = FALSE as gboolean;
    loop {
        found_pos = safe_c2rust_scan_for_newline(
            stream,
            &raw mut checked,
            &raw mut last_saw_cr,
            &raw mut newline_len,
        );
        if !(found_pos == -(1 as ::core::ffi::c_int) as gssize) {
            break;
        }
        if g_buffered_input_stream_get_available(bstream)
            == g_buffered_input_stream_get_buffer_size(bstream)
        {
            g_buffered_input_stream_set_buffer_size(
                bstream,
                (2 as gsize).wrapping_mul(g_buffered_input_stream_get_buffer_size(bstream)),
            );
        }
        res = g_buffered_input_stream_fill(
            bstream,
            -(1 as ::core::ffi::c_int) as gssize,
            cancellable,
            error,
        );
        if res < 0 as gssize {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if !(res == 0 as gssize) {
            continue;
        }
        if g_buffered_input_stream_get_available(bstream) == 0 as gsize {
            if !length.is_null() {
                *length = 0 as gsize;
            }
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        } else {
            found_pos = checked as gssize;
            newline_len = 0 as ::core::ffi::c_int;
            break;
        }
    }
    line = g_malloc((found_pos + newline_len as gssize + 1 as gssize) as gsize)
        as *mut ::core::ffi::c_char;
    res = g_input_stream_read(
        stream as *mut ::core::ffi::c_void as *mut GInputStream,
        line as *mut ::core::ffi::c_void,
        (found_pos + newline_len as gssize) as gsize,
        ::core::ptr::null_mut::<GCancellable>(),
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if !length.is_null() {
        *length = found_pos as gsize;
    }
    if !(({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if res == found_pos + newline_len as gssize {
            _g_boolean_var_24 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_24 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_24
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdatainputstream.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            797 as ::core::ffi::c_int,
            G_STRFUNC,
            b"res == found_pos + newline_len\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    *line.offset(found_pos as isize) = 0 as ::core::ffi::c_char;
    return line;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_input_stream_read_line_utf8(
    mut stream: *mut GDataInputStream,
    mut length: *mut gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut ::core::ffi::c_char {
    let mut res: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    res = safe_c2rust_g_data_input_stream_read_line(stream, length, cancellable, error);
    if res.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if g_utf8_validate(
        res,
        -(1 as ::core::ffi::c_int) as gssize,
        ::core::ptr::null_mut::<*const gchar>(),
    ) == 0
    {
        g_set_error_literal(
            error,
            g_convert_error_quark(),
            G_CONVERT_ERROR_ILLEGAL_SEQUENCE as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Invalid byte sequence in conversion input\0" as *const u8 as *const gchar,
            ),
        );
        g_free(res as gpointer);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return res;
}
unsafe extern "C" fn safe_c2rust_scan_for_chars(
    mut stream: *mut GDataInputStream,
    mut checked_out: *mut gsize,
    mut stop_chars: *const ::core::ffi::c_char,
    mut stop_chars_len: gsize,
) -> gssize {
    let mut bstream: *mut GBufferedInputStream = ::core::ptr::null_mut::<GBufferedInputStream>();
    let mut buffer: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut start: gsize = 0;
    let mut end: gsize = 0;
    let mut peeked: gsize = 0;
    let mut i: gsize = 0;
    let mut available: gsize = 0;
    let mut checked: gsize = 0;
    let mut stop_char: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut stop_end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    bstream = stream as *mut ::core::ffi::c_void as *mut GBufferedInputStream;
    stop_end = stop_chars.offset(stop_chars_len as isize);
    checked = *checked_out;
    start = checked;
    buffer = (g_buffered_input_stream_peek_buffer(bstream, &raw mut available)
        as *const ::core::ffi::c_char)
        .offset(start as isize);
    end = available;
    peeked = end.wrapping_sub(start);
    i = 0 as gsize;
    while checked < available && i < peeked {
        stop_char = stop_chars;
        while stop_char != stop_end {
            if *buffer.offset(i as isize) as ::core::ffi::c_int == *stop_char as ::core::ffi::c_int
            {
                return start.wrapping_add(i) as gssize;
            }
            stop_char = stop_char.offset(1);
        }
        i = i.wrapping_add(1);
    }
    checked = end;
    *checked_out = checked;
    return -(1 as ::core::ffi::c_int) as gssize;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_input_stream_read_until(
    mut stream: *mut GDataInputStream,
    mut stop_chars: *const gchar,
    mut length: *mut gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut ::core::ffi::c_char {
    let mut bstream: *mut GBufferedInputStream = ::core::ptr::null_mut::<GBufferedInputStream>();
    let mut result: *mut gchar = ::core::ptr::null_mut::<gchar>();
    bstream = stream as *mut ::core::ffi::c_void as *mut GBufferedInputStream;
    result = safe_c2rust_g_data_input_stream_read_upto(
        stream,
        stop_chars,
        -(1 as ::core::ffi::c_int) as gssize,
        length,
        cancellable,
        error,
    ) as *mut gchar;
    if !result.is_null() && g_buffered_input_stream_get_available(bstream) > 0 as gsize {
        let mut res: gsize = 0;
        let mut b: gchar = 0;
        res = g_input_stream_read(
            stream as *mut ::core::ffi::c_void as *mut GInputStream,
            &raw mut b as *mut ::core::ffi::c_void,
            1 as gsize,
            ::core::ptr::null_mut::<GCancellable>(),
            ::core::ptr::null_mut::<*mut GError>(),
        ) as gsize;
        if ({
            let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
            if res == 1 as gsize {
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdatainputstream.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                937 as ::core::ffi::c_int,
                G_STRFUNC,
                b"res == 1\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
    return result as *mut ::core::ffi::c_char;
}
unsafe extern "C" fn safe_c2rust_g_data_input_stream_read_complete(
    mut task: *mut GTask,
    mut read_length: gsize,
    mut skip_length: gsize,
) {
    let mut data: *mut GDataInputStreamReadData =
        g_task_get_task_data(task) as *mut GDataInputStreamReadData;
    let mut stream: *mut GInputStream = g_task_get_source_object(task) as *mut GInputStream;
    let mut line: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if read_length != 0 || skip_length != 0 {
        let mut bytes: gssize = 0;
        (*data).length = read_length;
        line = g_malloc(read_length.wrapping_add(1 as gsize)) as *mut ::core::ffi::c_char;
        *line.offset(read_length as isize) = '\0' as i32 as ::core::ffi::c_char;
        bytes = g_input_stream_read(
            stream,
            line as *mut ::core::ffi::c_void,
            read_length,
            ::core::ptr::null_mut::<GCancellable>(),
            ::core::ptr::null_mut::<*mut GError>(),
        );
        let mut __n1: gint64 = bytes as gint64;
        let mut __n2: gint64 = read_length as gint64;
        if !(__n1 == __n2) {
            g_assertion_message_cmpint(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdatainputstream.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                972 as ::core::ffi::c_int,
                G_STRFUNC,
                b"bytes == read_length\0" as *const u8 as *const ::core::ffi::c_char,
                __n1 as guint64,
                b"==\0" as *const u8 as *const ::core::ffi::c_char,
                __n2 as guint64,
                'i' as i32 as ::core::ffi::c_char,
            );
        }
        bytes = g_input_stream_skip(
            stream,
            skip_length,
            ::core::ptr::null_mut::<GCancellable>(),
            ::core::ptr::null_mut::<*mut GError>(),
        );
        let mut __n1_0: gint64 = bytes as gint64;
        let mut __n2_0: gint64 = skip_length as gint64;
        if !(__n1_0 == __n2_0) {
            g_assertion_message_cmpint(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdatainputstream.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                975 as ::core::ffi::c_int,
                G_STRFUNC,
                b"bytes == skip_length\0" as *const u8 as *const ::core::ffi::c_char,
                __n1_0 as guint64,
                b"==\0" as *const u8 as *const ::core::ffi::c_char,
                __n2_0 as guint64,
                'i' as i32 as ::core::ffi::c_char,
            );
        }
    }
    g_task_return_pointer(
        task,
        line as gpointer,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_data_input_stream_read_line_ready(
    mut object: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut data: *mut GDataInputStreamReadData =
        g_task_get_task_data(task) as *mut GDataInputStreamReadData;
    let mut buffer: *mut GBufferedInputStream =
        g_task_get_source_object(task) as *mut GBufferedInputStream;
    let mut found_pos: gssize = 0;
    let mut newline_len: gint = 0;
    if !result.is_null() {
        let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
        let mut bytes: gssize = 0;
        bytes = g_buffered_input_stream_fill_finish(buffer, result, &raw mut error);
        if bytes <= 0 as gssize {
            if bytes < 0 as gssize {
                g_task_return_error(task, error);
                g_object_unref(task as gpointer);
                return;
            }
            safe_c2rust_g_data_input_stream_read_complete(task, (*data).checked, 0 as gsize);
            return;
        }
    }
    if !(*data).stop_chars.is_null() {
        found_pos = safe_c2rust_scan_for_chars(
            buffer as *mut ::core::ffi::c_void as *mut GDataInputStream,
            &raw mut (*data).checked,
            (*data).stop_chars,
            (*data).stop_chars_len,
        );
        newline_len = 0 as ::core::ffi::c_int as gint;
    } else {
        found_pos = safe_c2rust_scan_for_newline(
            buffer as *mut ::core::ffi::c_void as *mut GDataInputStream,
            &raw mut (*data).checked,
            &raw mut (*data).last_saw_cr,
            &raw mut newline_len,
        );
    }
    if found_pos == -(1 as ::core::ffi::c_int) as gssize {
        let mut size: gsize = 0;
        size = g_buffered_input_stream_get_buffer_size(buffer);
        if g_buffered_input_stream_get_available(buffer) == size {
            g_buffered_input_stream_set_buffer_size(buffer, size.wrapping_mul(2 as gsize));
        }
        g_buffered_input_stream_fill_async(
            buffer,
            -(1 as ::core::ffi::c_int) as gssize,
            g_task_get_priority(task) as ::core::ffi::c_int,
            g_task_get_cancellable(task),
            Some(
                safe_c2rust_g_data_input_stream_read_line_ready
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            user_data,
        );
    } else {
        safe_c2rust_g_data_input_stream_read_complete(
            task,
            found_pos as gsize,
            newline_len as gsize,
        );
    };
}
unsafe extern "C" fn safe_c2rust_g_data_input_stream_read_data_free(mut user_data: gpointer) {
    let mut data: *mut GDataInputStreamReadData = user_data as *mut GDataInputStreamReadData;
    g_free((*data).stop_chars as gpointer);
    g_slice_free1(
        ::core::mem::size_of::<GDataInputStreamReadData>() as gsize,
        data as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_g_data_input_stream_read_async(
    mut stream: *mut GDataInputStream,
    mut stop_chars: *const gchar,
    mut stop_chars_len: gssize,
    mut io_priority: gint,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut data: *mut GDataInputStreamReadData =
        ::core::ptr::null_mut::<GDataInputStreamReadData>();
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut stop_chars_len_unsigned: gsize = 0;
    data = ({
        let mut __s: gsize = ::core::mem::size_of::<GDataInputStreamReadData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut GDataInputStreamReadData;
    if stop_chars_len < 0 as gssize {
        stop_chars_len_unsigned = strlen(stop_chars as *const ::core::ffi::c_char) as gsize;
    } else {
        stop_chars_len_unsigned = stop_chars_len as gsize;
    }
    (*data).stop_chars =
        g_memdup2(stop_chars as gconstpointer, stop_chars_len_unsigned) as *mut gchar;
    (*data).stop_chars_len = stop_chars_len_unsigned;
    (*data).last_saw_cr = FALSE as gboolean;
    task = g_task_new(stream as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GDataInputStream,
                    *const gchar,
                    gssize,
                    gint,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_data_input_stream_read_async
                as unsafe extern "C" fn(
                    *mut GDataInputStream,
                    *const gchar,
                    gssize,
                    gint,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"g_data_input_stream_read_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        data as gpointer,
        Some(
            safe_c2rust_g_data_input_stream_read_data_free as unsafe extern "C" fn(gpointer) -> (),
        ),
    );
    g_task_set_priority(task, io_priority);
    safe_c2rust_g_data_input_stream_read_line_ready(
        ::core::ptr::null_mut::<GObject>(),
        ::core::ptr::null_mut::<GAsyncResult>(),
        task as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_g_data_input_stream_read_finish(
    mut stream: *mut GDataInputStream,
    mut result: *mut GAsyncResult,
    mut length: *mut gsize,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut task: *mut GTask = result as *mut ::core::ffi::c_void as *mut GTask;
    let mut line: *mut gchar = ::core::ptr::null_mut::<gchar>();
    line = g_task_propagate_pointer(task, error) as *mut gchar;
    if !length.is_null() && !line.is_null() {
        let mut data: *mut GDataInputStreamReadData =
            g_task_get_task_data(task) as *mut GDataInputStreamReadData;
        *length = (*data).length;
    }
    return line;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_input_stream_read_line_async(
    mut stream: *mut GDataInputStream,
    mut io_priority: gint,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_data_input_stream_get_type();
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
            _g_boolean_var_26 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_26 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_26
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DATA_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if cancellable.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = cancellable as *mut GTypeInstance;
                let mut __t: GType = g_cancellable_get_type();
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
            _g_boolean_var_27 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_27 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_27
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_data_input_stream_read_async(
        stream,
        ::core::ptr::null::<gchar>(),
        0 as gssize,
        io_priority,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_input_stream_read_until_async(
    mut stream: *mut GDataInputStream,
    mut stop_chars: *const gchar,
    mut io_priority: gint,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_data_input_stream_get_type();
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
            _g_boolean_var_28 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_28 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_28
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DATA_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if cancellable.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = cancellable as *mut GTypeInstance;
                let mut __t: GType = g_cancellable_get_type();
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
            _g_boolean_var_29 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_29 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_29
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if !stop_chars.is_null() {
            _g_boolean_var_30 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_30 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_30
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"stop_chars != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_data_input_stream_read_async(
        stream,
        stop_chars,
        -(1 as ::core::ffi::c_int) as gssize,
        io_priority,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_input_stream_read_line_finish(
    mut stream: *mut GDataInputStream,
    mut result: *mut GAsyncResult,
    mut length: *mut gsize,
    mut error: *mut *mut GError,
) -> *mut ::core::ffi::c_char {
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, stream as gpointer) != 0 {
            _g_boolean_var_31 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_31 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_31
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (result, stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return safe_c2rust_g_data_input_stream_read_finish(stream, result, length, error)
        as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_input_stream_read_line_finish_utf8(
    mut stream: *mut GDataInputStream,
    mut result: *mut GAsyncResult,
    mut length: *mut gsize,
    mut error: *mut *mut GError,
) -> *mut ::core::ffi::c_char {
    let mut res: *mut gchar = ::core::ptr::null_mut::<gchar>();
    res = safe_c2rust_g_data_input_stream_read_line_finish(stream, result, length, error)
        as *mut gchar;
    if res.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if g_utf8_validate(
        res,
        -(1 as ::core::ffi::c_int) as gssize,
        ::core::ptr::null_mut::<*const gchar>(),
    ) == 0
    {
        g_set_error_literal(
            error,
            g_convert_error_quark(),
            G_CONVERT_ERROR_ILLEGAL_SEQUENCE as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Invalid byte sequence in conversion input\0" as *const u8 as *const gchar,
            ),
        );
        g_free(res as gpointer);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return res as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_input_stream_read_until_finish(
    mut stream: *mut GDataInputStream,
    mut result: *mut GAsyncResult,
    mut length: *mut gsize,
    mut error: *mut *mut GError,
) -> *mut ::core::ffi::c_char {
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, stream as gpointer) != 0 {
            _g_boolean_var_32 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_32 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_32
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (result, stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return safe_c2rust_g_data_input_stream_read_finish(stream, result, length, error)
        as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_input_stream_read_upto(
    mut stream: *mut GDataInputStream,
    mut stop_chars: *const gchar,
    mut stop_chars_len: gssize,
    mut length: *mut gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut ::core::ffi::c_char {
    let mut bstream: *mut GBufferedInputStream = ::core::ptr::null_mut::<GBufferedInputStream>();
    let mut checked: gsize = 0;
    let mut found_pos: gssize = 0;
    let mut res: gssize = 0;
    let mut data_until: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut stop_chars_len_unsigned: gsize = 0;
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_data_input_stream_get_type();
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
            _g_boolean_var_33 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_33 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_33
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DATA_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if stop_chars_len < 0 as gssize {
        stop_chars_len_unsigned = strlen(stop_chars as *const ::core::ffi::c_char) as gsize;
    } else {
        stop_chars_len_unsigned = stop_chars_len as gsize;
    }
    bstream = stream as *mut ::core::ffi::c_void as *mut GBufferedInputStream;
    checked = 0 as gsize;
    loop {
        found_pos = safe_c2rust_scan_for_chars(
            stream,
            &raw mut checked,
            stop_chars as *const ::core::ffi::c_char,
            stop_chars_len_unsigned,
        );
        if !(found_pos == -(1 as ::core::ffi::c_int) as gssize) {
            break;
        }
        if g_buffered_input_stream_get_available(bstream)
            == g_buffered_input_stream_get_buffer_size(bstream)
        {
            g_buffered_input_stream_set_buffer_size(
                bstream,
                (2 as gsize).wrapping_mul(g_buffered_input_stream_get_buffer_size(bstream)),
            );
        }
        res = g_buffered_input_stream_fill(
            bstream,
            -(1 as ::core::ffi::c_int) as gssize,
            cancellable,
            error,
        );
        if res < 0 as gssize {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if !(res == 0 as gssize) {
            continue;
        }
        if g_buffered_input_stream_get_available(bstream) == 0 as gsize {
            if !length.is_null() {
                *length = 0 as gsize;
            }
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        } else {
            found_pos = checked as gssize;
            break;
        }
    }
    data_until = g_malloc((found_pos + 1 as gssize) as gsize) as *mut ::core::ffi::c_char;
    res = g_input_stream_read(
        stream as *mut ::core::ffi::c_void as *mut GInputStream,
        data_until as *mut ::core::ffi::c_void,
        found_pos as gsize,
        ::core::ptr::null_mut::<GCancellable>(),
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if !length.is_null() {
        *length = found_pos as gsize;
    }
    if !(({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if res == found_pos {
            _g_boolean_var_34 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_34 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_34
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdatainputstream.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1388 as ::core::ffi::c_int,
            G_STRFUNC,
            b"res == found_pos\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    *data_until.offset(found_pos as isize) = 0 as ::core::ffi::c_char;
    return data_until;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_input_stream_read_upto_async(
    mut stream: *mut GDataInputStream,
    mut stop_chars: *const gchar,
    mut stop_chars_len: gssize,
    mut io_priority: gint,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_data_input_stream_get_type();
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
            _g_boolean_var_35 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_35 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_35
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DATA_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if cancellable.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = cancellable as *mut GTypeInstance;
                let mut __t: GType = g_cancellable_get_type();
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
            _g_boolean_var_36 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_36 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_36
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if !stop_chars.is_null() {
            _g_boolean_var_37 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_37 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_37
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"stop_chars != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_data_input_stream_read_async(
        stream,
        stop_chars,
        stop_chars_len,
        io_priority,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_input_stream_read_upto_finish(
    mut stream: *mut GDataInputStream,
    mut result: *mut GAsyncResult,
    mut length: *mut gsize,
    mut error: *mut *mut GError,
) -> *mut ::core::ffi::c_char {
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, stream as gpointer) != 0 {
            _g_boolean_var_38 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_38 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_38
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (result, stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return safe_c2rust_g_data_input_stream_read_finish(stream, result, length, error)
        as *mut ::core::ffi::c_char;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
