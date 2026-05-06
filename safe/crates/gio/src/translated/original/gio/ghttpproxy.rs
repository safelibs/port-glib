extern "C" {
    pub type _GData;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _GInetSocketAddressPrivate;
    pub type _GInputStreamPrivate;
    pub type _GIOExtension;
    pub type _GOutputStreamPrivate;
    pub type _GIOStreamPrivate;
    pub type _GSocketConnectable;
    pub type _GTask;
    pub type _GTlsConnectionPrivate;
    pub type _GProxy;
    pub type _GProxyAddressPrivate;
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
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_base64_encode(data: *const guchar, len: gsize) -> *mut gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_realloc(mem: gpointer, n_bytes: gsize) -> gpointer;
    fn g_hostname_to_ascii(hostname: *const gchar) -> *mut gchar;
    fn g_str_has_suffix(str: *const gchar, suffix: *const gchar) -> gboolean;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_string_new(init: *const gchar) -> *mut GString;
    fn g_string_free(string: *mut GString, free_segment: gboolean) -> *mut gchar;
    fn g_string_free_and_steal(string: *mut GString) -> *mut gchar;
    fn g_string_insert_len(
        string: *mut GString,
        pos: gssize,
        val: *const gchar,
        len: gssize,
    ) -> *mut GString;
    fn g_string_append_len(string: *mut GString, val: *const gchar, len: gssize) -> *mut GString;
    fn g_string_append_printf(string: *mut GString, format: *const gchar, ...);
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
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_io_extension_point_implement(
        extension_point_name: *const ::core::ffi::c_char,
        type_0: GType,
        extension_name: *const ::core::ffi::c_char,
        priority: gint,
    ) -> *mut GIOExtension;
    fn _g_io_modules_ensure_extension_points_registered();
    fn g_input_stream_read(
        stream: *mut GInputStream,
        buffer: *mut ::core::ffi::c_void,
        count: gsize,
        cancellable: *mut GCancellable,
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
    fn g_io_error_quark() -> GQuark;
    fn g_io_stream_get_input_stream(stream: *mut GIOStream) -> *mut GInputStream;
    fn g_io_stream_get_output_stream(stream: *mut GIOStream) -> *mut GOutputStream;
    fn glib_gettext(str: *const gchar) -> *const gchar;
    fn g_proxy_get_type() -> GType;
    fn g_proxy_address_get_destination_hostname(proxy: *mut GProxyAddress) -> *const gchar;
    fn g_proxy_address_get_destination_port(proxy: *mut GProxyAddress) -> guint16;
    fn g_proxy_address_get_username(proxy: *mut GProxyAddress) -> *const gchar;
    fn g_proxy_address_get_password(proxy: *mut GProxyAddress) -> *const gchar;
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
    fn g_task_get_name(task: *mut GTask) -> *const gchar;
    fn g_task_run_in_thread(task: *mut GTask, task_func: GTaskThreadFunc);
    fn g_task_return_pointer(task: *mut GTask, result: gpointer, result_destroy: GDestroyNotify);
    fn g_task_return_error(task: *mut GTask, error: *mut GError);
    fn g_task_propagate_pointer(task: *mut GTask, error: *mut *mut GError) -> gpointer;
    fn g_tls_connection_handshake(
        conn: *mut GTlsConnection,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_tls_client_connection_new(
        base_io_stream: *mut GIOStream,
        server_identity: *mut GSocketConnectable,
        error: *mut *mut GError,
    ) -> *mut GIOStream;
}
pub type size_t = usize;
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
pub type guchar = ::core::ffi::c_uchar;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub type GString = _GString;
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
pub type GSocketConnectable = _GSocketConnectable;
pub type GTask = _GTask;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTlsConnection {
    pub parent_instance: GIOStream,
    pub priv_0: *mut GTlsConnectionPrivate,
}
pub type GTlsConnectionPrivate = _GTlsConnectionPrivate;
pub type GTlsConnection = _GTlsConnection;
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
pub struct _GHttpProxy {
    pub parent: GObject,
}
pub type GHttpProxy = _GHttpProxy;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GHttpProxyClass {
    pub parent_class: GObjectClass,
}
pub type GHttpProxyClass = _GHttpProxyClass;
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
    pub proxy_address: *mut GProxyAddress,
}
pub type GHttpsProxy = _GHttpsProxy;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GHttpsProxy {
    pub parent: GHttpProxy,
}
pub type GHttpsProxyClass = _GHttpsProxyClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GHttpsProxyClass {
    pub parent_class: GHttpProxyClass,
}
pub type GTaskThreadFunc =
    Option<unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> ()>;
pub const GLIB_MAJOR_VERSION: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const GLIB_MINOR_VERSION: ::core::ffi::c_int = 80 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn safe_c2rust_atoi(
    mut __nptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn safe_c2rust_g_string_append_len_inline(
    mut gstring: *mut GString,
    mut val: *const ::core::ffi::c_char,
    mut len: gssize,
) -> *mut GString {
    let mut len_unsigned: gsize = 0;
    if ({
        let mut _g_boolean_var_4: ::core::ffi::c_int = 0;
        if gstring.is_null() {
            _g_boolean_var_4 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_4 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_4
    }) as ::core::ffi::c_long
        != 0
    {
        return g_string_append_len(gstring, val as *const gchar, len);
    }
    if ({
        let mut _g_boolean_var_5: ::core::ffi::c_int = 0;
        if val.is_null() {
            _g_boolean_var_5 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_5 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_5
    }) as ::core::ffi::c_long
        != 0
    {
        return if len != 0 as gssize {
            g_string_append_len(gstring, val as *const gchar, len)
        } else {
            gstring
        };
    }
    if len < 0 as gssize {
        len_unsigned = strlen(val) as gsize;
    } else {
        len_unsigned = len as gsize;
    }
    if ({
        let mut _g_boolean_var_6: ::core::ffi::c_int = 0;
        if (*gstring).len.wrapping_add(len_unsigned) < (*gstring).allocated_len {
            _g_boolean_var_6 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_6 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_6
    }) as ::core::ffi::c_long
        != 0
    {
        let mut end: *mut ::core::ffi::c_char = (*gstring).str_0.offset((*gstring).len as isize);
        if ({
            let mut _g_boolean_var_7: ::core::ffi::c_int = 0;
            if val.offset(len_unsigned as isize) <= end as *const ::core::ffi::c_char
                || val > end.offset(len_unsigned as isize) as *const ::core::ffi::c_char
            {
                _g_boolean_var_7 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_7 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_7
        }) as ::core::ffi::c_long
            != 0
        {
            memcpy(
                end as *mut ::core::ffi::c_void,
                val as *const ::core::ffi::c_void,
                len_unsigned as size_t,
            );
        } else {
            memmove(
                end as *mut ::core::ffi::c_void,
                val as *const ::core::ffi::c_void,
                len_unsigned as size_t,
            );
        }
        (*gstring).len = (*gstring).len.wrapping_add(len_unsigned);
        *(*gstring).str_0.offset((*gstring).len as isize) = 0 as gchar;
        return gstring;
    } else {
        return g_string_insert_len(
            gstring,
            -(1 as ::core::ffi::c_int) as gssize,
            val as *const gchar,
            len,
        );
    };
}
static mut safe_c2rust_g_http_proxy_parent_class: gpointer = NULL_0;
unsafe extern "C" fn safe_c2rust_g_http_proxy_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_http_proxy_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GHttpProxy_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut safe_c2rust_GHttpProxy_private_offset);
    }
    safe_c2rust_g_http_proxy_class_init(klass as *mut GHttpProxyClass);
}
static mut safe_c2rust_GHttpProxy_private_offset: gint = 0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_http_proxy_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GHttpProxy\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GHttpProxyClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_http_proxy_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GHttpProxy>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GHttpProxy) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_http_proxy_init as unsafe extern "C" fn(*mut GHttpProxy) -> (),
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
            safe_c2rust_g_http_proxy_iface_init as unsafe extern "C" fn(*mut GProxyInterface) -> (),
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
        b"http\0" as *const u8 as *const ::core::ffi::c_char,
        0 as gint,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_http_proxy_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_http_proxy_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_http_proxy_init(mut proxy: *mut GHttpProxy) {}
unsafe extern "C" fn safe_c2rust_create_request(
    mut proxy_address: *mut GProxyAddress,
    mut has_cred: *mut gboolean,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut hostname: *const gchar = ::core::ptr::null::<gchar>();
    let mut port: gint = 0;
    let mut username: *const gchar = ::core::ptr::null::<gchar>();
    let mut password: *const gchar = ::core::ptr::null::<gchar>();
    let mut request: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut ascii_hostname: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if !has_cred.is_null() {
        *has_cred = FALSE as gboolean;
    }
    hostname = g_proxy_address_get_destination_hostname(proxy_address);
    ascii_hostname = g_hostname_to_ascii(hostname);
    if ascii_hostname.is_null() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(b"Invalid hostname\0" as *const u8 as *const gchar),
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    port = g_proxy_address_get_destination_port(proxy_address) as gint;
    username = g_proxy_address_get_username(proxy_address);
    password = g_proxy_address_get_password(proxy_address);
    request = g_string_new(::core::ptr::null::<gchar>());
    g_string_append_printf(
        request,
        b"CONNECT %s:%i HTTP/1.0\r\nHost: %s:%i\r\nProxy-Connection: keep-alive\r\nUser-Agent: GLib/%i.%i\r\n\0"
            as *const u8 as *const gchar,
        ascii_hostname,
        port,
        ascii_hostname,
        port,
        GLIB_MAJOR_VERSION,
        GLIB_MINOR_VERSION,
    );
    g_free(ascii_hostname as gpointer);
    if !username.is_null() && !password.is_null() {
        let mut cred: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut base64_cred: *mut gchar = ::core::ptr::null_mut::<gchar>();
        if !has_cred.is_null() {
            *has_cred = TRUE as gboolean;
        }
        cred = g_strdup_printf(b"%s:%s\0" as *const u8 as *const gchar, username, password);
        base64_cred = g_base64_encode(cred as *mut guchar, strlen(cred) as gsize);
        g_free(cred as gpointer);
        g_string_append_printf(
            request,
            b"Proxy-Authorization: Basic %s\r\n\0" as *const u8 as *const gchar,
            base64_cred,
        );
        g_free(base64_cred as gpointer);
    }
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char =
                b"\r\n\0" as *const u8 as *const ::core::ffi::c_char;
            safe_c2rust_g_string_append_len_inline(
                request,
                __val,
                if ({
                    let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
                    if !__val.is_null() {
                        _g_boolean_var_10 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_10 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_10
                }) as ::core::ffi::c_long
                    != 0
                {
                    strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize)) as gssize
                } else {
                    -(1 as ::core::ffi::c_int) as gssize
                },
            );
        });
    } else {
        safe_c2rust_g_string_append_len_inline(
            request,
            b"\r\n\0" as *const u8 as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(request, 0 as gboolean)
        } else {
            g_string_free_and_steal(request)
        }
    } else {
        g_string_free(request, 0 as gboolean)
    };
}
unsafe extern "C" fn safe_c2rust_check_reply(
    mut buffer: *const gchar,
    mut has_cred: gboolean,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut err_code: gint = 0;
    let mut ptr: *const gchar = buffer.offset(7 as ::core::ffi::c_int as isize);
    if strncmp(
        buffer as *const ::core::ffi::c_char,
        b"HTTP/1.\0" as *const u8 as *const ::core::ffi::c_char,
        7 as size_t,
    ) != 0 as ::core::ffi::c_int
        || *ptr as ::core::ffi::c_int != '0' as i32 && *ptr as ::core::ffi::c_int != '1' as i32
    {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_PROXY_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(b"Bad HTTP proxy reply\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    ptr = ptr.offset(1);
    while *ptr as ::core::ffi::c_int == ' ' as i32 {
        ptr = ptr.offset(1);
    }
    err_code = safe_c2rust_atoi(ptr as *const ::core::ffi::c_char) as gint;
    if err_code < 200 as ::core::ffi::c_int || err_code >= 300 as ::core::ffi::c_int {
        match err_code {
            403 => {
                g_set_error_literal(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_PROXY_NOT_ALLOWED as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"HTTP proxy connection not allowed\0" as *const u8 as *const gchar,
                    ),
                );
            }
            407 => {
                if has_cred != 0 {
                    g_set_error_literal(
                        error,
                        g_io_error_quark(),
                        G_IO_ERROR_PROXY_AUTH_FAILED as ::core::ffi::c_int as gint,
                        glib_gettext(
                            b"HTTP proxy authentication failed\0" as *const u8 as *const gchar,
                        ),
                    );
                } else {
                    g_set_error_literal(
                        error,
                        g_io_error_quark(),
                        G_IO_ERROR_PROXY_NEED_AUTH as ::core::ffi::c_int as gint,
                        glib_gettext(
                            b"HTTP proxy authentication required\0" as *const u8 as *const gchar,
                        ),
                    );
                }
            }
            _ => {
                g_set_error(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_PROXY_FAILED as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"HTTP proxy connection failed: %i\0" as *const u8 as *const gchar,
                    ),
                    err_code,
                );
            }
        }
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_http_proxy_connect(
    mut proxy: *mut GProxy,
    mut io_stream: *mut GIOStream,
    mut proxy_address: *mut GProxyAddress,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GIOStream {
    let mut current_block: u64;
    let mut in_0: *mut GInputStream = ::core::ptr::null_mut::<GInputStream>();
    let mut out: *mut GOutputStream = ::core::ptr::null_mut::<GOutputStream>();
    let mut buffer: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut buffer_length: gsize = 0;
    let mut bytes_read: gsize = 0;
    let mut has_cred: gboolean = 0;
    let mut tlsconn: *mut GIOStream = ::core::ptr::null_mut::<GIOStream>();
    if ({
        let mut __inst: *mut GTypeInstance = proxy as *mut GTypeInstance;
        let mut __t: GType = safe_c2rust__g_https_proxy_get_type();
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
        tlsconn = g_tls_client_connection_new(
            io_stream,
            proxy_address as *mut ::core::ffi::c_void as *mut GSocketConnectable,
            error,
        );
        if tlsconn.is_null() {
            current_block = 8969963095280344276;
        } else if g_tls_connection_handshake(
            tlsconn as *mut ::core::ffi::c_void as *mut GTlsConnection,
            cancellable,
            error,
        ) == 0
        {
            current_block = 8969963095280344276;
        } else {
            io_stream = tlsconn;
            current_block = 11006700562992250127;
        }
    } else {
        current_block = 11006700562992250127;
    }
    match current_block {
        11006700562992250127 => {
            in_0 = g_io_stream_get_input_stream(io_stream);
            out = g_io_stream_get_output_stream(io_stream);
            buffer = safe_c2rust_create_request(proxy_address, &raw mut has_cred, error);
            if !buffer.is_null() {
                if !(g_output_stream_write_all(
                    out,
                    buffer as *const ::core::ffi::c_void,
                    strlen(buffer) as gsize,
                    ::core::ptr::null_mut::<gsize>(),
                    cancellable,
                    error,
                ) == 0)
                {
                    g_free(buffer as gpointer);
                    bytes_read = 0 as gsize;
                    buffer_length = 1024 as gsize;
                    buffer = g_malloc(buffer_length) as *mut gchar;
                    loop {
                        let mut signed_nread: gssize = 0;
                        let mut nread: gsize = 0;
                        signed_nread = g_input_stream_read(
                            in_0,
                            buffer.offset(bytes_read as isize) as *mut ::core::ffi::c_void,
                            1 as gsize,
                            cancellable,
                            error,
                        );
                        if signed_nread == -(1 as ::core::ffi::c_int) as gssize {
                            current_block = 8969963095280344276;
                            break;
                        }
                        nread = signed_nread as gsize;
                        if nread == 0 as gsize {
                            current_block = 18386322304582297246;
                            break;
                        }
                        bytes_read = bytes_read.wrapping_add(1);
                        if bytes_read == buffer_length {
                            if buffer_length > 98304 as gsize {
                                g_set_error_literal(
                                    error,
                                    g_io_error_quark(),
                                    G_IO_ERROR_PROXY_FAILED as ::core::ffi::c_int as gint,
                                    glib_gettext(
                                        b"HTTP proxy response too big\0" as *const u8
                                            as *const gchar,
                                    ),
                                );
                                current_block = 8969963095280344276;
                                break;
                            } else {
                                buffer_length = (2 as gsize).wrapping_mul(buffer_length);
                                buffer = g_realloc(buffer as gpointer, buffer_length) as *mut gchar;
                            }
                        }
                        *buffer.offset(bytes_read as isize) = '\0' as i32 as gchar;
                        if if 0 != 0 {
                            ({
                                let __str: *const ::core::ffi::c_char = buffer;
                                let __suffix: *const ::core::ffi::c_char =
                                    b"\r\n\r\n\0" as *const u8 as *const ::core::ffi::c_char;
                                let mut __result: gboolean = FALSE;
                                if ({
                                    let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
                                    if __str.is_null() || __suffix.is_null() {
                                        _g_boolean_var_11 = 1 as ::core::ffi::c_int;
                                    } else {
                                        _g_boolean_var_11 = 0 as ::core::ffi::c_int;
                                    }
                                    _g_boolean_var_11
                                }) as ::core::ffi::c_long
                                    != 0
                                {
                                    __result = g_str_has_suffix(
                                        __str as *const gchar,
                                        __suffix as *const gchar,
                                    );
                                } else {
                                    let __str_len: size_t = strlen(
                                        __str
                                            .offset(__str.is_null() as ::core::ffi::c_int as isize),
                                    )
                                        as size_t;
                                    let __suffix_len: size_t =
                                        strlen(__suffix.offset(__suffix.is_null()
                                            as ::core::ffi::c_int
                                            as isize))
                                            as size_t;
                                    if __str_len >= __suffix_len {
                                        __result = (memcmp(
                                            __str
                                                .offset(__str_len as isize)
                                                .offset(-(__suffix_len as isize))
                                                as *const ::core::ffi::c_void,
                                            __suffix
                                                .offset(__suffix.is_null() as ::core::ffi::c_int
                                                    as isize)
                                                as *const ::core::ffi::c_void,
                                            __suffix_len,
                                        ) == 0 as ::core::ffi::c_int)
                                            as ::core::ffi::c_int
                                            as gboolean;
                                    }
                                }
                                __result
                            })
                        } else {
                            g_str_has_suffix(buffer, b"\r\n\r\n\0" as *const u8 as *const gchar)
                        } != 0
                        {
                            current_block = 18386322304582297246;
                            break;
                        }
                        if !(FALSE == 0) {
                            current_block = 18386322304582297246;
                            break;
                        }
                    }
                    match current_block {
                        8969963095280344276 => {}
                        _ => {
                            if bytes_read == 0 as gsize {
                                g_set_error_literal(
                                    error,
                                    g_io_error_quark(),
                                    G_IO_ERROR_PROXY_FAILED as ::core::ffi::c_int as gint,
                                    glib_gettext(
                                        b"HTTP proxy server closed connection unexpectedly.\0"
                                            as *const u8
                                            as *const gchar,
                                    ),
                                );
                            } else if !(safe_c2rust_check_reply(buffer, has_cred, error) == 0) {
                                g_free(buffer as gpointer);
                                g_object_ref(io_stream as gpointer);
                                let mut _pp: *mut *mut GIOStream = &raw mut tlsconn;
                                let mut _ptr: *mut GIOStream = *_pp;
                                *_pp = ::core::ptr::null_mut::<GIOStream>();
                                if !_ptr.is_null() {
                                    g_object_unref(_ptr as gpointer);
                                }
                                return io_stream;
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    let mut _pp_0: *mut *mut GIOStream = &raw mut tlsconn;
    let mut _ptr_0: *mut GIOStream = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<GIOStream>();
    if !_ptr_0.is_null() {
        g_object_unref(_ptr_0 as gpointer);
    }
    g_free(buffer as gpointer);
    return ::core::ptr::null_mut::<GIOStream>();
}
unsafe extern "C" fn safe_c2rust_free_connect_data(mut data: *mut ConnectAsyncData) {
    g_object_unref((*data).io_stream as gpointer);
    g_object_unref((*data).proxy_address as gpointer);
    g_slice_free1(
        ::core::mem::size_of::<ConnectAsyncData>() as gsize,
        data as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_connect_thread(
    mut task: *mut GTask,
    mut source_object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut proxy: *mut GProxy = source_object as *mut GProxy;
    let mut data: *mut ConnectAsyncData = task_data as *mut ConnectAsyncData;
    let mut res: *mut GIOStream = ::core::ptr::null_mut::<GIOStream>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    res = safe_c2rust_g_http_proxy_connect(
        proxy,
        (*data).io_stream,
        (*data).proxy_address,
        cancellable,
        &raw mut error,
    );
    if res.is_null() {
        g_task_return_error(task, error);
    } else {
        g_task_return_pointer(
            task,
            res as gpointer,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    };
}
unsafe extern "C" fn safe_c2rust_g_http_proxy_connect_async(
    mut proxy: *mut GProxy,
    mut io_stream: *mut GIOStream,
    mut proxy_address: *mut GProxyAddress,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut data: *mut ConnectAsyncData = ::core::ptr::null_mut::<ConnectAsyncData>();
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
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
    (*data).proxy_address =
        g_object_ref(proxy_address as gpointer) as *mut GProxyAddress as *mut GProxyAddress;
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
            safe_c2rust_g_http_proxy_connect_async
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
            b"g_http_proxy_connect_async\0" as *const u8 as *const gchar,
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
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_connect_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_http_proxy_connect_finish(
    mut proxy: *mut GProxy,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GIOStream {
    return g_task_propagate_pointer(result as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GIOStream;
}
unsafe extern "C" fn safe_c2rust_g_http_proxy_supports_hostname(
    mut proxy: *mut GProxy,
) -> gboolean {
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_http_proxy_class_init(mut class: *mut GHttpProxyClass) {}
unsafe extern "C" fn safe_c2rust_g_http_proxy_iface_init(mut proxy_iface: *mut GProxyInterface) {
    (*proxy_iface).connect = Some(
        safe_c2rust_g_http_proxy_connect
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
        safe_c2rust_g_http_proxy_connect_async
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
        safe_c2rust_g_http_proxy_connect_finish
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
        safe_c2rust_g_http_proxy_supports_hostname as unsafe extern "C" fn(*mut GProxy) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GProxy) -> gboolean>;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_https_proxy_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_https_proxy_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_https_proxy_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_https_proxy_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GHttpsProxy_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut safe_c2rust_GHttpsProxy_private_offset);
    }
    safe_c2rust_g_https_proxy_class_init(klass as *mut GHttpsProxyClass);
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_https_proxy_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        safe_c2rust__g_http_proxy_get_type(),
        g_intern_static_string(b"GHttpsProxy\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GHttpsProxyClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_https_proxy_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GHttpsProxy>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GHttpsProxy) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_https_proxy_init as unsafe extern "C" fn(*mut GHttpsProxy) -> (),
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
            safe_c2rust_g_http_proxy_iface_init as unsafe extern "C" fn(*mut GProxyInterface) -> (),
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
        b"https\0" as *const u8 as *const ::core::ffi::c_char,
        0 as gint,
    );
    return g_define_type_id;
}
static mut safe_c2rust_GHttpsProxy_private_offset: gint = 0;
static mut safe_c2rust_g_https_proxy_parent_class: gpointer = NULL_0;
unsafe extern "C" fn safe_c2rust_g_https_proxy_init(mut proxy: *mut GHttpsProxy) {}
unsafe extern "C" fn safe_c2rust_g_https_proxy_class_init(mut class: *mut GHttpsProxyClass) {}
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
