use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _GInputStreamPrivate;
    pub type _GOutputStreamPrivate;
    pub type _GTask;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_error_matches(error: *const GError, domain: GQuark, code: gint) -> gboolean;
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_clear_error(err: *mut *mut GError);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
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
    fn glib_gettext(str: *const gchar) -> *const gchar;
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
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_value_set_object(value: *mut GValue, v_object: gpointer);
    fn g_param_spec_boolean(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: gboolean,
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
    fn g_input_stream_get_type() -> GType;
    fn g_input_stream_close(
        stream: *mut GInputStream,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
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
    fn g_output_stream_get_type() -> GType;
    fn g_output_stream_close(
        stream: *mut GOutputStream,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_output_stream_splice_async(
        stream: *mut GOutputStream,
        source: *mut GInputStream,
        flags: GOutputStreamSpliceFlags,
        io_priority: ::core::ffi::c_int,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_output_stream_splice_finish(
        stream: *mut GOutputStream,
        result: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gssize;
    fn g_output_stream_close_async(
        stream: *mut GOutputStream,
        io_priority: ::core::ffi::c_int,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_output_stream_close_finish(
        stream: *mut GOutputStream,
        result: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_cancellable_new() -> *mut GCancellable;
    fn g_cancellable_is_cancelled(cancellable: *mut GCancellable) -> gboolean;
    fn g_cancellable_push_current(cancellable: *mut GCancellable);
    fn g_cancellable_pop_current(cancellable: *mut GCancellable);
    fn g_cancellable_reset(cancellable: *mut GCancellable);
    fn g_cancellable_connect(
        cancellable: *mut GCancellable,
        callback: GCallback,
        data: gpointer,
        data_destroy_func: GDestroyNotify,
    ) -> gulong;
    fn g_cancellable_disconnect(cancellable: *mut GCancellable, handler_id: gulong);
    fn g_cancellable_cancel(cancellable: *mut GCancellable);
    fn g_io_error_quark() -> GQuark;
    fn g_async_result_legacy_propagate_error(
        res: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_input_stream_async_close_is_via_threads(stream: *mut GInputStream) -> gboolean;
    fn g_output_stream_async_close_is_via_threads(stream: *mut GOutputStream) -> gboolean;
    fn g_task_new(
        source_object: gpointer,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        callback_data: gpointer,
    ) -> *mut GTask;
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
    fn g_task_return_boolean(task: *mut GTask, result: gboolean);
    fn g_task_return_error(task: *mut GTask, error: *mut GError);
    fn g_task_propagate_boolean(task: *mut GTask, error: *mut *mut GError) -> gboolean;
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
pub type GCallback = Option<unsafe extern "C" fn() -> ()>;
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
pub type GIOStreamSpliceFlags = ::core::ffi::c_uint;
pub const G_IO_STREAM_SPLICE_WAIT_FOR_BOTH: GIOStreamSpliceFlags = 4;
pub const G_IO_STREAM_SPLICE_CLOSE_STREAM2: GIOStreamSpliceFlags = 2;
pub const G_IO_STREAM_SPLICE_CLOSE_STREAM1: GIOStreamSpliceFlags = 1;
pub const G_IO_STREAM_SPLICE_NONE: GIOStreamSpliceFlags = 0;
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
pub struct _GInputStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GInputStreamPrivate,
}
pub type GInputStreamPrivate = _GInputStreamPrivate;
pub type GInputStream = _GInputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GOutputStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GOutputStreamPrivate,
}
pub type GOutputStreamPrivate = _GOutputStreamPrivate;
pub type GOutputStream = _GOutputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GIOStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GIOStreamPrivate,
}
pub type GIOStreamPrivate = _GIOStreamPrivate;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GIOStreamPrivate {
    #[bitfield(name = "closed", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "pending", ty = "guint", bits = "1..=1")]
    pub closed_pending: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type GIOStream = _GIOStream;
pub type GTask = _GTask;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GIOStreamClass {
    pub parent_class: GObjectClass,
    pub get_input_stream: Option<unsafe extern "C" fn(*mut GIOStream) -> *mut GInputStream>,
    pub get_output_stream: Option<unsafe extern "C" fn(*mut GIOStream) -> *mut GOutputStream>,
    pub close_fn: Option<
        unsafe extern "C" fn(*mut GIOStream, *mut GCancellable, *mut *mut GError) -> gboolean,
    >,
    pub close_async: Option<
        unsafe extern "C" fn(
            *mut GIOStream,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub close_finish: Option<
        unsafe extern "C" fn(*mut GIOStream, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    >,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved6: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved7: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved8: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved9: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved10: Option<unsafe extern "C" fn() -> ()>,
}
pub type GIOStreamClass = _GIOStreamClass;
pub const PROP_OUTPUT_STREAM: C2RustUnnamed_1 = 2;
pub const PROP_INPUT_STREAM: C2RustUnnamed_1 = 1;
pub const PROP_CLOSED: C2RustUnnamed_1 = 3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CloseAsyncData {
    pub error: *mut GError,
    pub pending: gint,
}
pub type GTaskThreadFunc =
    Option<unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SpliceContext {
    pub stream1: *mut GIOStream,
    pub stream2: *mut GIOStream,
    pub flags: GIOStreamSpliceFlags,
    pub io_priority: gint,
    pub cancellable: *mut GCancellable,
    pub cancelled_id: gulong,
    pub op1_cancellable: *mut GCancellable,
    pub op2_cancellable: *mut GCancellable,
    pub completed: guint,
    pub error: *mut GError,
}
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_1 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_io_stream_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GIOStream\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GIOStreamClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_io_stream_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GIOStream>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GIOStream) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_io_stream_init as unsafe extern "C" fn(*mut GIOStream) -> (),
            )),
        ),
        G_TYPE_FLAG_ABSTRACT,
    );
    safe_c2rust_GIOStream_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GIOStreamPrivate>() as gsize,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_stream_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_io_stream_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_io_stream_get_instance_private(
    mut self_0: *mut GIOStream,
) -> gpointer {
    return (self_0 as *mut guint8).offset(safe_c2rust_GIOStream_private_offset as glong as isize)
        as gpointer;
}
static mut safe_c2rust_GIOStream_private_offset: gint = 0;
static mut safe_c2rust_g_io_stream_parent_class: gpointer = NULL;
unsafe extern "C" fn safe_c2rust_g_io_stream_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_io_stream_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GIOStream_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut safe_c2rust_GIOStream_private_offset);
    }
    safe_c2rust_g_io_stream_class_init(klass as *mut GIOStreamClass);
}
unsafe extern "C" fn safe_c2rust_g_io_stream_dispose(mut object: *mut GObject) {
    let mut stream: *mut GIOStream = ::core::ptr::null_mut::<GIOStream>();
    stream = object as *mut ::core::ffi::c_void as *mut GIOStream;
    if (*(*stream).priv_0).closed() == 0 {
        safe_c2rust_g_io_stream_close(
            stream,
            ::core::ptr::null_mut::<GCancellable>(),
            ::core::ptr::null_mut::<*mut GError>(),
        );
    }
    (*(safe_c2rust_g_io_stream_parent_class as *mut GObjectClass))
        .dispose
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_io_stream_init(mut stream: *mut GIOStream) {
    (*stream).priv_0 =
        safe_c2rust_g_io_stream_get_instance_private(stream) as *mut GIOStreamPrivate;
}
unsafe extern "C" fn safe_c2rust_g_io_stream_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut stream: *mut GIOStream = object as *mut ::core::ffi::c_void as *mut GIOStream;
    match prop_id {
        3 => {
            g_value_set_boolean(value, (*(*stream).priv_0).closed() as gboolean);
        }
        1 => {
            g_value_set_object(
                value,
                safe_c2rust_g_io_stream_get_input_stream(stream) as gpointer,
            );
        }
        2 => {
            g_value_set_object(
                value,
                safe_c2rust_g_io_stream_get_output_stream(stream) as gpointer,
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/giostream.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                158 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_io_stream_class_init(mut klass: *mut GIOStreamClass) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).dispose =
        Some(safe_c2rust_g_io_stream_dispose as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_io_stream_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*klass).close_fn = Some(
        safe_c2rust_g_io_stream_real_close
            as unsafe extern "C" fn(
                *mut GIOStream,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut GIOStream, *mut GCancellable, *mut *mut GError) -> gboolean,
        >;
    (*klass).close_async = Some(
        safe_c2rust_g_io_stream_real_close_async
            as unsafe extern "C" fn(
                *mut GIOStream,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GIOStream,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*klass).close_finish = Some(
        safe_c2rust_g_io_stream_real_close_finish
            as unsafe extern "C" fn(
                *mut GIOStream,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut GIOStream, *mut GAsyncResult, *mut *mut GError) -> gboolean,
        >;
    g_object_class_install_property(
        gobject_class,
        PROP_CLOSED as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"closed\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_INPUT_STREAM as ::core::ffi::c_int as guint,
        g_param_spec_object(
            b"input-stream\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_input_stream_get_type(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_OUTPUT_STREAM as ::core::ffi::c_int as guint,
        g_param_spec_object(
            b"output-stream\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_output_stream_get_type(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_stream_is_closed(mut stream: *mut GIOStream) -> gboolean {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_io_stream_get_type();
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
            b"G_IS_IO_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    return (*(*stream).priv_0).closed() as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_stream_get_input_stream(
    mut stream: *mut GIOStream,
) -> *mut GInputStream {
    let mut klass: *mut GIOStreamClass = ::core::ptr::null_mut::<GIOStreamClass>();
    klass = (*(stream as *mut GTypeInstance)).g_class as *mut GIOStreamClass;
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if (*klass).get_input_stream.is_some() {
            _g_boolean_var_11 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_11 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_11
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/giostream.c\0" as *const u8
                as *const ::core::ffi::c_char,
            248 as ::core::ffi::c_int,
            G_STRFUNC,
            b"klass->get_input_stream != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return (*klass)
        .get_input_stream
        .expect("non-null function pointer")(stream);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_stream_get_output_stream(
    mut stream: *mut GIOStream,
) -> *mut GOutputStream {
    let mut klass: *mut GIOStreamClass = ::core::ptr::null_mut::<GIOStreamClass>();
    klass = (*(stream as *mut GTypeInstance)).g_class as *mut GIOStreamClass;
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if (*klass).get_output_stream.is_some() {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/giostream.c\0" as *const u8
                as *const ::core::ffi::c_char,
            272 as ::core::ffi::c_int,
            G_STRFUNC,
            b"klass->get_output_stream != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return (*klass)
        .get_output_stream
        .expect("non-null function pointer")(stream);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_stream_has_pending(
    mut stream: *mut GIOStream,
) -> gboolean {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_io_stream_get_type();
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
            b"G_IS_IO_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*(*stream).priv_0).pending() as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_stream_set_pending(
    mut stream: *mut GIOStream,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_io_stream_get_type();
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
            b"G_IS_IO_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn safe_c2rust_g_io_stream_clear_pending(mut stream: *mut GIOStream) {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_io_stream_get_type();
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
            b"G_IS_IO_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*(*stream).priv_0).set_pending(FALSE as guint as guint);
}
unsafe extern "C" fn safe_c2rust_g_io_stream_real_close(
    mut stream: *mut GIOStream,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut res: gboolean = 0;
    res = g_output_stream_close(
        safe_c2rust_g_io_stream_get_output_stream(stream),
        cancellable,
        error,
    );
    if !error.is_null() && !(*error).is_null() {
        error = ::core::ptr::null_mut::<*mut GError>();
    }
    res &= g_input_stream_close(
        safe_c2rust_g_io_stream_get_input_stream(stream),
        cancellable,
        error,
    );
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_stream_close(
    mut stream: *mut GIOStream,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut class: *mut GIOStreamClass = ::core::ptr::null_mut::<GIOStreamClass>();
    let mut res: gboolean = 0;
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_io_stream_get_type();
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
            b"G_IS_IO_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GIOStreamClass;
    if (*(*stream).priv_0).closed() != 0 {
        return TRUE;
    }
    if safe_c2rust_g_io_stream_set_pending(stream, error) == 0 {
        return FALSE;
    }
    if !cancellable.is_null() {
        g_cancellable_push_current(cancellable);
    }
    res = TRUE as gboolean;
    if (*class).close_fn.is_some() {
        res = (*class).close_fn.expect("non-null function pointer")(stream, cancellable, error);
    }
    if !cancellable.is_null() {
        g_cancellable_pop_current(cancellable);
    }
    (*(*stream).priv_0).set_closed(TRUE as guint as guint);
    safe_c2rust_g_io_stream_clear_pending(stream);
    return res;
}
unsafe extern "C" fn safe_c2rust_async_ready_close_callback_wrapper(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut stream: *mut GIOStream = source_object as *mut ::core::ffi::c_void as *mut GIOStream;
    let mut klass: *mut GIOStreamClass =
        (*(stream as *mut GTypeInstance)).g_class as *mut GIOStreamClass;
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut success: gboolean = 0;
    (*(*stream).priv_0).set_closed(TRUE as guint as guint);
    safe_c2rust_g_io_stream_clear_pending(stream);
    if g_async_result_legacy_propagate_error(res, &raw mut error) != 0 {
        success = FALSE as gboolean;
    } else {
        success =
            (*klass).close_finish.expect("non-null function pointer")(stream, res, &raw mut error);
    }
    if !error.is_null() {
        g_task_return_error(task, error);
    } else {
        g_task_return_boolean(task, success);
    }
    g_object_unref(task as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_stream_close_async(
    mut stream: *mut GIOStream,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut class: *mut GIOStreamClass = ::core::ptr::null_mut::<GIOStreamClass>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_io_stream_get_type();
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
            b"G_IS_IO_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
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
                    *mut GIOStream,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_io_stream_close_async
                as unsafe extern "C" fn(
                    *mut GIOStream,
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
            b"g_io_stream_close_async\0" as *const u8 as *const gchar,
        );
    }
    if (*(*stream).priv_0).closed() != 0 {
        g_task_return_boolean(task, TRUE);
        g_object_unref(task as gpointer);
        return;
    }
    if safe_c2rust_g_io_stream_set_pending(stream, &raw mut error) == 0 {
        g_task_return_error(task, error);
        g_object_unref(task as gpointer);
        return;
    }
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GIOStreamClass;
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
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_stream_close_finish(
    mut stream: *mut GIOStream,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_io_stream_get_type();
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
            b"G_IS_IO_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, stream as gpointer) != 0 {
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
    let mut stream: *mut GIOStream = source_object as *mut GIOStream;
    let mut class: *mut GIOStreamClass = ::core::ptr::null_mut::<GIOStreamClass>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut result: gboolean = 0;
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GIOStreamClass;
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
unsafe extern "C" fn safe_c2rust_stream_close_complete(
    mut source: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut data: *mut CloseAsyncData = ::core::ptr::null_mut::<CloseAsyncData>();
    data = g_task_get_task_data(task) as *mut CloseAsyncData;
    (*data).pending -= 1;
    if ({
        let mut __inst: *mut GTypeInstance = source as *mut GTypeInstance;
        let mut __t: GType = g_output_stream_get_type();
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
    {
        let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
        g_output_stream_close_finish(
            source as *mut ::core::ffi::c_void as *mut GOutputStream,
            result,
            &raw mut error,
        );
        if !error.is_null() {
            if !(*data).error.is_null() {
                g_error_free((*data).error);
            }
            (*data).error = error;
        }
    } else {
        g_input_stream_close_finish(
            source as *mut ::core::ffi::c_void as *mut GInputStream,
            result,
            if !(*data).error.is_null() {
                ::core::ptr::null_mut::<*mut GError>()
            } else {
                &raw mut (*data).error
            },
        );
    }
    if (*data).pending == 0 as ::core::ffi::c_int {
        if !(*data).error.is_null() {
            g_task_return_error(task, (*data).error);
        } else {
            g_task_return_boolean(task, TRUE);
        }
        g_slice_free1(
            ::core::mem::size_of::<CloseAsyncData>() as gsize,
            data as gpointer,
        );
        g_object_unref(task as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_g_io_stream_real_close_async(
    mut stream: *mut GIOStream,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut input: *mut GInputStream = ::core::ptr::null_mut::<GInputStream>();
    let mut output: *mut GOutputStream = ::core::ptr::null_mut::<GOutputStream>();
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    task = g_task_new(stream as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GIOStream,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_io_stream_real_close_async
                as unsafe extern "C" fn(
                    *mut GIOStream,
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
            b"g_io_stream_real_close_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_check_cancellable(task, FALSE);
    g_task_set_priority(task, io_priority as gint);
    input = safe_c2rust_g_io_stream_get_input_stream(stream);
    output = safe_c2rust_g_io_stream_get_output_stream(stream);
    if g_input_stream_async_close_is_via_threads(input) != 0
        && g_output_stream_async_close_is_via_threads(output) != 0
    {
        g_task_run_in_thread(
            task,
            Some(
                safe_c2rust_close_async_thread
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
        let mut data: *mut CloseAsyncData = ::core::ptr::null_mut::<CloseAsyncData>();
        data =
            g_slice_alloc(::core::mem::size_of::<CloseAsyncData>() as gsize) as *mut CloseAsyncData;
        (*data).error = ::core::ptr::null_mut::<GError>();
        (*data).pending = 2 as ::core::ffi::c_int as gint;
        g_task_set_task_data(task, data as gpointer, None);
        g_input_stream_close_async(
            input,
            io_priority,
            cancellable,
            Some(
                safe_c2rust_stream_close_complete
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            task as gpointer,
        );
        g_output_stream_close_async(
            output,
            io_priority,
            cancellable,
            Some(
                safe_c2rust_stream_close_complete
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            task as gpointer,
        );
    };
}
unsafe extern "C" fn safe_c2rust_g_io_stream_real_close_finish(
    mut stream: *mut GIOStream,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, stream as gpointer) != 0 {
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
            b"g_task_is_valid (result, stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
}
unsafe extern "C" fn safe_c2rust_splice_context_free(mut ctx: *mut SpliceContext) {
    g_object_unref((*ctx).stream1 as gpointer);
    g_object_unref((*ctx).stream2 as gpointer);
    if !(*ctx).cancellable.is_null() {
        g_object_unref((*ctx).cancellable as gpointer);
    }
    g_object_unref((*ctx).op1_cancellable as gpointer);
    g_object_unref((*ctx).op2_cancellable as gpointer);
    g_clear_error(&raw mut (*ctx).error);
    g_slice_free1(
        ::core::mem::size_of::<SpliceContext>() as gsize,
        ctx as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_splice_complete(
    mut task: *mut GTask,
    mut ctx: *mut SpliceContext,
) {
    if (*ctx).cancelled_id != 0 as gulong {
        g_cancellable_disconnect((*ctx).cancellable, (*ctx).cancelled_id);
    }
    (*ctx).cancelled_id = 0 as gulong;
    if !(*ctx).error.is_null() {
        g_task_return_error(task, (*ctx).error);
        (*ctx).error = ::core::ptr::null_mut::<GError>();
    } else {
        g_task_return_boolean(task, TRUE);
    };
}
unsafe extern "C" fn safe_c2rust_splice_close_cb(
    mut iostream: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut ctx: *mut SpliceContext = g_task_get_task_data(task) as *mut SpliceContext;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    safe_c2rust_g_io_stream_close_finish(
        iostream as *mut ::core::ffi::c_void as *mut GIOStream,
        res,
        &raw mut error,
    );
    (*ctx).completed = (*ctx).completed.wrapping_add(1);
    if !error.is_null() && (*ctx).error.is_null() {
        (*ctx).error = error;
    } else {
        g_clear_error(&raw mut error);
    }
    if (*ctx).completed == 4 as guint {
        safe_c2rust_splice_complete(task, ctx);
    }
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_splice_cb(
    mut ostream: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut ctx: *mut SpliceContext = g_task_get_task_data(task) as *mut SpliceContext;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    g_output_stream_splice_finish(
        ostream as *mut ::core::ffi::c_void as *mut GOutputStream,
        res,
        &raw mut error,
    );
    (*ctx).completed = (*ctx).completed.wrapping_add(1);
    if !error.is_null()
        && g_error_matches(
            error,
            g_io_error_quark(),
            G_IO_ERROR_CANCELLED as ::core::ffi::c_int as gint,
        ) != 0
        && ((*ctx).cancellable.is_null() || g_cancellable_is_cancelled((*ctx).cancellable) == 0)
    {
        g_clear_error(&raw mut error);
    }
    if !error.is_null() && (*ctx).error.is_null() {
        (*ctx).error = error;
    } else {
        g_clear_error(&raw mut error);
    }
    if (*ctx).completed == 1 as guint
        && (*ctx).flags as ::core::ffi::c_uint
            & G_IO_STREAM_SPLICE_WAIT_FOR_BOTH as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint
    {
        g_cancellable_cancel((*ctx).op1_cancellable);
        g_cancellable_cancel((*ctx).op2_cancellable);
    } else if (*ctx).completed == 2 as guint {
        if (*ctx).cancellable.is_null() || g_cancellable_is_cancelled((*ctx).cancellable) == 0 {
            g_cancellable_reset((*ctx).op1_cancellable);
            g_cancellable_reset((*ctx).op2_cancellable);
        }
        if (*ctx).flags as ::core::ffi::c_uint
            & G_IO_STREAM_SPLICE_CLOSE_STREAM1 as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint
        {
            safe_c2rust_g_io_stream_close_async(
                (*ctx).stream1,
                g_task_get_priority(task) as ::core::ffi::c_int,
                (*ctx).op1_cancellable,
                Some(
                    safe_c2rust_splice_close_cb
                        as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
                ),
                g_object_ref(task as gpointer) as *mut GTask as gpointer,
            );
        } else {
            (*ctx).completed = (*ctx).completed.wrapping_add(1);
        }
        if (*ctx).flags as ::core::ffi::c_uint
            & G_IO_STREAM_SPLICE_CLOSE_STREAM2 as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint
        {
            safe_c2rust_g_io_stream_close_async(
                (*ctx).stream2,
                g_task_get_priority(task) as ::core::ffi::c_int,
                (*ctx).op2_cancellable,
                Some(
                    safe_c2rust_splice_close_cb
                        as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
                ),
                g_object_ref(task as gpointer) as *mut GTask as gpointer,
            );
        } else {
            (*ctx).completed = (*ctx).completed.wrapping_add(1);
        }
        if (*ctx).completed == 4 as guint {
            safe_c2rust_splice_complete(task, ctx);
        }
    }
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_splice_cancelled_cb(
    mut cancellable: *mut GCancellable,
    mut task: *mut GTask,
) {
    let mut ctx: *mut SpliceContext = ::core::ptr::null_mut::<SpliceContext>();
    ctx = g_task_get_task_data(task) as *mut SpliceContext;
    g_cancellable_cancel((*ctx).op1_cancellable);
    g_cancellable_cancel((*ctx).op2_cancellable);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_stream_splice_async(
    mut stream1: *mut GIOStream,
    mut stream2: *mut GIOStream,
    mut flags: GIOStreamSpliceFlags,
    mut io_priority: gint,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut ctx: *mut SpliceContext = ::core::ptr::null_mut::<SpliceContext>();
    let mut istream: *mut GInputStream = ::core::ptr::null_mut::<GInputStream>();
    let mut ostream: *mut GOutputStream = ::core::ptr::null_mut::<GOutputStream>();
    if !cancellable.is_null() && g_cancellable_is_cancelled(cancellable) != 0 {
        g_task_report_new_error(
            NULL,
            callback,
            user_data,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GIOStream,
                        *mut GIOStream,
                        GIOStreamSpliceFlags,
                        ::core::ffi::c_int,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_io_stream_splice_async
                    as unsafe extern "C" fn(
                        *mut GIOStream,
                        *mut GIOStream,
                        GIOStreamSpliceFlags,
                        gint,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )),
            g_io_error_quark(),
            G_IO_ERROR_CANCELLED as ::core::ffi::c_int as gint,
            b"Operation has been cancelled\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    ctx = ({
        let mut __s: gsize = ::core::mem::size_of::<SpliceContext>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut SpliceContext;
    (*ctx).stream1 = g_object_ref(stream1 as gpointer) as *mut GIOStream as *mut GIOStream;
    (*ctx).stream2 = g_object_ref(stream2 as gpointer) as *mut GIOStream as *mut GIOStream;
    (*ctx).flags = flags;
    (*ctx).op1_cancellable = g_cancellable_new();
    (*ctx).op2_cancellable = g_cancellable_new();
    (*ctx).completed = 0 as guint;
    task = g_task_new(NULL, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GIOStream,
                    *mut GIOStream,
                    GIOStreamSpliceFlags,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_io_stream_splice_async
                as unsafe extern "C" fn(
                    *mut GIOStream,
                    *mut GIOStream,
                    GIOStreamSpliceFlags,
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
            b"g_io_stream_splice_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        ctx as gpointer,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut SpliceContext) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_splice_context_free as unsafe extern "C" fn(*mut SpliceContext) -> (),
        )),
    );
    if !cancellable.is_null() {
        (*ctx).cancellable =
            g_object_ref(cancellable as gpointer) as *mut GCancellable as *mut GCancellable;
        (*ctx).cancelled_id = g_cancellable_connect(
            cancellable,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GCancellable, *mut GTask) -> ()>,
                GCallback,
            >(Some(
                safe_c2rust_splice_cancelled_cb
                    as unsafe extern "C" fn(*mut GCancellable, *mut GTask) -> (),
            )),
            g_object_ref(task as gpointer) as *mut GTask as gpointer,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    }
    istream = safe_c2rust_g_io_stream_get_input_stream(stream1);
    ostream = safe_c2rust_g_io_stream_get_output_stream(stream2);
    g_output_stream_splice_async(
        ostream,
        istream,
        G_OUTPUT_STREAM_SPLICE_NONE,
        io_priority as ::core::ffi::c_int,
        (*ctx).op1_cancellable,
        Some(
            safe_c2rust_splice_cb
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        g_object_ref(task as gpointer) as *mut GTask as gpointer,
    );
    istream = safe_c2rust_g_io_stream_get_input_stream(stream2);
    ostream = safe_c2rust_g_io_stream_get_output_stream(stream1);
    g_output_stream_splice_async(
        ostream,
        istream,
        G_OUTPUT_STREAM_SPLICE_NONE,
        io_priority as ::core::ffi::c_int,
        (*ctx).op2_cancellable,
        Some(
            safe_c2rust_splice_cb
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        g_object_ref(task as gpointer) as *mut GTask as gpointer,
    );
    g_object_unref(task as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_stream_splice_finish(
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if g_task_is_valid(
            result as gpointer,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ) != 0
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
            b"g_task_is_valid (result, NULL)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
