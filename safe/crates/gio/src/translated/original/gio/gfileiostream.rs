extern "C" {
    pub type _GData;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _GFileInfo;
    pub type _GFileOutputStreamPrivate;
    pub type _GOutputStreamPrivate;
    pub type _GIOStreamPrivate;
    pub type _GInputStreamPrivate;
    pub type _GSeekable;
    pub type _GTask;
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
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
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
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_cancellable_push_current(cancellable: *mut GCancellable);
    fn g_cancellable_pop_current(cancellable: *mut GCancellable);
    fn g_io_error_quark() -> GQuark;
    fn g_io_stream_get_type() -> GType;
    fn g_io_stream_get_output_stream(stream: *mut GIOStream) -> *mut GOutputStream;
    fn g_io_stream_is_closed(stream: *mut GIOStream) -> gboolean;
    fn g_io_stream_set_pending(stream: *mut GIOStream, error: *mut *mut GError) -> gboolean;
    fn g_io_stream_clear_pending(stream: *mut GIOStream);
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
    fn g_async_result_get_type() -> GType;
    fn g_async_result_legacy_propagate_error(
        res: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_async_result_is_tagged(res: *mut GAsyncResult, source_tag: gpointer) -> gboolean;
    fn g_task_report_error(
        source_object: gpointer,
        callback: GAsyncReadyCallback,
        callback_data: gpointer,
        source_tag: gpointer,
        error: *mut GError,
    );
    fn g_task_propagate_pointer(task: *mut GTask, error: *mut *mut GError) -> gpointer;
    fn g_file_output_stream_query_info(
        stream: *mut GFileOutputStream,
        attributes: *const ::core::ffi::c_char,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GFileInfo;
    fn g_file_output_stream_query_info_async(
        stream: *mut GFileOutputStream,
        attributes: *const ::core::ffi::c_char,
        io_priority: ::core::ffi::c_int,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_file_output_stream_query_info_finish(
        stream: *mut GFileOutputStream,
        result: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> *mut GFileInfo;
    fn g_file_output_stream_get_etag(stream: *mut GFileOutputStream) -> *mut ::core::ffi::c_char;
    fn glib_gettext(str: *const gchar) -> *const gchar;
}
pub type guint8 = ::core::ffi::c_uchar;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
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
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
pub type GFileInfo = _GFileInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileOutputStream {
    pub parent_instance: GOutputStream,
    pub priv_0: *mut GFileOutputStreamPrivate,
}
pub type GFileOutputStreamPrivate = _GFileOutputStreamPrivate;
pub type GOutputStream = _GOutputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GOutputStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GOutputStreamPrivate,
}
pub type GOutputStreamPrivate = _GOutputStreamPrivate;
pub type GFileOutputStream = _GFileOutputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileIOStream {
    pub parent_instance: GIOStream,
    pub priv_0: *mut GFileIOStreamPrivate,
}
pub type GFileIOStreamPrivate = _GFileIOStreamPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileIOStreamPrivate {
    pub outstanding_callback: GAsyncReadyCallback,
}
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
pub type GIOStream = _GIOStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GIOStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GIOStreamPrivate,
}
pub type GIOStreamPrivate = _GIOStreamPrivate;
pub type GFileIOStream = _GFileIOStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInputStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GInputStreamPrivate,
}
pub type GInputStreamPrivate = _GInputStreamPrivate;
pub type GInputStream = _GInputStream;
pub type GSeekable = _GSeekable;
pub type GTask = _GTask;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileIOStreamClass {
    pub parent_class: GIOStreamClass,
    pub tell: Option<unsafe extern "C" fn(*mut GFileIOStream) -> goffset>,
    pub can_seek: Option<unsafe extern "C" fn(*mut GFileIOStream) -> gboolean>,
    pub seek: Option<
        unsafe extern "C" fn(
            *mut GFileIOStream,
            goffset,
            GSeekType,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub can_truncate: Option<unsafe extern "C" fn(*mut GFileIOStream) -> gboolean>,
    pub truncate_fn: Option<
        unsafe extern "C" fn(
            *mut GFileIOStream,
            goffset,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub query_info: Option<
        unsafe extern "C" fn(
            *mut GFileIOStream,
            *const ::core::ffi::c_char,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileInfo,
    >,
    pub query_info_async: Option<
        unsafe extern "C" fn(
            *mut GFileIOStream,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub query_info_finish: Option<
        unsafe extern "C" fn(
            *mut GFileIOStream,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> *mut GFileInfo,
    >,
    pub get_etag: Option<unsafe extern "C" fn(*mut GFileIOStream) -> *mut ::core::ffi::c_char>,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
}
pub type GFileIOStreamClass = _GFileIOStreamClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AsyncOpWrapper {
    pub object: *mut GObject,
    pub callback: GAsyncReadyCallback,
    pub user_data: gpointer,
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
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn safe_c2rust_g_file_io_stream_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_file_io_stream_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GFileIOStream_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GFileIOStream_private_offset,
        );
    }
    safe_c2rust_g_file_io_stream_class_init(klass as *mut GFileIOStreamClass);
}
static mut safe_c2rust_GFileIOStream_private_offset: gint = 0;
#[inline]
unsafe extern "C" fn safe_c2rust_g_file_io_stream_get_instance_private(
    mut self_0: *mut GFileIOStream,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GFileIOStream_private_offset as glong as isize) as gpointer;
}
static mut safe_c2rust_g_file_io_stream_parent_class: gpointer = NULL;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_file_io_stream_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_io_stream_get_type(),
        g_intern_static_string(b"GFileIOStream\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GFileIOStreamClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_file_io_stream_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GFileIOStream>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GFileIOStream) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_file_io_stream_init as unsafe extern "C" fn(*mut GFileIOStream) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GFileIOStream_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GFileIOStreamPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GSeekableIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_file_io_stream_seekable_iface_init
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
pub unsafe extern "C" fn safe_c2rust_g_file_io_stream_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_file_io_stream_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_file_io_stream_seekable_iface_init(
    mut iface: *mut GSeekableIface,
) {
    (*iface).tell = Some(
        safe_c2rust_g_file_io_stream_seekable_tell
            as unsafe extern "C" fn(*mut GSeekable) -> goffset,
    ) as Option<unsafe extern "C" fn(*mut GSeekable) -> goffset>;
    (*iface).can_seek = Some(
        safe_c2rust_g_file_io_stream_seekable_can_seek
            as unsafe extern "C" fn(*mut GSeekable) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut GSeekable) -> gboolean>;
    (*iface).seek = Some(
        safe_c2rust_g_file_io_stream_seekable_seek
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
        safe_c2rust_g_file_io_stream_seekable_can_truncate
            as unsafe extern "C" fn(*mut GSeekable) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut GSeekable) -> gboolean>;
    (*iface).truncate_fn = Some(
        safe_c2rust_g_file_io_stream_seekable_truncate
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
unsafe extern "C" fn safe_c2rust_g_file_io_stream_init(mut stream: *mut GFileIOStream) {
    (*stream).priv_0 =
        safe_c2rust_g_file_io_stream_get_instance_private(stream) as *mut GFileIOStreamPrivate;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_io_stream_query_info(
    mut stream: *mut GFileIOStream,
    mut attributes: *const ::core::ffi::c_char,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileInfo {
    let mut class: *mut GFileIOStreamClass = ::core::ptr::null_mut::<GFileIOStreamClass>();
    let mut io_stream: *mut GIOStream = ::core::ptr::null_mut::<GIOStream>();
    let mut info: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_io_stream_get_type();
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
            b"G_IS_FILE_IO_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileInfo>();
    }
    io_stream = stream as *mut ::core::ffi::c_void as *mut GIOStream;
    if g_io_stream_set_pending(io_stream, error) == 0 {
        return ::core::ptr::null_mut::<GFileInfo>();
    }
    info = ::core::ptr::null_mut::<GFileInfo>();
    if !cancellable.is_null() {
        g_cancellable_push_current(cancellable);
    }
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GFileIOStreamClass;
    if (*class).query_info.is_some() {
        info = (*class).query_info.expect("non-null function pointer")(
            stream,
            attributes,
            cancellable,
            error,
        );
    } else {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Stream doesn\xE2\x80\x99t support query_info\0" as *const u8 as *const gchar,
            ),
        );
    }
    if !cancellable.is_null() {
        g_cancellable_pop_current(cancellable);
    }
    g_io_stream_clear_pending(io_stream);
    return info;
}
unsafe extern "C" fn safe_c2rust_async_ready_callback_wrapper(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut stream: *mut GFileIOStream =
        source_object as *mut ::core::ffi::c_void as *mut GFileIOStream;
    g_io_stream_clear_pending(stream as *mut ::core::ffi::c_void as *mut GIOStream);
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
pub unsafe extern "C" fn safe_c2rust_g_file_io_stream_query_info_async(
    mut stream: *mut GFileIOStream,
    mut attributes: *const ::core::ffi::c_char,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut klass: *mut GFileIOStreamClass = ::core::ptr::null_mut::<GFileIOStreamClass>();
    let mut io_stream: *mut GIOStream = ::core::ptr::null_mut::<GIOStream>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_io_stream_get_type();
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
            b"G_IS_FILE_IO_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    io_stream = stream as *mut ::core::ffi::c_void as *mut GIOStream;
    if g_io_stream_set_pending(io_stream, &raw mut error) == 0 {
        g_task_report_error(
            stream as gpointer,
            callback,
            user_data,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GFileIOStream,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_file_io_stream_query_info_async
                    as unsafe extern "C" fn(
                        *mut GFileIOStream,
                        *const ::core::ffi::c_char,
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
    klass = (*(stream as *mut GTypeInstance)).g_class as *mut GFileIOStreamClass;
    (*(*stream).priv_0).outstanding_callback = callback;
    g_object_ref(stream as gpointer);
    (*klass)
        .query_info_async
        .expect("non-null function pointer")(
        stream,
        attributes,
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
pub unsafe extern "C" fn safe_c2rust_g_file_io_stream_query_info_finish(
    mut stream: *mut GFileIOStream,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GFileInfo {
    let mut class: *mut GFileIOStreamClass = ::core::ptr::null_mut::<GFileIOStreamClass>();
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_io_stream_get_type();
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
            b"G_IS_FILE_IO_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileInfo>();
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
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
            b"G_IS_ASYNC_RESULT (result)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileInfo>();
    }
    if g_async_result_legacy_propagate_error(result, error) != 0 {
        return ::core::ptr::null_mut::<GFileInfo>();
    } else if g_async_result_is_tagged(
        result,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GFileIOStream,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_io_stream_query_info_async
                as unsafe extern "C" fn(
                    *mut GFileIOStream,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    ) != 0
    {
        return g_task_propagate_pointer(result as *mut ::core::ffi::c_void as *mut GTask, error)
            as *mut GFileInfo;
    }
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GFileIOStreamClass;
    return (*class)
        .query_info_finish
        .expect("non-null function pointer")(stream, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_io_stream_get_etag(
    mut stream: *mut GFileIOStream,
) -> *mut ::core::ffi::c_char {
    let mut class: *mut GFileIOStreamClass = ::core::ptr::null_mut::<GFileIOStreamClass>();
    let mut io_stream: *mut GIOStream = ::core::ptr::null_mut::<GIOStream>();
    let mut etag: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_io_stream_get_type();
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
            b"G_IS_FILE_IO_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    io_stream = stream as *mut ::core::ffi::c_void as *mut GIOStream;
    if g_io_stream_is_closed(io_stream) == 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"stream is not closed yet, can't get etag\0" as *const u8 as *const gchar,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    etag = ::core::ptr::null_mut::<::core::ffi::c_char>();
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GFileIOStreamClass;
    if (*class).get_etag.is_some() {
        etag = (*class).get_etag.expect("non-null function pointer")(stream);
    }
    return etag;
}
unsafe extern "C" fn safe_c2rust_g_file_io_stream_tell(mut stream: *mut GFileIOStream) -> goffset {
    let mut class: *mut GFileIOStreamClass = ::core::ptr::null_mut::<GFileIOStreamClass>();
    let mut offset: goffset = 0;
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_io_stream_get_type();
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
            b"G_IS_FILE_IO_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as goffset;
    }
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GFileIOStreamClass;
    offset = 0 as goffset;
    if (*class).tell.is_some() {
        offset = (*class).tell.expect("non-null function pointer")(stream);
    }
    return offset;
}
unsafe extern "C" fn safe_c2rust_g_file_io_stream_seekable_tell(
    mut seekable: *mut GSeekable,
) -> goffset {
    return safe_c2rust_g_file_io_stream_tell(
        seekable as *mut ::core::ffi::c_void as *mut GFileIOStream,
    );
}
unsafe extern "C" fn safe_c2rust_g_file_io_stream_can_seek(
    mut stream: *mut GFileIOStream,
) -> gboolean {
    let mut class: *mut GFileIOStreamClass = ::core::ptr::null_mut::<GFileIOStreamClass>();
    let mut can_seek: gboolean = 0;
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_io_stream_get_type();
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
            b"G_IS_FILE_IO_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GFileIOStreamClass;
    can_seek = FALSE as gboolean;
    if (*class).seek.is_some() {
        can_seek = TRUE as gboolean;
        if (*class).can_seek.is_some() {
            can_seek = (*class).can_seek.expect("non-null function pointer")(stream);
        }
    }
    return can_seek;
}
unsafe extern "C" fn safe_c2rust_g_file_io_stream_seekable_can_seek(
    mut seekable: *mut GSeekable,
) -> gboolean {
    return safe_c2rust_g_file_io_stream_can_seek(
        seekable as *mut ::core::ffi::c_void as *mut GFileIOStream,
    );
}
unsafe extern "C" fn safe_c2rust_g_file_io_stream_seek(
    mut stream: *mut GFileIOStream,
    mut offset: goffset,
    mut type_0: GSeekType,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut class: *mut GFileIOStreamClass = ::core::ptr::null_mut::<GFileIOStreamClass>();
    let mut io_stream: *mut GIOStream = ::core::ptr::null_mut::<GIOStream>();
    let mut res: gboolean = 0;
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_io_stream_get_type();
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
            b"G_IS_FILE_IO_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    io_stream = stream as *mut ::core::ffi::c_void as *mut GIOStream;
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GFileIOStreamClass;
    if (*class).seek.is_none() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(b"Seek not supported on stream\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    if g_io_stream_set_pending(io_stream, error) == 0 {
        return FALSE;
    }
    if !cancellable.is_null() {
        g_cancellable_push_current(cancellable);
    }
    res = (*class).seek.expect("non-null function pointer")(
        stream,
        offset,
        type_0,
        cancellable,
        error,
    );
    if !cancellable.is_null() {
        g_cancellable_pop_current(cancellable);
    }
    g_io_stream_clear_pending(io_stream);
    return res;
}
unsafe extern "C" fn safe_c2rust_g_file_io_stream_seekable_seek(
    mut seekable: *mut GSeekable,
    mut offset: goffset,
    mut type_0: GSeekType,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    return safe_c2rust_g_file_io_stream_seek(
        seekable as *mut ::core::ffi::c_void as *mut GFileIOStream,
        offset,
        type_0,
        cancellable,
        error,
    );
}
unsafe extern "C" fn safe_c2rust_g_file_io_stream_can_truncate(
    mut stream: *mut GFileIOStream,
) -> gboolean {
    let mut class: *mut GFileIOStreamClass = ::core::ptr::null_mut::<GFileIOStreamClass>();
    let mut can_truncate: gboolean = 0;
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_io_stream_get_type();
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
            b"G_IS_FILE_IO_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GFileIOStreamClass;
    can_truncate = FALSE as gboolean;
    if (*class).truncate_fn.is_some() {
        can_truncate = TRUE as gboolean;
        if (*class).can_truncate.is_some() {
            can_truncate = (*class).can_truncate.expect("non-null function pointer")(stream);
        }
    }
    return can_truncate;
}
unsafe extern "C" fn safe_c2rust_g_file_io_stream_seekable_can_truncate(
    mut seekable: *mut GSeekable,
) -> gboolean {
    return safe_c2rust_g_file_io_stream_can_truncate(
        seekable as *mut ::core::ffi::c_void as *mut GFileIOStream,
    );
}
unsafe extern "C" fn safe_c2rust_g_file_io_stream_truncate(
    mut stream: *mut GFileIOStream,
    mut size: goffset,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut class: *mut GFileIOStreamClass = ::core::ptr::null_mut::<GFileIOStreamClass>();
    let mut io_stream: *mut GIOStream = ::core::ptr::null_mut::<GIOStream>();
    let mut res: gboolean = 0;
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_io_stream_get_type();
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
            b"G_IS_FILE_IO_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    io_stream = stream as *mut ::core::ffi::c_void as *mut GIOStream;
    class = (*(stream as *mut GTypeInstance)).g_class as *mut GFileIOStreamClass;
    if (*class).truncate_fn.is_none() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(b"Truncate not supported on stream\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    if g_io_stream_set_pending(io_stream, error) == 0 {
        return FALSE;
    }
    if !cancellable.is_null() {
        g_cancellable_push_current(cancellable);
    }
    res =
        (*class).truncate_fn.expect("non-null function pointer")(stream, size, cancellable, error);
    if !cancellable.is_null() {
        g_cancellable_pop_current(cancellable);
    }
    g_io_stream_clear_pending(io_stream);
    return res;
}
unsafe extern "C" fn safe_c2rust_g_file_io_stream_seekable_truncate(
    mut seekable: *mut GSeekable,
    mut size: goffset,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    return safe_c2rust_g_file_io_stream_truncate(
        seekable as *mut ::core::ffi::c_void as *mut GFileIOStream,
        size,
        cancellable,
        error,
    );
}
unsafe extern "C" fn safe_c2rust_g_file_io_stream_real_tell(
    mut stream: *mut GFileIOStream,
) -> goffset {
    let mut out: *mut GOutputStream = ::core::ptr::null_mut::<GOutputStream>();
    let mut seekable: *mut GSeekable = ::core::ptr::null_mut::<GSeekable>();
    out = g_io_stream_get_output_stream(stream as *mut ::core::ffi::c_void as *mut GIOStream);
    seekable = out as *mut ::core::ffi::c_void as *mut GSeekable;
    return g_seekable_tell(seekable);
}
unsafe extern "C" fn safe_c2rust_g_file_io_stream_real_can_seek(
    mut stream: *mut GFileIOStream,
) -> gboolean {
    let mut out: *mut GOutputStream = ::core::ptr::null_mut::<GOutputStream>();
    let mut seekable: *mut GSeekable = ::core::ptr::null_mut::<GSeekable>();
    out = g_io_stream_get_output_stream(stream as *mut ::core::ffi::c_void as *mut GIOStream);
    seekable = out as *mut ::core::ffi::c_void as *mut GSeekable;
    return g_seekable_can_seek(seekable);
}
unsafe extern "C" fn safe_c2rust_g_file_io_stream_real_seek(
    mut stream: *mut GFileIOStream,
    mut offset: goffset,
    mut type_0: GSeekType,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut out: *mut GOutputStream = ::core::ptr::null_mut::<GOutputStream>();
    let mut seekable: *mut GSeekable = ::core::ptr::null_mut::<GSeekable>();
    out = g_io_stream_get_output_stream(stream as *mut ::core::ffi::c_void as *mut GIOStream);
    seekable = out as *mut ::core::ffi::c_void as *mut GSeekable;
    return g_seekable_seek(seekable, offset, type_0, cancellable, error);
}
unsafe extern "C" fn safe_c2rust_g_file_io_stream_real_can_truncate(
    mut stream: *mut GFileIOStream,
) -> gboolean {
    let mut out: *mut GOutputStream = ::core::ptr::null_mut::<GOutputStream>();
    let mut seekable: *mut GSeekable = ::core::ptr::null_mut::<GSeekable>();
    out = g_io_stream_get_output_stream(stream as *mut ::core::ffi::c_void as *mut GIOStream);
    seekable = out as *mut ::core::ffi::c_void as *mut GSeekable;
    return g_seekable_can_truncate(seekable);
}
unsafe extern "C" fn safe_c2rust_g_file_io_stream_real_truncate_fn(
    mut stream: *mut GFileIOStream,
    mut size: goffset,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut out: *mut GOutputStream = ::core::ptr::null_mut::<GOutputStream>();
    let mut seekable: *mut GSeekable = ::core::ptr::null_mut::<GSeekable>();
    out = g_io_stream_get_output_stream(stream as *mut ::core::ffi::c_void as *mut GIOStream);
    seekable = out as *mut ::core::ffi::c_void as *mut GSeekable;
    return g_seekable_truncate(seekable, size, cancellable, error);
}
unsafe extern "C" fn safe_c2rust_g_file_io_stream_real_get_etag(
    mut stream: *mut GFileIOStream,
) -> *mut ::core::ffi::c_char {
    let mut out: *mut GOutputStream = ::core::ptr::null_mut::<GOutputStream>();
    let mut file_out: *mut GFileOutputStream = ::core::ptr::null_mut::<GFileOutputStream>();
    out = g_io_stream_get_output_stream(stream as *mut ::core::ffi::c_void as *mut GIOStream);
    file_out = out as *mut ::core::ffi::c_void as *mut GFileOutputStream;
    return g_file_output_stream_get_etag(file_out);
}
unsafe extern "C" fn safe_c2rust_g_file_io_stream_real_query_info(
    mut stream: *mut GFileIOStream,
    mut attributes: *const ::core::ffi::c_char,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileInfo {
    let mut out: *mut GOutputStream = ::core::ptr::null_mut::<GOutputStream>();
    let mut file_out: *mut GFileOutputStream = ::core::ptr::null_mut::<GFileOutputStream>();
    out = g_io_stream_get_output_stream(stream as *mut ::core::ffi::c_void as *mut GIOStream);
    file_out = out as *mut ::core::ffi::c_void as *mut GFileOutputStream;
    return g_file_output_stream_query_info(file_out, attributes, cancellable, error);
}
unsafe extern "C" fn safe_c2rust_async_op_wrapper_new(
    mut object: gpointer,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) -> *mut AsyncOpWrapper {
    let mut data: *mut AsyncOpWrapper = ::core::ptr::null_mut::<AsyncOpWrapper>();
    data = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<AsyncOpWrapper>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut AsyncOpWrapper;
    (*data).object = g_object_ref(object) as *mut GObject;
    (*data).callback = callback;
    (*data).user_data = user_data;
    return data;
}
unsafe extern "C" fn safe_c2rust_async_op_wrapper_callback(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut data: *mut AsyncOpWrapper = user_data as *mut AsyncOpWrapper;
    (*data).callback.expect("non-null function pointer")((*data).object, res, (*data).user_data);
    g_object_unref((*data).object as gpointer);
    g_free(data as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_file_io_stream_real_query_info_async(
    mut stream: *mut GFileIOStream,
    mut attributes: *const ::core::ffi::c_char,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut out: *mut GOutputStream = ::core::ptr::null_mut::<GOutputStream>();
    let mut file_out: *mut GFileOutputStream = ::core::ptr::null_mut::<GFileOutputStream>();
    let mut data: *mut AsyncOpWrapper = ::core::ptr::null_mut::<AsyncOpWrapper>();
    out = g_io_stream_get_output_stream(stream as *mut ::core::ffi::c_void as *mut GIOStream);
    file_out = out as *mut ::core::ffi::c_void as *mut GFileOutputStream;
    data = safe_c2rust_async_op_wrapper_new(stream as gpointer, callback, user_data);
    g_file_output_stream_query_info_async(
        file_out,
        attributes,
        io_priority,
        cancellable,
        Some(
            safe_c2rust_async_op_wrapper_callback
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        data as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_g_file_io_stream_real_query_info_finish(
    mut stream: *mut GFileIOStream,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GFileInfo {
    let mut out: *mut GOutputStream = ::core::ptr::null_mut::<GOutputStream>();
    let mut file_out: *mut GFileOutputStream = ::core::ptr::null_mut::<GFileOutputStream>();
    out = g_io_stream_get_output_stream(stream as *mut ::core::ffi::c_void as *mut GIOStream);
    file_out = out as *mut ::core::ffi::c_void as *mut GFileOutputStream;
    return g_file_output_stream_query_info_finish(file_out, res, error);
}
unsafe extern "C" fn safe_c2rust_g_file_io_stream_class_init(mut klass: *mut GFileIOStreamClass) {
    (*klass).tell = Some(
        safe_c2rust_g_file_io_stream_real_tell
            as unsafe extern "C" fn(*mut GFileIOStream) -> goffset,
    ) as Option<unsafe extern "C" fn(*mut GFileIOStream) -> goffset>;
    (*klass).can_seek = Some(
        safe_c2rust_g_file_io_stream_real_can_seek
            as unsafe extern "C" fn(*mut GFileIOStream) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut GFileIOStream) -> gboolean>;
    (*klass).seek = Some(
        safe_c2rust_g_file_io_stream_real_seek
            as unsafe extern "C" fn(
                *mut GFileIOStream,
                goffset,
                GSeekType,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFileIOStream,
                goffset,
                GSeekType,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
        >;
    (*klass).can_truncate = Some(
        safe_c2rust_g_file_io_stream_real_can_truncate
            as unsafe extern "C" fn(*mut GFileIOStream) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut GFileIOStream) -> gboolean>;
    (*klass).truncate_fn = Some(
        safe_c2rust_g_file_io_stream_real_truncate_fn
            as unsafe extern "C" fn(
                *mut GFileIOStream,
                goffset,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFileIOStream,
                goffset,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
        >;
    (*klass).query_info = Some(
        safe_c2rust_g_file_io_stream_real_query_info
            as unsafe extern "C" fn(
                *mut GFileIOStream,
                *const ::core::ffi::c_char,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileInfo,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFileIOStream,
                *const ::core::ffi::c_char,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileInfo,
        >;
    (*klass).query_info_async = Some(
        safe_c2rust_g_file_io_stream_real_query_info_async
            as unsafe extern "C" fn(
                *mut GFileIOStream,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFileIOStream,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*klass).query_info_finish = Some(
        safe_c2rust_g_file_io_stream_real_query_info_finish
            as unsafe extern "C" fn(
                *mut GFileIOStream,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GFileInfo,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFileIOStream,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GFileInfo,
        >;
    (*klass).get_etag = Some(
        safe_c2rust_g_file_io_stream_real_get_etag
            as unsafe extern "C" fn(*mut GFileIOStream) -> *mut ::core::ffi::c_char,
    )
        as Option<unsafe extern "C" fn(*mut GFileIOStream) -> *mut ::core::ffi::c_char>;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
