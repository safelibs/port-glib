extern "C" {
    pub type _GBytes;
    pub type _GData;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GAsyncResult;
    pub type _GInputStreamPrivate;
    pub type _GCancellablePrivate;
    pub type _GPollableInputStream;
    pub type _GSeekable;
    pub type _GTask;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
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
    fn g_bytes_new_with_free_func(
        data: gconstpointer,
        size: gsize,
        free_func: GDestroyNotify,
        user_data: gpointer,
    ) -> *mut GBytes;
    fn g_bytes_get_data(bytes: *mut GBytes, size: *mut gsize) -> gconstpointer;
    fn g_bytes_get_size(bytes: *mut GBytes) -> gsize;
    fn g_bytes_ref(bytes: *mut GBytes) -> *mut GBytes;
    fn g_bytes_unref(bytes: *mut GBytes);
    fn g_slist_free_full(list: *mut GSList, free_func: GDestroyNotify);
    fn g_slist_append(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_source_unref(source: *mut GSource);
    fn g_timeout_source_new(interval: guint) -> *mut GSource;
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
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_input_stream_get_type() -> GType;
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
    fn g_task_return_int(task: *mut GTask, result: gssize);
    fn g_task_return_error(task: *mut GTask, error: *mut GError);
    fn g_task_propagate_int(task: *mut GTask, error: *mut *mut GError) -> gssize;
    fn g_pollable_input_stream_get_type() -> GType;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMemoryInputStream {
    pub parent_instance: GInputStream,
    pub priv_0: *mut GMemoryInputStreamPrivate,
}
pub type GMemoryInputStreamPrivate = _GMemoryInputStreamPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMemoryInputStreamPrivate {
    pub chunks: *mut GSList,
    pub len: gsize,
    pub pos: gsize,
}
pub type GMemoryInputStream = _GMemoryInputStream;
pub type GPollableInputStream = _GPollableInputStream;
pub type GSeekable = _GSeekable;
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
pub struct _GMemoryInputStreamClass {
    pub parent_class: GInputStreamClass,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
}
pub type GMemoryInputStreamClass = _GMemoryInputStreamClass;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut safe_c2rust_GMemoryInputStream_private_offset: gint = 0;
#[inline]
unsafe extern "C" fn safe_c2rust_g_memory_input_stream_get_instance_private(
    mut self_0: *mut GMemoryInputStream,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GMemoryInputStream_private_offset as glong as isize)
        as gpointer;
}
unsafe extern "C" fn safe_c2rust_g_memory_input_stream_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_memory_input_stream_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GMemoryInputStream_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GMemoryInputStream_private_offset,
        );
    }
    safe_c2rust_g_memory_input_stream_class_init(klass as *mut GMemoryInputStreamClass);
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_memory_input_stream_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_input_stream_get_type(),
        g_intern_static_string(b"GMemoryInputStream\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GMemoryInputStreamClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_memory_input_stream_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GMemoryInputStream>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GMemoryInputStream) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_memory_input_stream_init
                    as unsafe extern "C" fn(*mut GMemoryInputStream) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GMemoryInputStream_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GMemoryInputStreamPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GSeekableIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_memory_input_stream_seekable_iface_init
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
            Option<unsafe extern "C" fn(*mut GPollableInputStreamInterface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_memory_input_stream_pollable_iface_init
                as unsafe extern "C" fn(*mut GPollableInputStreamInterface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_pollable_input_stream_get_type(),
        &raw const g_implement_interface_info_0,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_memory_input_stream_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_memory_input_stream_get_type_once();
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
static mut safe_c2rust_g_memory_input_stream_parent_class: gpointer = NULL;
unsafe extern "C" fn safe_c2rust_g_memory_input_stream_class_init(
    mut klass: *mut GMemoryInputStreamClass,
) {
    let mut object_class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    let mut istream_class: *mut GInputStreamClass = ::core::ptr::null_mut::<GInputStreamClass>();
    object_class = klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*object_class).finalize = Some(
        safe_c2rust_g_memory_input_stream_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    istream_class = klass as *mut ::core::ffi::c_void as *mut GInputStreamClass;
    (*istream_class).read_fn = Some(
        safe_c2rust_g_memory_input_stream_read
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
    (*istream_class).skip = Some(
        safe_c2rust_g_memory_input_stream_skip
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
    (*istream_class).close_fn = Some(
        safe_c2rust_g_memory_input_stream_close
            as unsafe extern "C" fn(
                *mut GInputStream,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GInputStream,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
        >;
    (*istream_class).skip_async = Some(
        safe_c2rust_g_memory_input_stream_skip_async
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
        safe_c2rust_g_memory_input_stream_skip_finish
            as unsafe extern "C" fn(
                *mut GInputStream,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> gssize,
    )
        as Option<
            unsafe extern "C" fn(*mut GInputStream, *mut GAsyncResult, *mut *mut GError) -> gssize,
        >;
    (*istream_class).close_async = Some(
        safe_c2rust_g_memory_input_stream_close_async
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
    (*istream_class).close_finish = Some(
        safe_c2rust_g_memory_input_stream_close_finish
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
unsafe extern "C" fn safe_c2rust_g_memory_input_stream_finalize(mut object: *mut GObject) {
    let mut stream: *mut GMemoryInputStream = ::core::ptr::null_mut::<GMemoryInputStream>();
    let mut priv_0: *mut GMemoryInputStreamPrivate =
        ::core::ptr::null_mut::<GMemoryInputStreamPrivate>();
    stream = object as *mut ::core::ffi::c_void as *mut GMemoryInputStream;
    priv_0 = (*stream).priv_0;
    g_slist_free_full(
        (*priv_0).chunks,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut GBytes) -> ()>, GDestroyNotify>(
            Some(g_bytes_unref as unsafe extern "C" fn(*mut GBytes) -> ()),
        ),
    );
    (*(safe_c2rust_g_memory_input_stream_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_memory_input_stream_seekable_iface_init(
    mut iface: *mut GSeekableIface,
) {
    (*iface).tell = Some(
        safe_c2rust_g_memory_input_stream_tell as unsafe extern "C" fn(*mut GSeekable) -> goffset,
    ) as Option<unsafe extern "C" fn(*mut GSeekable) -> goffset>;
    (*iface).can_seek = Some(
        safe_c2rust_g_memory_input_stream_can_seek
            as unsafe extern "C" fn(*mut GSeekable) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut GSeekable) -> gboolean>;
    (*iface).seek = Some(
        safe_c2rust_g_memory_input_stream_seek
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
        safe_c2rust_g_memory_input_stream_can_truncate
            as unsafe extern "C" fn(*mut GSeekable) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut GSeekable) -> gboolean>;
    (*iface).truncate_fn = Some(
        safe_c2rust_g_memory_input_stream_truncate
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
unsafe extern "C" fn safe_c2rust_g_memory_input_stream_pollable_iface_init(
    mut iface: *mut GPollableInputStreamInterface,
) {
    (*iface).is_readable = Some(
        safe_c2rust_g_memory_input_stream_is_readable
            as unsafe extern "C" fn(*mut GPollableInputStream) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GPollableInputStream) -> gboolean>;
    (*iface).create_source = Some(
        safe_c2rust_g_memory_input_stream_create_source
            as unsafe extern "C" fn(*mut GPollableInputStream, *mut GCancellable) -> *mut GSource,
    )
        as Option<
            unsafe extern "C" fn(*mut GPollableInputStream, *mut GCancellable) -> *mut GSource,
        >;
}
unsafe extern "C" fn safe_c2rust_g_memory_input_stream_init(mut stream: *mut GMemoryInputStream) {
    (*stream).priv_0 = safe_c2rust_g_memory_input_stream_get_instance_private(stream)
        as *mut GMemoryInputStreamPrivate;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_memory_input_stream_new() -> *mut GInputStream {
    let mut stream: *mut GInputStream = ::core::ptr::null_mut::<GInputStream>();
    stream = g_object_new(
        safe_c2rust_g_memory_input_stream_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GInputStream;
    return stream;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_memory_input_stream_new_from_data(
    mut data: *const ::core::ffi::c_void,
    mut len: gssize,
    mut destroy: GDestroyNotify,
) -> *mut GInputStream {
    let mut stream: *mut GInputStream = ::core::ptr::null_mut::<GInputStream>();
    stream = safe_c2rust_g_memory_input_stream_new();
    safe_c2rust_g_memory_input_stream_add_data(
        stream as *mut ::core::ffi::c_void as *mut GMemoryInputStream,
        data,
        len,
        destroy,
    );
    return stream;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_memory_input_stream_new_from_bytes(
    mut bytes: *mut GBytes,
) -> *mut GInputStream {
    let mut stream: *mut GInputStream = ::core::ptr::null_mut::<GInputStream>();
    stream = safe_c2rust_g_memory_input_stream_new();
    safe_c2rust_g_memory_input_stream_add_bytes(
        stream as *mut ::core::ffi::c_void as *mut GMemoryInputStream,
        bytes,
    );
    return stream;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_memory_input_stream_add_data(
    mut stream: *mut GMemoryInputStream,
    mut data: *const ::core::ffi::c_void,
    mut len: gssize,
    mut destroy: GDestroyNotify,
) {
    let mut bytes: *mut GBytes = ::core::ptr::null_mut::<GBytes>();
    if len == -(1 as ::core::ffi::c_int) as gssize {
        len = strlen(data as *const ::core::ffi::c_char) as gssize;
    }
    bytes = g_bytes_new_with_free_func(
        data as gconstpointer,
        len as gsize,
        destroy,
        data as gpointer,
    );
    safe_c2rust_g_memory_input_stream_add_bytes(stream, bytes);
    g_bytes_unref(bytes);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_memory_input_stream_add_bytes(
    mut stream: *mut GMemoryInputStream,
    mut bytes: *mut GBytes,
) {
    let mut priv_0: *mut GMemoryInputStreamPrivate =
        ::core::ptr::null_mut::<GMemoryInputStreamPrivate>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_memory_input_stream_get_type();
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
            b"G_IS_MEMORY_INPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !bytes.is_null() {
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
            b"bytes != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    priv_0 = (*stream).priv_0;
    (*priv_0).chunks = g_slist_append((*priv_0).chunks, g_bytes_ref(bytes) as gpointer);
    (*priv_0).len = (*priv_0).len.wrapping_add(g_bytes_get_size(bytes));
}
unsafe extern "C" fn safe_c2rust_g_memory_input_stream_read(
    mut stream: *mut GInputStream,
    mut buffer: *mut ::core::ffi::c_void,
    mut count: gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    let mut memory_stream: *mut GMemoryInputStream = ::core::ptr::null_mut::<GMemoryInputStream>();
    let mut priv_0: *mut GMemoryInputStreamPrivate =
        ::core::ptr::null_mut::<GMemoryInputStreamPrivate>();
    let mut l: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut chunk: *mut GBytes = ::core::ptr::null_mut::<GBytes>();
    let mut len: gsize = 0;
    let mut offset: gsize = 0;
    let mut start: gsize = 0;
    let mut rest: gsize = 0;
    let mut size: gsize = 0;
    memory_stream = stream as *mut ::core::ffi::c_void as *mut GMemoryInputStream;
    priv_0 = (*memory_stream).priv_0;
    count = if count < (*priv_0).len.wrapping_sub((*priv_0).pos) {
        count
    } else {
        (*priv_0).len.wrapping_sub((*priv_0).pos)
    };
    offset = 0 as gsize;
    l = (*priv_0).chunks;
    while !l.is_null() {
        chunk = (*l).data as *mut GBytes;
        len = g_bytes_get_size(chunk);
        if offset.wrapping_add(len) > (*priv_0).pos {
            break;
        }
        offset = offset.wrapping_add(len);
        l = (*l).next;
    }
    start = (*priv_0).pos.wrapping_sub(offset);
    rest = count;
    while !l.is_null() && rest > 0 as gsize {
        let mut chunk_data: *const guint8 = ::core::ptr::null::<guint8>();
        chunk = (*l).data as *mut GBytes;
        chunk_data = g_bytes_get_data(chunk, &raw mut len) as *const guint8;
        size = if rest < len.wrapping_sub(start) {
            rest
        } else {
            len.wrapping_sub(start)
        };
        memcpy(
            (buffer as *mut guint8).offset(count.wrapping_sub(rest) as isize)
                as *mut ::core::ffi::c_void,
            chunk_data.offset(start as isize) as *const ::core::ffi::c_void,
            size as size_t,
        );
        rest = rest.wrapping_sub(size);
        start = 0 as gsize;
        l = (*l).next;
    }
    (*priv_0).pos = (*priv_0).pos.wrapping_add(count);
    return count as gssize;
}
unsafe extern "C" fn safe_c2rust_g_memory_input_stream_skip(
    mut stream: *mut GInputStream,
    mut count: gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    let mut memory_stream: *mut GMemoryInputStream = ::core::ptr::null_mut::<GMemoryInputStream>();
    let mut priv_0: *mut GMemoryInputStreamPrivate =
        ::core::ptr::null_mut::<GMemoryInputStreamPrivate>();
    memory_stream = stream as *mut ::core::ffi::c_void as *mut GMemoryInputStream;
    priv_0 = (*memory_stream).priv_0;
    count = if count < (*priv_0).len.wrapping_sub((*priv_0).pos) {
        count
    } else {
        (*priv_0).len.wrapping_sub((*priv_0).pos)
    };
    (*priv_0).pos = (*priv_0).pos.wrapping_add(count);
    return count as gssize;
}
unsafe extern "C" fn safe_c2rust_g_memory_input_stream_close(
    mut stream: *mut GInputStream,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_memory_input_stream_skip_async(
    mut stream: *mut GInputStream,
    mut count: gsize,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut nskipped: gssize = 0;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    nskipped =
        (*((*(stream as *mut GTypeInstance)).g_class as *mut GInputStreamClass))
            .skip
            .expect("non-null function pointer")(stream, count, cancellable, &raw mut error);
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
            safe_c2rust_g_memory_input_stream_skip_async
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
            b"g_memory_input_stream_skip_async\0" as *const u8 as *const gchar,
        );
    }
    if !error.is_null() {
        g_task_return_error(task, error);
    } else {
        g_task_return_int(task, nskipped);
    }
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_memory_input_stream_skip_finish(
    mut stream: *mut GInputStream,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gssize {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, stream as gpointer) != 0 {
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
            b"g_task_is_valid (result, stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    return g_task_propagate_int(result as *mut ::core::ffi::c_void as *mut GTask, error);
}
unsafe extern "C" fn safe_c2rust_g_memory_input_stream_close_async(
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
            safe_c2rust_g_memory_input_stream_close_async
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
            b"g_memory_input_stream_close_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_return_boolean(task, TRUE);
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_memory_input_stream_close_finish(
    mut stream: *mut GInputStream,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_memory_input_stream_tell(
    mut seekable: *mut GSeekable,
) -> goffset {
    let mut memory_stream: *mut GMemoryInputStream = ::core::ptr::null_mut::<GMemoryInputStream>();
    let mut priv_0: *mut GMemoryInputStreamPrivate =
        ::core::ptr::null_mut::<GMemoryInputStreamPrivate>();
    memory_stream = seekable as *mut ::core::ffi::c_void as *mut GMemoryInputStream;
    priv_0 = (*memory_stream).priv_0;
    return (*priv_0).pos as goffset;
}
unsafe extern "C" fn safe_c2rust_g_memory_input_stream_can_seek(
    mut seekable: *mut GSeekable,
) -> gboolean {
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_memory_input_stream_seek(
    mut seekable: *mut GSeekable,
    mut offset: goffset,
    mut type_0: GSeekType,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut memory_stream: *mut GMemoryInputStream = ::core::ptr::null_mut::<GMemoryInputStream>();
    let mut priv_0: *mut GMemoryInputStreamPrivate =
        ::core::ptr::null_mut::<GMemoryInputStreamPrivate>();
    let mut absolute: goffset = 0;
    memory_stream = seekable as *mut ::core::ffi::c_void as *mut GMemoryInputStream;
    priv_0 = (*memory_stream).priv_0;
    match type_0 as ::core::ffi::c_uint {
        0 => {
            absolute = (*priv_0).pos.wrapping_add(offset as gsize) as goffset;
        }
        1 => {
            absolute = offset;
        }
        2 => {
            absolute = (*priv_0).len.wrapping_add(offset as gsize) as goffset;
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
    if absolute < 0 as goffset || absolute as gsize > (*priv_0).len {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(b"Invalid seek request\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    (*priv_0).pos = absolute as gsize;
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_memory_input_stream_can_truncate(
    mut seekable: *mut GSeekable,
) -> gboolean {
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_memory_input_stream_truncate(
    mut seekable: *mut GSeekable,
    mut offset: goffset,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    g_set_error_literal(
        error,
        g_io_error_quark(),
        G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
        glib_gettext(b"Cannot truncate GMemoryInputStream\0" as *const u8 as *const gchar),
    );
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_memory_input_stream_is_readable(
    mut stream: *mut GPollableInputStream,
) -> gboolean {
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_memory_input_stream_create_source(
    mut stream: *mut GPollableInputStream,
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
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
