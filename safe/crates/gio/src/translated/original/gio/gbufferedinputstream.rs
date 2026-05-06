extern "C" {
    pub type _GData;
    pub type _GAsyncResult;
    pub type _GInputStreamPrivate;
    pub type _GCancellablePrivate;
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
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
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
    fn g_value_set_uint(value: *mut GValue, v_uint: guint);
    fn g_value_get_uint(value: *const GValue) -> guint;
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
    fn g_input_stream_read_async(
        stream: *mut GInputStream,
        buffer: *mut ::core::ffi::c_void,
        count: gsize,
        io_priority: ::core::ffi::c_int,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_input_stream_read_finish(
        stream: *mut GInputStream,
        result: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gssize;
    fn g_input_stream_skip_async(
        stream: *mut GInputStream,
        count: gsize,
        io_priority: ::core::ffi::c_int,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_input_stream_skip_finish(
        stream: *mut GInputStream,
        result: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gssize;
    fn g_input_stream_is_closed(stream: *mut GInputStream) -> gboolean;
    fn g_input_stream_set_pending(stream: *mut GInputStream, error: *mut *mut GError) -> gboolean;
    fn g_input_stream_clear_pending(stream: *mut GInputStream);
    fn g_filter_input_stream_get_type() -> GType;
    fn g_cancellable_push_current(cancellable: *mut GCancellable);
    fn g_cancellable_pop_current(cancellable: *mut GCancellable);
    fn g_async_result_get_type() -> GType;
    fn g_async_result_legacy_propagate_error(
        res: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_async_result_is_tagged(res: *mut GAsyncResult, source_tag: gpointer) -> gboolean;
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
    fn g_task_report_new_error(
        source_object: gpointer,
        callback: GAsyncReadyCallback,
        callback_data: gpointer,
        source_tag: gpointer,
        domain: GQuark,
        code: gint,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn g_task_set_task_data(
        task: *mut GTask,
        task_data: gpointer,
        task_data_destroy: GDestroyNotify,
    );
    fn g_task_set_source_tag(task: *mut GTask, source_tag: gpointer);
    fn g_task_set_static_name(task: *mut GTask, name: *const gchar);
    fn g_task_get_source_object(task: *mut GTask) -> gpointer;
    fn g_task_get_task_data(task: *mut GTask) -> gpointer;
    fn g_task_get_name(task: *mut GTask) -> *const gchar;
    fn g_task_is_valid(result: gpointer, source_object: gpointer) -> gboolean;
    fn g_task_return_int(task: *mut GTask, result: gssize);
    fn g_task_return_error(task: *mut GTask, error: *mut GError);
    fn g_task_propagate_int(task: *mut GTask, error: *mut *mut GError) -> gssize;
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
pub type GAsyncResult = _GAsyncResult;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GBufferedInputStream {
    pub parent_instance: GFilterInputStream,
    pub priv_0: *mut GBufferedInputStreamPrivate,
}
pub type GBufferedInputStreamPrivate = _GBufferedInputStreamPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GBufferedInputStreamPrivate {
    pub buffer: *mut guint8,
    pub len: gsize,
    pub pos: gsize,
    pub end: gsize,
    pub outstanding_callback: GAsyncReadyCallback,
}
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
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
pub type GSeekable = _GSeekable;
pub type GTask = _GTask;
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
pub const PROP_BUFSIZE: C2RustUnnamed_1 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SkipAsyncData {
    pub bytes_skipped: gsize,
    pub count: gsize,
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
pub const DEFAULT_BUFFER_SIZE: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
static mut safe_c2rust_g_buffered_input_stream_parent_class: gpointer = NULL;
unsafe extern "C" fn safe_c2rust_g_buffered_input_stream_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_buffered_input_stream_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GBufferedInputStream_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GBufferedInputStream_private_offset,
        );
    }
    safe_c2rust_g_buffered_input_stream_class_init(klass as *mut GBufferedInputStreamClass);
}
static mut safe_c2rust_GBufferedInputStream_private_offset: gint = 0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_buffered_input_stream_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_buffered_input_stream_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_buffered_input_stream_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_filter_input_stream_get_type(),
        g_intern_static_string(b"GBufferedInputStream\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GBufferedInputStreamClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_buffered_input_stream_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GBufferedInputStream>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GBufferedInputStream) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_buffered_input_stream_init
                    as unsafe extern "C" fn(*mut GBufferedInputStream) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GBufferedInputStream_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GBufferedInputStreamPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GSeekableIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_buffered_input_stream_seekable_iface_init
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
#[inline]
unsafe extern "C" fn safe_c2rust_g_buffered_input_stream_get_instance_private(
    mut self_0: *mut GBufferedInputStream,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GBufferedInputStream_private_offset as glong as isize)
        as gpointer;
}
unsafe extern "C" fn safe_c2rust_g_buffered_input_stream_class_init(
    mut klass: *mut GBufferedInputStreamClass,
) {
    let mut object_class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    let mut istream_class: *mut GInputStreamClass = ::core::ptr::null_mut::<GInputStreamClass>();
    let mut bstream_class: *mut GBufferedInputStreamClass =
        ::core::ptr::null_mut::<GBufferedInputStreamClass>();
    object_class = klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*object_class).get_property = Some(
        safe_c2rust_g_buffered_input_stream_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*object_class).set_property = Some(
        safe_c2rust_g_buffered_input_stream_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*object_class).finalize = Some(
        safe_c2rust_g_buffered_input_stream_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    istream_class = klass as *mut ::core::ffi::c_void as *mut GInputStreamClass;
    (*istream_class).skip = Some(
        safe_c2rust_g_buffered_input_stream_skip
            as unsafe extern "C" fn(
                *mut GInputStream,
                gsize,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gssize,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GInputStream,
                gsize,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gssize,
        >;
    (*istream_class).skip_async = Some(
        safe_c2rust_g_buffered_input_stream_skip_async
            as unsafe extern "C" fn(
                *mut GInputStream,
                gsize,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GInputStream,
                gsize,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*istream_class).skip_finish = Some(
        safe_c2rust_g_buffered_input_stream_skip_finish
            as unsafe extern "C" fn(
                *mut GInputStream,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> gssize,
    )
        as Option<
            unsafe extern "C" fn(*mut GInputStream, *mut GAsyncResult, *mut *mut GError) -> gssize,
        >;
    (*istream_class).read_fn = Some(
        safe_c2rust_g_buffered_input_stream_read
            as unsafe extern "C" fn(
                *mut GInputStream,
                *mut ::core::ffi::c_void,
                gsize,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gssize,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GInputStream,
                *mut ::core::ffi::c_void,
                gsize,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gssize,
        >;
    bstream_class = klass as *mut ::core::ffi::c_void as *mut GBufferedInputStreamClass;
    (*bstream_class).fill = Some(
        safe_c2rust_g_buffered_input_stream_real_fill
            as unsafe extern "C" fn(
                *mut GBufferedInputStream,
                gssize,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gssize,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GBufferedInputStream,
                gssize,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gssize,
        >;
    (*bstream_class).fill_async = Some(
        safe_c2rust_g_buffered_input_stream_real_fill_async
            as unsafe extern "C" fn(
                *mut GBufferedInputStream,
                gssize,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GBufferedInputStream,
                gssize,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*bstream_class).fill_finish = Some(
        safe_c2rust_g_buffered_input_stream_real_fill_finish
            as unsafe extern "C" fn(
                *mut GBufferedInputStream,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> gssize,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GBufferedInputStream,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> gssize,
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
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_buffered_input_stream_get_buffer_size(
    mut stream: *mut GBufferedInputStream,
) -> gsize {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_buffered_input_stream_get_type();
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
            b"G_IS_BUFFERED_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gsize;
    }
    return (*(*stream).priv_0).len;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_buffered_input_stream_set_buffer_size(
    mut stream: *mut GBufferedInputStream,
    mut size: gsize,
) {
    let mut priv_0: *mut GBufferedInputStreamPrivate =
        ::core::ptr::null_mut::<GBufferedInputStreamPrivate>();
    let mut in_buffer: gsize = 0;
    let mut buffer: *mut guint8 = ::core::ptr::null_mut::<guint8>();
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_buffered_input_stream_get_type();
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
            b"G_IS_BUFFERED_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    priv_0 = (*stream).priv_0;
    if (*priv_0).len == size {
        return;
    }
    if !(*priv_0).buffer.is_null() {
        in_buffer = (*priv_0).end.wrapping_sub((*priv_0).pos);
        size = if size > in_buffer { size } else { in_buffer };
        buffer = g_malloc(size) as *mut guint8;
        memcpy(
            buffer as *mut ::core::ffi::c_void,
            (*priv_0).buffer.offset((*priv_0).pos as isize) as *const ::core::ffi::c_void,
            in_buffer as size_t,
        );
        (*priv_0).len = size;
        (*priv_0).pos = 0 as gsize;
        (*priv_0).end = in_buffer;
        g_free((*priv_0).buffer as gpointer);
        (*priv_0).buffer = buffer;
    } else {
        (*priv_0).len = size;
        (*priv_0).pos = 0 as gsize;
        (*priv_0).end = 0 as gsize;
        (*priv_0).buffer = g_malloc(size) as *mut guint8;
    }
    g_object_notify(
        stream as *mut ::core::ffi::c_void as *mut GObject,
        b"buffer-size\0" as *const u8 as *const gchar,
    );
}
unsafe extern "C" fn safe_c2rust_g_buffered_input_stream_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut bstream: *mut GBufferedInputStream = ::core::ptr::null_mut::<GBufferedInputStream>();
    bstream = object as *mut ::core::ffi::c_void as *mut GBufferedInputStream;
    match prop_id {
        1 => {
            safe_c2rust_g_buffered_input_stream_set_buffer_size(
                bstream,
                g_value_get_uint(value) as gsize,
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gbufferedinputstream.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                260 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_buffered_input_stream_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut priv_0: *mut GBufferedInputStreamPrivate =
        ::core::ptr::null_mut::<GBufferedInputStreamPrivate>();
    let mut bstream: *mut GBufferedInputStream = ::core::ptr::null_mut::<GBufferedInputStream>();
    bstream = object as *mut ::core::ffi::c_void as *mut GBufferedInputStream;
    priv_0 = (*bstream).priv_0;
    match prop_id {
        1 => {
            g_value_set_uint(value, (*priv_0).len as guint);
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gbufferedinputstream.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                284 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_buffered_input_stream_finalize(mut object: *mut GObject) {
    let mut priv_0: *mut GBufferedInputStreamPrivate =
        ::core::ptr::null_mut::<GBufferedInputStreamPrivate>();
    let mut stream: *mut GBufferedInputStream = ::core::ptr::null_mut::<GBufferedInputStream>();
    stream = object as *mut ::core::ffi::c_void as *mut GBufferedInputStream;
    priv_0 = (*stream).priv_0;
    g_free((*priv_0).buffer as gpointer);
    (*(safe_c2rust_g_buffered_input_stream_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_buffered_input_stream_seekable_iface_init(
    mut iface: *mut GSeekableIface,
) {
    (*iface).tell = Some(
        safe_c2rust_g_buffered_input_stream_tell as unsafe extern "C" fn(*mut GSeekable) -> goffset,
    ) as Option<unsafe extern "C" fn(*mut GSeekable) -> goffset>;
    (*iface).can_seek = Some(
        safe_c2rust_g_buffered_input_stream_can_seek
            as unsafe extern "C" fn(*mut GSeekable) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut GSeekable) -> gboolean>;
    (*iface).seek = Some(
        safe_c2rust_g_buffered_input_stream_seek
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
        safe_c2rust_g_buffered_input_stream_can_truncate
            as unsafe extern "C" fn(*mut GSeekable) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut GSeekable) -> gboolean>;
    (*iface).truncate_fn = Some(
        safe_c2rust_g_buffered_input_stream_truncate
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
unsafe extern "C" fn safe_c2rust_g_buffered_input_stream_init(
    mut stream: *mut GBufferedInputStream,
) {
    (*stream).priv_0 = safe_c2rust_g_buffered_input_stream_get_instance_private(stream)
        as *mut GBufferedInputStreamPrivate;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_buffered_input_stream_new(
    mut base_stream: *mut GInputStream,
) -> *mut GInputStream {
    let mut stream: *mut GInputStream = ::core::ptr::null_mut::<GInputStream>();
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
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
            b"G_IS_INPUT_STREAM (base_stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GInputStream>();
    }
    stream = g_object_new(
        safe_c2rust_g_buffered_input_stream_get_type(),
        b"base-stream\0" as *const u8 as *const gchar,
        base_stream,
        NULL,
    ) as *mut GInputStream;
    return stream;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_buffered_input_stream_new_sized(
    mut base_stream: *mut GInputStream,
    mut size: gsize,
) -> *mut GInputStream {
    let mut stream: *mut GInputStream = ::core::ptr::null_mut::<GInputStream>();
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
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
            b"G_IS_INPUT_STREAM (base_stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GInputStream>();
    }
    stream = g_object_new(
        safe_c2rust_g_buffered_input_stream_get_type(),
        b"base-stream\0" as *const u8 as *const gchar,
        base_stream,
        b"buffer-size\0" as *const u8 as *const ::core::ffi::c_char,
        size as guint,
        NULL,
    ) as *mut GInputStream;
    return stream;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_buffered_input_stream_fill(
    mut stream: *mut GBufferedInputStream,
    mut count: gssize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    let mut class: *mut GBufferedInputStreamClass =
        ::core::ptr::null_mut::<GBufferedInputStreamClass>();
    let mut input_stream: *mut GInputStream = ::core::ptr::null_mut::<GInputStream>();
    let mut res: gssize = 0;
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_buffered_input_stream_get_type();
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
            b"G_IS_BUFFERED_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    input_stream = stream as *mut ::core::ffi::c_void as *mut GInputStream;
    if count < -(1 as ::core::ffi::c_int) as gssize {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(b"Too large count value passed to %s\0" as *const u8 as *const gchar),
            G_STRFUNC,
        );
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    if g_input_stream_set_pending(input_stream, error) == 0 {
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    if !cancellable.is_null() {
        g_cancellable_push_current(cancellable);
    }
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GBufferedInputStreamClass;
    res = (*class).fill.expect("non-null function pointer")(stream, count, cancellable, error);
    if !cancellable.is_null() {
        g_cancellable_pop_current(cancellable);
    }
    g_input_stream_clear_pending(input_stream);
    return res;
}
unsafe extern "C" fn safe_c2rust_async_fill_callback_wrapper(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut stream: *mut GBufferedInputStream =
        source_object as *mut ::core::ffi::c_void as *mut GBufferedInputStream;
    g_input_stream_clear_pending(stream as *mut ::core::ffi::c_void as *mut GInputStream);
    Some(
        (*(*stream).priv_0)
            .outstanding_callback
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(source_object, res, user_data);
    g_object_unref(stream as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_buffered_input_stream_fill_async(
    mut stream: *mut GBufferedInputStream,
    mut count: gssize,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut class: *mut GBufferedInputStreamClass =
        ::core::ptr::null_mut::<GBufferedInputStreamClass>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_buffered_input_stream_get_type();
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
            b"G_IS_BUFFERED_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if count == 0 as gssize {
        let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
        task = g_task_new(stream as gpointer, cancellable, callback, user_data);
        let mut _task: *mut GTask = task;
        g_task_set_source_tag(
            _task,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GBufferedInputStream,
                        gssize,
                        ::core::ffi::c_int,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_buffered_input_stream_fill_async
                    as unsafe extern "C" fn(
                        *mut GBufferedInputStream,
                        gssize,
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
                b"g_buffered_input_stream_fill_async\0" as *const u8 as *const gchar,
            );
        }
        g_task_return_int(task, 0 as gssize);
        g_object_unref(task as gpointer);
        return;
    }
    if count < -(1 as ::core::ffi::c_int) as gssize {
        g_task_report_new_error(
            stream as gpointer,
            callback,
            user_data,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GBufferedInputStream,
                        gssize,
                        ::core::ffi::c_int,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_buffered_input_stream_fill_async
                    as unsafe extern "C" fn(
                        *mut GBufferedInputStream,
                        gssize,
                        ::core::ffi::c_int,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )),
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(b"Too large count value passed to %s\0" as *const u8 as *const gchar)
                as *const ::core::ffi::c_char,
            G_STRFUNC,
        );
        return;
    }
    if g_input_stream_set_pending(
        stream as *mut ::core::ffi::c_void as *mut GInputStream,
        &raw mut error,
    ) == 0
    {
        g_task_report_error(
            stream as gpointer,
            callback,
            user_data,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GBufferedInputStream,
                        gssize,
                        ::core::ffi::c_int,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_buffered_input_stream_fill_async
                    as unsafe extern "C" fn(
                        *mut GBufferedInputStream,
                        gssize,
                        ::core::ffi::c_int,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )),
            error,
        );
        return;
    }
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GBufferedInputStreamClass;
    (*(*stream).priv_0).outstanding_callback = callback;
    g_object_ref(stream as gpointer);
    (*class).fill_async.expect("non-null function pointer")(
        stream,
        count,
        io_priority,
        cancellable,
        Some(
            safe_c2rust_async_fill_callback_wrapper
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_buffered_input_stream_fill_finish(
    mut stream: *mut GBufferedInputStream,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gssize {
    let mut class: *mut GBufferedInputStreamClass =
        ::core::ptr::null_mut::<GBufferedInputStreamClass>();
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_buffered_input_stream_get_type();
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
            b"G_IS_BUFFERED_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = result as *mut GTypeInstance;
            let mut __t: GType = g_async_result_get_type();
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
            b"G_IS_ASYNC_RESULT (result)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    if g_async_result_legacy_propagate_error(result, error) != 0 {
        return -(1 as ::core::ffi::c_int) as gssize;
    } else if g_async_result_is_tagged(
        result,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GBufferedInputStream,
                    gssize,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_buffered_input_stream_fill_async
                as unsafe extern "C" fn(
                    *mut GBufferedInputStream,
                    gssize,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    ) != 0
    {
        return g_task_propagate_int(result as *mut ::core::ffi::c_void as *mut GTask, error);
    }
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GBufferedInputStreamClass;
    return (*class).fill_finish.expect("non-null function pointer")(stream, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_buffered_input_stream_get_available(
    mut stream: *mut GBufferedInputStream,
) -> gsize {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_buffered_input_stream_get_type();
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
            b"G_IS_BUFFERED_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gsize;
    }
    return (*(*stream).priv_0)
        .end
        .wrapping_sub((*(*stream).priv_0).pos);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_buffered_input_stream_peek(
    mut stream: *mut GBufferedInputStream,
    mut buffer: *mut ::core::ffi::c_void,
    mut offset: gsize,
    mut count: gsize,
) -> gsize {
    let mut available: gsize = 0;
    let mut end: gsize = 0;
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_buffered_input_stream_get_type();
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
            b"G_IS_BUFFERED_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gsize;
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if !buffer.is_null() {
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
            b"buffer != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gsize;
    }
    available = safe_c2rust_g_buffered_input_stream_get_available(stream);
    if offset > available || offset > G_MAXSIZE.wrapping_sub(count) {
        return 0 as gsize;
    }
    end = if offset.wrapping_add(count) < available {
        offset.wrapping_add(count)
    } else {
        available
    };
    count = end.wrapping_sub(offset);
    memcpy(
        buffer,
        (*(*stream).priv_0)
            .buffer
            .offset((*(*stream).priv_0).pos as isize)
            .offset(offset as isize) as *const ::core::ffi::c_void,
        count as size_t,
    );
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_buffered_input_stream_peek_buffer(
    mut stream: *mut GBufferedInputStream,
    mut count: *mut gsize,
) -> *const ::core::ffi::c_void {
    let mut priv_0: *mut GBufferedInputStreamPrivate =
        ::core::ptr::null_mut::<GBufferedInputStreamPrivate>();
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_buffered_input_stream_get_type();
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
            b"G_IS_BUFFERED_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_void>();
    }
    priv_0 = (*stream).priv_0;
    if !count.is_null() {
        *count = (*priv_0).end.wrapping_sub((*priv_0).pos);
    }
    return (*priv_0).buffer.offset((*priv_0).pos as isize) as *const ::core::ffi::c_void;
}
unsafe extern "C" fn safe_c2rust_compact_buffer(mut stream: *mut GBufferedInputStream) {
    let mut priv_0: *mut GBufferedInputStreamPrivate =
        ::core::ptr::null_mut::<GBufferedInputStreamPrivate>();
    let mut current_size: gsize = 0;
    priv_0 = (*stream).priv_0;
    current_size = (*priv_0).end.wrapping_sub((*priv_0).pos);
    memmove(
        (*priv_0).buffer as *mut ::core::ffi::c_void,
        (*priv_0).buffer.offset((*priv_0).pos as isize) as *const ::core::ffi::c_void,
        current_size as size_t,
    );
    (*priv_0).pos = 0 as gsize;
    (*priv_0).end = current_size;
}
unsafe extern "C" fn safe_c2rust_g_buffered_input_stream_real_fill(
    mut stream: *mut GBufferedInputStream,
    mut count: gssize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    let mut priv_0: *mut GBufferedInputStreamPrivate =
        ::core::ptr::null_mut::<GBufferedInputStreamPrivate>();
    let mut base_stream: *mut GInputStream = ::core::ptr::null_mut::<GInputStream>();
    let mut nread: gssize = 0;
    let mut in_buffer: gsize = 0;
    priv_0 = (*stream).priv_0;
    if count == -(1 as ::core::ffi::c_int) as gssize {
        count = (*priv_0).len as gssize;
    }
    in_buffer = (*priv_0).end.wrapping_sub((*priv_0).pos);
    count = (if (count as gsize) < (*priv_0).len.wrapping_sub(in_buffer) {
        count as gsize
    } else {
        (*priv_0).len.wrapping_sub(in_buffer)
    }) as gssize;
    if (*priv_0).len.wrapping_sub((*priv_0).end) < count as gsize {
        safe_c2rust_compact_buffer(stream);
    }
    base_stream = (*(stream as *mut ::core::ffi::c_void as *mut GFilterInputStream)).base_stream;
    nread = g_input_stream_read(
        base_stream,
        (*priv_0).buffer.offset((*priv_0).end as isize) as *mut ::core::ffi::c_void,
        count as gsize,
        cancellable,
        error,
    );
    if nread > 0 as gssize {
        (*priv_0).end = (*priv_0).end.wrapping_add(nread as gsize);
    }
    return nread;
}
unsafe extern "C" fn safe_c2rust_g_buffered_input_stream_skip(
    mut stream: *mut GInputStream,
    mut count: gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    let mut bstream: *mut GBufferedInputStream = ::core::ptr::null_mut::<GBufferedInputStream>();
    let mut priv_0: *mut GBufferedInputStreamPrivate =
        ::core::ptr::null_mut::<GBufferedInputStreamPrivate>();
    let mut class: *mut GBufferedInputStreamClass =
        ::core::ptr::null_mut::<GBufferedInputStreamClass>();
    let mut base_stream: *mut GInputStream = ::core::ptr::null_mut::<GInputStream>();
    let mut available: gsize = 0;
    let mut bytes_skipped: gsize = 0;
    let mut nread: gssize = 0;
    bstream = stream as *mut ::core::ffi::c_void as *mut GBufferedInputStream;
    priv_0 = (*bstream).priv_0;
    available = (*priv_0).end.wrapping_sub((*priv_0).pos);
    if count <= available {
        (*priv_0).pos = (*priv_0).pos.wrapping_add(count);
        return count as gssize;
    }
    (*priv_0).pos = 0 as gsize;
    (*priv_0).end = 0 as gsize;
    bytes_skipped = available;
    count = count.wrapping_sub(available);
    if bytes_skipped > 0 as gsize {
        error = ::core::ptr::null_mut::<*mut GError>();
    }
    if count > (*priv_0).len {
        base_stream =
            (*(stream as *mut ::core::ffi::c_void as *mut GFilterInputStream)).base_stream;
        nread = g_input_stream_skip(base_stream, count, cancellable, error);
        if nread < 0 as gssize && bytes_skipped == 0 as gsize {
            return -(1 as ::core::ffi::c_int) as gssize;
        }
        if nread > 0 as gssize {
            bytes_skipped = bytes_skipped.wrapping_add(nread as gsize);
        }
        return bytes_skipped as gssize;
    }
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GBufferedInputStreamClass;
    nread = (*class).fill.expect("non-null function pointer")(
        bstream,
        (*priv_0).len as gssize,
        cancellable,
        error,
    );
    if nread < 0 as gssize {
        if bytes_skipped == 0 as gsize {
            return -(1 as ::core::ffi::c_int) as gssize;
        } else {
            return bytes_skipped as gssize;
        }
    }
    available = (*priv_0).end.wrapping_sub((*priv_0).pos);
    count = if count < available { count } else { available };
    bytes_skipped = bytes_skipped.wrapping_add(count);
    (*priv_0).pos = (*priv_0).pos.wrapping_add(count);
    return bytes_skipped as gssize;
}
unsafe extern "C" fn safe_c2rust_g_buffered_input_stream_read(
    mut stream: *mut GInputStream,
    mut buffer: *mut ::core::ffi::c_void,
    mut count: gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    let mut bstream: *mut GBufferedInputStream = ::core::ptr::null_mut::<GBufferedInputStream>();
    let mut priv_0: *mut GBufferedInputStreamPrivate =
        ::core::ptr::null_mut::<GBufferedInputStreamPrivate>();
    let mut class: *mut GBufferedInputStreamClass =
        ::core::ptr::null_mut::<GBufferedInputStreamClass>();
    let mut base_stream: *mut GInputStream = ::core::ptr::null_mut::<GInputStream>();
    let mut available: gsize = 0;
    let mut bytes_read: gsize = 0;
    let mut nread: gssize = 0;
    bstream = stream as *mut ::core::ffi::c_void as *mut GBufferedInputStream;
    priv_0 = (*bstream).priv_0;
    available = (*priv_0).end.wrapping_sub((*priv_0).pos);
    if count <= available {
        memcpy(
            buffer,
            (*priv_0).buffer.offset((*priv_0).pos as isize) as *const ::core::ffi::c_void,
            count as size_t,
        );
        (*priv_0).pos = (*priv_0).pos.wrapping_add(count);
        return count as gssize;
    }
    memcpy(
        buffer,
        (*priv_0).buffer.offset((*priv_0).pos as isize) as *const ::core::ffi::c_void,
        available as size_t,
    );
    (*priv_0).pos = 0 as gsize;
    (*priv_0).end = 0 as gsize;
    bytes_read = available;
    count = count.wrapping_sub(available);
    if bytes_read > 0 as gsize {
        error = ::core::ptr::null_mut::<*mut GError>();
    }
    if count > (*priv_0).len {
        base_stream =
            (*(stream as *mut ::core::ffi::c_void as *mut GFilterInputStream)).base_stream;
        nread = g_input_stream_read(
            base_stream,
            (buffer as *mut ::core::ffi::c_char).offset(bytes_read as isize)
                as *mut ::core::ffi::c_void,
            count,
            cancellable,
            error,
        );
        if nread < 0 as gssize && bytes_read == 0 as gsize {
            return -(1 as ::core::ffi::c_int) as gssize;
        }
        if nread > 0 as gssize {
            bytes_read = bytes_read.wrapping_add(nread as gsize);
        }
        return bytes_read as gssize;
    }
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GBufferedInputStreamClass;
    nread = (*class).fill.expect("non-null function pointer")(
        bstream,
        (*priv_0).len as gssize,
        cancellable,
        error,
    );
    if nread < 0 as gssize {
        if bytes_read == 0 as gsize {
            return -(1 as ::core::ffi::c_int) as gssize;
        } else {
            return bytes_read as gssize;
        }
    }
    available = (*priv_0).end.wrapping_sub((*priv_0).pos);
    count = if count < available { count } else { available };
    memcpy(
        (buffer as *mut ::core::ffi::c_char).offset(bytes_read as isize)
            as *mut ::core::ffi::c_void,
        ((*priv_0).buffer as *mut ::core::ffi::c_char).offset((*priv_0).pos as isize)
            as *const ::core::ffi::c_void,
        count as size_t,
    );
    bytes_read = bytes_read.wrapping_add(count);
    (*priv_0).pos = (*priv_0).pos.wrapping_add(count);
    return bytes_read as gssize;
}
unsafe extern "C" fn safe_c2rust_g_buffered_input_stream_tell(
    mut seekable: *mut GSeekable,
) -> goffset {
    let mut bstream: *mut GBufferedInputStream = ::core::ptr::null_mut::<GBufferedInputStream>();
    let mut priv_0: *mut GBufferedInputStreamPrivate =
        ::core::ptr::null_mut::<GBufferedInputStreamPrivate>();
    let mut base_stream: *mut GInputStream = ::core::ptr::null_mut::<GInputStream>();
    let mut base_stream_seekable: *mut GSeekable = ::core::ptr::null_mut::<GSeekable>();
    let mut available: gsize = 0;
    let mut base_offset: goffset = 0;
    bstream = seekable as *mut ::core::ffi::c_void as *mut GBufferedInputStream;
    priv_0 = (*bstream).priv_0;
    base_stream = (*(seekable as *mut ::core::ffi::c_void as *mut GFilterInputStream)).base_stream;
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
    available = (*priv_0).end.wrapping_sub((*priv_0).pos);
    base_offset = g_seekable_tell(base_stream_seekable);
    return (base_offset as gsize).wrapping_sub(available) as goffset;
}
unsafe extern "C" fn safe_c2rust_g_buffered_input_stream_can_seek(
    mut seekable: *mut GSeekable,
) -> gboolean {
    let mut base_stream: *mut GInputStream = ::core::ptr::null_mut::<GInputStream>();
    base_stream = (*(seekable as *mut ::core::ffi::c_void as *mut GFilterInputStream)).base_stream;
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
unsafe extern "C" fn safe_c2rust_g_buffered_input_stream_seek(
    mut seekable: *mut GSeekable,
    mut offset: goffset,
    mut type_0: GSeekType,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut bstream: *mut GBufferedInputStream = ::core::ptr::null_mut::<GBufferedInputStream>();
    let mut priv_0: *mut GBufferedInputStreamPrivate =
        ::core::ptr::null_mut::<GBufferedInputStreamPrivate>();
    let mut base_stream: *mut GInputStream = ::core::ptr::null_mut::<GInputStream>();
    let mut base_stream_seekable: *mut GSeekable = ::core::ptr::null_mut::<GSeekable>();
    bstream = seekable as *mut ::core::ffi::c_void as *mut GBufferedInputStream;
    priv_0 = (*bstream).priv_0;
    base_stream = (*(seekable as *mut ::core::ffi::c_void as *mut GFilterInputStream)).base_stream;
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
    if type_0 as ::core::ffi::c_uint == G_SEEK_CUR as ::core::ffi::c_int as ::core::ffi::c_uint {
        if offset <= (*priv_0).end.wrapping_sub((*priv_0).pos) as goffset
            && offset >= (*priv_0).pos.wrapping_neg() as goffset
        {
            (*priv_0).pos = (*priv_0).pos.wrapping_add(offset as gsize);
            return TRUE;
        } else {
            offset = (offset as gsize).wrapping_sub((*priv_0).end.wrapping_sub((*priv_0).pos))
                as goffset as goffset;
        }
    }
    if g_seekable_seek(base_stream_seekable, offset, type_0, cancellable, error) != 0 {
        (*priv_0).pos = 0 as gsize;
        (*priv_0).end = 0 as gsize;
        return TRUE;
    } else {
        return FALSE;
    };
}
unsafe extern "C" fn safe_c2rust_g_buffered_input_stream_can_truncate(
    mut seekable: *mut GSeekable,
) -> gboolean {
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_buffered_input_stream_truncate(
    mut seekable: *mut GSeekable,
    mut offset: goffset,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    g_set_error_literal(
        error,
        g_io_error_quark(),
        G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
        glib_gettext(b"Cannot truncate GBufferedInputStream\0" as *const u8 as *const gchar),
    );
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_buffered_input_stream_read_byte(
    mut stream: *mut GBufferedInputStream,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> ::core::ffi::c_int {
    let mut priv_0: *mut GBufferedInputStreamPrivate =
        ::core::ptr::null_mut::<GBufferedInputStreamPrivate>();
    let mut class: *mut GBufferedInputStreamClass =
        ::core::ptr::null_mut::<GBufferedInputStreamClass>();
    let mut input_stream: *mut GInputStream = ::core::ptr::null_mut::<GInputStream>();
    let mut available: gsize = 0;
    let mut nread: gssize = 0;
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_buffered_input_stream_get_type();
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
            b"G_IS_BUFFERED_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    priv_0 = (*stream).priv_0;
    input_stream = stream as *mut ::core::ffi::c_void as *mut GInputStream;
    if g_input_stream_is_closed(input_stream) != 0 {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_CLOSED as ::core::ffi::c_int as gint,
            glib_gettext(b"Stream is already closed\0" as *const u8 as *const gchar),
        );
        return -(1 as ::core::ffi::c_int);
    }
    if g_input_stream_set_pending(input_stream, error) == 0 {
        return -(1 as ::core::ffi::c_int);
    }
    available = (*priv_0).end.wrapping_sub((*priv_0).pos);
    if available != 0 as gsize {
        g_input_stream_clear_pending(input_stream);
        let fresh0 = (*priv_0).pos;
        (*priv_0).pos = (*priv_0).pos.wrapping_add(1);
        return *(*priv_0).buffer.offset(fresh0 as isize) as ::core::ffi::c_int;
    }
    if !cancellable.is_null() {
        g_cancellable_push_current(cancellable);
    }
    (*priv_0).pos = 0 as gsize;
    (*priv_0).end = 0 as gsize;
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GBufferedInputStreamClass;
    nread = (*class).fill.expect("non-null function pointer")(
        stream,
        (*priv_0).len as gssize,
        cancellable,
        error,
    );
    if !cancellable.is_null() {
        g_cancellable_pop_current(cancellable);
    }
    g_input_stream_clear_pending(input_stream);
    if nread <= 0 as gssize {
        return -(1 as ::core::ffi::c_int);
    }
    let fresh1 = (*priv_0).pos;
    (*priv_0).pos = (*priv_0).pos.wrapping_add(1);
    return *(*priv_0).buffer.offset(fresh1 as isize) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_fill_async_callback(
    mut source_object: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut res: gssize = 0;
    let mut task: *mut GTask = user_data as *mut GTask;
    error = ::core::ptr::null_mut::<GError>();
    res = g_input_stream_read_finish(
        source_object as *mut ::core::ffi::c_void as *mut GInputStream,
        result,
        &raw mut error,
    );
    if res == -(1 as ::core::ffi::c_int) as gssize {
        g_task_return_error(task, error);
    } else {
        let mut stream: *mut GBufferedInputStream = ::core::ptr::null_mut::<GBufferedInputStream>();
        let mut priv_0: *mut GBufferedInputStreamPrivate =
            ::core::ptr::null_mut::<GBufferedInputStreamPrivate>();
        stream = g_task_get_source_object(task) as *mut GBufferedInputStream;
        priv_0 = (*(stream as *mut ::core::ffi::c_void as *mut GBufferedInputStream)).priv_0;
        let mut __n1: gint64 = (*priv_0).end.wrapping_add(res as gsize) as gint64;
        let mut __n2: gint64 = (*priv_0).len as gint64;
        if !(__n1 <= __n2) {
            g_assertion_message_cmpint(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gbufferedinputstream.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1047 as ::core::ffi::c_int,
                G_STRFUNC,
                b"priv->end + res <= priv->len\0" as *const u8
                    as *const ::core::ffi::c_char,
                __n1 as guint64,
                b"<=\0" as *const u8 as *const ::core::ffi::c_char,
                __n2 as guint64,
                'i' as i32 as ::core::ffi::c_char,
            );
        }
        (*priv_0).end = (*priv_0).end.wrapping_add(res as gsize);
        g_task_return_int(task, res);
    }
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_buffered_input_stream_real_fill_async(
    mut stream: *mut GBufferedInputStream,
    mut count: gssize,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut priv_0: *mut GBufferedInputStreamPrivate =
        ::core::ptr::null_mut::<GBufferedInputStreamPrivate>();
    let mut base_stream: *mut GInputStream = ::core::ptr::null_mut::<GInputStream>();
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut in_buffer: gsize = 0;
    priv_0 = (*stream).priv_0;
    if count == -(1 as ::core::ffi::c_int) as gssize {
        count = (*priv_0).len as gssize;
    }
    in_buffer = (*priv_0).end.wrapping_sub((*priv_0).pos);
    count = (if (count as gsize) < (*priv_0).len.wrapping_sub(in_buffer) {
        count as gsize
    } else {
        (*priv_0).len.wrapping_sub(in_buffer)
    }) as gssize;
    if (*priv_0).len.wrapping_sub((*priv_0).end) < count as gsize {
        safe_c2rust_compact_buffer(stream);
    }
    task = g_task_new(stream as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GBufferedInputStream,
                    gssize,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_buffered_input_stream_real_fill_async
                as unsafe extern "C" fn(
                    *mut GBufferedInputStream,
                    gssize,
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
            b"g_buffered_input_stream_real_fill_async\0" as *const u8 as *const gchar,
        );
    }
    base_stream = (*(stream as *mut ::core::ffi::c_void as *mut GFilterInputStream)).base_stream;
    g_input_stream_read_async(
        base_stream,
        (*priv_0).buffer.offset((*priv_0).end as isize) as *mut ::core::ffi::c_void,
        count as gsize,
        io_priority,
        cancellable,
        Some(
            safe_c2rust_fill_async_callback
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        task as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_g_buffered_input_stream_real_fill_finish(
    mut stream: *mut GBufferedInputStream,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gssize {
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, stream as gpointer) != 0 {
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
            b"g_task_is_valid (result, stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    return g_task_propagate_int(result as *mut ::core::ffi::c_void as *mut GTask, error);
}
unsafe extern "C" fn safe_c2rust_free_skip_async_data(mut _data: gpointer) {
    let mut data: *mut SkipAsyncData = _data as *mut SkipAsyncData;
    g_slice_free1(
        ::core::mem::size_of::<SkipAsyncData>() as gsize,
        data as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_large_skip_callback(
    mut source_object: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut data: *mut SkipAsyncData = ::core::ptr::null_mut::<SkipAsyncData>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut nread: gssize = 0;
    data = g_task_get_task_data(task) as *mut SkipAsyncData;
    error = ::core::ptr::null_mut::<GError>();
    nread = g_input_stream_skip_finish(
        source_object as *mut ::core::ffi::c_void as *mut GInputStream,
        result,
        &raw mut error,
    );
    if nread < 0 as gssize && (*data).bytes_skipped == 0 as gsize {
        g_task_return_error(task, error);
    } else {
        if !error.is_null() {
            g_error_free(error);
        }
        if nread > 0 as gssize {
            (*data).bytes_skipped = (*data).bytes_skipped.wrapping_add(nread as gsize);
        }
        g_task_return_int(task, (*data).bytes_skipped as gssize);
    }
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_skip_fill_buffer_callback(
    mut source_object: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut bstream: *mut GBufferedInputStream = ::core::ptr::null_mut::<GBufferedInputStream>();
    let mut priv_0: *mut GBufferedInputStreamPrivate =
        ::core::ptr::null_mut::<GBufferedInputStreamPrivate>();
    let mut data: *mut SkipAsyncData = ::core::ptr::null_mut::<SkipAsyncData>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut nread: gssize = 0;
    let mut available: gsize = 0;
    bstream = source_object as *mut ::core::ffi::c_void as *mut GBufferedInputStream;
    priv_0 = (*bstream).priv_0;
    data = g_task_get_task_data(task) as *mut SkipAsyncData;
    error = ::core::ptr::null_mut::<GError>();
    nread = safe_c2rust_g_buffered_input_stream_fill_finish(bstream, result, &raw mut error);
    if nread < 0 as gssize && (*data).bytes_skipped == 0 as gsize {
        g_task_return_error(task, error);
    } else {
        if !error.is_null() {
            g_error_free(error);
        }
        if nread > 0 as gssize {
            available = (*priv_0).end.wrapping_sub((*priv_0).pos);
            (*data).count = if (*data).count < available {
                (*data).count
            } else {
                available
            };
            (*data).bytes_skipped = (*data).bytes_skipped.wrapping_add((*data).count);
            (*priv_0).pos = (*priv_0).pos.wrapping_add((*data).count);
        }
        if ({
            let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
            if (*data).bytes_skipped <= 9223372036854775807 as ::core::ffi::c_long as gsize {
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gbufferedinputstream.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1190 as ::core::ffi::c_int,
                G_STRFUNC,
                b"data->bytes_skipped <= G_MAXSSIZE\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        g_task_return_int(task, (*data).bytes_skipped as gssize);
    }
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_buffered_input_stream_skip_async(
    mut stream: *mut GInputStream,
    mut count: gsize,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut bstream: *mut GBufferedInputStream = ::core::ptr::null_mut::<GBufferedInputStream>();
    let mut priv_0: *mut GBufferedInputStreamPrivate =
        ::core::ptr::null_mut::<GBufferedInputStreamPrivate>();
    let mut class: *mut GBufferedInputStreamClass =
        ::core::ptr::null_mut::<GBufferedInputStreamClass>();
    let mut base_stream: *mut GInputStream = ::core::ptr::null_mut::<GInputStream>();
    let mut available: gsize = 0;
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut data: *mut SkipAsyncData = ::core::ptr::null_mut::<SkipAsyncData>();
    bstream = stream as *mut ::core::ffi::c_void as *mut GBufferedInputStream;
    priv_0 = (*bstream).priv_0;
    data = g_slice_alloc(::core::mem::size_of::<SkipAsyncData>() as gsize) as *mut SkipAsyncData;
    (*data).bytes_skipped = 0 as gsize;
    task = g_task_new(stream as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GInputStream,
                    gsize,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_buffered_input_stream_skip_async
                as unsafe extern "C" fn(
                    *mut GInputStream,
                    gsize,
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
            b"g_buffered_input_stream_skip_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        data as gpointer,
        Some(safe_c2rust_free_skip_async_data as unsafe extern "C" fn(gpointer) -> ()),
    );
    available = (*priv_0).end.wrapping_sub((*priv_0).pos);
    if count <= available {
        (*priv_0).pos = (*priv_0).pos.wrapping_add(count);
        g_task_return_int(task, count as gssize);
        g_object_unref(task as gpointer);
        return;
    }
    (*priv_0).pos = 0 as gsize;
    (*priv_0).end = 0 as gsize;
    count = count.wrapping_sub(available);
    (*data).bytes_skipped = available;
    (*data).count = count;
    if count > (*priv_0).len {
        base_stream =
            (*(stream as *mut ::core::ffi::c_void as *mut GFilterInputStream)).base_stream;
        if ({
            let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
            if count as gssize >= 0 as gssize {
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gbufferedinputstream.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1253 as ::core::ffi::c_int,
                G_STRFUNC,
                b"(gssize) count >= 0\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        g_input_stream_skip_async(
            base_stream,
            count,
            io_priority,
            cancellable,
            Some(
                safe_c2rust_large_skip_callback
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            task as gpointer,
        );
    } else {
        class = (*(stream as *mut GTypeInstance)).g_class as *mut GBufferedInputStreamClass;
        (*class).fill_async.expect("non-null function pointer")(
            bstream,
            (*priv_0).len as gssize,
            io_priority,
            cancellable,
            Some(
                safe_c2rust_skip_fill_buffer_callback
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            task as gpointer,
        );
    };
}
unsafe extern "C" fn safe_c2rust_g_buffered_input_stream_skip_finish(
    mut stream: *mut GInputStream,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gssize {
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, stream as gpointer) != 0 {
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
            b"g_task_is_valid (result, stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    return g_task_propagate_int(result as *mut ::core::ffi::c_void as *mut GTask, error);
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
