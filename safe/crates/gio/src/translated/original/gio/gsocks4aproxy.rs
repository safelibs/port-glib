extern "C" {
    pub type _GData;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _GInetAddressPrivate;
    pub type _GInetSocketAddressPrivate;
    pub type _GInputStreamPrivate;
    pub type _GIOExtension;
    pub type _GOutputStreamPrivate;
    pub type _GIOStreamPrivate;
    pub type _GTask;
    pub type _GProxy;
    pub type _GProxyAddressPrivate;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
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
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_hostname_is_ip_address(hostname: *const gchar) -> gboolean;
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
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
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_io_extension_point_implement(
        extension_point_name: *const ::core::ffi::c_char,
        type_0: GType,
        extension_name: *const ::core::ffi::c_char,
        priority: gint,
    ) -> *mut GIOExtension;
    fn _g_io_modules_ensure_extension_points_registered();
    fn g_input_stream_read_all(
        stream: *mut GInputStream,
        buffer: *mut ::core::ffi::c_void,
        count: gsize,
        bytes_read: *mut gsize,
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
    fn g_output_stream_write_all(
        stream: *mut GOutputStream,
        buffer: *const ::core::ffi::c_void,
        count: gsize,
        bytes_written: *mut gsize,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_output_stream_write_async(
        stream: *mut GOutputStream,
        buffer: *const ::core::ffi::c_void,
        count: gsize,
        io_priority: ::core::ffi::c_int,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_output_stream_write_finish(
        stream: *mut GOutputStream,
        result: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gssize;
    fn g_io_error_quark() -> GQuark;
    fn g_io_stream_get_input_stream(stream: *mut GIOStream) -> *mut GInputStream;
    fn g_io_stream_get_output_stream(stream: *mut GIOStream) -> *mut GOutputStream;
    fn g_inet_address_new_from_string(string: *const gchar) -> *mut GInetAddress;
    fn g_inet_address_to_bytes(address: *mut GInetAddress) -> *const guint8;
    fn g_inet_address_get_native_size(address: *mut GInetAddress) -> gsize;
    fn glib_gettext(str: *const gchar) -> *const gchar;
    fn g_proxy_get_type() -> GType;
    fn g_proxy_address_get_destination_hostname(proxy: *mut GProxyAddress) -> *const gchar;
    fn g_proxy_address_get_destination_port(proxy: *mut GProxyAddress) -> guint16;
    fn g_proxy_address_get_username(proxy: *mut GProxyAddress) -> *const gchar;
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
    fn g_task_set_source_tag(task: *mut GTask, source_tag: gpointer);
    fn g_task_set_static_name(task: *mut GTask, name: *const gchar);
    fn g_task_get_task_data(task: *mut GTask) -> gpointer;
    fn g_task_get_priority(task: *mut GTask) -> gint;
    fn g_task_get_cancellable(task: *mut GTask) -> *mut GCancellable;
    fn g_task_get_name(task: *mut GTask) -> *const gchar;
    fn g_task_return_pointer(task: *mut GTask, result: gpointer, result_destroy: GDestroyNotify);
    fn g_task_return_error(task: *mut GTask, error: *mut GError);
    fn g_task_propagate_pointer(task: *mut GTask, error: *mut *mut GError) -> gpointer;
}
pub type size_t = usize;
pub type guint8 = ::core::ffi::c_uchar;
pub type guint16 = ::core::ffi::c_ushort;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInetAddress {
    pub parent_instance: GObject,
    pub priv_0: *mut GInetAddressPrivate,
}
pub type GInetAddressPrivate = _GInetAddressPrivate;
pub type GInetAddress = _GInetAddress;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInetSocketAddress {
    pub parent_instance: GSocketAddress,
    pub priv_0: *mut GInetSocketAddressPrivate,
}
pub type GInetSocketAddressPrivate = _GInetSocketAddressPrivate;
pub type GSocketAddress = _GSocketAddress;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketAddress {
    pub parent_instance: GObject,
}
pub type GInetSocketAddress = _GInetSocketAddress;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInputStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GInputStreamPrivate,
}
pub type GInputStreamPrivate = _GInputStreamPrivate;
pub type GInputStream = _GInputStream;
pub type GIOExtension = _GIOExtension;
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
pub type GIOStream = _GIOStream;
pub type GTask = _GTask;
pub type GProxy = _GProxy;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GProxyAddress {
    pub parent_instance: GInetSocketAddress,
    pub priv_0: *mut GProxyAddressPrivate,
}
pub type GProxyAddressPrivate = _GProxyAddressPrivate;
pub type GProxyAddress = _GProxyAddress;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocks4aProxy {
    pub parent: GObject,
    pub supports_hostname: gboolean,
}
pub type GSocks4aProxy = _GSocks4aProxy;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocks4aProxyClass {
    pub parent_class: GObjectClass,
}
pub type GSocks4aProxyClass = _GSocks4aProxyClass;
pub type GProxyInterface = _GProxyInterface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GProxyInterface {
    pub g_iface: GTypeInterface,
    pub connect: Option<
        unsafe extern "C" fn(
            *mut GProxy,
            *mut GIOStream,
            *mut GProxyAddress,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GIOStream,
    >,
    pub connect_async: Option<
        unsafe extern "C" fn(
            *mut GProxy,
            *mut GIOStream,
            *mut GProxyAddress,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub connect_finish: Option<
        unsafe extern "C" fn(*mut GProxy, *mut GAsyncResult, *mut *mut GError) -> *mut GIOStream,
    >,
    pub supports_hostname: Option<unsafe extern "C" fn(*mut GProxy) -> gboolean>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ConnectAsyncData {
    pub io_stream: *mut GIOStream,
    pub buffer: *mut guint8,
    pub length: gssize,
    pub offset: gssize,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SOCKS4_VERSION: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SOCKS4_CMD_CONNECT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SOCKS4_MAX_LEN: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
pub const SOCKS4_REP_VERSION: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SOCKS4_REP_GRANTED: ::core::ffi::c_int = 90 as ::core::ffi::c_int;
static mut safe_c2rust_GSocks4aProxy_private_offset: gint = 0;
static mut safe_c2rust_g_socks4a_proxy_parent_class: gpointer = NULL;
unsafe extern "C" fn safe_c2rust_g_socks4a_proxy_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_socks4a_proxy_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GSocks4aProxy_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GSocks4aProxy_private_offset,
        );
    }
    safe_c2rust_g_socks4a_proxy_class_init(klass as *mut GSocks4aProxyClass);
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_socks4a_proxy_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GSocks4aProxy\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GSocks4aProxyClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_socks4a_proxy_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GSocks4aProxy>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GSocks4aProxy) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_socks4a_proxy_init as unsafe extern "C" fn(*mut GSocks4aProxy) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GProxyInterface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_socks4a_proxy_iface_init
                as unsafe extern "C" fn(*mut GProxyInterface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_proxy_get_type(),
        &raw const g_implement_interface_info,
    );
    _g_io_modules_ensure_extension_points_registered();
    g_io_extension_point_implement(
        b"gio-proxy\0" as *const u8 as *const ::core::ffi::c_char,
        g_define_type_id,
        b"socks4a\0" as *const u8 as *const ::core::ffi::c_char,
        0 as gint,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_socks4a_proxy_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_socks4a_proxy_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_socks4a_proxy_finalize(mut object: *mut GObject) {
    (*(safe_c2rust_g_socks4a_proxy_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_socks4a_proxy_init(mut proxy: *mut GSocks4aProxy) {
    (*proxy).supports_hostname = TRUE as gboolean;
}
pub const SOCKS4_CONN_MSG_LEN: ::core::ffi::c_int =
    10 as ::core::ffi::c_int + SOCKS4_MAX_LEN * 2 as ::core::ffi::c_int;
unsafe extern "C" fn safe_c2rust_set_connect_msg(
    mut msg: *mut guint8,
    mut hostname: *const gchar,
    mut port: guint16,
    mut username: *const ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> gint {
    let mut addr: *mut GInetAddress = ::core::ptr::null_mut::<GInetAddress>();
    let mut len: guint = 0 as guint;
    let mut addr_len: gsize = 0;
    let mut is_ip: gboolean = 0;
    let mut ip: *const gchar = ::core::ptr::null::<gchar>();
    let fresh0 = len;
    len = len.wrapping_add(1);
    *msg.offset(fresh0 as isize) = SOCKS4_VERSION as guint8;
    let fresh1 = len;
    len = len.wrapping_add(1);
    *msg.offset(fresh1 as isize) = SOCKS4_CMD_CONNECT as guint8;
    let mut hp: guint16 = ((port as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as guint16
        as ::core::ffi::c_int
        | ((port as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as guint16
            as ::core::ffi::c_int) as guint16;
    memcpy(
        msg.offset(len as isize) as *mut ::core::ffi::c_void,
        &raw mut hp as *const ::core::ffi::c_void,
        2 as size_t,
    );
    len = len.wrapping_add(2 as guint);
    is_ip = g_hostname_is_ip_address(hostname);
    if is_ip != 0 {
        ip = hostname;
    } else {
        ip = b"0.0.0.1\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    }
    addr = g_inet_address_new_from_string(ip);
    addr_len = g_inet_address_get_native_size(addr);
    if addr_len != 4 as gsize {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_PROXY_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"SOCKSv4 does not support IPv6 address \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8
                    as *const gchar,
            ),
            ip,
        );
        g_object_unref(addr as gpointer);
        return -(1 as gint);
    }
    memcpy(
        msg.offset(len as isize) as *mut ::core::ffi::c_void,
        g_inet_address_to_bytes(addr) as *const ::core::ffi::c_void,
        addr_len as size_t,
    );
    len = (len as gsize).wrapping_add(addr_len) as guint as guint;
    g_object_unref(addr as gpointer);
    if !username.is_null() {
        let mut user_len: gsize = strlen(username) as gsize;
        if user_len > SOCKS4_MAX_LEN as gsize {
            g_set_error_literal(
                error,
                g_io_error_quark(),
                G_IO_ERROR_PROXY_FAILED as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Username is too long for SOCKSv4 protocol\0" as *const u8 as *const gchar,
                ),
            );
            return -(1 as gint);
        }
        memcpy(
            msg.offset(len as isize) as *mut ::core::ffi::c_void,
            username as *const ::core::ffi::c_void,
            user_len as size_t,
        );
        len = (len as gsize).wrapping_add(user_len) as guint as guint;
    }
    let fresh2 = len;
    len = len.wrapping_add(1);
    *msg.offset(fresh2 as isize) = '\0' as i32 as guint8;
    if is_ip == 0 {
        let mut host_len: gsize = strlen(hostname as *const ::core::ffi::c_char) as gsize;
        if host_len > SOCKS4_MAX_LEN as gsize {
            g_set_error(
                error,
                g_io_error_quark(),
                G_IO_ERROR_PROXY_FAILED as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Hostname \xE2\x80\x9C%s\xE2\x80\x9D is too long for SOCKSv4 protocol\0"
                        as *const u8 as *const gchar,
                ),
                hostname,
            );
            return -(1 as gint);
        }
        memcpy(
            msg.offset(len as isize) as *mut ::core::ffi::c_void,
            hostname as *const ::core::ffi::c_void,
            host_len as size_t,
        );
        len = (len as gsize).wrapping_add(host_len) as guint as guint;
        let fresh3 = len;
        len = len.wrapping_add(1);
        *msg.offset(fresh3 as isize) = '\0' as i32 as guint8;
    }
    return len as gint;
}
pub const SOCKS4_CONN_REP_LEN: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
unsafe extern "C" fn safe_c2rust_parse_connect_reply(
    mut data: *const guint8,
    mut error: *mut *mut GError,
) -> gboolean {
    if *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != SOCKS4_REP_VERSION {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_PROXY_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"The server is not a SOCKSv4 proxy server.\0" as *const u8 as *const gchar,
            ),
        );
        return FALSE;
    }
    if *data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != SOCKS4_REP_GRANTED {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_PROXY_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Connection through SOCKSv4 server was rejected\0" as *const u8 as *const gchar,
            ),
        );
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_socks4a_proxy_connect(
    mut proxy: *mut GProxy,
    mut io_stream: *mut GIOStream,
    mut proxy_address: *mut GProxyAddress,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GIOStream {
    let mut in_0: *mut GInputStream = ::core::ptr::null_mut::<GInputStream>();
    let mut out: *mut GOutputStream = ::core::ptr::null_mut::<GOutputStream>();
    let mut hostname: *const gchar = ::core::ptr::null::<gchar>();
    let mut port: guint16 = 0;
    let mut username: *const gchar = ::core::ptr::null::<gchar>();
    hostname = g_proxy_address_get_destination_hostname(proxy_address);
    port = g_proxy_address_get_destination_port(proxy_address);
    username = g_proxy_address_get_username(proxy_address);
    in_0 = g_io_stream_get_input_stream(io_stream);
    out = g_io_stream_get_output_stream(io_stream);
    let mut msg: [guint8; 520] = [0; 520];
    let mut len: gint = 0;
    len = safe_c2rust_set_connect_msg(
        &raw mut msg as *mut guint8,
        hostname,
        port,
        username as *const ::core::ffi::c_char,
        error,
    );
    if !(len < 0 as ::core::ffi::c_int) {
        if !(g_output_stream_write_all(
            out,
            &raw mut msg as *mut guint8 as *const ::core::ffi::c_void,
            len as gsize,
            ::core::ptr::null_mut::<gsize>(),
            cancellable,
            error,
        ) == 0)
        {
            let mut data: [guint8; 8] = [0; 8];
            if !(g_input_stream_read_all(
                in_0,
                &raw mut data as *mut guint8 as *mut ::core::ffi::c_void,
                SOCKS4_CONN_REP_LEN as gsize,
                ::core::ptr::null_mut::<gsize>(),
                cancellable,
                error,
            ) == 0)
            {
                if !(safe_c2rust_parse_connect_reply(&raw mut data as *mut guint8, error) == 0) {
                    return g_object_ref(io_stream as gpointer) as *mut GIOStream;
                }
            }
        }
    }
    return ::core::ptr::null_mut::<GIOStream>();
}
unsafe extern "C" fn safe_c2rust_free_connect_data(mut data: *mut ConnectAsyncData) {
    g_object_unref((*data).io_stream as gpointer);
    g_slice_free1(
        ::core::mem::size_of::<ConnectAsyncData>() as gsize,
        data as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_do_read(
    mut callback: GAsyncReadyCallback,
    mut task: *mut GTask,
    mut data: *mut ConnectAsyncData,
) {
    let mut in_0: *mut GInputStream = ::core::ptr::null_mut::<GInputStream>();
    in_0 = g_io_stream_get_input_stream((*data).io_stream);
    g_input_stream_read_async(
        in_0,
        (*data).buffer.offset((*data).offset as isize) as *mut ::core::ffi::c_void,
        ((*data).length - (*data).offset) as gsize,
        g_task_get_priority(task) as ::core::ffi::c_int,
        g_task_get_cancellable(task),
        callback,
        task as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_do_write(
    mut callback: GAsyncReadyCallback,
    mut task: *mut GTask,
    mut data: *mut ConnectAsyncData,
) {
    let mut out: *mut GOutputStream = ::core::ptr::null_mut::<GOutputStream>();
    out = g_io_stream_get_output_stream((*data).io_stream);
    g_output_stream_write_async(
        out,
        (*data).buffer.offset((*data).offset as isize) as *const ::core::ffi::c_void,
        ((*data).length - (*data).offset) as gsize,
        g_task_get_priority(task) as ::core::ffi::c_int,
        g_task_get_cancellable(task),
        callback,
        task as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_g_socks4a_proxy_connect_async(
    mut proxy: *mut GProxy,
    mut io_stream: *mut GIOStream,
    mut proxy_address: *mut GProxyAddress,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut data: *mut ConnectAsyncData = ::core::ptr::null_mut::<ConnectAsyncData>();
    let mut hostname: *const gchar = ::core::ptr::null::<gchar>();
    let mut port: guint16 = 0;
    let mut username: *const gchar = ::core::ptr::null::<gchar>();
    data = ({
        let mut __s: gsize = ::core::mem::size_of::<ConnectAsyncData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut ConnectAsyncData;
    (*data).io_stream = g_object_ref(io_stream as gpointer) as *mut GIOStream as *mut GIOStream;
    task = g_task_new(proxy as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GProxy,
                    *mut GIOStream,
                    *mut GProxyAddress,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_socks4a_proxy_connect_async
                as unsafe extern "C" fn(
                    *mut GProxy,
                    *mut GIOStream,
                    *mut GProxyAddress,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"g_socks4a_proxy_connect_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        data as gpointer,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut ConnectAsyncData) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_free_connect_data as unsafe extern "C" fn(*mut ConnectAsyncData) -> (),
        )),
    );
    hostname = g_proxy_address_get_destination_hostname(proxy_address);
    port = g_proxy_address_get_destination_port(proxy_address);
    username = g_proxy_address_get_username(proxy_address);
    (*data).buffer = g_malloc0(SOCKS4_CONN_MSG_LEN as gsize) as *mut guint8;
    (*data).length = safe_c2rust_set_connect_msg(
        (*data).buffer,
        hostname,
        port,
        username as *const ::core::ffi::c_char,
        &raw mut error,
    ) as gssize;
    (*data).offset = 0 as gssize;
    if (*data).length < 0 as gssize {
        g_task_return_error(task, error);
        g_object_unref(task as gpointer);
    } else {
        safe_c2rust_do_write(
            Some(
                safe_c2rust_connect_msg_write_cb
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            task,
            data,
        );
    };
}
unsafe extern "C" fn safe_c2rust_connect_msg_write_cb(
    mut source: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut data: *mut ConnectAsyncData = g_task_get_task_data(task) as *mut ConnectAsyncData;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut written: gssize = 0;
    written = g_output_stream_write_finish(
        source as *mut ::core::ffi::c_void as *mut GOutputStream,
        result,
        &raw mut error,
    );
    if written < 0 as gssize {
        g_task_return_error(task, error);
        g_object_unref(task as gpointer);
        return;
    }
    (*data).offset += written;
    if (*data).offset == (*data).length {
        g_free((*data).buffer as gpointer);
        (*data).buffer = g_malloc0(SOCKS4_CONN_REP_LEN as gsize) as *mut guint8;
        (*data).length = SOCKS4_CONN_REP_LEN as gssize;
        (*data).offset = 0 as gssize;
        safe_c2rust_do_read(
            Some(
                safe_c2rust_connect_reply_read_cb
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            task,
            data,
        );
    } else {
        safe_c2rust_do_write(
            Some(
                safe_c2rust_connect_msg_write_cb
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            task,
            data,
        );
    };
}
unsafe extern "C" fn safe_c2rust_connect_reply_read_cb(
    mut source: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut data: *mut ConnectAsyncData = g_task_get_task_data(task) as *mut ConnectAsyncData;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut read: gssize = 0;
    read = g_input_stream_read_finish(
        source as *mut ::core::ffi::c_void as *mut GInputStream,
        result,
        &raw mut error,
    );
    if read < 0 as gssize {
        g_task_return_error(task, error);
        g_object_unref(task as gpointer);
        return;
    }
    (*data).offset += read;
    if (*data).offset == (*data).length {
        if safe_c2rust_parse_connect_reply((*data).buffer, &raw mut error) == 0 {
            g_task_return_error(task, error);
            g_object_unref(task as gpointer);
            return;
        } else {
            g_task_return_pointer(
                task,
                g_object_ref((*data).io_stream as gpointer) as *mut GIOStream as gpointer,
                Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
            );
            g_object_unref(task as gpointer);
            return;
        }
    } else {
        safe_c2rust_do_read(
            Some(
                safe_c2rust_connect_reply_read_cb
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            task,
            data,
        );
    };
}
unsafe extern "C" fn safe_c2rust_g_socks4a_proxy_connect_finish(
    mut proxy: *mut GProxy,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GIOStream {
    return g_task_propagate_pointer(result as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GIOStream;
}
unsafe extern "C" fn safe_c2rust_g_socks4a_proxy_supports_hostname(
    mut proxy: *mut GProxy,
) -> gboolean {
    return (*(proxy as *mut ::core::ffi::c_void as *mut GSocks4aProxy)).supports_hostname;
}
unsafe extern "C" fn safe_c2rust_g_socks4a_proxy_class_init(mut class: *mut GSocks4aProxyClass) {
    let mut object_class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    object_class = class as *mut GObjectClass;
    (*object_class).finalize =
        Some(safe_c2rust_g_socks4a_proxy_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
}
unsafe extern "C" fn safe_c2rust_g_socks4a_proxy_iface_init(mut proxy_iface: *mut GProxyInterface) {
    (*proxy_iface).connect = Some(
        safe_c2rust_g_socks4a_proxy_connect
            as unsafe extern "C" fn(
                *mut GProxy,
                *mut GIOStream,
                *mut GProxyAddress,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GIOStream,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GProxy,
                *mut GIOStream,
                *mut GProxyAddress,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GIOStream,
        >;
    (*proxy_iface).connect_async = Some(
        safe_c2rust_g_socks4a_proxy_connect_async
            as unsafe extern "C" fn(
                *mut GProxy,
                *mut GIOStream,
                *mut GProxyAddress,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GProxy,
                *mut GIOStream,
                *mut GProxyAddress,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*proxy_iface).connect_finish = Some(
        safe_c2rust_g_socks4a_proxy_connect_finish
            as unsafe extern "C" fn(
                *mut GProxy,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GIOStream,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GProxy,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GIOStream,
        >;
    (*proxy_iface).supports_hostname = Some(
        safe_c2rust_g_socks4a_proxy_supports_hostname
            as unsafe extern "C" fn(*mut GProxy) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GProxy) -> gboolean>;
}
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
