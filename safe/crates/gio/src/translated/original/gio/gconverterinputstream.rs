extern "C" {
    pub type _GData;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GAsyncResult;
    pub type _GInputStreamPrivate;
    pub type _GCancellablePrivate;
    pub type _GConverter;
    pub type _GPollableInputStream;
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
    fn g_error_matches(error: *const GError, domain: GQuark, code: gint) -> gboolean;
    fn g_propagate_error(dest: *mut *mut GError, src: *mut GError);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
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
    fn g_value_set_object(value: *mut GValue, v_object: gpointer);
    fn g_value_dup_object(value: *const GValue) -> gpointer;
    fn g_param_spec_object(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        object_type: GType,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_input_stream_get_type() -> GType;
    fn g_filter_input_stream_get_type() -> GType;
    fn g_converter_get_type() -> GType;
    fn g_converter_convert(
        converter: *mut GConverter,
        inbuf: *const ::core::ffi::c_void,
        inbuf_size: gsize,
        outbuf: *mut ::core::ffi::c_void,
        outbuf_size: gsize,
        flags: GConverterFlags,
        bytes_read: *mut gsize,
        bytes_written: *mut gsize,
        error: *mut *mut GError,
    ) -> GConverterResult;
    fn g_io_error_quark() -> GQuark;
    fn g_pollable_source_new_full(
        pollable_stream: gpointer,
        child_source: *mut GSource,
        cancellable: *mut GCancellable,
    ) -> *mut GSource;
    fn g_pollable_stream_read(
        stream: *mut GInputStream,
        buffer: *mut ::core::ffi::c_void,
        count: gsize,
        blocking: gboolean,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gssize;
    fn g_pollable_input_stream_get_type() -> GType;
    fn g_pollable_input_stream_can_poll(stream: *mut GPollableInputStream) -> gboolean;
    fn g_pollable_input_stream_is_readable(stream: *mut GPollableInputStream) -> gboolean;
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
pub type GConverterFlags = ::core::ffi::c_uint;
pub const G_CONVERTER_FLUSH: GConverterFlags = 2;
pub const G_CONVERTER_INPUT_AT_END: GConverterFlags = 1;
pub const G_CONVERTER_NO_FLAGS: GConverterFlags = 0;
pub type GConverterResult = ::core::ffi::c_uint;
pub const G_CONVERTER_FLUSHED: GConverterResult = 3;
pub const G_CONVERTER_FINISHED: GConverterResult = 2;
pub const G_CONVERTER_CONVERTED: GConverterResult = 1;
pub const G_CONVERTER_ERROR: GConverterResult = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
pub type GConverter = _GConverter;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GConverterInputStream {
    pub parent_instance: GFilterInputStream,
    pub priv_0: *mut GConverterInputStreamPrivate,
}
pub type GConverterInputStreamPrivate = _GConverterInputStreamPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GConverterInputStreamPrivate {
    pub at_input_end: gboolean,
    pub finished: gboolean,
    pub need_input: gboolean,
    pub converter: *mut GConverter,
    pub input_buffer: Buffer,
    pub converted_buffer: Buffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Buffer {
    pub data: *mut ::core::ffi::c_char,
    pub start: gsize,
    pub end: gsize,
    pub size: gsize,
}
pub type GConverterInputStream = _GConverterInputStream;
pub type GPollableInputStream = _GPollableInputStream;
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
pub struct _GConverterInputStreamClass {
    pub parent_class: GFilterInputStreamClass,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
}
pub type GConverterInputStreamClass = _GConverterInputStreamClass;
pub const PROP_CONVERTER: C2RustUnnamed_1 = 1;
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
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_1 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const INITIAL_BUFFER_SIZE: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
static mut safe_c2rust_GConverterInputStream_private_offset: gint = 0;
static mut safe_c2rust_g_converter_input_stream_parent_class: gpointer = NULL;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_converter_input_stream_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_filter_input_stream_get_type(),
        g_intern_static_string(b"GConverterInputStream\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GConverterInputStreamClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_converter_input_stream_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GConverterInputStream>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GConverterInputStream) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_converter_input_stream_init
                    as unsafe extern "C" fn(*mut GConverterInputStream) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GConverterInputStream_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GConverterInputStreamPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GPollableInputStreamInterface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_converter_input_stream_pollable_iface_init
                as unsafe extern "C" fn(*mut GPollableInputStreamInterface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_pollable_input_stream_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_converter_input_stream_get_instance_private(
    mut self_0: *mut GConverterInputStream,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GConverterInputStream_private_offset as glong as isize)
        as gpointer;
}
unsafe extern "C" fn safe_c2rust_g_converter_input_stream_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_converter_input_stream_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GConverterInputStream_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GConverterInputStream_private_offset,
        );
    }
    safe_c2rust_g_converter_input_stream_class_init(klass as *mut GConverterInputStreamClass);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_converter_input_stream_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_converter_input_stream_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_converter_input_stream_class_init(
    mut klass: *mut GConverterInputStreamClass,
) {
    let mut object_class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    let mut istream_class: *mut GInputStreamClass = ::core::ptr::null_mut::<GInputStreamClass>();
    object_class = klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*object_class).get_property = Some(
        safe_c2rust_g_converter_input_stream_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*object_class).set_property = Some(
        safe_c2rust_g_converter_input_stream_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*object_class).finalize = Some(
        safe_c2rust_g_converter_input_stream_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    istream_class = klass as *mut ::core::ffi::c_void as *mut GInputStreamClass;
    (*istream_class).read_fn = Some(
        safe_c2rust_g_converter_input_stream_read
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
    g_object_class_install_property(
        object_class,
        PROP_CONVERTER as ::core::ffi::c_int as guint,
        g_param_spec_object(
            b"converter\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_converter_get_type(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
unsafe extern "C" fn safe_c2rust_g_converter_input_stream_pollable_iface_init(
    mut iface: *mut GPollableInputStreamInterface,
) {
    (*iface).can_poll = Some(
        safe_c2rust_g_converter_input_stream_can_poll
            as unsafe extern "C" fn(*mut GPollableInputStream) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GPollableInputStream) -> gboolean>;
    (*iface).is_readable = Some(
        safe_c2rust_g_converter_input_stream_is_readable
            as unsafe extern "C" fn(*mut GPollableInputStream) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GPollableInputStream) -> gboolean>;
    (*iface).read_nonblocking = Some(
        safe_c2rust_g_converter_input_stream_read_nonblocking
            as unsafe extern "C" fn(
                *mut GPollableInputStream,
                *mut ::core::ffi::c_void,
                gsize,
                *mut *mut GError,
            ) -> gssize,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GPollableInputStream,
                *mut ::core::ffi::c_void,
                gsize,
                *mut *mut GError,
            ) -> gssize,
        >;
    (*iface).create_source = Some(
        safe_c2rust_g_converter_input_stream_create_source
            as unsafe extern "C" fn(*mut GPollableInputStream, *mut GCancellable) -> *mut GSource,
    )
        as Option<
            unsafe extern "C" fn(*mut GPollableInputStream, *mut GCancellable) -> *mut GSource,
        >;
}
unsafe extern "C" fn safe_c2rust_g_converter_input_stream_finalize(mut object: *mut GObject) {
    let mut priv_0: *mut GConverterInputStreamPrivate =
        ::core::ptr::null_mut::<GConverterInputStreamPrivate>();
    let mut stream: *mut GConverterInputStream = ::core::ptr::null_mut::<GConverterInputStream>();
    stream = object as *mut ::core::ffi::c_void as *mut GConverterInputStream;
    priv_0 = (*stream).priv_0;
    g_free((*priv_0).input_buffer.data as gpointer);
    g_free((*priv_0).converted_buffer.data as gpointer);
    if !(*priv_0).converter.is_null() {
        g_object_unref((*priv_0).converter as gpointer);
    }
    (*(safe_c2rust_g_converter_input_stream_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_converter_input_stream_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut cstream: *mut GConverterInputStream = ::core::ptr::null_mut::<GConverterInputStream>();
    cstream = object as *mut ::core::ffi::c_void as *mut GConverterInputStream;
    match prop_id {
        1 => {
            (*(*cstream).priv_0).converter = g_value_dup_object(value) as *mut GConverter;
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gconverterinputstream.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                174 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_converter_input_stream_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut priv_0: *mut GConverterInputStreamPrivate =
        ::core::ptr::null_mut::<GConverterInputStreamPrivate>();
    let mut cstream: *mut GConverterInputStream = ::core::ptr::null_mut::<GConverterInputStream>();
    cstream = object as *mut ::core::ffi::c_void as *mut GConverterInputStream;
    priv_0 = (*cstream).priv_0;
    match prop_id {
        1 => {
            g_value_set_object(value, (*priv_0).converter as gpointer);
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gconverterinputstream.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                199 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_converter_input_stream_init(
    mut stream: *mut GConverterInputStream,
) {
    (*stream).priv_0 = safe_c2rust_g_converter_input_stream_get_instance_private(stream)
        as *mut GConverterInputStreamPrivate;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_converter_input_stream_new(
    mut base_stream: *mut GInputStream,
    mut converter: *mut GConverter,
) -> *mut GInputStream {
    let mut stream: *mut GInputStream = ::core::ptr::null_mut::<GInputStream>();
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
        return ::core::ptr::null_mut::<GInputStream>();
    }
    stream = g_object_new(
        safe_c2rust_g_converter_input_stream_get_type(),
        b"base-stream\0" as *const u8 as *const gchar,
        base_stream,
        b"converter\0" as *const u8 as *const ::core::ffi::c_char,
        converter,
        NULL,
    ) as *mut GInputStream;
    return stream;
}
unsafe extern "C" fn safe_c2rust_buffer_data_size(mut buffer: *mut Buffer) -> gsize {
    return (*buffer).end.wrapping_sub((*buffer).start);
}
unsafe extern "C" fn safe_c2rust_buffer_tailspace(mut buffer: *mut Buffer) -> gsize {
    return (*buffer).size.wrapping_sub((*buffer).end);
}
unsafe extern "C" fn safe_c2rust_buffer_data(mut buffer: *mut Buffer) -> *mut ::core::ffi::c_char {
    return (*buffer).data.offset((*buffer).start as isize);
}
unsafe extern "C" fn safe_c2rust_buffer_consumed(mut buffer: *mut Buffer, mut count: gsize) {
    (*buffer).start = (*buffer).start.wrapping_add(count);
    if (*buffer).start == (*buffer).end {
        (*buffer).end = 0 as gsize;
        (*buffer).start = (*buffer).end;
    }
}
unsafe extern "C" fn safe_c2rust_buffer_read(
    mut buffer: *mut Buffer,
    mut dest: *mut ::core::ffi::c_char,
    mut count: gsize,
) {
    if count != 0 as gsize {
        memcpy(
            dest as *mut ::core::ffi::c_void,
            (*buffer).data.offset((*buffer).start as isize) as *const ::core::ffi::c_void,
            count as size_t,
        );
    }
    safe_c2rust_buffer_consumed(buffer, count);
}
unsafe extern "C" fn safe_c2rust_compact_buffer(mut buffer: *mut Buffer) {
    let mut in_buffer: gsize = 0;
    in_buffer = safe_c2rust_buffer_data_size(buffer);
    memmove(
        (*buffer).data as *mut ::core::ffi::c_void,
        (*buffer).data.offset((*buffer).start as isize) as *const ::core::ffi::c_void,
        in_buffer as size_t,
    );
    (*buffer).end = (*buffer).end.wrapping_sub((*buffer).start);
    (*buffer).start = 0 as gsize;
}
unsafe extern "C" fn safe_c2rust_grow_buffer(mut buffer: *mut Buffer) {
    let mut data: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut size: gsize = 0;
    let mut in_buffer: gsize = 0;
    if (*buffer).size == 0 as gsize {
        size = INITIAL_BUFFER_SIZE as gsize;
    } else {
        size = (*buffer).size.wrapping_mul(2 as gsize);
    }
    data = g_malloc(size) as *mut ::core::ffi::c_char;
    in_buffer = safe_c2rust_buffer_data_size(buffer);
    if in_buffer != 0 as gsize {
        memcpy(
            data as *mut ::core::ffi::c_void,
            (*buffer).data.offset((*buffer).start as isize) as *const ::core::ffi::c_void,
            in_buffer as size_t,
        );
    }
    g_free((*buffer).data as gpointer);
    (*buffer).data = data;
    (*buffer).end = (*buffer).end.wrapping_sub((*buffer).start);
    (*buffer).start = 0 as gsize;
    (*buffer).size = size;
}
unsafe extern "C" fn safe_c2rust_buffer_ensure_space(
    mut buffer: *mut Buffer,
    mut at_least_size: gsize,
) {
    let mut in_buffer: gsize = 0;
    let mut left_to_fill: gsize = 0;
    in_buffer = safe_c2rust_buffer_data_size(buffer);
    if in_buffer >= at_least_size {
        return;
    }
    left_to_fill = safe_c2rust_buffer_tailspace(buffer);
    if in_buffer.wrapping_add(left_to_fill) >= at_least_size {
        if in_buffer < 256 as gsize {
            safe_c2rust_compact_buffer(buffer);
        }
    } else if (*buffer).size >= at_least_size {
        safe_c2rust_compact_buffer(buffer);
    } else {
        while (*buffer).size < at_least_size {
            safe_c2rust_grow_buffer(buffer);
        }
    };
}
unsafe extern "C" fn safe_c2rust_fill_input_buffer(
    mut stream: *mut GConverterInputStream,
    mut at_least_size: gsize,
    mut blocking: gboolean,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    let mut priv_0: *mut GConverterInputStreamPrivate =
        ::core::ptr::null_mut::<GConverterInputStreamPrivate>();
    let mut base_stream: *mut GInputStream = ::core::ptr::null_mut::<GInputStream>();
    let mut nread: gssize = 0;
    priv_0 = (*stream).priv_0;
    safe_c2rust_buffer_ensure_space(&raw mut (*priv_0).input_buffer, at_least_size);
    base_stream = (*(stream as *mut ::core::ffi::c_void as *mut GFilterInputStream)).base_stream;
    nread = g_pollable_stream_read(
        base_stream,
        (*priv_0)
            .input_buffer
            .data
            .offset((*priv_0).input_buffer.end as isize) as *mut ::core::ffi::c_void,
        safe_c2rust_buffer_tailspace(&raw mut (*priv_0).input_buffer),
        blocking,
        cancellable,
        error,
    );
    if nread > 0 as gssize {
        (*priv_0).input_buffer.end = (*priv_0).input_buffer.end.wrapping_add(nread as gsize);
        (*priv_0).need_input = FALSE as gboolean;
    }
    return nread;
}
unsafe extern "C" fn safe_c2rust_read_internal(
    mut stream: *mut GInputStream,
    mut buffer: *mut ::core::ffi::c_void,
    mut count: gsize,
    mut blocking: gboolean,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    let mut cstream: *mut GConverterInputStream = ::core::ptr::null_mut::<GConverterInputStream>();
    let mut priv_0: *mut GConverterInputStreamPrivate =
        ::core::ptr::null_mut::<GConverterInputStreamPrivate>();
    let mut available: gsize = 0;
    let mut total_bytes_read: gsize = 0;
    let mut nread: gssize = 0;
    let mut res: GConverterResult = G_CONVERTER_ERROR;
    let mut bytes_read: gsize = 0;
    let mut bytes_written: gsize = 0;
    let mut my_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut my_error2: *mut GError = ::core::ptr::null_mut::<GError>();
    cstream = stream as *mut ::core::ffi::c_void as *mut GConverterInputStream;
    priv_0 = (*cstream).priv_0;
    available = safe_c2rust_buffer_data_size(&raw mut (*priv_0).converted_buffer);
    if available > 0 as gsize && count <= available {
        safe_c2rust_buffer_read(
            &raw mut (*priv_0).converted_buffer,
            buffer as *mut ::core::ffi::c_char,
            count,
        );
        return count as gssize;
    }
    safe_c2rust_buffer_read(
        &raw mut (*priv_0).converted_buffer,
        buffer as *mut ::core::ffi::c_char,
        available,
    );
    total_bytes_read = available;
    buffer =
        (buffer as *mut ::core::ffi::c_char).offset(available as isize) as *mut ::core::ffi::c_void;
    count = count.wrapping_sub(available);
    if safe_c2rust_buffer_data_size(&raw mut (*priv_0).input_buffer) == 0 as gsize
        && total_bytes_read == 0 as gsize
        && (*priv_0).at_input_end == 0
    {
        nread = safe_c2rust_fill_input_buffer(cstream, count, blocking, cancellable, error);
        if nread < 0 as gssize {
            return -(1 as ::core::ffi::c_int) as gssize;
        }
        if nread == 0 as gssize {
            (*priv_0).at_input_end = TRUE as gboolean;
        }
    }
    if (*priv_0).finished == 0 {
        my_error = ::core::ptr::null_mut::<GError>();
        res = g_converter_convert(
            (*priv_0).converter,
            safe_c2rust_buffer_data(&raw mut (*priv_0).input_buffer) as *const ::core::ffi::c_void,
            safe_c2rust_buffer_data_size(&raw mut (*priv_0).input_buffer),
            buffer,
            count,
            (if (*priv_0).at_input_end != 0 {
                G_CONVERTER_INPUT_AT_END as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as GConverterFlags,
            &raw mut bytes_read,
            &raw mut bytes_written,
            &raw mut my_error,
        );
        if res as ::core::ffi::c_uint
            != G_CONVERTER_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            total_bytes_read = total_bytes_read.wrapping_add(bytes_written);
            safe_c2rust_buffer_consumed(&raw mut (*priv_0).input_buffer, bytes_read);
            if res as ::core::ffi::c_uint
                == G_CONVERTER_FINISHED as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*priv_0).finished = TRUE as gboolean;
            }
        } else if total_bytes_read == 0 as gsize
            && g_error_matches(
                my_error,
                g_io_error_quark(),
                G_IO_ERROR_PARTIAL_INPUT as ::core::ffi::c_int as gint,
            ) == 0
            && g_error_matches(
                my_error,
                g_io_error_quark(),
                G_IO_ERROR_NO_SPACE as ::core::ffi::c_int as gint,
            ) == 0
        {
            g_propagate_error(error, my_error);
            return -(1 as ::core::ffi::c_int) as gssize;
        } else {
            g_error_free(my_error);
        }
    }
    if total_bytes_read > 0 as gsize {
        return total_bytes_read as gssize;
    }
    if (*priv_0).finished != 0 {
        if ({
            let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
            if safe_c2rust_buffer_data_size(&raw mut (*priv_0).converted_buffer) == 0 as gsize {
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gconverterinputstream.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                476 as ::core::ffi::c_int,
                G_STRFUNC,
                b"buffer_data_size (&priv->converted_buffer) == 0\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        return 0 as gssize;
    }
    safe_c2rust_buffer_ensure_space(&raw mut (*priv_0).converted_buffer, count);
    while FALSE == 0 {
        if ({
            let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
            if (*priv_0).finished == 0 {
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gconverterinputstream.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                490 as ::core::ffi::c_int,
                G_STRFUNC,
                b"!priv->finished\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        my_error = ::core::ptr::null_mut::<GError>();
        res = g_converter_convert(
            (*priv_0).converter,
            safe_c2rust_buffer_data(&raw mut (*priv_0).input_buffer) as *const ::core::ffi::c_void,
            safe_c2rust_buffer_data_size(&raw mut (*priv_0).input_buffer),
            safe_c2rust_buffer_data(&raw mut (*priv_0).converted_buffer)
                as *mut ::core::ffi::c_void,
            safe_c2rust_buffer_tailspace(&raw mut (*priv_0).converted_buffer),
            (if (*priv_0).at_input_end != 0 {
                G_CONVERTER_INPUT_AT_END as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as GConverterFlags,
            &raw mut bytes_read,
            &raw mut bytes_written,
            &raw mut my_error,
        );
        if res as ::core::ffi::c_uint
            != G_CONVERTER_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*priv_0).converted_buffer.end =
                (*priv_0).converted_buffer.end.wrapping_add(bytes_written);
            safe_c2rust_buffer_consumed(&raw mut (*priv_0).input_buffer, bytes_read);
            if safe_c2rust_buffer_data_size(&raw mut (*priv_0).converted_buffer) == 0 as gsize
                && res as ::core::ffi::c_uint
                    != G_CONVERTER_FINISHED as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                continue;
            }
            if res as ::core::ffi::c_uint
                == G_CONVERTER_FINISHED as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*priv_0).finished = TRUE as gboolean;
            }
            total_bytes_read =
                if count < safe_c2rust_buffer_data_size(&raw mut (*priv_0).converted_buffer) {
                    count
                } else {
                    safe_c2rust_buffer_data_size(&raw mut (*priv_0).converted_buffer)
                };
            safe_c2rust_buffer_read(
                &raw mut (*priv_0).converted_buffer,
                buffer as *mut ::core::ffi::c_char,
                total_bytes_read,
            );
            if ({
                let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
                if (*priv_0).finished != 0 || total_bytes_read > 0 as gsize {
                    _g_boolean_var_13 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_13 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_13
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gconverterinputstream.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    518 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"priv->finished || total_bytes_read > 0\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            return total_bytes_read as gssize;
        } else if g_error_matches(
            my_error,
            g_io_error_quark(),
            G_IO_ERROR_PARTIAL_INPUT as ::core::ffi::c_int as gint,
        ) != 0
            && (*priv_0).at_input_end == 0
        {
            my_error2 = ::core::ptr::null_mut::<GError>();
            nread = safe_c2rust_fill_input_buffer(
                cstream,
                safe_c2rust_buffer_data_size(&raw mut (*priv_0).input_buffer)
                    .wrapping_add(4096 as gsize),
                blocking,
                cancellable,
                &raw mut my_error2,
            );
            if nread < 0 as gssize {
                g_error_free(my_error);
                g_propagate_error(error, my_error2);
                (*priv_0).need_input = TRUE as gboolean;
                return -(1 as ::core::ffi::c_int) as gssize;
            } else if nread == 0 as gssize {
                (*priv_0).at_input_end = TRUE as gboolean;
            }
            g_error_free(my_error);
        } else if g_error_matches(
            my_error,
            g_io_error_quark(),
            G_IO_ERROR_NO_SPACE as ::core::ffi::c_int as gint,
        ) != 0
        {
            safe_c2rust_buffer_ensure_space(
                &raw mut (*priv_0).converted_buffer,
                (*priv_0).converted_buffer.size.wrapping_add(1 as gsize),
            );
            g_error_free(my_error);
        } else {
            g_propagate_error(error, my_error);
            return -(1 as ::core::ffi::c_int) as gssize;
        }
    }
    g_assertion_message_expr(
        G_LOG_DOMAIN.as_ptr(),
        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gconverterinputstream.c\0"
            as *const u8 as *const ::core::ffi::c_char,
        572 as ::core::ffi::c_int,
        G_STRFUNC,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
unsafe extern "C" fn safe_c2rust_g_converter_input_stream_read(
    mut stream: *mut GInputStream,
    mut buffer: *mut ::core::ffi::c_void,
    mut count: gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    return safe_c2rust_read_internal(stream, buffer, count, TRUE, cancellable, error);
}
unsafe extern "C" fn safe_c2rust_g_converter_input_stream_can_poll(
    mut stream: *mut GPollableInputStream,
) -> gboolean {
    let mut base_stream: *mut GInputStream =
        (*(stream as *mut ::core::ffi::c_void as *mut GFilterInputStream)).base_stream;
    return (({
        let mut __inst: *mut GTypeInstance = base_stream as *mut GTypeInstance;
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
            base_stream as *mut ::core::ffi::c_void as *mut GPollableInputStream,
        ) != 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_converter_input_stream_is_readable(
    mut stream: *mut GPollableInputStream,
) -> gboolean {
    let mut base_stream: *mut GInputStream =
        (*(stream as *mut ::core::ffi::c_void as *mut GFilterInputStream)).base_stream;
    let mut cstream: *mut GConverterInputStream =
        stream as *mut ::core::ffi::c_void as *mut GConverterInputStream;
    if safe_c2rust_buffer_data_size(&raw mut (*(*cstream).priv_0).converted_buffer) != 0 {
        return TRUE;
    } else if safe_c2rust_buffer_data_size(&raw mut (*(*cstream).priv_0).input_buffer) != 0
        && (*(*cstream).priv_0).need_input == 0
    {
        return TRUE;
    } else {
        return g_pollable_input_stream_is_readable(
            base_stream as *mut ::core::ffi::c_void as *mut GPollableInputStream,
        );
    };
}
unsafe extern "C" fn safe_c2rust_g_converter_input_stream_read_nonblocking(
    mut stream: *mut GPollableInputStream,
    mut buffer: *mut ::core::ffi::c_void,
    mut count: gsize,
    mut error: *mut *mut GError,
) -> gssize {
    return safe_c2rust_read_internal(
        stream as *mut ::core::ffi::c_void as *mut GInputStream,
        buffer,
        count,
        FALSE,
        ::core::ptr::null_mut::<GCancellable>(),
        error,
    );
}
unsafe extern "C" fn safe_c2rust_g_converter_input_stream_create_source(
    mut stream: *mut GPollableInputStream,
    mut cancellable: *mut GCancellable,
) -> *mut GSource {
    let mut base_stream: *mut GInputStream =
        (*(stream as *mut ::core::ffi::c_void as *mut GFilterInputStream)).base_stream;
    let mut base_source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    let mut pollable_source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    if g_pollable_input_stream_is_readable(stream) != 0 {
        base_source = g_timeout_source_new(0 as guint);
    } else {
        base_source = g_pollable_input_stream_create_source(
            base_stream as *mut ::core::ffi::c_void as *mut GPollableInputStream,
            ::core::ptr::null_mut::<GCancellable>(),
        );
    }
    pollable_source = g_pollable_source_new_full(stream as gpointer, base_source, cancellable);
    g_source_unref(base_source);
    return pollable_source;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_converter_input_stream_get_converter(
    mut converter_stream: *mut GConverterInputStream,
) -> *mut GConverter {
    return (*(*converter_stream).priv_0).converter;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
