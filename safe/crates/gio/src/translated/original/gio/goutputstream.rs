use ::c2rust_bitfields;
extern "C" {
    pub type _GBytes;
    pub type _GData;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GAsyncResult;
    pub type _GInputStreamPrivate;
    pub type _GCancellablePrivate;
    pub type _GPollableOutputStream;
    pub type _GTask;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
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
    fn g_bytes_get_data(bytes: *mut GBytes, size: *mut gsize) -> gconstpointer;
    fn g_bytes_ref(bytes: *mut GBytes) -> *mut GBytes;
    fn g_bytes_unref(bytes: *mut GBytes);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_source_unref(source: *mut GSource);
    fn g_strdup_vprintf(format: *const gchar, args: ::core::ffi::VaList) -> *mut gchar;
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
    fn g_cancellable_get_type() -> GType;
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
    fn g_task_set_task_data(
        task: *mut GTask,
        task_data: gpointer,
        task_data_destroy: GDestroyNotify,
    );
    fn g_task_set_priority(task: *mut GTask, priority: gint);
    fn g_task_set_check_cancellable(task: *mut GTask, check_cancellable: gboolean);
    fn g_task_set_source_tag(task: *mut GTask, source_tag: gpointer);
    fn g_task_set_static_name(task: *mut GTask, name: *const gchar);
    fn g_task_get_source_object(task: *mut GTask) -> gpointer;
    fn g_task_get_task_data(task: *mut GTask) -> gpointer;
    fn g_task_get_priority(task: *mut GTask) -> gint;
    fn g_task_get_cancellable(task: *mut GTask) -> *mut GCancellable;
    fn g_task_get_name(task: *mut GTask) -> *const gchar;
    fn g_task_is_valid(result: gpointer, source_object: gpointer) -> gboolean;
    fn g_task_run_in_thread(task: *mut GTask, task_func: GTaskThreadFunc);
    fn g_task_attach_source(task: *mut GTask, source: *mut GSource, callback: GSourceFunc);
    fn g_task_return_boolean(task: *mut GTask, result: gboolean);
    fn g_task_return_int(task: *mut GTask, result: gssize);
    fn g_task_return_error(task: *mut GTask, error: *mut GError);
    fn g_task_return_new_error(
        task: *mut GTask,
        domain: GQuark,
        code: gint,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn g_task_return_new_error_literal(
        task: *mut GTask,
        domain: GQuark,
        code: gint,
        message: *const ::core::ffi::c_char,
    );
    fn g_task_return_error_if_cancelled(task: *mut GTask) -> gboolean;
    fn g_task_propagate_boolean(task: *mut GTask, error: *mut *mut GError) -> gboolean;
    fn g_task_propagate_int(task: *mut GTask, error: *mut *mut GError) -> gssize;
    fn g_input_stream_get_type() -> GType;
    fn g_input_stream_read(
        stream: *mut GInputStream,
        buffer: *mut ::core::ffi::c_void,
        count: gsize,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gssize;
    fn g_input_stream_close(
        stream: *mut GInputStream,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
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
    fn g_input_stream_close_async(
        stream: *mut GInputStream,
        io_priority: ::core::ffi::c_int,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_input_stream_close_finish(
        stream: *mut GInputStream,
        result: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_input_stream_is_closed(stream: *mut GInputStream) -> gboolean;
    fn g_io_error_quark() -> GQuark;
    fn g_input_stream_async_read_is_via_threads(stream: *mut GInputStream) -> gboolean;
    fn glib_gettext(str: *const gchar) -> *const gchar;
    fn g_pollable_output_stream_get_type() -> GType;
    fn g_pollable_output_stream_can_poll(stream: *mut GPollableOutputStream) -> gboolean;
    fn g_pollable_output_stream_create_source(
        stream: *mut GPollableOutputStream,
        cancellable: *mut GCancellable,
    ) -> *mut GSource;
}
pub type __builtin_va_list = [__va_list_tag; 1];
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
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gfloat = ::core::ffi::c_float;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GBytes = _GBytes;
pub type va_list = __builtin_va_list;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GOutputStreamPrivate {
    #[bitfield(name = "closed", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "pending", ty = "guint", bits = "1..=1")]
    #[bitfield(name = "closing", ty = "guint", bits = "2..=2")]
    pub closed_pending_closing: [u8; 1],
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
pub type GPollableOutputStream = _GPollableOutputStream;
pub type GTask = _GTask;
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
pub type GTaskThreadFunc =
    Option<unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> ()>;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct SpliceData {
    pub source: *mut GInputStream,
    pub flags: GOutputStreamSpliceFlags,
    #[bitfield(name = "istream_closed", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "ostream_closed", ty = "guint", bits = "1..=1")]
    pub istream_closed_ostream_closed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub n_read: gssize,
    pub n_written: gssize,
    pub bytes_copied: gsize,
    pub error: *mut GError,
    pub buffer: *mut guint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WriteData {
    pub buffer: *const ::core::ffi::c_void,
    pub count_requested: gsize,
    pub count_written: gssize,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WritevData {
    pub vectors: *const GOutputVector,
    pub n_vectors: gsize,
    pub bytes_written: gsize,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AsyncWriteAll {
    pub buffer: *const guint8,
    pub to_write: gsize,
    pub bytes_written: gsize,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AsyncWritevAll {
    pub vectors: *mut GOutputVector,
    pub n_vectors: gsize,
    pub bytes_written: gsize,
}
pub const LONG_MAX: ::core::ffi::c_long = __LONG_MAX__;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXLONG: ::core::ffi::c_long = LONG_MAX;
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const G_MAXSSIZE: ::core::ffi::c_long = G_MAXLONG;
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
unsafe extern "C" fn safe_c2rust_g_output_stream_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_output_stream_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GOutputStream_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GOutputStream_private_offset,
        );
    }
    safe_c2rust_g_output_stream_class_init(klass as *mut GOutputStreamClass);
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_output_stream_get_instance_private(
    mut self_0: *mut GOutputStream,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GOutputStream_private_offset as glong as isize) as gpointer;
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_output_stream_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GOutputStream\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GOutputStreamClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_output_stream_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GOutputStream>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GOutputStream) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_output_stream_init as unsafe extern "C" fn(*mut GOutputStream) -> (),
            )),
        ),
        G_TYPE_FLAG_ABSTRACT,
    );
    safe_c2rust_GOutputStream_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GOutputStreamPrivate>() as gsize,
    );
    return g_define_type_id;
}
static mut safe_c2rust_g_output_stream_parent_class: gpointer = NULL_0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_output_stream_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_output_stream_get_type_once();
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
static mut safe_c2rust_GOutputStream_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_output_stream_dispose(mut object: *mut GObject) {
    let mut stream: *mut GOutputStream = ::core::ptr::null_mut::<GOutputStream>();
    stream = object as *mut ::core::ffi::c_void as *mut GOutputStream;
    if (*(*stream).priv_0).closed() == 0 {
        safe_c2rust_g_output_stream_close(
            stream,
            ::core::ptr::null_mut::<GCancellable>(),
            ::core::ptr::null_mut::<*mut GError>(),
        );
    }
    (*(safe_c2rust_g_output_stream_parent_class as *mut GObjectClass))
        .dispose
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_output_stream_class_init(mut klass: *mut GOutputStreamClass) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).dispose =
        Some(safe_c2rust_g_output_stream_dispose as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*klass).splice = Some(
        safe_c2rust_g_output_stream_real_splice
            as unsafe extern "C" fn(
                *mut GOutputStream,
                *mut GInputStream,
                GOutputStreamSpliceFlags,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gssize,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GOutputStream,
                *mut GInputStream,
                GOutputStreamSpliceFlags,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gssize,
        >;
    (*klass).write_async = Some(
        safe_c2rust_g_output_stream_real_write_async
            as unsafe extern "C" fn(
                *mut GOutputStream,
                *const ::core::ffi::c_void,
                gsize,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GOutputStream,
                *const ::core::ffi::c_void,
                gsize,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*klass).write_finish = Some(
        safe_c2rust_g_output_stream_real_write_finish
            as unsafe extern "C" fn(
                *mut GOutputStream,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> gssize,
    )
        as Option<
            unsafe extern "C" fn(*mut GOutputStream, *mut GAsyncResult, *mut *mut GError) -> gssize,
        >;
    (*klass).writev_fn = Some(
        safe_c2rust_g_output_stream_real_writev
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
    (*klass).writev_async = Some(
        safe_c2rust_g_output_stream_real_writev_async
            as unsafe extern "C" fn(
                *mut GOutputStream,
                *const GOutputVector,
                gsize,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GOutputStream,
                *const GOutputVector,
                gsize,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*klass).writev_finish = Some(
        safe_c2rust_g_output_stream_real_writev_finish
            as unsafe extern "C" fn(
                *mut GOutputStream,
                *mut GAsyncResult,
                *mut gsize,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GOutputStream,
                *mut GAsyncResult,
                *mut gsize,
                *mut *mut GError,
            ) -> gboolean,
        >;
    (*klass).splice_async = Some(
        safe_c2rust_g_output_stream_real_splice_async
            as unsafe extern "C" fn(
                *mut GOutputStream,
                *mut GInputStream,
                GOutputStreamSpliceFlags,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GOutputStream,
                *mut GInputStream,
                GOutputStreamSpliceFlags,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*klass).splice_finish = Some(
        safe_c2rust_g_output_stream_real_splice_finish
            as unsafe extern "C" fn(
                *mut GOutputStream,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> gssize,
    )
        as Option<
            unsafe extern "C" fn(*mut GOutputStream, *mut GAsyncResult, *mut *mut GError) -> gssize,
        >;
    (*klass).flush_async = Some(
        safe_c2rust_g_output_stream_real_flush_async
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
    (*klass).flush_finish = Some(
        safe_c2rust_g_output_stream_real_flush_finish
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
    (*klass).close_async = Some(
        safe_c2rust_g_output_stream_real_close_async
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
    (*klass).close_finish = Some(
        safe_c2rust_g_output_stream_real_close_finish
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
}
unsafe extern "C" fn safe_c2rust_g_output_stream_init(mut stream: *mut GOutputStream) {
    (*stream).priv_0 =
        safe_c2rust_g_output_stream_get_instance_private(stream) as *mut GOutputStreamPrivate;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_output_stream_write(
    mut stream: *mut GOutputStream,
    mut buffer: *const ::core::ffi::c_void,
    mut count: gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    let mut class: *mut GOutputStreamClass = ::core::ptr::null_mut::<GOutputStreamClass>();
    let mut res: gssize = 0;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_output_stream_get_type();
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
            b"G_IS_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
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
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GOutputStreamClass;
    if (*class).write_fn.is_none() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Output stream doesn\xE2\x80\x99t implement write\0" as *const u8 as *const gchar,
            ),
        );
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    if safe_c2rust_g_output_stream_set_pending(stream, error) == 0 {
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    if !cancellable.is_null() {
        g_cancellable_push_current(cancellable);
    }
    res = (*class).write_fn.expect("non-null function pointer")(
        stream,
        buffer,
        count,
        cancellable,
        error,
    );
    if !cancellable.is_null() {
        g_cancellable_pop_current(cancellable);
    }
    safe_c2rust_g_output_stream_clear_pending(stream);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_output_stream_write_all(
    mut stream: *mut GOutputStream,
    mut buffer: *const ::core::ffi::c_void,
    mut count: gsize,
    mut bytes_written: *mut gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _bytes_written: gsize = 0;
    let mut res: gssize = 0;
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_output_stream_get_type();
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
            b"G_IS_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !buffer.is_null() || count == 0 as gsize {
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
            b"buffer != NULL || count == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    _bytes_written = 0 as gsize;
    while _bytes_written < count {
        res = safe_c2rust_g_output_stream_write(
            stream,
            (buffer as *mut ::core::ffi::c_char).offset(_bytes_written as isize)
                as *const ::core::ffi::c_void,
            count.wrapping_sub(_bytes_written),
            cancellable,
            error,
        );
        if res == -(1 as ::core::ffi::c_int) as gssize {
            if !bytes_written.is_null() {
                *bytes_written = _bytes_written;
            }
            return FALSE;
        }
        if ({
            let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
            if res > 0 as gssize {
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
                b"res > 0\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return 0 as gboolean;
        }
        _bytes_written = _bytes_written.wrapping_add(res as gsize);
    }
    if !bytes_written.is_null() {
        *bytes_written = _bytes_written;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_output_stream_writev(
    mut stream: *mut GOutputStream,
    mut vectors: *const GOutputVector,
    mut n_vectors: gsize,
    mut bytes_written: *mut gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut class: *mut GOutputStreamClass = ::core::ptr::null_mut::<GOutputStreamClass>();
    let mut res: gboolean = 0;
    let mut _bytes_written: gsize = 0 as gsize;
    if !bytes_written.is_null() {
        *bytes_written = 0 as gsize;
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_output_stream_get_type();
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
            b"G_IS_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !vectors.is_null() || n_vectors == 0 as gsize {
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
            b"vectors != NULL || n_vectors == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
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
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if n_vectors == 0 as gsize {
        return TRUE;
    }
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GOutputStreamClass;
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if (*class).writev_fn.is_some() {
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
            b"class->writev_fn != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if safe_c2rust_g_output_stream_set_pending(stream, error) == 0 {
        return FALSE;
    }
    if !cancellable.is_null() {
        g_cancellable_push_current(cancellable);
    }
    res = (*class).writev_fn.expect("non-null function pointer")(
        stream,
        vectors,
        n_vectors,
        &raw mut _bytes_written,
        cancellable,
        error,
    );
    if !(({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if res != 0 || _bytes_written == 0 as gsize {
            _g_boolean_var_20 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_20 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_20
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/goutputstream.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            397 as ::core::ffi::c_int,
            G_STRFUNC,
            b"res || _bytes_written == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !(({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if res != 0 || (error.is_null() || !(*error).is_null()) {
            _g_boolean_var_21 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_21 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_21
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/goutputstream.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            398 as ::core::ffi::c_int,
            G_STRFUNC,
            b"res || (error == NULL || *error != NULL)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if !cancellable.is_null() {
        g_cancellable_pop_current(cancellable);
    }
    safe_c2rust_g_output_stream_clear_pending(stream);
    if !bytes_written.is_null() {
        *bytes_written = _bytes_written;
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_output_stream_writev_all(
    mut stream: *mut GOutputStream,
    mut vectors: *mut GOutputVector,
    mut n_vectors: gsize,
    mut bytes_written: *mut gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _bytes_written: gsize = 0 as gsize;
    let mut i: gsize = 0;
    let mut to_be_written: gsize = 0 as gsize;
    if !bytes_written.is_null() {
        *bytes_written = 0 as gsize;
    }
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_output_stream_get_type();
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
            b"G_IS_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if !vectors.is_null() || n_vectors == 0 as gsize {
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
            b"vectors != NULL || n_vectors == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
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
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    i = 0 as gsize;
    while i < n_vectors {
        if to_be_written > G_MAXSIZE.wrapping_sub((*vectors.offset(i as isize)).size) {
            g_set_error(
                error,
                g_io_error_quark(),
                G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Sum of vectors passed to %s too large\0" as *const u8 as *const gchar,
                ),
                G_STRFUNC,
            );
            return FALSE;
        }
        to_be_written = to_be_written.wrapping_add((*vectors.offset(i as isize)).size);
        i = i.wrapping_add(1);
    }
    _bytes_written = 0 as gsize;
    while n_vectors > 0 as gsize && to_be_written > 0 as gsize {
        let mut n_written: gsize = 0 as gsize;
        let mut res: gboolean = 0;
        res = safe_c2rust_g_output_stream_writev(
            stream,
            vectors,
            n_vectors,
            &raw mut n_written,
            cancellable,
            error,
        );
        if res == 0 {
            if !bytes_written.is_null() {
                *bytes_written = _bytes_written;
            }
            return FALSE;
        }
        if ({
            let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
            if n_written > 0 as gsize {
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
                b"n_written > 0\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return 0 as gboolean;
        }
        _bytes_written = _bytes_written.wrapping_add(n_written);
        while n_vectors > 0 as gsize
            && n_written >= (*vectors.offset(0 as ::core::ffi::c_int as isize)).size
        {
            n_written =
                n_written.wrapping_sub((*vectors.offset(0 as ::core::ffi::c_int as isize)).size);
            vectors = vectors.offset(1);
            n_vectors = n_vectors.wrapping_sub(1);
        }
        if n_written > 0 as gsize && n_vectors > 0 as gsize {
            let ref mut fresh0 = (*vectors.offset(0 as ::core::ffi::c_int as isize)).size;
            *fresh0 = (*fresh0).wrapping_sub(n_written);
            let ref mut fresh1 = (*vectors.offset(0 as ::core::ffi::c_int as isize)).buffer;
            *fresh1 = ((*vectors.offset(0 as ::core::ffi::c_int as isize)).buffer as *mut guint8)
                .offset(n_written as isize) as gconstpointer;
        }
    }
    if !bytes_written.is_null() {
        *bytes_written = _bytes_written;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_output_stream_printf(
    mut stream: *mut GOutputStream,
    mut bytes_written: *mut gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
    mut format: *const gchar,
    mut args: ...
) -> gboolean {
    let mut args_0: ::core::ffi::VaList;
    let mut success: gboolean = 0;
    args_0 = args.clone();
    success = safe_c2rust_g_output_stream_vprintf(
        stream,
        bytes_written,
        cancellable,
        error,
        format,
        args_0,
    );
    return success;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_output_stream_vprintf(
    mut stream: *mut GOutputStream,
    mut bytes_written: *mut gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
    mut format: *const gchar,
    mut args: ::core::ffi::VaList,
) -> gboolean {
    let mut text: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut success: gboolean = 0;
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_output_stream_get_type();
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
            b"G_IS_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
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
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if !format.is_null() {
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
            b"format != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    text = g_strdup_vprintf(format, args);
    success = safe_c2rust_g_output_stream_write_all(
        stream,
        text as *const ::core::ffi::c_void,
        strlen(text) as gsize,
        bytes_written,
        cancellable,
        error,
    );
    g_free(text as gpointer);
    return success;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_output_stream_write_bytes(
    mut stream: *mut GOutputStream,
    mut bytes: *mut GBytes,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    let mut size: gsize = 0;
    let mut data: gconstpointer = ::core::ptr::null::<::core::ffi::c_void>();
    data = g_bytes_get_data(bytes, &raw mut size);
    return safe_c2rust_g_output_stream_write(
        stream,
        data as *const ::core::ffi::c_void,
        size,
        cancellable,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_output_stream_flush(
    mut stream: *mut GOutputStream,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut class: *mut GOutputStreamClass = ::core::ptr::null_mut::<GOutputStreamClass>();
    let mut res: gboolean = 0;
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_output_stream_get_type();
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
            b"G_IS_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if safe_c2rust_g_output_stream_set_pending(stream, error) == 0 {
        return FALSE;
    }
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GOutputStreamClass;
    res = TRUE as gboolean;
    if (*class).flush.is_some() {
        if !cancellable.is_null() {
            g_cancellable_push_current(cancellable);
        }
        res = (*class).flush.expect("non-null function pointer")(stream, cancellable, error);
        if !cancellable.is_null() {
            g_cancellable_pop_current(cancellable);
        }
    }
    safe_c2rust_g_output_stream_clear_pending(stream);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_output_stream_splice(
    mut stream: *mut GOutputStream,
    mut source: *mut GInputStream,
    mut flags: GOutputStreamSpliceFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    let mut class: *mut GOutputStreamClass = ::core::ptr::null_mut::<GOutputStreamClass>();
    let mut bytes_copied: gssize = 0;
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_output_stream_get_type();
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
            b"G_IS_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = source as *mut GTypeInstance;
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
            b"G_IS_INPUT_STREAM (source)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    if g_input_stream_is_closed(source) != 0 {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_CLOSED as ::core::ffi::c_int as gint,
            glib_gettext(b"Source stream is already closed\0" as *const u8 as *const gchar),
        );
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    if safe_c2rust_g_output_stream_set_pending(stream, error) == 0 {
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GOutputStreamClass;
    if !cancellable.is_null() {
        g_cancellable_push_current(cancellable);
    }
    bytes_copied = (*class).splice.expect("non-null function pointer")(
        stream,
        source,
        flags,
        cancellable,
        error,
    );
    if !cancellable.is_null() {
        g_cancellable_pop_current(cancellable);
    }
    safe_c2rust_g_output_stream_clear_pending(stream);
    return bytes_copied;
}
unsafe extern "C" fn safe_c2rust_g_output_stream_real_splice(
    mut stream: *mut GOutputStream,
    mut source: *mut GInputStream,
    mut flags: GOutputStreamSpliceFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    let mut class: *mut GOutputStreamClass =
        (*(stream as *mut GTypeInstance)).g_class as *mut GOutputStreamClass;
    let mut n_read: gssize = 0;
    let mut n_written: gssize = 0;
    let mut bytes_copied: gsize = 0;
    let mut buffer: [::core::ffi::c_char; 8192] = [0; 8192];
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut res: gboolean = 0;
    bytes_copied = 0 as gsize;
    if (*class).write_fn.is_none() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Output stream doesn\xE2\x80\x99t implement write\0" as *const u8 as *const gchar,
            ),
        );
        res = FALSE as gboolean;
    } else {
        res = TRUE as gboolean;
        loop {
            n_read = g_input_stream_read(
                source,
                &raw mut buffer as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<[::core::ffi::c_char; 8192]>() as gsize,
                cancellable,
                error,
            );
            if n_read == -(1 as ::core::ffi::c_int) as gssize {
                res = FALSE as gboolean;
                break;
            } else {
                if n_read == 0 as gssize {
                    break;
                }
                p = &raw mut buffer as *mut ::core::ffi::c_char;
                while n_read > 0 as gssize {
                    n_written = (*class).write_fn.expect("non-null function pointer")(
                        stream,
                        p as *const ::core::ffi::c_void,
                        n_read as gsize,
                        cancellable,
                        error,
                    );
                    if n_written == -(1 as ::core::ffi::c_int) as gssize {
                        res = FALSE as gboolean;
                        break;
                    } else {
                        p = p.offset(n_written as isize);
                        n_read -= n_written;
                        bytes_copied = bytes_copied.wrapping_add(n_written as gsize);
                    }
                }
                if bytes_copied > G_MAXSSIZE as gsize {
                    bytes_copied = G_MAXSSIZE as gsize;
                }
                if !(res != 0) {
                    break;
                }
            }
        }
    }
    if res == 0 {
        error = ::core::ptr::null_mut::<*mut GError>();
    }
    if flags as ::core::ffi::c_uint
        & G_OUTPUT_STREAM_SPLICE_CLOSE_SOURCE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        g_input_stream_close(source, cancellable, ::core::ptr::null_mut::<*mut GError>());
    }
    if flags as ::core::ffi::c_uint
        & G_OUTPUT_STREAM_SPLICE_CLOSE_TARGET as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        if safe_c2rust_g_output_stream_internal_close(stream, cancellable, error) == 0 {
            res = FALSE as gboolean;
        }
    }
    if res != 0 {
        return bytes_copied as gssize;
    }
    return -(1 as ::core::ffi::c_int) as gssize;
}
unsafe extern "C" fn safe_c2rust_g_output_stream_internal_close(
    mut stream: *mut GOutputStream,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut class: *mut GOutputStreamClass = ::core::ptr::null_mut::<GOutputStreamClass>();
    let mut res: gboolean = 0;
    if (*(*stream).priv_0).closed() != 0 {
        return TRUE;
    }
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GOutputStreamClass;
    (*(*stream).priv_0).set_closing(TRUE as guint as guint);
    if !cancellable.is_null() {
        g_cancellable_push_current(cancellable);
    }
    if (*class).flush.is_some() {
        res = (*class).flush.expect("non-null function pointer")(stream, cancellable, error);
    } else {
        res = TRUE as gboolean;
    }
    if res == 0 {
        if (*class).close_fn.is_some() {
            (*class).close_fn.expect("non-null function pointer")(
                stream,
                cancellable,
                ::core::ptr::null_mut::<*mut GError>(),
            );
        }
    } else {
        res = TRUE as gboolean;
        if (*class).close_fn.is_some() {
            res = (*class).close_fn.expect("non-null function pointer")(stream, cancellable, error);
        }
    }
    if !cancellable.is_null() {
        g_cancellable_pop_current(cancellable);
    }
    (*(*stream).priv_0).set_closing(FALSE as guint as guint);
    (*(*stream).priv_0).set_closed(TRUE as guint as guint);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_output_stream_close(
    mut stream: *mut GOutputStream,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut res: gboolean = 0;
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_output_stream_get_type();
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
            b"G_IS_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if (*(*stream).priv_0).closed() != 0 {
        return TRUE;
    }
    if safe_c2rust_g_output_stream_set_pending(stream, error) == 0 {
        return FALSE;
    }
    res = safe_c2rust_g_output_stream_internal_close(stream, cancellable, error);
    safe_c2rust_g_output_stream_clear_pending(stream);
    return res;
}
unsafe extern "C" fn safe_c2rust_async_ready_write_callback_wrapper(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut stream: *mut GOutputStream =
        source_object as *mut ::core::ffi::c_void as *mut GOutputStream;
    let mut class: *mut GOutputStreamClass = ::core::ptr::null_mut::<GOutputStreamClass>();
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut nwrote: gssize = 0;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    safe_c2rust_g_output_stream_clear_pending(stream);
    if g_async_result_legacy_propagate_error(res, &raw mut error) != 0 {
        nwrote = -(1 as ::core::ffi::c_int) as gssize;
    } else {
        class = (*(stream as *mut GTypeInstance)).g_class as *mut GOutputStreamClass;
        nwrote =
            (*class).write_finish.expect("non-null function pointer")(stream, res, &raw mut error);
    }
    if nwrote >= 0 as gssize {
        g_task_return_int(task, nwrote);
    } else {
        g_task_return_error(task, error);
    }
    g_object_unref(task as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_output_stream_write_async(
    mut stream: *mut GOutputStream,
    mut buffer: *const ::core::ffi::c_void,
    mut count: gsize,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut class: *mut GOutputStreamClass = ::core::ptr::null_mut::<GOutputStreamClass>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_output_stream_get_type();
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
            b"G_IS_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if !buffer.is_null() {
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
            b"buffer != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    task = g_task_new(stream as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
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
            gpointer,
        >(Some(
            safe_c2rust_g_output_stream_write_async
                as unsafe extern "C" fn(
                    *mut GOutputStream,
                    *const ::core::ffi::c_void,
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
            b"g_output_stream_write_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_priority(task, io_priority as gint);
    if count == 0 as gsize {
        g_task_return_int(task, 0 as gssize);
        g_object_unref(task as gpointer);
        return;
    }
    if (count as gssize) < 0 as gssize {
        g_task_return_new_error(
            task,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(b"Too large count value passed to %s\0" as *const u8 as *const gchar)
                as *const ::core::ffi::c_char,
            G_STRFUNC,
        );
        g_object_unref(task as gpointer);
        return;
    }
    if safe_c2rust_g_output_stream_set_pending(stream, &raw mut error) == 0 {
        g_task_return_error(task, error);
        g_object_unref(task as gpointer);
        return;
    }
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GOutputStreamClass;
    (*class).write_async.expect("non-null function pointer")(
        stream,
        buffer,
        count,
        io_priority,
        cancellable,
        Some(
            safe_c2rust_async_ready_write_callback_wrapper
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        task as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_output_stream_write_finish(
    mut stream: *mut GOutputStream,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gssize {
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_output_stream_get_type();
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
            b"G_IS_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gssize;
    }
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
        return 0 as gssize;
    }
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if g_async_result_is_tagged(
            result,
            ::core::mem::transmute::<
                Option<
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
                gpointer,
            >(Some(
                safe_c2rust_g_output_stream_write_async
                    as unsafe extern "C" fn(
                        *mut GOutputStream,
                        *const ::core::ffi::c_void,
                        gsize,
                        ::core::ffi::c_int,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )),
        ) != 0
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
            b"g_async_result_is_tagged (result, g_output_stream_write_async)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gssize;
    }
    return g_task_propagate_int(result as *mut ::core::ffi::c_void as *mut GTask, error);
}
unsafe extern "C" fn safe_c2rust_free_async_write_all(mut data: gpointer) {
    g_slice_free1(::core::mem::size_of::<AsyncWriteAll>() as gsize, data);
}
unsafe extern "C" fn safe_c2rust_write_all_callback(
    mut stream: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut data: *mut AsyncWriteAll = g_task_get_task_data(task) as *mut AsyncWriteAll;
    if !result.is_null() {
        let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
        let mut nwritten: gssize = 0;
        nwritten = safe_c2rust_g_output_stream_write_finish(
            stream as *mut ::core::ffi::c_void as *mut GOutputStream,
            result,
            &raw mut error,
        );
        if nwritten == -(1 as ::core::ffi::c_int) as gssize {
            g_task_return_error(task, error);
            g_object_unref(task as gpointer);
            return;
        }
        let mut __n1: gint64 = nwritten as gint64;
        let mut __n2: gint64 = (*data).to_write as gint64;
        if !(__n1 <= __n2) {
            g_assertion_message_cmpint(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/goutputstream.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1132 as ::core::ffi::c_int,
                G_STRFUNC,
                b"nwritten <= data->to_write\0" as *const u8 as *const ::core::ffi::c_char,
                __n1 as guint64,
                b"<=\0" as *const u8 as *const ::core::ffi::c_char,
                __n2 as guint64,
                'i' as i32 as ::core::ffi::c_char,
            );
        }
        if !(({
            let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
            if nwritten > 0 as gssize {
                _g_boolean_var_40 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_40 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_40
        }) as ::core::ffi::c_long
            != 0)
        {
            g_warn_message(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/goutputstream.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1133 as ::core::ffi::c_int,
                G_STRFUNC,
                b"nwritten > 0\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        (*data).to_write = (*data).to_write.wrapping_sub(nwritten as gsize);
        (*data).bytes_written = (*data).bytes_written.wrapping_add(nwritten as gsize);
    }
    if (*data).to_write == 0 as gsize {
        g_task_return_boolean(task, TRUE);
        g_object_unref(task as gpointer);
    } else {
        safe_c2rust_g_output_stream_write_async(
            stream as *mut ::core::ffi::c_void as *mut GOutputStream,
            (*data).buffer.offset((*data).bytes_written as isize) as *const ::core::ffi::c_void,
            (*data).to_write,
            g_task_get_priority(task) as ::core::ffi::c_int,
            g_task_get_cancellable(task),
            Some(
                safe_c2rust_write_all_callback
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            task as gpointer,
        );
    };
}
unsafe extern "C" fn safe_c2rust_write_all_async_thread(
    mut task: *mut GTask,
    mut source_object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut stream: *mut GOutputStream = source_object as *mut GOutputStream;
    let mut data: *mut AsyncWriteAll = task_data as *mut AsyncWriteAll;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if safe_c2rust_g_output_stream_write_all(
        stream,
        (*data).buffer as *const ::core::ffi::c_void,
        (*data).to_write,
        &raw mut (*data).bytes_written,
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
pub unsafe extern "C" fn safe_c2rust_g_output_stream_write_all_async(
    mut stream: *mut GOutputStream,
    mut buffer: *const ::core::ffi::c_void,
    mut count: gsize,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut data: *mut AsyncWriteAll = ::core::ptr::null_mut::<AsyncWriteAll>();
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_output_stream_get_type();
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
            b"G_IS_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if !buffer.is_null() || count == 0 as gsize {
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
            b"buffer != NULL || count == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    task = g_task_new(stream as gpointer, cancellable, callback, user_data);
    data = ({
        let mut __s: gsize = ::core::mem::size_of::<AsyncWriteAll>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut AsyncWriteAll;
    (*data).buffer = buffer as *const guint8;
    (*data).to_write = count;
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
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
            gpointer,
        >(Some(
            safe_c2rust_g_output_stream_write_all_async
                as unsafe extern "C" fn(
                    *mut GOutputStream,
                    *const ::core::ffi::c_void,
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
            b"g_output_stream_write_all_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        data as gpointer,
        Some(safe_c2rust_free_async_write_all as unsafe extern "C" fn(gpointer) -> ()),
    );
    g_task_set_priority(task, io_priority as gint);
    if safe_c2rust_g_output_stream_async_write_is_via_threads(stream) != 0 {
        g_task_run_in_thread(
            task,
            Some(
                safe_c2rust_write_all_async_thread
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
        safe_c2rust_write_all_callback(
            stream as *mut ::core::ffi::c_void as *mut GObject,
            ::core::ptr::null_mut::<GAsyncResult>(),
            task as gpointer,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_output_stream_write_all_finish(
    mut stream: *mut GOutputStream,
    mut result: *mut GAsyncResult,
    mut bytes_written: *mut gsize,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_output_stream_get_type();
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
            _g_boolean_var_43 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_43 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_43
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, stream as gpointer) != 0 {
            _g_boolean_var_44 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_44 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_44
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
    if !bytes_written.is_null() {
        let mut data: *mut AsyncWriteAll = g_task_get_task_data(task) as *mut AsyncWriteAll;
        *bytes_written = (*data).bytes_written;
    }
    return g_task_propagate_boolean(task, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_output_stream_writev_async(
    mut stream: *mut GOutputStream,
    mut vectors: *const GOutputVector,
    mut n_vectors: gsize,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut class: *mut GOutputStreamClass = ::core::ptr::null_mut::<GOutputStreamClass>();
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_output_stream_get_type();
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
            _g_boolean_var_45 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_45 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_45
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if !vectors.is_null() || n_vectors == 0 as gsize {
            _g_boolean_var_46 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_46 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_46
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"vectors != NULL || n_vectors == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
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
            _g_boolean_var_47 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_47 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_47
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
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GOutputStreamClass;
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if (*class).writev_async.is_some() {
            _g_boolean_var_48 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_48 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_48
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"class->writev_async != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*class).writev_async.expect("non-null function pointer")(
        stream,
        vectors,
        n_vectors,
        io_priority,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_output_stream_writev_finish(
    mut stream: *mut GOutputStream,
    mut result: *mut GAsyncResult,
    mut bytes_written: *mut gsize,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut class: *mut GOutputStreamClass = ::core::ptr::null_mut::<GOutputStreamClass>();
    let mut res: gboolean = 0;
    let mut _bytes_written: gsize = 0 as gsize;
    if ({
        let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_output_stream_get_type();
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
            _g_boolean_var_49 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_49 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_49
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_50: ::core::ffi::c_int = 0;
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
            _g_boolean_var_50 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_50 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_50
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
    if ({
        let mut _g_boolean_var_51: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_51 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_51 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_51
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GOutputStreamClass;
    if ({
        let mut _g_boolean_var_52: ::core::ffi::c_int = 0;
        if (*class).writev_finish.is_some() {
            _g_boolean_var_52 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_52 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_52
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"class->writev_finish != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    res = (*class).writev_finish.expect("non-null function pointer")(
        stream,
        result,
        &raw mut _bytes_written,
        error,
    );
    if !(({
        let mut _g_boolean_var_53: ::core::ffi::c_int = 0;
        if res != 0 || _bytes_written == 0 as gsize {
            _g_boolean_var_53 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_53 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_53
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/goutputstream.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1380 as ::core::ffi::c_int,
            G_STRFUNC,
            b"res || _bytes_written == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !(({
        let mut _g_boolean_var_54: ::core::ffi::c_int = 0;
        if res != 0 || (error.is_null() || !(*error).is_null()) {
            _g_boolean_var_54 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_54 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_54
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/goutputstream.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1381 as ::core::ffi::c_int,
            G_STRFUNC,
            b"res || (error == NULL || *error != NULL)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if !bytes_written.is_null() {
        *bytes_written = _bytes_written;
    }
    return res;
}
unsafe extern "C" fn safe_c2rust_free_async_writev_all(mut data: gpointer) {
    g_slice_free1(::core::mem::size_of::<AsyncWritevAll>() as gsize, data);
}
unsafe extern "C" fn safe_c2rust_writev_all_callback(
    mut stream: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut data: *mut AsyncWritevAll = g_task_get_task_data(task) as *mut AsyncWritevAll;
    let mut priority: gint = g_task_get_priority(task);
    let mut cancellable: *mut GCancellable = g_task_get_cancellable(task);
    if !result.is_null() {
        let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
        let mut res: gboolean = 0;
        let mut n_written: gsize = 0 as gsize;
        res = safe_c2rust_g_output_stream_writev_finish(
            stream as *mut ::core::ffi::c_void as *mut GOutputStream,
            result,
            &raw mut n_written,
            &raw mut error,
        );
        if res == 0 {
            g_task_return_error(
                task,
                safe_c2rust_g_steal_pointer(&raw mut error as gpointer) as *mut GError,
            );
            g_object_unref(task as gpointer);
            return;
        }
        if !(({
            let mut _g_boolean_var_55: ::core::ffi::c_int = 0;
            if n_written > 0 as gsize {
                _g_boolean_var_55 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_55 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_55
        }) as ::core::ffi::c_long
            != 0)
        {
            g_warn_message(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/goutputstream.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1427 as ::core::ffi::c_int,
                G_STRFUNC,
                b"n_written > 0\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        (*data).bytes_written = (*data).bytes_written.wrapping_add(n_written);
        while (*data).n_vectors > 0 as gsize
            && n_written >= (*(*data).vectors.offset(0 as ::core::ffi::c_int as isize)).size
        {
            n_written = n_written
                .wrapping_sub((*(*data).vectors.offset(0 as ::core::ffi::c_int as isize)).size);
            (*data).vectors = (*data).vectors.offset(1);
            (*data).n_vectors = (*data).n_vectors.wrapping_sub(1);
        }
        if n_written > 0 as gsize && (*data).n_vectors > 0 as gsize {
            let ref mut fresh2 = (*(*data).vectors.offset(0 as ::core::ffi::c_int as isize)).size;
            *fresh2 = (*fresh2).wrapping_sub(n_written);
            let ref mut fresh3 = (*(*data).vectors.offset(0 as ::core::ffi::c_int as isize)).buffer;
            *fresh3 = ((*(*data).vectors.offset(0 as ::core::ffi::c_int as isize)).buffer
                as *mut guint8)
                .offset(n_written as isize) as gconstpointer;
        }
    }
    if (*data).n_vectors == 0 as gsize {
        g_task_return_boolean(task, TRUE);
        g_object_unref(task as gpointer);
    } else {
        safe_c2rust_g_output_stream_writev_async(
            stream as *mut ::core::ffi::c_void as *mut GOutputStream,
            (*data).vectors,
            (*data).n_vectors,
            priority as ::core::ffi::c_int,
            cancellable,
            Some(
                safe_c2rust_writev_all_callback
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            safe_c2rust_g_steal_pointer(&raw mut task as gpointer) as *mut GTask as gpointer,
        );
    };
}
unsafe extern "C" fn safe_c2rust_writev_all_async_thread(
    mut task: *mut GTask,
    mut source_object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut stream: *mut GOutputStream = source_object as *mut GOutputStream;
    let mut data: *mut AsyncWritevAll = task_data as *mut AsyncWritevAll;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if safe_c2rust_g_output_stream_writev_all(
        stream,
        (*data).vectors,
        (*data).n_vectors,
        &raw mut (*data).bytes_written,
        g_task_get_cancellable(task),
        &raw mut error,
    ) != 0
    {
        g_task_return_boolean(task, TRUE);
    } else {
        g_task_return_error(
            task,
            safe_c2rust_g_steal_pointer(&raw mut error as gpointer) as *mut GError,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_output_stream_writev_all_async(
    mut stream: *mut GOutputStream,
    mut vectors: *mut GOutputVector,
    mut n_vectors: gsize,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut data: *mut AsyncWritevAll = ::core::ptr::null_mut::<AsyncWritevAll>();
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut i: gsize = 0;
    let mut to_be_written: gsize = 0 as gsize;
    if ({
        let mut _g_boolean_var_56: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_output_stream_get_type();
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
            _g_boolean_var_56 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_56 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_56
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_57: ::core::ffi::c_int = 0;
        if !vectors.is_null() || n_vectors == 0 as gsize {
            _g_boolean_var_57 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_57 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_57
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"vectors != NULL || n_vectors == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_58: ::core::ffi::c_int = 0;
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
            _g_boolean_var_58 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_58 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_58
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
    task = g_task_new(stream as gpointer, cancellable, callback, user_data);
    data = ({
        let mut __s: gsize = ::core::mem::size_of::<AsyncWritevAll>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut AsyncWritevAll;
    (*data).vectors = vectors;
    (*data).n_vectors = n_vectors;
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GOutputStream,
                    *mut GOutputVector,
                    gsize,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_output_stream_writev_all_async
                as unsafe extern "C" fn(
                    *mut GOutputStream,
                    *mut GOutputVector,
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
            b"g_output_stream_writev_all_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        data as gpointer,
        Some(safe_c2rust_free_async_writev_all as unsafe extern "C" fn(gpointer) -> ()),
    );
    g_task_set_priority(task, io_priority as gint);
    i = 0 as gsize;
    while i < n_vectors {
        if to_be_written > G_MAXSIZE.wrapping_sub((*vectors.offset(i as isize)).size) {
            g_task_return_new_error(
                task,
                g_io_error_quark(),
                G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Sum of vectors passed to %s too large\0" as *const u8 as *const gchar,
                ) as *const ::core::ffi::c_char,
                G_STRFUNC,
            );
            g_object_unref(task as gpointer);
            return;
        }
        to_be_written = to_be_written.wrapping_add((*vectors.offset(i as isize)).size);
        i = i.wrapping_add(1);
    }
    if safe_c2rust_g_output_stream_async_writev_is_via_threads(stream) != 0 {
        g_task_run_in_thread(
            task,
            Some(
                safe_c2rust_writev_all_async_thread
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
        safe_c2rust_writev_all_callback(
            stream as *mut ::core::ffi::c_void as *mut GObject,
            ::core::ptr::null_mut::<GAsyncResult>(),
            safe_c2rust_g_steal_pointer(&raw mut task as gpointer) as *mut GTask as gpointer,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_output_stream_writev_all_finish(
    mut stream: *mut GOutputStream,
    mut result: *mut GAsyncResult,
    mut bytes_written: *mut gsize,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    if ({
        let mut _g_boolean_var_59: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_output_stream_get_type();
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
            _g_boolean_var_59 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_59 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_59
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_60: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, stream as gpointer) != 0 {
            _g_boolean_var_60 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_60 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_60
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
    if ({
        let mut _g_boolean_var_61: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_61 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_61 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_61
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    task = result as *mut ::core::ffi::c_void as *mut GTask;
    if !bytes_written.is_null() {
        let mut data: *mut AsyncWritevAll = g_task_get_task_data(task) as *mut AsyncWritevAll;
        *bytes_written = (*data).bytes_written;
    }
    return g_task_propagate_boolean(task, error);
}
unsafe extern "C" fn safe_c2rust_write_bytes_callback(
    mut stream: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut nwrote: gssize = 0;
    nwrote = safe_c2rust_g_output_stream_write_finish(
        stream as *mut ::core::ffi::c_void as *mut GOutputStream,
        result,
        &raw mut error,
    );
    if nwrote == -(1 as ::core::ffi::c_int) as gssize {
        g_task_return_error(task, error);
    } else {
        g_task_return_int(task, nwrote);
    }
    g_object_unref(task as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_output_stream_write_bytes_async(
    mut stream: *mut GOutputStream,
    mut bytes: *mut GBytes,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut size: gsize = 0;
    let mut data: gconstpointer = ::core::ptr::null::<::core::ffi::c_void>();
    data = g_bytes_get_data(bytes, &raw mut size);
    task = g_task_new(stream as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GOutputStream,
                    *mut GBytes,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_output_stream_write_bytes_async
                as unsafe extern "C" fn(
                    *mut GOutputStream,
                    *mut GBytes,
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
            b"g_output_stream_write_bytes_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        g_bytes_ref(bytes) as gpointer,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut GBytes) -> ()>, GDestroyNotify>(
            Some(g_bytes_unref as unsafe extern "C" fn(*mut GBytes) -> ()),
        ),
    );
    safe_c2rust_g_output_stream_write_async(
        stream,
        data as *const ::core::ffi::c_void,
        size,
        io_priority,
        cancellable,
        Some(
            safe_c2rust_write_bytes_callback
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        task as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_output_stream_write_bytes_finish(
    mut stream: *mut GOutputStream,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gssize {
    if ({
        let mut _g_boolean_var_62: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_output_stream_get_type();
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
            _g_boolean_var_62 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_62 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_62
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    if ({
        let mut _g_boolean_var_63: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, stream as gpointer) != 0 {
            _g_boolean_var_63 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_63 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_63
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
unsafe extern "C" fn safe_c2rust_async_ready_splice_callback_wrapper(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut _data: gpointer,
) {
    let mut stream: *mut GOutputStream =
        source_object as *mut ::core::ffi::c_void as *mut GOutputStream;
    let mut class: *mut GOutputStreamClass = ::core::ptr::null_mut::<GOutputStreamClass>();
    let mut task: *mut GTask = _data as *mut GTask;
    let mut nspliced: gssize = 0;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    safe_c2rust_g_output_stream_clear_pending(stream);
    if g_async_result_legacy_propagate_error(res, &raw mut error) != 0 {
        nspliced = -(1 as ::core::ffi::c_int) as gssize;
    } else {
        class = (*(stream as *mut GTypeInstance)).g_class as *mut GOutputStreamClass;
        nspliced =
            (*class).splice_finish.expect("non-null function pointer")(stream, res, &raw mut error);
    }
    if nspliced >= 0 as gssize {
        g_task_return_int(task, nspliced);
    } else {
        g_task_return_error(task, error);
    }
    g_object_unref(task as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_output_stream_splice_async(
    mut stream: *mut GOutputStream,
    mut source: *mut GInputStream,
    mut flags: GOutputStreamSpliceFlags,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut class: *mut GOutputStreamClass = ::core::ptr::null_mut::<GOutputStreamClass>();
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_64: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_output_stream_get_type();
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
            _g_boolean_var_64 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_64 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_64
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_65: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = source as *mut GTypeInstance;
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
            _g_boolean_var_65 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_65 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_65
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_INPUT_STREAM (source)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    task = g_task_new(stream as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
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
            gpointer,
        >(Some(
            safe_c2rust_g_output_stream_splice_async
                as unsafe extern "C" fn(
                    *mut GOutputStream,
                    *mut GInputStream,
                    GOutputStreamSpliceFlags,
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
            b"g_output_stream_splice_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_priority(task, io_priority as gint);
    g_task_set_task_data(
        task,
        g_object_ref(source as gpointer) as *mut GInputStream as gpointer,
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
    if g_input_stream_is_closed(source) != 0 {
        g_task_return_new_error_literal(
            task,
            g_io_error_quark(),
            G_IO_ERROR_CLOSED as ::core::ffi::c_int as gint,
            glib_gettext(b"Source stream is already closed\0" as *const u8 as *const gchar)
                as *const ::core::ffi::c_char,
        );
        g_object_unref(task as gpointer);
        return;
    }
    if safe_c2rust_g_output_stream_set_pending(stream, &raw mut error) == 0 {
        g_task_return_error(task, error);
        g_object_unref(task as gpointer);
        return;
    }
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GOutputStreamClass;
    (*class).splice_async.expect("non-null function pointer")(
        stream,
        source,
        flags,
        io_priority,
        cancellable,
        Some(
            safe_c2rust_async_ready_splice_callback_wrapper
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        task as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_output_stream_splice_finish(
    mut stream: *mut GOutputStream,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gssize {
    if ({
        let mut _g_boolean_var_66: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_output_stream_get_type();
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
            _g_boolean_var_66 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_66 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_66
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gssize;
    }
    if ({
        let mut _g_boolean_var_67: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, stream as gpointer) != 0 {
            _g_boolean_var_67 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_67 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_67
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (result, stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gssize;
    }
    if ({
        let mut _g_boolean_var_68: ::core::ffi::c_int = 0;
        if g_async_result_is_tagged(
            result,
            ::core::mem::transmute::<
                Option<
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
                gpointer,
            >(Some(
                safe_c2rust_g_output_stream_splice_async
                    as unsafe extern "C" fn(
                        *mut GOutputStream,
                        *mut GInputStream,
                        GOutputStreamSpliceFlags,
                        ::core::ffi::c_int,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )),
        ) != 0
        {
            _g_boolean_var_68 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_68 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_68
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_async_result_is_tagged (result, g_output_stream_splice_async)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gssize;
    }
    return g_task_propagate_int(result as *mut ::core::ffi::c_void as *mut GTask, error);
}
unsafe extern "C" fn safe_c2rust_async_ready_flush_callback_wrapper(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut stream: *mut GOutputStream =
        source_object as *mut ::core::ffi::c_void as *mut GOutputStream;
    let mut class: *mut GOutputStreamClass = ::core::ptr::null_mut::<GOutputStreamClass>();
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut flushed: gboolean = 0;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    safe_c2rust_g_output_stream_clear_pending(stream);
    if g_async_result_legacy_propagate_error(res, &raw mut error) != 0 {
        flushed = FALSE as gboolean;
    } else {
        class = (*(stream as *mut GTypeInstance)).g_class as *mut GOutputStreamClass;
        flushed =
            (*class).flush_finish.expect("non-null function pointer")(stream, res, &raw mut error);
    }
    if flushed != 0 {
        g_task_return_boolean(task, TRUE);
    } else {
        g_task_return_error(task, error);
    }
    g_object_unref(task as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_output_stream_flush_async(
    mut stream: *mut GOutputStream,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut class: *mut GOutputStreamClass = ::core::ptr::null_mut::<GOutputStreamClass>();
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_69: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_output_stream_get_type();
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
            _g_boolean_var_69 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_69 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_69
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    task = g_task_new(stream as gpointer, cancellable, callback, user_data);
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
            safe_c2rust_g_output_stream_flush_async
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
            b"g_output_stream_flush_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_priority(task, io_priority as gint);
    if safe_c2rust_g_output_stream_set_pending(stream, &raw mut error) == 0 {
        g_task_return_error(task, error);
        g_object_unref(task as gpointer);
        return;
    }
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GOutputStreamClass;
    if (*class).flush_async.is_none() {
        safe_c2rust_g_output_stream_clear_pending(stream);
        g_task_return_boolean(task, TRUE);
        g_object_unref(task as gpointer);
        return;
    }
    (*class).flush_async.expect("non-null function pointer")(
        stream,
        io_priority,
        cancellable,
        Some(
            safe_c2rust_async_ready_flush_callback_wrapper
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        task as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_output_stream_flush_finish(
    mut stream: *mut GOutputStream,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_70: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_output_stream_get_type();
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
            _g_boolean_var_70 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_70 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_70
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_71: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, stream as gpointer) != 0 {
            _g_boolean_var_71 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_71 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_71
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
    if ({
        let mut _g_boolean_var_72: ::core::ffi::c_int = 0;
        if g_async_result_is_tagged(
            result,
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
                safe_c2rust_g_output_stream_flush_async
                    as unsafe extern "C" fn(
                        *mut GOutputStream,
                        ::core::ffi::c_int,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )),
        ) != 0
        {
            _g_boolean_var_72 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_72 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_72
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_async_result_is_tagged (result, g_output_stream_flush_async)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
}
unsafe extern "C" fn safe_c2rust_async_ready_close_callback_wrapper(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut stream: *mut GOutputStream =
        source_object as *mut ::core::ffi::c_void as *mut GOutputStream;
    let mut class: *mut GOutputStreamClass = ::core::ptr::null_mut::<GOutputStreamClass>();
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut error: *mut GError = g_task_get_task_data(task) as *mut GError;
    (*(*stream).priv_0).set_closing(FALSE as guint as guint);
    (*(*stream).priv_0).set_closed(TRUE as guint as guint);
    if error.is_null() && g_async_result_legacy_propagate_error(res, &raw mut error) == 0 {
        class = (*(stream as *mut GTypeInstance)).g_class as *mut GOutputStreamClass;
        (*class).close_finish.expect("non-null function pointer")(
            stream,
            res,
            if !error.is_null() {
                ::core::ptr::null_mut::<*mut GError>()
            } else {
                &raw mut error
            },
        );
    }
    if !error.is_null() {
        g_task_return_error(task, error);
    } else {
        g_task_return_boolean(task, TRUE);
    }
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_async_ready_close_flushed_callback_wrapper(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut stream: *mut GOutputStream =
        source_object as *mut ::core::ffi::c_void as *mut GOutputStream;
    let mut class: *mut GOutputStreamClass = ::core::ptr::null_mut::<GOutputStreamClass>();
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GOutputStreamClass;
    if g_async_result_legacy_propagate_error(res, &raw mut error) == 0 {
        (*class).flush_finish.expect("non-null function pointer")(stream, res, &raw mut error);
    }
    if !error.is_null() {
        g_task_set_task_data(task, error as gpointer, None);
    }
    (*class).close_async.expect("non-null function pointer")(
        stream,
        g_task_get_priority(task) as ::core::ffi::c_int,
        g_task_get_cancellable(task),
        Some(
            safe_c2rust_async_ready_close_callback_wrapper
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        task as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_real_close_async_cb(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut stream: *mut GOutputStream =
        source_object as *mut ::core::ffi::c_void as *mut GOutputStream;
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut ret: gboolean = 0;
    safe_c2rust_g_output_stream_clear_pending(stream);
    ret = safe_c2rust_g_output_stream_internal_close_finish(stream, res, &raw mut error);
    if !error.is_null() {
        g_task_return_error(task, error);
    } else {
        g_task_return_boolean(task, ret);
    }
    g_object_unref(task as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_output_stream_close_async(
    mut stream: *mut GOutputStream,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_73: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_output_stream_get_type();
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
            _g_boolean_var_73 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_73 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_73
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    task = g_task_new(stream as gpointer, cancellable, callback, user_data);
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
            safe_c2rust_g_output_stream_close_async
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
            b"g_output_stream_close_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_priority(task, io_priority as gint);
    if safe_c2rust_g_output_stream_set_pending(stream, &raw mut error) == 0 {
        g_task_return_error(task, error);
        g_object_unref(task as gpointer);
        return;
    }
    safe_c2rust_g_output_stream_internal_close_async(
        stream,
        io_priority,
        cancellable,
        Some(
            safe_c2rust_real_close_async_cb
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        task as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_g_output_stream_internal_close_async(
    mut stream: *mut GOutputStream,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut class: *mut GOutputStreamClass = ::core::ptr::null_mut::<GOutputStreamClass>();
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    task = g_task_new(stream as gpointer, cancellable, callback, user_data);
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
            safe_c2rust_g_output_stream_internal_close_async
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
            b"g_output_stream_internal_close_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_priority(task, io_priority as gint);
    if (*(*stream).priv_0).closed() != 0 {
        g_task_return_boolean(task, TRUE);
        g_object_unref(task as gpointer);
        return;
    }
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GOutputStreamClass;
    (*(*stream).priv_0).set_closing(TRUE as guint as guint);
    if (*class).flush_async.is_none()
        || (*class).flush_async
            == Some(
                safe_c2rust_g_output_stream_real_flush_async
                    as unsafe extern "C" fn(
                        *mut GOutputStream,
                        ::core::ffi::c_int,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )
            && ((*class).flush.is_none()
                || (*class).close_async
                    == Some(
                        safe_c2rust_g_output_stream_real_close_async
                            as unsafe extern "C" fn(
                                *mut GOutputStream,
                                ::core::ffi::c_int,
                                *mut GCancellable,
                                GAsyncReadyCallback,
                                gpointer,
                            ) -> (),
                    ))
    {
        (*class).close_async.expect("non-null function pointer")(
            stream,
            io_priority,
            cancellable,
            Some(
                safe_c2rust_async_ready_close_callback_wrapper
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            task as gpointer,
        );
    } else {
        (*class).flush_async.expect("non-null function pointer")(
            stream,
            io_priority,
            cancellable,
            Some(
                safe_c2rust_async_ready_close_flushed_callback_wrapper
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            task as gpointer,
        );
    };
}
unsafe extern "C" fn safe_c2rust_g_output_stream_internal_close_finish(
    mut stream: *mut GOutputStream,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_74: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_output_stream_get_type();
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
            _g_boolean_var_74 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_74 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_74
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_75: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, stream as gpointer) != 0 {
            _g_boolean_var_75 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_75 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_75
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
    if ({
        let mut _g_boolean_var_76: ::core::ffi::c_int = 0;
        if g_async_result_is_tagged(
            result,
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
                safe_c2rust_g_output_stream_internal_close_async
                    as unsafe extern "C" fn(
                        *mut GOutputStream,
                        ::core::ffi::c_int,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )),
        ) != 0
        {
            _g_boolean_var_76 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_76 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_76
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_async_result_is_tagged (result, g_output_stream_internal_close_async)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_output_stream_close_finish(
    mut stream: *mut GOutputStream,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_77: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_output_stream_get_type();
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
            _g_boolean_var_77 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_77 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_77
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_78: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, stream as gpointer) != 0 {
            _g_boolean_var_78 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_78 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_78
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
    if ({
        let mut _g_boolean_var_79: ::core::ffi::c_int = 0;
        if g_async_result_is_tagged(
            result,
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
                safe_c2rust_g_output_stream_close_async
                    as unsafe extern "C" fn(
                        *mut GOutputStream,
                        ::core::ffi::c_int,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )),
        ) != 0
        {
            _g_boolean_var_79 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_79 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_79
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_async_result_is_tagged (result, g_output_stream_close_async)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_output_stream_is_closed(
    mut stream: *mut GOutputStream,
) -> gboolean {
    if ({
        let mut _g_boolean_var_80: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_output_stream_get_type();
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
            _g_boolean_var_80 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_80 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_80
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    return (*(*stream).priv_0).closed() as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_output_stream_is_closing(
    mut stream: *mut GOutputStream,
) -> gboolean {
    if ({
        let mut _g_boolean_var_81: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_output_stream_get_type();
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
            _g_boolean_var_81 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_81 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_81
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    return (*(*stream).priv_0).closing() as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_output_stream_has_pending(
    mut stream: *mut GOutputStream,
) -> gboolean {
    if ({
        let mut _g_boolean_var_82: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_output_stream_get_type();
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
            _g_boolean_var_82 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_82 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_82
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*(*stream).priv_0).pending() as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_output_stream_set_pending(
    mut stream: *mut GOutputStream,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_83: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_output_stream_get_type();
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
            _g_boolean_var_83 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_83 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_83
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn safe_c2rust_g_output_stream_clear_pending(mut stream: *mut GOutputStream) {
    if ({
        let mut _g_boolean_var_84: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_output_stream_get_type();
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
            _g_boolean_var_84 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_84 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_84
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*(*stream).priv_0).set_pending(FALSE as guint as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_output_stream_async_write_is_via_threads(
    mut stream: *mut GOutputStream,
) -> gboolean {
    let mut class: *mut GOutputStreamClass = ::core::ptr::null_mut::<GOutputStreamClass>();
    if ({
        let mut _g_boolean_var_85: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_output_stream_get_type();
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
            _g_boolean_var_85 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_85 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_85
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GOutputStreamClass;
    return ((*class).write_async
        == Some(
            safe_c2rust_g_output_stream_real_write_async
                as unsafe extern "C" fn(
                    *mut GOutputStream,
                    *const ::core::ffi::c_void,
                    gsize,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )
        && !(({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = g_pollable_output_stream_get_type();
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
            && g_pollable_output_stream_can_poll(
                stream as *mut ::core::ffi::c_void as *mut GPollableOutputStream,
            ) != 0)) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_output_stream_async_writev_is_via_threads(
    mut stream: *mut GOutputStream,
) -> gboolean {
    let mut class: *mut GOutputStreamClass = ::core::ptr::null_mut::<GOutputStreamClass>();
    if ({
        let mut _g_boolean_var_86: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_output_stream_get_type();
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
            _g_boolean_var_86 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_86 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_86
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GOutputStreamClass;
    return ((*class).writev_async
        == Some(
            safe_c2rust_g_output_stream_real_writev_async
                as unsafe extern "C" fn(
                    *mut GOutputStream,
                    *const GOutputVector,
                    gsize,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )
        && !(({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = g_pollable_output_stream_get_type();
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
            && g_pollable_output_stream_can_poll(
                stream as *mut ::core::ffi::c_void as *mut GPollableOutputStream,
            ) != 0)) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_output_stream_async_close_is_via_threads(
    mut stream: *mut GOutputStream,
) -> gboolean {
    let mut class: *mut GOutputStreamClass = ::core::ptr::null_mut::<GOutputStreamClass>();
    if ({
        let mut _g_boolean_var_87: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_output_stream_get_type();
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
            _g_boolean_var_87 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_87 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_87
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GOutputStreamClass;
    return ((*class).close_async
        == Some(
            safe_c2rust_g_output_stream_real_close_async
                as unsafe extern "C" fn(
                    *mut GOutputStream,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_output_stream_real_writev(
    mut stream: *mut GOutputStream,
    mut vectors: *const GOutputVector,
    mut n_vectors: gsize,
    mut bytes_written: *mut gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut class: *mut GOutputStreamClass = ::core::ptr::null_mut::<GOutputStreamClass>();
    let mut _bytes_written: gsize = 0 as gsize;
    let mut i: gsize = 0;
    let mut err: *mut GError = ::core::ptr::null_mut::<GError>();
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GOutputStreamClass;
    if !bytes_written.is_null() {
        *bytes_written = 0 as gsize;
    }
    i = 0 as gsize;
    while i < n_vectors {
        let mut res: gssize = 0 as gssize;
        if _bytes_written > G_MAXSIZE.wrapping_sub((*vectors.offset(i as isize)).size) {
            break;
        }
        res = (*class).write_fn.expect("non-null function pointer")(
            stream,
            (*vectors.offset(i as isize)).buffer as *const ::core::ffi::c_void,
            (*vectors.offset(i as isize)).size,
            cancellable,
            &raw mut err,
        );
        if res == -(1 as ::core::ffi::c_int) as gssize {
            if _bytes_written > 0 as gsize {
                if !bytes_written.is_null() {
                    *bytes_written = _bytes_written;
                }
                g_clear_error(&raw mut err);
                return TRUE;
            }
            g_propagate_error(error, err);
            return FALSE;
        }
        _bytes_written = _bytes_written.wrapping_add(res as gsize);
        if (res as gsize) < (*vectors.offset(i as isize)).size {
            break;
        }
        i = i.wrapping_add(1);
    }
    if !bytes_written.is_null() {
        *bytes_written = _bytes_written;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_free_write_data(mut op: *mut WriteData) {
    g_slice_free1(::core::mem::size_of::<WriteData>() as gsize, op as gpointer);
}
unsafe extern "C" fn safe_c2rust_write_async_thread(
    mut task: *mut GTask,
    mut source_object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut stream: *mut GOutputStream = source_object as *mut GOutputStream;
    let mut op: *mut WriteData = task_data as *mut WriteData;
    let mut class: *mut GOutputStreamClass = ::core::ptr::null_mut::<GOutputStreamClass>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut count_written: gssize = 0;
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GOutputStreamClass;
    count_written = (*class).write_fn.expect("non-null function pointer")(
        stream,
        (*op).buffer,
        (*op).count_requested,
        cancellable,
        &raw mut error,
    );
    if count_written == -(1 as ::core::ffi::c_int) as gssize {
        g_task_return_error(task, error);
    } else {
        g_task_return_int(task, count_written);
    };
}
unsafe extern "C" fn safe_c2rust_write_async_pollable_ready(
    mut stream: *mut GPollableOutputStream,
    mut user_data: gpointer,
) -> gboolean {
    let mut task: *mut GTask = user_data as *mut GTask;
    safe_c2rust_write_async_pollable(stream, task);
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_write_async_pollable(
    mut stream: *mut GPollableOutputStream,
    mut task: *mut GTask,
) {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut op: *mut WriteData = g_task_get_task_data(task) as *mut WriteData;
    let mut count_written: gssize = 0;
    if g_task_return_error_if_cancelled(task) != 0 {
        return;
    }
    count_written = (*(g_type_interface_peek(
        (*(stream as *mut GTypeInstance)).g_class as gpointer,
        g_pollable_output_stream_get_type(),
    ) as *mut GPollableOutputStreamInterface))
        .write_nonblocking
        .expect("non-null function pointer")(
        stream,
        (*op).buffer,
        (*op).count_requested,
        &raw mut error,
    );
    if g_error_matches(
        error,
        g_io_error_quark(),
        G_IO_ERROR_WOULD_BLOCK as ::core::ffi::c_int as gint,
    ) != 0
    {
        let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
        g_error_free(error);
        source = g_pollable_output_stream_create_source(stream, g_task_get_cancellable(task));
        g_task_attach_source(
            task,
            source,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GPollableOutputStream, gpointer) -> gboolean>,
                GSourceFunc,
            >(Some(
                safe_c2rust_write_async_pollable_ready
                    as unsafe extern "C" fn(*mut GPollableOutputStream, gpointer) -> gboolean,
            )),
        );
        g_source_unref(source);
        return;
    }
    if count_written == -(1 as ::core::ffi::c_int) as gssize {
        g_task_return_error(task, error);
    } else {
        g_task_return_int(task, count_written);
    };
}
unsafe extern "C" fn safe_c2rust_g_output_stream_real_write_async(
    mut stream: *mut GOutputStream,
    mut buffer: *const ::core::ffi::c_void,
    mut count: gsize,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut op: *mut WriteData = ::core::ptr::null_mut::<WriteData>();
    op = ({
        let mut __s: gsize = ::core::mem::size_of::<WriteData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut WriteData;
    task = g_task_new(stream as gpointer, cancellable, callback, user_data);
    g_task_set_check_cancellable(task, FALSE);
    g_task_set_task_data(
        task,
        op as gpointer,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut WriteData) -> ()>, GDestroyNotify>(
            Some(safe_c2rust_free_write_data as unsafe extern "C" fn(*mut WriteData) -> ()),
        ),
    );
    (*op).buffer = buffer;
    (*op).count_requested = count;
    if safe_c2rust_g_output_stream_async_write_is_via_threads(stream) == 0 {
        safe_c2rust_write_async_pollable(
            stream as *mut ::core::ffi::c_void as *mut GPollableOutputStream,
            task,
        );
    } else {
        g_task_run_in_thread(
            task,
            Some(
                safe_c2rust_write_async_thread
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
unsafe extern "C" fn safe_c2rust_g_output_stream_real_write_finish(
    mut stream: *mut GOutputStream,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gssize {
    if ({
        let mut _g_boolean_var_88: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, stream as gpointer) != 0 {
            _g_boolean_var_88 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_88 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_88
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (result, stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gssize;
    }
    return g_task_propagate_int(result as *mut ::core::ffi::c_void as *mut GTask, error);
}
unsafe extern "C" fn safe_c2rust_free_writev_data(mut op: *mut WritevData) {
    g_slice_free1(
        ::core::mem::size_of::<WritevData>() as gsize,
        op as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_writev_async_thread(
    mut task: *mut GTask,
    mut source_object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut stream: *mut GOutputStream = source_object as *mut GOutputStream;
    let mut op: *mut WritevData = task_data as *mut WritevData;
    let mut class: *mut GOutputStreamClass = ::core::ptr::null_mut::<GOutputStreamClass>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut res: gboolean = 0;
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GOutputStreamClass;
    res = (*class).writev_fn.expect("non-null function pointer")(
        stream,
        (*op).vectors,
        (*op).n_vectors,
        &raw mut (*op).bytes_written,
        cancellable,
        &raw mut error,
    );
    if !(({
        let mut _g_boolean_var_89: ::core::ffi::c_int = 0;
        if res != 0 || (*op).bytes_written == 0 as gsize {
            _g_boolean_var_89 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_89 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_89
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/goutputstream.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            2515 as ::core::ffi::c_int,
            G_STRFUNC,
            b"res || op->bytes_written == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !(({
        let mut _g_boolean_var_90: ::core::ffi::c_int = 0;
        if res != 0 || !error.is_null() {
            _g_boolean_var_90 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_90 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_90
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/goutputstream.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            2516 as ::core::ffi::c_int,
            G_STRFUNC,
            b"res || error != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if res == 0 {
        g_task_return_error(
            task,
            safe_c2rust_g_steal_pointer(&raw mut error as gpointer) as *mut GError,
        );
    } else {
        g_task_return_boolean(task, TRUE);
    };
}
unsafe extern "C" fn safe_c2rust_writev_async_pollable_ready(
    mut stream: *mut GPollableOutputStream,
    mut user_data: gpointer,
) -> gboolean {
    let mut task: *mut GTask = user_data as *mut GTask;
    safe_c2rust_writev_async_pollable(stream, task);
    return G_SOURCE_REMOVE;
}
unsafe extern "C" fn safe_c2rust_writev_async_pollable(
    mut stream: *mut GPollableOutputStream,
    mut task: *mut GTask,
) {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut op: *mut WritevData = g_task_get_task_data(task) as *mut WritevData;
    let mut res: GPollableReturn = G_POLLABLE_RETURN_FAILED;
    let mut bytes_written: gsize = 0 as gsize;
    if g_task_return_error_if_cancelled(task) != 0 {
        return;
    }
    res = (*(g_type_interface_peek(
        (*(stream as *mut GTypeInstance)).g_class as gpointer,
        g_pollable_output_stream_get_type(),
    ) as *mut GPollableOutputStreamInterface))
        .writev_nonblocking
        .expect("non-null function pointer")(
        stream,
        (*op).vectors,
        (*op).n_vectors,
        &raw mut bytes_written,
        &raw mut error,
    );
    match res as ::core::ffi::c_int {
        -27 => {
            let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
            if !(({
                let mut _g_boolean_var_91: ::core::ffi::c_int = 0;
                if error.is_null() {
                    _g_boolean_var_91 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_91 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_91
            }) as ::core::ffi::c_long
                != 0)
            {
                g_warn_message(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/goutputstream.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    2558 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if !(({
                let mut _g_boolean_var_92: ::core::ffi::c_int = 0;
                if bytes_written == 0 as gsize {
                    _g_boolean_var_92 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_92 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_92
            }) as ::core::ffi::c_long
                != 0)
            {
                g_warn_message(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/goutputstream.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    2559 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"bytes_written == 0\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            source = g_pollable_output_stream_create_source(stream, g_task_get_cancellable(task));
            g_task_attach_source(
                task,
                source,
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GPollableOutputStream, gpointer) -> gboolean>,
                    GSourceFunc,
                >(Some(
                    safe_c2rust_writev_async_pollable_ready
                        as unsafe extern "C" fn(*mut GPollableOutputStream, gpointer) -> gboolean,
                )),
            );
            g_source_unref(source);
        }
        1 => {
            if !(({
                let mut _g_boolean_var_93: ::core::ffi::c_int = 0;
                if error.is_null() {
                    _g_boolean_var_93 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_93 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_93
            }) as ::core::ffi::c_long
                != 0)
            {
                g_warn_message(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/goutputstream.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    2569 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            (*op).bytes_written = bytes_written;
            g_task_return_boolean(task, TRUE);
        }
        0 => {
            if !(({
                let mut _g_boolean_var_94: ::core::ffi::c_int = 0;
                if bytes_written == 0 as gsize {
                    _g_boolean_var_94 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_94 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_94
            }) as ::core::ffi::c_long
                != 0)
            {
                g_warn_message(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/goutputstream.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    2574 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"bytes_written == 0\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if !(({
                let mut _g_boolean_var_95: ::core::ffi::c_int = 0;
                if !error.is_null() {
                    _g_boolean_var_95 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_95 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_95
            }) as ::core::ffi::c_long
                != 0)
            {
                g_warn_message(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/goutputstream.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    2575 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"error != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            g_task_return_error(
                task,
                safe_c2rust_g_steal_pointer(&raw mut error as gpointer) as *mut GError,
            );
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/goutputstream.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                2579 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_output_stream_real_writev_async(
    mut stream: *mut GOutputStream,
    mut vectors: *const GOutputVector,
    mut n_vectors: gsize,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut op: *mut WritevData = ::core::ptr::null_mut::<WritevData>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    op = ({
        let mut __s: gsize = ::core::mem::size_of::<WritevData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut WritevData;
    task = g_task_new(stream as gpointer, cancellable, callback, user_data);
    (*op).vectors = vectors;
    (*op).n_vectors = n_vectors;
    g_task_set_check_cancellable(task, FALSE);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
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
            gpointer,
        >(Some(
            safe_c2rust_g_output_stream_writev_async
                as unsafe extern "C" fn(
                    *mut GOutputStream,
                    *const GOutputVector,
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
            b"g_output_stream_writev_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_priority(task, io_priority as gint);
    g_task_set_task_data(
        task,
        op as gpointer,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut WritevData) -> ()>, GDestroyNotify>(
            Some(safe_c2rust_free_writev_data as unsafe extern "C" fn(*mut WritevData) -> ()),
        ),
    );
    if n_vectors == 0 as gsize {
        g_task_return_boolean(task, TRUE);
        g_object_unref(task as gpointer);
        return;
    }
    if safe_c2rust_g_output_stream_set_pending(stream, &raw mut error) == 0 {
        g_task_return_error(
            task,
            safe_c2rust_g_steal_pointer(&raw mut error as gpointer) as *mut GError,
        );
        g_object_unref(task as gpointer);
        return;
    }
    if safe_c2rust_g_output_stream_async_writev_is_via_threads(stream) == 0 {
        safe_c2rust_writev_async_pollable(
            stream as *mut ::core::ffi::c_void as *mut GPollableOutputStream,
            task,
        );
    } else {
        g_task_run_in_thread(
            task,
            Some(
                safe_c2rust_writev_async_thread
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
unsafe extern "C" fn safe_c2rust_g_output_stream_real_writev_finish(
    mut stream: *mut GOutputStream,
    mut result: *mut GAsyncResult,
    mut bytes_written: *mut gsize,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    if ({
        let mut _g_boolean_var_96: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, stream as gpointer) != 0 {
            _g_boolean_var_96 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_96 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_96
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
    if ({
        let mut _g_boolean_var_97: ::core::ffi::c_int = 0;
        if g_async_result_is_tagged(
            result,
            ::core::mem::transmute::<
                Option<
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
                gpointer,
            >(Some(
                safe_c2rust_g_output_stream_writev_async
                    as unsafe extern "C" fn(
                        *mut GOutputStream,
                        *const GOutputVector,
                        gsize,
                        ::core::ffi::c_int,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )),
        ) != 0
        {
            _g_boolean_var_97 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_97 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_97
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_async_result_is_tagged (result, g_output_stream_writev_async)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    safe_c2rust_g_output_stream_clear_pending(stream);
    task = result as *mut ::core::ffi::c_void as *mut GTask;
    if !bytes_written.is_null() {
        let mut op: *mut WritevData = g_task_get_task_data(task) as *mut WritevData;
        *bytes_written = (*op).bytes_written;
    }
    return g_task_propagate_boolean(task, error);
}
unsafe extern "C" fn safe_c2rust_free_splice_data(mut op: *mut SpliceData) {
    let mut _pp: *mut *mut guint8 = &raw mut (*op).buffer;
    let mut _ptr: *mut guint8 = *_pp;
    *_pp = ::core::ptr::null_mut::<guint8>();
    if !_ptr.is_null() {
        g_free(_ptr as gpointer);
    }
    g_object_unref((*op).source as gpointer);
    g_clear_error(&raw mut (*op).error);
    g_free(op as gpointer);
}
unsafe extern "C" fn safe_c2rust_real_splice_async_complete_cb(mut task: *mut GTask) {
    let mut op: *mut SpliceData = g_task_get_task_data(task) as *mut SpliceData;
    if (*op).flags as ::core::ffi::c_uint
        & G_OUTPUT_STREAM_SPLICE_CLOSE_SOURCE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
        && (*op).istream_closed() == 0
    {
        return;
    }
    if (*op).flags as ::core::ffi::c_uint
        & G_OUTPUT_STREAM_SPLICE_CLOSE_TARGET as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
        && (*op).ostream_closed() == 0
    {
        return;
    }
    if !(*op).error.is_null() {
        g_task_return_error(task, (*op).error);
        (*op).error = ::core::ptr::null_mut::<GError>();
    } else {
        g_task_return_int(task, (*op).bytes_copied as gssize);
    }
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_real_splice_async_close_input_cb(
    mut source: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut op: *mut SpliceData = g_task_get_task_data(task) as *mut SpliceData;
    g_input_stream_close_finish(
        source as *mut ::core::ffi::c_void as *mut GInputStream,
        res,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    (*op).set_istream_closed(TRUE as guint as guint);
    safe_c2rust_real_splice_async_complete_cb(task);
}
unsafe extern "C" fn safe_c2rust_real_splice_async_close_output_cb(
    mut source: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut op: *mut SpliceData = g_task_get_task_data(task) as *mut SpliceData;
    let mut error: *mut *mut GError = if (*op).error.is_null() {
        &raw mut (*op).error
    } else {
        ::core::ptr::null_mut::<*mut GError>()
    };
    safe_c2rust_g_output_stream_internal_close_finish(
        source as *mut ::core::ffi::c_void as *mut GOutputStream,
        res,
        error,
    );
    (*op).set_ostream_closed(TRUE as guint as guint);
    safe_c2rust_real_splice_async_complete_cb(task);
}
unsafe extern "C" fn safe_c2rust_real_splice_async_complete(mut task: *mut GTask) {
    let mut op: *mut SpliceData = g_task_get_task_data(task) as *mut SpliceData;
    let mut done: gboolean = TRUE;
    if (*op).flags as ::core::ffi::c_uint
        & G_OUTPUT_STREAM_SPLICE_CLOSE_SOURCE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        done = FALSE as gboolean;
        g_input_stream_close_async(
            (*op).source,
            g_task_get_priority(task) as ::core::ffi::c_int,
            g_task_get_cancellable(task),
            Some(
                safe_c2rust_real_splice_async_close_input_cb
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            task as gpointer,
        );
    }
    if (*op).flags as ::core::ffi::c_uint
        & G_OUTPUT_STREAM_SPLICE_CLOSE_TARGET as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        done = FALSE as gboolean;
        safe_c2rust_g_output_stream_internal_close_async(
            g_task_get_source_object(task) as *mut GOutputStream,
            g_task_get_priority(task) as ::core::ffi::c_int,
            g_task_get_cancellable(task),
            Some(
                safe_c2rust_real_splice_async_close_output_cb
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            task as gpointer,
        );
    }
    if done != 0 {
        safe_c2rust_real_splice_async_complete_cb(task);
    }
}
unsafe extern "C" fn safe_c2rust_real_splice_async_write_cb(
    mut source: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut class: *mut GOutputStreamClass = ::core::ptr::null_mut::<GOutputStreamClass>();
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut op: *mut SpliceData = g_task_get_task_data(task) as *mut SpliceData;
    let mut ret: gssize = 0;
    class = (*(g_task_get_source_object(task) as *mut GTypeInstance)).g_class
        as *mut GOutputStreamClass;
    ret = (*class).write_finish.expect("non-null function pointer")(
        source as *mut ::core::ffi::c_void as *mut GOutputStream,
        res,
        &raw mut (*op).error,
    );
    if ret == -(1 as ::core::ffi::c_int) as gssize {
        safe_c2rust_real_splice_async_complete(task);
        return;
    }
    (*op).n_written += ret;
    (*op).bytes_copied = (*op).bytes_copied.wrapping_add(ret as gsize);
    if (*op).bytes_copied > G_MAXSSIZE as gsize {
        (*op).bytes_copied = G_MAXSSIZE as gsize;
    }
    if (*op).n_written < (*op).n_read {
        (*class).write_async.expect("non-null function pointer")(
            g_task_get_source_object(task) as *mut GOutputStream,
            (*op).buffer.offset((*op).n_written as isize) as *const ::core::ffi::c_void,
            ((*op).n_read - (*op).n_written) as gsize,
            g_task_get_priority(task) as ::core::ffi::c_int,
            g_task_get_cancellable(task),
            Some(
                safe_c2rust_real_splice_async_write_cb
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            task as gpointer,
        );
        return;
    }
    g_input_stream_read_async(
        (*op).source,
        (*op).buffer as *mut ::core::ffi::c_void,
        8192 as gsize,
        g_task_get_priority(task) as ::core::ffi::c_int,
        g_task_get_cancellable(task),
        Some(
            safe_c2rust_real_splice_async_read_cb
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        task as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_real_splice_async_read_cb(
    mut source: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut class: *mut GOutputStreamClass = ::core::ptr::null_mut::<GOutputStreamClass>();
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut op: *mut SpliceData = g_task_get_task_data(task) as *mut SpliceData;
    let mut ret: gssize = 0;
    class = (*(g_task_get_source_object(task) as *mut GTypeInstance)).g_class
        as *mut GOutputStreamClass;
    ret = g_input_stream_read_finish((*op).source, res, &raw mut (*op).error);
    if ret == -(1 as ::core::ffi::c_int) as gssize || ret == 0 as gssize {
        safe_c2rust_real_splice_async_complete(task);
        return;
    }
    (*op).n_read = ret;
    (*op).n_written = 0 as gssize;
    (*class).write_async.expect("non-null function pointer")(
        g_task_get_source_object(task) as *mut GOutputStream,
        (*op).buffer as *const ::core::ffi::c_void,
        (*op).n_read as gsize,
        g_task_get_priority(task) as ::core::ffi::c_int,
        g_task_get_cancellable(task),
        Some(
            safe_c2rust_real_splice_async_write_cb
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        task as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_splice_async_thread(
    mut task: *mut GTask,
    mut source_object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut stream: *mut GOutputStream = source_object as *mut GOutputStream;
    let mut op: *mut SpliceData = task_data as *mut SpliceData;
    let mut class: *mut GOutputStreamClass = ::core::ptr::null_mut::<GOutputStreamClass>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut bytes_copied: gssize = 0;
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GOutputStreamClass;
    bytes_copied = (*class).splice.expect("non-null function pointer")(
        stream,
        (*op).source,
        (*op).flags,
        cancellable,
        &raw mut error,
    );
    if bytes_copied == -(1 as ::core::ffi::c_int) as gssize {
        g_task_return_error(task, error);
    } else {
        g_task_return_int(task, bytes_copied);
    };
}
unsafe extern "C" fn safe_c2rust_g_output_stream_real_splice_async(
    mut stream: *mut GOutputStream,
    mut source: *mut GInputStream,
    mut flags: GOutputStreamSpliceFlags,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut op: *mut SpliceData = ::core::ptr::null_mut::<SpliceData>();
    op = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<SpliceData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut SpliceData;
    task = g_task_new(stream as gpointer, cancellable, callback, user_data);
    g_task_set_task_data(
        task,
        op as gpointer,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut SpliceData) -> ()>, GDestroyNotify>(
            Some(safe_c2rust_free_splice_data as unsafe extern "C" fn(*mut SpliceData) -> ()),
        ),
    );
    (*op).flags = flags;
    (*op).source = g_object_ref(source as gpointer) as *mut GInputStream as *mut GInputStream;
    if g_input_stream_async_read_is_via_threads(source) != 0
        && safe_c2rust_g_output_stream_async_write_is_via_threads(stream) != 0
    {
        g_task_run_in_thread(
            task,
            Some(
                safe_c2rust_splice_async_thread
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
        (*op).buffer = g_malloc(8192 as gsize) as *mut guint8;
        g_input_stream_read_async(
            (*op).source,
            (*op).buffer as *mut ::core::ffi::c_void,
            8192 as gsize,
            g_task_get_priority(task) as ::core::ffi::c_int,
            g_task_get_cancellable(task),
            Some(
                safe_c2rust_real_splice_async_read_cb
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            task as gpointer,
        );
    };
}
unsafe extern "C" fn safe_c2rust_g_output_stream_real_splice_finish(
    mut stream: *mut GOutputStream,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gssize {
    if ({
        let mut _g_boolean_var_98: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, stream as gpointer) != 0 {
            _g_boolean_var_98 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_98 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_98
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (result, stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gssize;
    }
    return g_task_propagate_int(result as *mut ::core::ffi::c_void as *mut GTask, error);
}
unsafe extern "C" fn safe_c2rust_flush_async_thread(
    mut task: *mut GTask,
    mut source_object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut stream: *mut GOutputStream = source_object as *mut GOutputStream;
    let mut class: *mut GOutputStreamClass = ::core::ptr::null_mut::<GOutputStreamClass>();
    let mut result: gboolean = 0;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GOutputStreamClass;
    result = TRUE as gboolean;
    if (*class).flush.is_some() {
        result =
            (*class).flush.expect("non-null function pointer")(stream, cancellable, &raw mut error);
    }
    if result != 0 {
        g_task_return_boolean(task, TRUE);
    } else {
        g_task_return_error(task, error);
    };
}
unsafe extern "C" fn safe_c2rust_g_output_stream_real_flush_async(
    mut stream: *mut GOutputStream,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    task = g_task_new(stream as gpointer, cancellable, callback, user_data);
    g_task_set_priority(task, io_priority as gint);
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_flush_async_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_output_stream_real_flush_finish(
    mut stream: *mut GOutputStream,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_99: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, stream as gpointer) != 0 {
            _g_boolean_var_99 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_99 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_99
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
unsafe extern "C" fn safe_c2rust_close_async_thread(
    mut task: *mut GTask,
    mut source_object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut stream: *mut GOutputStream = source_object as *mut GOutputStream;
    let mut class: *mut GOutputStreamClass = ::core::ptr::null_mut::<GOutputStreamClass>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut result: gboolean = TRUE;
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GOutputStreamClass;
    if (*class).flush.is_some()
        && ((*class).flush_async.is_none()
            || (*class).flush_async
                == Some(
                    safe_c2rust_g_output_stream_real_flush_async
                        as unsafe extern "C" fn(
                            *mut GOutputStream,
                            ::core::ffi::c_int,
                            *mut GCancellable,
                            GAsyncReadyCallback,
                            gpointer,
                        ) -> (),
                ))
    {
        result =
            (*class).flush.expect("non-null function pointer")(stream, cancellable, &raw mut error);
    }
    if (*class).close_fn.is_some() {
        if result == 0 {
            (*class).close_fn.expect("non-null function pointer")(
                stream,
                cancellable,
                ::core::ptr::null_mut::<*mut GError>(),
            );
        } else {
            result = (*class).close_fn.expect("non-null function pointer")(
                stream,
                cancellable,
                &raw mut error,
            );
        }
    }
    if result != 0 {
        g_task_return_boolean(task, TRUE);
    } else {
        g_task_return_error(task, error);
    };
}
unsafe extern "C" fn safe_c2rust_g_output_stream_real_close_async(
    mut stream: *mut GOutputStream,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    task = g_task_new(stream as gpointer, cancellable, callback, user_data);
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
unsafe extern "C" fn safe_c2rust_g_output_stream_real_close_finish(
    mut stream: *mut GOutputStream,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_100: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, stream as gpointer) != 0 {
            _g_boolean_var_100 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_100 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_100
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
