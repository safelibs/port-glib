use ::c2rust_bitfields;
extern "C" {
    pub type _GBytes;
    pub type _GData;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _GPollableInputStream;
    pub type _GSeekable;
    pub type _GTask;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_error_matches(error: *const GError, domain: GQuark, code: gint) -> gboolean;
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
    fn g_bytes_new_take(data: gpointer, size: gsize) -> *mut GBytes;
    fn g_bytes_new_static(data: gconstpointer, size: gsize) -> *mut GBytes;
    fn g_bytes_unref(bytes: *mut GBytes);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_source_unref(source: *mut GSource);
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
    fn glib_gettext(str: *const gchar) -> *const gchar;
    fn g_type_class_peek_parent(g_class: gpointer) -> gpointer;
    fn g_type_interface_peek(instance_class: gpointer, iface_type: GType) -> gpointer;
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
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_cancellable_push_current(cancellable: *mut GCancellable);
    fn g_cancellable_pop_current(cancellable: *mut GCancellable);
    fn g_io_error_quark() -> GQuark;
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
    fn g_task_set_priority(task: *mut GTask, priority: gint);
    fn g_task_set_check_cancellable(task: *mut GTask, check_cancellable: gboolean);
    fn g_task_set_source_tag(task: *mut GTask, source_tag: gpointer);
    fn g_task_set_static_name(task: *mut GTask, name: *const gchar);
    fn g_task_get_task_data(task: *mut GTask) -> gpointer;
    fn g_task_get_priority(task: *mut GTask) -> gint;
    fn g_task_get_cancellable(task: *mut GTask) -> *mut GCancellable;
    fn g_task_get_name(task: *mut GTask) -> *const gchar;
    fn g_task_is_valid(result: gpointer, source_object: gpointer) -> gboolean;
    fn g_task_run_in_thread(task: *mut GTask, task_func: GTaskThreadFunc);
    fn g_task_attach_source(task: *mut GTask, source: *mut GSource, callback: GSourceFunc);
    fn g_task_return_pointer(task: *mut GTask, result: gpointer, result_destroy: GDestroyNotify);
    fn g_task_return_boolean(task: *mut GTask, result: gboolean);
    fn g_task_return_int(task: *mut GTask, result: gssize);
    fn g_task_return_error(task: *mut GTask, error: *mut GError);
    fn g_task_return_error_if_cancelled(task: *mut GTask) -> gboolean;
    fn g_task_propagate_pointer(task: *mut GTask, error: *mut *mut GError) -> gpointer;
    fn g_task_propagate_boolean(task: *mut GTask, error: *mut *mut GError) -> gboolean;
    fn g_task_propagate_int(task: *mut GTask, error: *mut *mut GError) -> gssize;
    fn g_pollable_input_stream_get_type() -> GType;
    fn g_pollable_input_stream_can_poll(stream: *mut GPollableInputStream) -> gboolean;
    fn g_pollable_input_stream_create_source(
        stream: *mut GPollableInputStream,
        cancellable: *mut GCancellable,
    ) -> *mut GSource;
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
pub type guchar = ::core::ffi::c_uchar;
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
pub type GInputStream = _GInputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInputStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GInputStreamPrivate,
}
pub type GInputStreamPrivate = _GInputStreamPrivate;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GInputStreamPrivate {
    #[bitfield(name = "closed", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "pending", ty = "guint", bits = "1..=1")]
    pub closed_pending: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub outstanding_callback: GAsyncReadyCallback,
}
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
pub type GPollableInputStream = _GPollableInputStream;
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
pub type GTaskThreadFunc =
    Option<unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SkipFallbackAsyncData {
    pub buffer: [::core::ffi::c_char; 8192],
    pub count: gsize,
    pub count_skipped: gsize,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ReadData {
    pub buffer: *mut ::core::ffi::c_void,
    pub count: gsize,
}
pub type GPollableInputStreamInterface = _GPollableInputStreamInterface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPollableInputStreamInterface {
    pub g_iface: GTypeInterface,
    pub can_poll: Option<unsafe extern "C" fn(*mut GPollableInputStream) -> gboolean>,
    pub is_readable: Option<unsafe extern "C" fn(*mut GPollableInputStream) -> gboolean>,
    pub create_source:
        Option<unsafe extern "C" fn(*mut GPollableInputStream, *mut GCancellable) -> *mut GSource>,
    pub read_nonblocking: Option<
        unsafe extern "C" fn(
            *mut GPollableInputStream,
            *mut ::core::ffi::c_void,
            gsize,
            *mut *mut GError,
        ) -> gssize,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AsyncReadAll {
    pub buffer: *mut gchar,
    pub to_read: gsize,
    pub bytes_read: gsize,
}
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const G_MAXOFFSET: ::core::ffi::c_long = G_MAXINT64;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_MAXINT64: ::core::ffi::c_long = 0x7fffffffffffffff as ::core::ffi::c_long;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_input_stream_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GInputStream\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GInputStreamClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_input_stream_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GInputStream>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GInputStream) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_input_stream_init as unsafe extern "C" fn(*mut GInputStream) -> (),
            )),
        ),
        G_TYPE_FLAG_ABSTRACT,
    );
    safe_c2rust_GInputStream_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GInputStreamPrivate>() as gsize,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_input_stream_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_input_stream_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GInputStream_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut safe_c2rust_GInputStream_private_offset);
    }
    safe_c2rust_g_input_stream_class_init(klass as *mut GInputStreamClass);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_input_stream_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_input_stream_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_input_stream_get_instance_private(
    mut self_0: *mut GInputStream,
) -> gpointer {
    return (self_0 as *mut guint8).offset(safe_c2rust_GInputStream_private_offset as glong as isize)
        as gpointer;
}
static mut safe_c2rust_GInputStream_private_offset: gint = 0;
static mut safe_c2rust_g_input_stream_parent_class: gpointer = NULL;
unsafe extern "C" fn safe_c2rust_g_input_stream_dispose(mut object: *mut GObject) {
    let mut stream: *mut GInputStream = ::core::ptr::null_mut::<GInputStream>();
    stream = object as *mut ::core::ffi::c_void as *mut GInputStream;
    if (*(*stream).priv_0).closed() == 0 {
        safe_c2rust_g_input_stream_close(
            stream,
            ::core::ptr::null_mut::<GCancellable>(),
            ::core::ptr::null_mut::<*mut GError>(),
        );
    }
    (*(safe_c2rust_g_input_stream_parent_class as *mut GObjectClass))
        .dispose
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_input_stream_class_init(mut klass: *mut GInputStreamClass) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).dispose =
        Some(safe_c2rust_g_input_stream_dispose as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*klass).skip = Some(
        safe_c2rust_g_input_stream_real_skip
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
    (*klass).read_async = Some(
        safe_c2rust_g_input_stream_real_read_async
            as unsafe extern "C" fn(
                *mut GInputStream,
                *mut ::core::ffi::c_void,
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
                *mut ::core::ffi::c_void,
                gsize,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*klass).read_finish = Some(
        safe_c2rust_g_input_stream_real_read_finish
            as unsafe extern "C" fn(
                *mut GInputStream,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> gssize,
    )
        as Option<
            unsafe extern "C" fn(*mut GInputStream, *mut GAsyncResult, *mut *mut GError) -> gssize,
        >;
    (*klass).skip_async = Some(
        safe_c2rust_g_input_stream_real_skip_async
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
    (*klass).skip_finish = Some(
        safe_c2rust_g_input_stream_real_skip_finish
            as unsafe extern "C" fn(
                *mut GInputStream,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> gssize,
    )
        as Option<
            unsafe extern "C" fn(*mut GInputStream, *mut GAsyncResult, *mut *mut GError) -> gssize,
        >;
    (*klass).close_async = Some(
        safe_c2rust_g_input_stream_real_close_async
            as unsafe extern "C" fn(
                *mut GInputStream,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GInputStream,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*klass).close_finish = Some(
        safe_c2rust_g_input_stream_real_close_finish
            as unsafe extern "C" fn(
                *mut GInputStream,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GInputStream,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> gboolean,
        >;
}
unsafe extern "C" fn safe_c2rust_g_input_stream_init(mut stream: *mut GInputStream) {
    (*stream).priv_0 =
        safe_c2rust_g_input_stream_get_instance_private(stream) as *mut GInputStreamPrivate;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_input_stream_read(
    mut stream: *mut GInputStream,
    mut buffer: *mut ::core::ffi::c_void,
    mut count: gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    let mut class: *mut GInputStreamClass = ::core::ptr::null_mut::<GInputStreamClass>();
    let mut res: gssize = 0;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_input_stream_get_type();
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
            b"G_IS_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !buffer.is_null() {
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
            b"buffer != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gssize;
    }
    if count == 0 as gsize {
        return 0 as gssize;
    }
    if (count as gssize) < 0 as gssize {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(b"Too large count value passed to %s\0" as *const u8 as *const gchar),
            G_STRFUNC,
        );
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GInputStreamClass;
    if (*class).read_fn.is_none() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Input stream doesn\xE2\x80\x99t implement read\0" as *const u8 as *const gchar,
            ),
        );
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    if safe_c2rust_g_input_stream_set_pending(stream, error) == 0 {
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    if !cancellable.is_null() {
        g_cancellable_push_current(cancellable);
    }
    res = (*class).read_fn.expect("non-null function pointer")(
        stream,
        buffer,
        count,
        cancellable,
        error,
    );
    if !cancellable.is_null() {
        g_cancellable_pop_current(cancellable);
    }
    safe_c2rust_g_input_stream_clear_pending(stream);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_input_stream_read_all(
    mut stream: *mut GInputStream,
    mut buffer: *mut ::core::ffi::c_void,
    mut count: gsize,
    mut bytes_read: *mut gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _bytes_read: gsize = 0;
    let mut res: gssize = 0;
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_input_stream_get_type();
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
            b"G_IS_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !buffer.is_null() {
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
            b"buffer != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    _bytes_read = 0 as gsize;
    while _bytes_read < count {
        res = safe_c2rust_g_input_stream_read(
            stream,
            (buffer as *mut ::core::ffi::c_char).offset(_bytes_read as isize)
                as *mut ::core::ffi::c_void,
            count.wrapping_sub(_bytes_read),
            cancellable,
            error,
        );
        if res == -(1 as ::core::ffi::c_int) as gssize {
            if !bytes_read.is_null() {
                *bytes_read = _bytes_read;
            }
            return FALSE;
        }
        if res == 0 as gssize {
            break;
        }
        _bytes_read = _bytes_read.wrapping_add(res as gsize);
    }
    if !bytes_read.is_null() {
        *bytes_read = _bytes_read;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_input_stream_read_bytes(
    mut stream: *mut GInputStream,
    mut count: gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GBytes {
    let mut buf: *mut guchar = ::core::ptr::null_mut::<guchar>();
    let mut nread: gssize = 0;
    buf = g_malloc(count) as *mut guchar;
    nread = safe_c2rust_g_input_stream_read(
        stream,
        buf as *mut ::core::ffi::c_void,
        count,
        cancellable,
        error,
    );
    if nread == -(1 as ::core::ffi::c_int) as gssize {
        g_free(buf as gpointer);
        return ::core::ptr::null_mut::<GBytes>();
    } else if nread == 0 as gssize {
        g_free(buf as gpointer);
        return g_bytes_new_static(
            b"\0" as *const u8 as *const ::core::ffi::c_char as gconstpointer,
            0 as gsize,
        );
    } else {
        return g_bytes_new_take(buf as gpointer, nread as gsize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_input_stream_skip(
    mut stream: *mut GInputStream,
    mut count: gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    let mut class: *mut GInputStreamClass = ::core::ptr::null_mut::<GInputStreamClass>();
    let mut res: gssize = 0;
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_input_stream_get_type();
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
            b"G_IS_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    if count == 0 as gsize {
        return 0 as gssize;
    }
    if (count as gssize) < 0 as gssize {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(b"Too large count value passed to %s\0" as *const u8 as *const gchar),
            G_STRFUNC,
        );
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GInputStreamClass;
    if safe_c2rust_g_input_stream_set_pending(stream, error) == 0 {
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    if !cancellable.is_null() {
        g_cancellable_push_current(cancellable);
    }
    res = (*class).skip.expect("non-null function pointer")(stream, count, cancellable, error);
    if !cancellable.is_null() {
        g_cancellable_pop_current(cancellable);
    }
    safe_c2rust_g_input_stream_clear_pending(stream);
    return res;
}
unsafe extern "C" fn safe_c2rust_g_input_stream_real_skip(
    mut stream: *mut GInputStream,
    mut count: gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    let mut class: *mut GInputStreamClass = ::core::ptr::null_mut::<GInputStreamClass>();
    let mut ret: gssize = 0;
    let mut read_bytes: gssize = 0;
    let mut buffer: [::core::ffi::c_char; 8192] = [0; 8192];
    let mut my_error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
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
        && g_seekable_can_seek(stream as *mut ::core::ffi::c_void as *mut GSeekable) != 0
    {
        let mut seekable: *mut GSeekable = stream as *mut ::core::ffi::c_void as *mut GSeekable;
        let mut start: goffset = 0;
        let mut end: goffset = 0;
        let mut success: gboolean = 0;
        (*(*stream).priv_0).set_pending(FALSE as guint as guint);
        start = g_seekable_tell(seekable);
        if g_seekable_seek(
            stream as *mut ::core::ffi::c_void as *mut GSeekable,
            0 as goffset,
            G_SEEK_END,
            cancellable,
            ::core::ptr::null_mut::<*mut GError>(),
        ) != 0
        {
            end = g_seekable_tell(seekable);
            if ({
                let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
                if start >= 0 as goffset {
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
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/ginputstream.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    432 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"start >= 0\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if ({
                let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
                if end >= start {
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
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/ginputstream.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    433 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"end >= start\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if start > (G_MAXOFFSET as gsize).wrapping_sub(count) as goffset
                || (start as gsize).wrapping_add(count) as goffset > end
            {
                (*(*stream).priv_0).set_pending(TRUE as guint as guint);
                return end as gssize - start as gssize;
            }
            success = g_seekable_seek(
                stream as *mut ::core::ffi::c_void as *mut GSeekable,
                (start as gsize).wrapping_add(count) as goffset,
                G_SEEK_SET,
                cancellable,
                error,
            );
            (*(*stream).priv_0).set_pending(TRUE as guint as guint);
            if success != 0 {
                return count as gssize;
            } else {
                return -(1 as ::core::ffi::c_int) as gssize;
            }
        }
    }
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GInputStreamClass;
    read_bytes = 0 as gssize;
    loop {
        my_error = ::core::ptr::null_mut::<GError>();
        ret = (*class).read_fn.expect("non-null function pointer")(
            stream,
            &raw mut buffer as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            if (::core::mem::size_of::<[::core::ffi::c_char; 8192]>() as usize) < count as usize {
                ::core::mem::size_of::<[::core::ffi::c_char; 8192]>() as gsize
            } else {
                count
            },
            cancellable,
            &raw mut my_error,
        );
        if ret == -(1 as ::core::ffi::c_int) as gssize {
            if read_bytes > 0 as gssize
                && (*my_error).domain == g_io_error_quark()
                && (*my_error).code == G_IO_ERROR_CANCELLED as ::core::ffi::c_int
            {
                g_error_free(my_error);
                return read_bytes;
            }
            g_propagate_error(error, my_error);
            return -(1 as ::core::ffi::c_int) as gssize;
        }
        count = count.wrapping_sub(ret as gsize);
        read_bytes += ret;
        if ret == 0 as gssize || count == 0 as gsize {
            return read_bytes;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_input_stream_close(
    mut stream: *mut GInputStream,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut class: *mut GInputStreamClass = ::core::ptr::null_mut::<GInputStreamClass>();
    let mut res: gboolean = 0;
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_input_stream_get_type();
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
            b"G_IS_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GInputStreamClass;
    if (*(*stream).priv_0).closed() != 0 {
        return TRUE;
    }
    res = TRUE as gboolean;
    if safe_c2rust_g_input_stream_set_pending(stream, error) == 0 {
        return FALSE;
    }
    if !cancellable.is_null() {
        g_cancellable_push_current(cancellable);
    }
    if (*class).close_fn.is_some() {
        res = (*class).close_fn.expect("non-null function pointer")(stream, cancellable, error);
    }
    if !cancellable.is_null() {
        g_cancellable_pop_current(cancellable);
    }
    safe_c2rust_g_input_stream_clear_pending(stream);
    (*(*stream).priv_0).set_closed(TRUE as guint as guint);
    return res;
}
unsafe extern "C" fn safe_c2rust_async_ready_callback_wrapper(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut stream: *mut GInputStream =
        source_object as *mut ::core::ffi::c_void as *mut GInputStream;
    safe_c2rust_g_input_stream_clear_pending(stream);
    if (*(*stream).priv_0).outstanding_callback.is_some() {
        Some(
            (*(*stream).priv_0)
                .outstanding_callback
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(source_object, res, user_data);
    }
    g_object_unref(stream as gpointer);
}
unsafe extern "C" fn safe_c2rust_async_ready_close_callback_wrapper(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut stream: *mut GInputStream =
        source_object as *mut ::core::ffi::c_void as *mut GInputStream;
    safe_c2rust_g_input_stream_clear_pending(stream);
    (*(*stream).priv_0).set_closed(TRUE as guint as guint);
    if (*(*stream).priv_0).outstanding_callback.is_some() {
        Some(
            (*(*stream).priv_0)
                .outstanding_callback
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(source_object, res, user_data);
    }
    g_object_unref(stream as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_input_stream_read_async(
    mut stream: *mut GInputStream,
    mut buffer: *mut ::core::ffi::c_void,
    mut count: gsize,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut class: *mut GInputStreamClass = ::core::ptr::null_mut::<GInputStreamClass>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_input_stream_get_type();
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
            b"G_IS_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !buffer.is_null() {
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
            b"buffer != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if count == 0 as gsize {
        let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
        task = g_task_new(stream as gpointer, cancellable, callback, user_data);
        let mut _task: *mut GTask = task;
        g_task_set_source_tag(
            _task,
            ::core::mem::transmute::<
                Option<
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
                gpointer,
            >(Some(
                safe_c2rust_g_input_stream_read_async
                    as unsafe extern "C" fn(
                        *mut GInputStream,
                        *mut ::core::ffi::c_void,
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
                b"g_input_stream_read_async\0" as *const u8 as *const gchar,
            );
        }
        g_task_return_int(task, 0 as gssize);
        g_object_unref(task as gpointer);
        return;
    }
    if (count as gssize) < 0 as gssize {
        g_task_report_new_error(
            stream as gpointer,
            callback,
            user_data,
            ::core::mem::transmute::<
                Option<
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
                gpointer,
            >(Some(
                safe_c2rust_g_input_stream_read_async
                    as unsafe extern "C" fn(
                        *mut GInputStream,
                        *mut ::core::ffi::c_void,
                        gsize,
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
    if safe_c2rust_g_input_stream_set_pending(stream, &raw mut error) == 0 {
        g_task_report_error(
            stream as gpointer,
            callback,
            user_data,
            ::core::mem::transmute::<
                Option<
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
                gpointer,
            >(Some(
                safe_c2rust_g_input_stream_read_async
                    as unsafe extern "C" fn(
                        *mut GInputStream,
                        *mut ::core::ffi::c_void,
                        gsize,
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
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GInputStreamClass;
    (*(*stream).priv_0).outstanding_callback = callback;
    g_object_ref(stream as gpointer);
    (*class).read_async.expect("non-null function pointer")(
        stream,
        buffer,
        count,
        io_priority,
        cancellable,
        Some(
            safe_c2rust_async_ready_callback_wrapper
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_input_stream_read_finish(
    mut stream: *mut GInputStream,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gssize {
    let mut class: *mut GInputStreamClass = ::core::ptr::null_mut::<GInputStreamClass>();
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_input_stream_get_type();
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
            b"G_IS_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
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
                    *mut GInputStream,
                    *mut ::core::ffi::c_void,
                    gsize,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_input_stream_read_async
                as unsafe extern "C" fn(
                    *mut GInputStream,
                    *mut ::core::ffi::c_void,
                    gsize,
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
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GInputStreamClass;
    return (*class).read_finish.expect("non-null function pointer")(stream, result, error);
}
unsafe extern "C" fn safe_c2rust_free_async_read_all(mut data: gpointer) {
    g_slice_free1(::core::mem::size_of::<AsyncReadAll>() as gsize, data);
}
unsafe extern "C" fn safe_c2rust_read_all_callback(
    mut stream: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut data: *mut AsyncReadAll = g_task_get_task_data(task) as *mut AsyncReadAll;
    let mut got_eof: gboolean = FALSE;
    if !result.is_null() {
        let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
        let mut nread: gssize = 0;
        nread = safe_c2rust_g_input_stream_read_finish(
            stream as *mut ::core::ffi::c_void as *mut GInputStream,
            result,
            &raw mut error,
        );
        if nread == -(1 as ::core::ffi::c_int) as gssize {
            g_task_return_error(task, error);
            g_object_unref(task as gpointer);
            return;
        }
        let mut __n1: gint64 = nread as gint64;
        let mut __n2: gint64 = (*data).to_read as gint64;
        if !(__n1 <= __n2) {
            g_assertion_message_cmpint(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/ginputstream.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                737 as ::core::ffi::c_int,
                G_STRFUNC,
                b"nread <= data->to_read\0" as *const u8 as *const ::core::ffi::c_char,
                __n1 as guint64,
                b"<=\0" as *const u8 as *const ::core::ffi::c_char,
                __n2 as guint64,
                'i' as i32 as ::core::ffi::c_char,
            );
        }
        (*data).to_read = (*data).to_read.wrapping_sub(nread as gsize);
        (*data).bytes_read = (*data).bytes_read.wrapping_add(nread as gsize);
        got_eof = (nread == 0 as gssize) as ::core::ffi::c_int as gboolean;
    }
    if got_eof != 0 || (*data).to_read == 0 as gsize {
        g_task_return_boolean(task, TRUE);
        g_object_unref(task as gpointer);
    } else {
        safe_c2rust_g_input_stream_read_async(
            stream as *mut ::core::ffi::c_void as *mut GInputStream,
            (*data).buffer.offset((*data).bytes_read as isize) as *mut ::core::ffi::c_void,
            (*data).to_read,
            g_task_get_priority(task) as ::core::ffi::c_int,
            g_task_get_cancellable(task),
            Some(
                safe_c2rust_read_all_callback
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            task as gpointer,
        );
    };
}
unsafe extern "C" fn safe_c2rust_read_all_async_thread(
    mut task: *mut GTask,
    mut source_object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut stream: *mut GInputStream = source_object as *mut GInputStream;
    let mut data: *mut AsyncReadAll = task_data as *mut AsyncReadAll;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if safe_c2rust_g_input_stream_read_all(
        stream,
        (*data).buffer as *mut ::core::ffi::c_void,
        (*data).to_read,
        &raw mut (*data).bytes_read,
        g_task_get_cancellable(task),
        &raw mut error,
    ) != 0
    {
        g_task_return_boolean(task, TRUE);
    } else {
        g_task_return_error(task, error);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_input_stream_read_all_async(
    mut stream: *mut GInputStream,
    mut buffer: *mut ::core::ffi::c_void,
    mut count: gsize,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut data: *mut AsyncReadAll = ::core::ptr::null_mut::<AsyncReadAll>();
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_input_stream_get_type();
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
            b"G_IS_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if !buffer.is_null() || count == 0 as gsize {
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
            b"buffer != NULL || count == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    task = g_task_new(stream as gpointer, cancellable, callback, user_data);
    data = ({
        let mut __s: gsize = ::core::mem::size_of::<AsyncReadAll>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut AsyncReadAll;
    (*data).buffer = buffer as *mut gchar;
    (*data).to_read = count;
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
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
            gpointer,
        >(Some(
            safe_c2rust_g_input_stream_read_all_async
                as unsafe extern "C" fn(
                    *mut GInputStream,
                    *mut ::core::ffi::c_void,
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
            b"g_input_stream_read_all_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        data as gpointer,
        Some(safe_c2rust_free_async_read_all as unsafe extern "C" fn(gpointer) -> ()),
    );
    g_task_set_priority(task, io_priority as gint);
    if safe_c2rust_g_input_stream_async_read_is_via_threads(stream) != 0 {
        g_task_run_in_thread(
            task,
            Some(
                safe_c2rust_read_all_async_thread
                    as unsafe extern "C" fn(
                        *mut GTask,
                        gpointer,
                        gpointer,
                        *mut GCancellable,
                    ) -> (),
            ),
        );
        g_object_unref(task as gpointer);
    } else {
        safe_c2rust_read_all_callback(
            stream as *mut ::core::ffi::c_void as *mut GObject,
            ::core::ptr::null_mut::<GAsyncResult>(),
            task as gpointer,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_input_stream_read_all_finish(
    mut stream: *mut GInputStream,
    mut result: *mut GAsyncResult,
    mut bytes_read: *mut gsize,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_input_stream_get_type();
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
            _g_boolean_var_24 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_24 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_24
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, stream as gpointer) != 0 {
            _g_boolean_var_25 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_25 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_25
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
    task = result as *mut ::core::ffi::c_void as *mut GTask;
    if !bytes_read.is_null() {
        let mut data: *mut AsyncReadAll = g_task_get_task_data(task) as *mut AsyncReadAll;
        *bytes_read = (*data).bytes_read;
    }
    return g_task_propagate_boolean(task, error);
}
unsafe extern "C" fn safe_c2rust_read_bytes_callback(
    mut stream: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut buf: *mut guchar = g_task_get_task_data(task) as *mut guchar;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut nread: gssize = 0;
    let mut bytes: *mut GBytes = ::core::ptr::null_mut::<GBytes>();
    nread = safe_c2rust_g_input_stream_read_finish(
        stream as *mut ::core::ffi::c_void as *mut GInputStream,
        result,
        &raw mut error,
    );
    if nread == -(1 as ::core::ffi::c_int) as gssize {
        g_free(buf as gpointer);
        g_task_return_error(task, error);
    } else if nread == 0 as gssize {
        g_free(buf as gpointer);
        bytes = g_bytes_new_static(
            b"\0" as *const u8 as *const ::core::ffi::c_char as gconstpointer,
            0 as gsize,
        );
    } else {
        bytes = g_bytes_new_take(buf as gpointer, nread as gsize);
    }
    if !bytes.is_null() {
        g_task_return_pointer(
            task,
            bytes as gpointer,
            ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut GBytes) -> ()>, GDestroyNotify>(
                Some(g_bytes_unref as unsafe extern "C" fn(*mut GBytes) -> ()),
            ),
        );
    }
    g_object_unref(task as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_input_stream_read_bytes_async(
    mut stream: *mut GInputStream,
    mut count: gsize,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut buf: *mut guchar = ::core::ptr::null_mut::<guchar>();
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
            safe_c2rust_g_input_stream_read_bytes_async
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
            b"g_input_stream_read_bytes_async\0" as *const u8 as *const gchar,
        );
    }
    buf = g_malloc(count) as *mut guchar;
    g_task_set_task_data(task, buf as gpointer, None);
    safe_c2rust_g_input_stream_read_async(
        stream,
        buf as *mut ::core::ffi::c_void,
        count,
        io_priority,
        cancellable,
        Some(
            safe_c2rust_read_bytes_callback
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        task as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_input_stream_read_bytes_finish(
    mut stream: *mut GInputStream,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GBytes {
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_input_stream_get_type();
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
            b"G_IS_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBytes>();
    }
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, stream as gpointer) != 0 {
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
            b"g_task_is_valid (result, stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBytes>();
    }
    return g_task_propagate_pointer(result as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GBytes;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_input_stream_skip_async(
    mut stream: *mut GInputStream,
    mut count: gsize,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut class: *mut GInputStreamClass = ::core::ptr::null_mut::<GInputStreamClass>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_input_stream_get_type();
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
            b"G_IS_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if count == 0 as gsize {
        let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
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
                safe_c2rust_g_input_stream_skip_async
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
                b"g_input_stream_skip_async\0" as *const u8 as *const gchar,
            );
        }
        g_task_return_int(task, 0 as gssize);
        g_object_unref(task as gpointer);
        return;
    }
    if (count as gssize) < 0 as gssize {
        g_task_report_new_error(
            stream as gpointer,
            callback,
            user_data,
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
                safe_c2rust_g_input_stream_skip_async
                    as unsafe extern "C" fn(
                        *mut GInputStream,
                        gsize,
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
    if safe_c2rust_g_input_stream_set_pending(stream, &raw mut error) == 0 {
        g_task_report_error(
            stream as gpointer,
            callback,
            user_data,
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
                safe_c2rust_g_input_stream_skip_async
                    as unsafe extern "C" fn(
                        *mut GInputStream,
                        gsize,
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
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GInputStreamClass;
    (*(*stream).priv_0).outstanding_callback = callback;
    g_object_ref(stream as gpointer);
    (*class).skip_async.expect("non-null function pointer")(
        stream,
        count,
        io_priority,
        cancellable,
        Some(
            safe_c2rust_async_ready_callback_wrapper
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_input_stream_skip_finish(
    mut stream: *mut GInputStream,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gssize {
    let mut class: *mut GInputStreamClass = ::core::ptr::null_mut::<GInputStreamClass>();
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_input_stream_get_type();
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
            b"G_IS_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
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
            safe_c2rust_g_input_stream_skip_async
                as unsafe extern "C" fn(
                    *mut GInputStream,
                    gsize,
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
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GInputStreamClass;
    return (*class).skip_finish.expect("non-null function pointer")(stream, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_input_stream_close_async(
    mut stream: *mut GInputStream,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut class: *mut GInputStreamClass = ::core::ptr::null_mut::<GInputStreamClass>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_input_stream_get_type();
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
            b"G_IS_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*(*stream).priv_0).closed() != 0 {
        let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
        task = g_task_new(stream as gpointer, cancellable, callback, user_data);
        let mut _task: *mut GTask = task;
        g_task_set_source_tag(
            _task,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GInputStream,
                        ::core::ffi::c_int,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_input_stream_close_async
                    as unsafe extern "C" fn(
                        *mut GInputStream,
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
                b"g_input_stream_close_async\0" as *const u8 as *const gchar,
            );
        }
        g_task_return_boolean(task, TRUE);
        g_object_unref(task as gpointer);
        return;
    }
    if safe_c2rust_g_input_stream_set_pending(stream, &raw mut error) == 0 {
        g_task_report_error(
            stream as gpointer,
            callback,
            user_data,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GInputStream,
                        ::core::ffi::c_int,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_input_stream_close_async
                    as unsafe extern "C" fn(
                        *mut GInputStream,
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
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GInputStreamClass;
    (*(*stream).priv_0).outstanding_callback = callback;
    g_object_ref(stream as gpointer);
    (*class).close_async.expect("non-null function pointer")(
        stream,
        io_priority,
        cancellable,
        Some(
            safe_c2rust_async_ready_close_callback_wrapper
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_input_stream_close_finish(
    mut stream: *mut GInputStream,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut class: *mut GInputStreamClass = ::core::ptr::null_mut::<GInputStreamClass>();
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_input_stream_get_type();
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
            b"G_IS_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
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
            b"G_IS_ASYNC_RESULT (result)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if g_async_result_legacy_propagate_error(result, error) != 0 {
        return FALSE;
    } else if g_async_result_is_tagged(
        result,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GInputStream,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_input_stream_close_async
                as unsafe extern "C" fn(
                    *mut GInputStream,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    ) != 0
    {
        return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
    }
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GInputStreamClass;
    return (*class).close_finish.expect("non-null function pointer")(stream, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_input_stream_is_closed(
    mut stream: *mut GInputStream,
) -> gboolean {
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_input_stream_get_type();
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
            _g_boolean_var_34 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_34 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_34
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    return (*(*stream).priv_0).closed() as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_input_stream_has_pending(
    mut stream: *mut GInputStream,
) -> gboolean {
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_input_stream_get_type();
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
            b"G_IS_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    return (*(*stream).priv_0).pending() as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_input_stream_set_pending(
    mut stream: *mut GInputStream,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_input_stream_get_type();
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
            b"G_IS_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if (*(*stream).priv_0).closed() != 0 {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_CLOSED as ::core::ffi::c_int as gint,
            glib_gettext(b"Stream is already closed\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    if (*(*stream).priv_0).pending() != 0 {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_PENDING as ::core::ffi::c_int as gint,
            glib_gettext(b"Stream has outstanding operation\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    (*(*stream).priv_0).set_pending(TRUE as guint as guint);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_input_stream_clear_pending(mut stream: *mut GInputStream) {
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_input_stream_get_type();
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
            b"G_IS_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*(*stream).priv_0).set_pending(FALSE as guint as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_input_stream_async_read_is_via_threads(
    mut stream: *mut GInputStream,
) -> gboolean {
    let mut class: *mut GInputStreamClass = ::core::ptr::null_mut::<GInputStreamClass>();
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_input_stream_get_type();
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
            b"G_IS_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GInputStreamClass;
    return ((*class).read_async
        == Some(
            safe_c2rust_g_input_stream_real_read_async
                as unsafe extern "C" fn(
                    *mut GInputStream,
                    *mut ::core::ffi::c_void,
                    gsize,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )
        && !(({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = g_pollable_input_stream_get_type();
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
            && g_pollable_input_stream_can_poll(
                stream as *mut ::core::ffi::c_void as *mut GPollableInputStream,
            ) != 0)) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_input_stream_async_close_is_via_threads(
    mut stream: *mut GInputStream,
) -> gboolean {
    let mut class: *mut GInputStreamClass = ::core::ptr::null_mut::<GInputStreamClass>();
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_input_stream_get_type();
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
            _g_boolean_var_39 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_39 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_39
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GInputStreamClass;
    return ((*class).close_async
        == Some(
            safe_c2rust_g_input_stream_real_close_async
                as unsafe extern "C" fn(
                    *mut GInputStream,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_free_read_data(mut op: *mut ReadData) {
    g_slice_free1(::core::mem::size_of::<ReadData>() as gsize, op as gpointer);
}
unsafe extern "C" fn safe_c2rust_read_async_thread(
    mut task: *mut GTask,
    mut source_object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut stream: *mut GInputStream = source_object as *mut GInputStream;
    let mut op: *mut ReadData = task_data as *mut ReadData;
    let mut class: *mut GInputStreamClass = ::core::ptr::null_mut::<GInputStreamClass>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut nread: gssize = 0;
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GInputStreamClass;
    nread = (*class).read_fn.expect("non-null function pointer")(
        stream,
        (*op).buffer,
        (*op).count,
        g_task_get_cancellable(task),
        &raw mut error,
    );
    if nread == -(1 as ::core::ffi::c_int) as gssize {
        g_task_return_error(task, error);
    } else {
        g_task_return_int(task, nread);
    };
}
unsafe extern "C" fn safe_c2rust_read_async_pollable_ready(
    mut stream: *mut GPollableInputStream,
    mut user_data: gpointer,
) -> gboolean {
    let mut task: *mut GTask = user_data as *mut GTask;
    safe_c2rust_read_async_pollable(stream, task);
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_read_async_pollable(
    mut stream: *mut GPollableInputStream,
    mut task: *mut GTask,
) {
    let mut op: *mut ReadData = g_task_get_task_data(task) as *mut ReadData;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut nread: gssize = 0;
    if g_task_return_error_if_cancelled(task) != 0 {
        return;
    }
    nread = (*(g_type_interface_peek(
        (*(stream as *mut GTypeInstance)).g_class as gpointer,
        g_pollable_input_stream_get_type(),
    ) as *mut GPollableInputStreamInterface))
        .read_nonblocking
        .expect("non-null function pointer")(
        stream, (*op).buffer, (*op).count, &raw mut error
    );
    if g_error_matches(
        error,
        g_io_error_quark(),
        G_IO_ERROR_WOULD_BLOCK as ::core::ffi::c_int as gint,
    ) != 0
    {
        let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
        g_error_free(error);
        source = g_pollable_input_stream_create_source(stream, g_task_get_cancellable(task));
        g_task_attach_source(
            task,
            source,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GPollableInputStream, gpointer) -> gboolean>,
                GSourceFunc,
            >(Some(
                safe_c2rust_read_async_pollable_ready
                    as unsafe extern "C" fn(*mut GPollableInputStream, gpointer) -> gboolean,
            )),
        );
        g_source_unref(source);
        return;
    }
    if nread == -(1 as ::core::ffi::c_int) as gssize {
        g_task_return_error(task, error);
    } else {
        g_task_return_int(task, nread);
    };
}
unsafe extern "C" fn safe_c2rust_g_input_stream_real_read_async(
    mut stream: *mut GInputStream,
    mut buffer: *mut ::core::ffi::c_void,
    mut count: gsize,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut op: *mut ReadData = ::core::ptr::null_mut::<ReadData>();
    op = ({
        let mut __s: gsize = ::core::mem::size_of::<ReadData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut ReadData;
    task = g_task_new(stream as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
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
            gpointer,
        >(Some(
            safe_c2rust_g_input_stream_real_read_async
                as unsafe extern "C" fn(
                    *mut GInputStream,
                    *mut ::core::ffi::c_void,
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
            b"g_input_stream_real_read_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        op as gpointer,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut ReadData) -> ()>, GDestroyNotify>(
            Some(safe_c2rust_free_read_data as unsafe extern "C" fn(*mut ReadData) -> ()),
        ),
    );
    g_task_set_priority(task, io_priority as gint);
    (*op).buffer = buffer;
    (*op).count = count;
    if safe_c2rust_g_input_stream_async_read_is_via_threads(stream) == 0 {
        safe_c2rust_read_async_pollable(
            stream as *mut ::core::ffi::c_void as *mut GPollableInputStream,
            task,
        );
    } else {
        g_task_run_in_thread(
            task,
            Some(
                safe_c2rust_read_async_thread
                    as unsafe extern "C" fn(
                        *mut GTask,
                        gpointer,
                        gpointer,
                        *mut GCancellable,
                    ) -> (),
            ),
        );
    }
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_input_stream_real_read_finish(
    mut stream: *mut GInputStream,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gssize {
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, stream as gpointer) != 0 {
            _g_boolean_var_40 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_40 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_40
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
unsafe extern "C" fn safe_c2rust_skip_async_thread(
    mut task: *mut GTask,
    mut source_object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut stream: *mut GInputStream = source_object as *mut GInputStream;
    let mut count: gsize = task_data as gsize;
    let mut class: *mut GInputStreamClass = ::core::ptr::null_mut::<GInputStreamClass>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut ret: gssize = 0;
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GInputStreamClass;
    ret = (*class).skip.expect("non-null function pointer")(
        stream,
        count,
        g_task_get_cancellable(task),
        &raw mut error,
    );
    if ret == -(1 as ::core::ffi::c_int) as gssize {
        g_task_return_error(task, error);
    } else {
        g_task_return_int(task, ret);
    };
}
unsafe extern "C" fn safe_c2rust_skip_callback_wrapper(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut class: *mut GInputStreamClass = ::core::ptr::null_mut::<GInputStreamClass>();
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut data: *mut SkipFallbackAsyncData =
        g_task_get_task_data(task) as *mut SkipFallbackAsyncData;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut ret: gssize = 0;
    ret = safe_c2rust_g_input_stream_read_finish(
        source_object as *mut ::core::ffi::c_void as *mut GInputStream,
        res,
        &raw mut error,
    );
    if ret > 0 as gssize {
        (*data).count = (*data).count.wrapping_sub(ret as gsize);
        (*data).count_skipped = (*data).count_skipped.wrapping_add(ret as gsize);
        if (*data).count > 0 as gsize {
            class = (*(source_object as *mut GTypeInstance)).g_class as *mut GInputStreamClass;
            (*class).read_async.expect("non-null function pointer")(
                source_object as *mut ::core::ffi::c_void as *mut GInputStream,
                &raw mut (*data).buffer as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                if (8192 as gsize) < (*data).count {
                    8192 as gsize
                } else {
                    (*data).count
                },
                g_task_get_priority(task) as ::core::ffi::c_int,
                g_task_get_cancellable(task),
                Some(
                    safe_c2rust_skip_callback_wrapper
                        as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
                ),
                task as gpointer,
            );
            return;
        }
    }
    if ret == -(1 as ::core::ffi::c_int) as gssize
        && g_error_matches(
            error,
            g_io_error_quark(),
            G_IO_ERROR_CANCELLED as ::core::ffi::c_int as gint,
        ) != 0
        && (*data).count_skipped != 0
    {
        g_clear_error(&raw mut error);
    }
    if !error.is_null() {
        g_task_return_error(task, error);
    } else {
        g_task_return_int(task, (*data).count_skipped as gssize);
    }
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_input_stream_real_skip_async(
    mut stream: *mut GInputStream,
    mut count: gsize,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut class: *mut GInputStreamClass = ::core::ptr::null_mut::<GInputStreamClass>();
    let mut data: *mut SkipFallbackAsyncData = ::core::ptr::null_mut::<SkipFallbackAsyncData>();
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GInputStreamClass;
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
            safe_c2rust_g_input_stream_real_skip_async
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
            b"g_input_stream_real_skip_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_priority(task, io_priority as gint);
    if safe_c2rust_g_input_stream_async_read_is_via_threads(stream) != 0 {
        g_task_set_task_data(task, count as gpointer, None);
        g_task_run_in_thread(
            task,
            Some(
                safe_c2rust_skip_async_thread
                    as unsafe extern "C" fn(
                        *mut GTask,
                        gpointer,
                        gpointer,
                        *mut GCancellable,
                    ) -> (),
            ),
        );
        g_object_unref(task as gpointer);
    } else {
        data = ({
            let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
            let mut __s: gsize = ::core::mem::size_of::<SkipFallbackAsyncData>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc_n(__n, __s);
            }
            __p
        }) as *mut SkipFallbackAsyncData;
        (*data).count = count;
        (*data).count_skipped = 0 as gsize;
        g_task_set_task_data(
            task,
            data as gpointer,
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        );
        g_task_set_check_cancellable(task, FALSE);
        (*class).read_async.expect("non-null function pointer")(
            stream,
            &raw mut (*data).buffer as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            if (8192 as gsize) < count {
                8192 as gsize
            } else {
                count
            },
            io_priority,
            cancellable,
            Some(
                safe_c2rust_skip_callback_wrapper
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            task as gpointer,
        );
    };
}
unsafe extern "C" fn safe_c2rust_g_input_stream_real_skip_finish(
    mut stream: *mut GInputStream,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gssize {
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, stream as gpointer) != 0 {
            _g_boolean_var_41 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_41 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_41
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
unsafe extern "C" fn safe_c2rust_close_async_thread(
    mut task: *mut GTask,
    mut source_object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut stream: *mut GInputStream = source_object as *mut GInputStream;
    let mut class: *mut GInputStreamClass = ::core::ptr::null_mut::<GInputStreamClass>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut result: gboolean = 0;
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GInputStreamClass;
    if (*class).close_fn.is_some() {
        result = (*class).close_fn.expect("non-null function pointer")(
            stream,
            g_task_get_cancellable(task),
            &raw mut error,
        );
        if result == 0 {
            g_task_return_error(task, error);
            return;
        }
    }
    g_task_return_boolean(task, TRUE);
}
unsafe extern "C" fn safe_c2rust_g_input_stream_real_close_async(
    mut stream: *mut GInputStream,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    task = g_task_new(stream as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GInputStream,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_input_stream_real_close_async
                as unsafe extern "C" fn(
                    *mut GInputStream,
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
            b"g_input_stream_real_close_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_check_cancellable(task, FALSE);
    g_task_set_priority(task, io_priority as gint);
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_close_async_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_input_stream_real_close_finish(
    mut stream: *mut GInputStream,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, stream as gpointer) != 0 {
            _g_boolean_var_42 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_42 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_42
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
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
