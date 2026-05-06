extern "C" {
    pub type _GData;
    pub type _GAsyncResult;
    pub type _GInputStreamPrivate;
    pub type _GOutputStreamPrivate;
    pub type _GCancellablePrivate;
    pub type _GIOStreamPrivate;
    pub type _GSocketPrivate;
    pub type _GSocketControlMessagePrivate;
    pub type _GSocketConnectionPrivate;
    pub type _GTask;
    pub type _GCredentials;
    pub type _GUnixCredentialsMessagePrivate;
    pub type _GUnixConnectionPrivate;
    pub type _GUnixFDMessagePrivate;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_prefix_error(err: *mut *mut GError, format: *const gchar, ...);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn g_dngettext(
        domain: *const gchar,
        msgid: *const gchar,
        msgid_plural: *const gchar,
        n: gulong,
    ) -> *const gchar;
    fn g_free(mem: gpointer);
    fn g_strerror(errnum: gint) -> *const gchar;
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn __errno_location() -> *mut ::core::ffi::c_int;
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
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_type_ensure(type_0: GType);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_object_get(object: gpointer, first_property_name: *const gchar, ...);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_credentials_new() -> *mut GCredentials;
    fn g_io_error_quark() -> GQuark;
    fn g_io_error_from_errno(err_no: gint) -> GIOErrorEnum;
    fn g_socket_receive_message(
        socket: *mut GSocket,
        address: *mut *mut GSocketAddress,
        vectors: *mut GInputVector,
        num_vectors: gint,
        messages: *mut *mut *mut GSocketControlMessage,
        num_messages: *mut gint,
        flags: *mut gint,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gssize;
    fn g_socket_send_message(
        socket: *mut GSocket,
        address: *mut GSocketAddress,
        vectors: *mut GOutputVector,
        num_vectors: gint,
        messages: *mut *mut GSocketControlMessage,
        num_messages: gint,
        flags: gint,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gssize;
    fn g_socket_get_credentials(socket: *mut GSocket, error: *mut *mut GError)
        -> *mut GCredentials;
    fn g_socket_get_option(
        socket: *mut GSocket,
        level: gint,
        optname: gint,
        value: *mut gint,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_socket_set_option(
        socket: *mut GSocket,
        level: gint,
        optname: gint,
        value: gint,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_socket_connection_get_type() -> GType;
    fn g_socket_connection_factory_register_type(
        g_type: GType,
        family: GSocketFamily,
        type_0: GSocketType,
        protocol: gint,
    );
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
    fn g_task_run_in_thread(task: *mut GTask, task_func: GTaskThreadFunc);
    fn g_task_return_pointer(task: *mut GTask, result: gpointer, result_destroy: GDestroyNotify);
    fn g_task_return_boolean(task: *mut GTask, result: gboolean);
    fn g_task_return_error(task: *mut GTask, error: *mut GError);
    fn g_task_propagate_pointer(task: *mut GTask, error: *mut *mut GError) -> gpointer;
    fn g_task_propagate_boolean(task: *mut GTask, error: *mut *mut GError) -> gboolean;
    fn g_unix_credentials_message_get_type() -> GType;
    fn g_unix_credentials_message_new_with_credentials(
        credentials: *mut GCredentials,
    ) -> *mut GSocketControlMessage;
    fn g_unix_credentials_message_get_credentials(
        message: *mut GUnixCredentialsMessage,
    ) -> *mut GCredentials;
    fn g_unix_credentials_message_is_supported() -> gboolean;
    fn g_unix_fd_message_get_type() -> GType;
    fn g_unix_fd_message_new() -> *mut GSocketControlMessage;
    fn g_unix_fd_message_steal_fds(message: *mut GUnixFDMessage, length: *mut gint) -> *mut gint;
    fn g_unix_fd_message_append_fd(
        message: *mut GUnixFDMessage,
        fd: gint,
        error: *mut *mut GError,
    ) -> gboolean;
    fn glib_gettext(str: *const gchar) -> *const gchar;
}
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
pub type GIOErrorEnum = ::core::ffi::c_uint;
pub const G_IO_ERROR_DESTINATION_UNSET: GIOErrorEnum = 48;
pub const G_IO_ERROR_NO_SUCH_DEVICE: GIOErrorEnum = 47;
pub const G_IO_ERROR_MESSAGE_TOO_LARGE: GIOErrorEnum = 46;
pub const G_IO_ERROR_NOT_CONNECTED: GIOErrorEnum = 45;
pub const G_IO_ERROR_CONNECTION_CLOSED: GIOErrorEnum = 44;
pub const G_IO_ERROR_BROKEN_PIPE: GIOErrorEnum = 44;
pub const G_IO_ERROR_PROXY_NOT_ALLOWED: GIOErrorEnum = 43;
pub const G_IO_ERROR_PROXY_NEED_AUTH: GIOErrorEnum = 42;
pub const G_IO_ERROR_PROXY_AUTH_FAILED: GIOErrorEnum = 41;
pub const G_IO_ERROR_PROXY_FAILED: GIOErrorEnum = 40;
pub const G_IO_ERROR_CONNECTION_REFUSED: GIOErrorEnum = 39;
pub const G_IO_ERROR_NETWORK_UNREACHABLE: GIOErrorEnum = 38;
pub const G_IO_ERROR_HOST_UNREACHABLE: GIOErrorEnum = 37;
pub const G_IO_ERROR_DBUS_ERROR: GIOErrorEnum = 36;
pub const G_IO_ERROR_INVALID_DATA: GIOErrorEnum = 35;
pub const G_IO_ERROR_PARTIAL_INPUT: GIOErrorEnum = 34;
pub const G_IO_ERROR_ADDRESS_IN_USE: GIOErrorEnum = 33;
pub const G_IO_ERROR_NOT_INITIALIZED: GIOErrorEnum = 32;
pub const G_IO_ERROR_TOO_MANY_OPEN_FILES: GIOErrorEnum = 31;
pub const G_IO_ERROR_FAILED_HANDLED: GIOErrorEnum = 30;
pub const G_IO_ERROR_WOULD_MERGE: GIOErrorEnum = 29;
pub const G_IO_ERROR_HOST_NOT_FOUND: GIOErrorEnum = 28;
pub const G_IO_ERROR_WOULD_BLOCK: GIOErrorEnum = 27;
pub const G_IO_ERROR_BUSY: GIOErrorEnum = 26;
pub const G_IO_ERROR_WOULD_RECURSE: GIOErrorEnum = 25;
pub const G_IO_ERROR_TIMED_OUT: GIOErrorEnum = 24;
pub const G_IO_ERROR_WRONG_ETAG: GIOErrorEnum = 23;
pub const G_IO_ERROR_CANT_CREATE_BACKUP: GIOErrorEnum = 22;
pub const G_IO_ERROR_READ_ONLY: GIOErrorEnum = 21;
pub const G_IO_ERROR_PENDING: GIOErrorEnum = 20;
pub const G_IO_ERROR_CANCELLED: GIOErrorEnum = 19;
pub const G_IO_ERROR_CLOSED: GIOErrorEnum = 18;
pub const G_IO_ERROR_ALREADY_MOUNTED: GIOErrorEnum = 17;
pub const G_IO_ERROR_NOT_MOUNTED: GIOErrorEnum = 16;
pub const G_IO_ERROR_NOT_SUPPORTED: GIOErrorEnum = 15;
pub const G_IO_ERROR_PERMISSION_DENIED: GIOErrorEnum = 14;
pub const G_IO_ERROR_INVALID_ARGUMENT: GIOErrorEnum = 13;
pub const G_IO_ERROR_NO_SPACE: GIOErrorEnum = 12;
pub const G_IO_ERROR_TOO_MANY_LINKS: GIOErrorEnum = 11;
pub const G_IO_ERROR_INVALID_FILENAME: GIOErrorEnum = 10;
pub const G_IO_ERROR_FILENAME_TOO_LONG: GIOErrorEnum = 9;
pub const G_IO_ERROR_NOT_MOUNTABLE_FILE: GIOErrorEnum = 8;
pub const G_IO_ERROR_NOT_SYMBOLIC_LINK: GIOErrorEnum = 7;
pub const G_IO_ERROR_NOT_REGULAR_FILE: GIOErrorEnum = 6;
pub const G_IO_ERROR_NOT_EMPTY: GIOErrorEnum = 5;
pub const G_IO_ERROR_NOT_DIRECTORY: GIOErrorEnum = 4;
pub const G_IO_ERROR_IS_DIRECTORY: GIOErrorEnum = 3;
pub const G_IO_ERROR_EXISTS: GIOErrorEnum = 2;
pub const G_IO_ERROR_NOT_FOUND: GIOErrorEnum = 1;
pub const G_IO_ERROR_FAILED: GIOErrorEnum = 0;
pub type GSocketFamily = ::core::ffi::c_uint;
pub const G_SOCKET_FAMILY_IPV6: GSocketFamily = 10;
pub const G_SOCKET_FAMILY_IPV4: GSocketFamily = 2;
pub const G_SOCKET_FAMILY_UNIX: GSocketFamily = 1;
pub const G_SOCKET_FAMILY_INVALID: GSocketFamily = 0;
pub type GSocketType = ::core::ffi::c_uint;
pub const G_SOCKET_TYPE_SEQPACKET: GSocketType = 3;
pub const G_SOCKET_TYPE_DATAGRAM: GSocketType = 2;
pub const G_SOCKET_TYPE_STREAM: GSocketType = 1;
pub const G_SOCKET_TYPE_INVALID: GSocketType = 0;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const G_SOCKET_MSG_DONTROUTE: C2RustUnnamed_0 = 4;
pub const G_SOCKET_MSG_PEEK: C2RustUnnamed_0 = 2;
pub const G_SOCKET_MSG_OOB: C2RustUnnamed_0 = 1;
pub const G_SOCKET_MSG_NONE: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = ::core::ffi::c_int;
pub const G_SOCKET_PROTOCOL_SCTP: C2RustUnnamed_1 = 132;
pub const G_SOCKET_PROTOCOL_UDP: C2RustUnnamed_1 = 17;
pub const G_SOCKET_PROTOCOL_TCP: C2RustUnnamed_1 = 6;
pub const G_SOCKET_PROTOCOL_DEFAULT: C2RustUnnamed_1 = 0;
pub const G_SOCKET_PROTOCOL_UNKNOWN: C2RustUnnamed_1 = -1;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
pub type GIOStream = _GIOStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GIOStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GIOStreamPrivate,
}
pub type GIOStreamPrivate = _GIOStreamPrivate;
pub type GSocketAddress = _GSocketAddress;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketAddress {
    pub parent_instance: GObject,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocket {
    pub parent_instance: GObject,
    pub priv_0: *mut GSocketPrivate,
}
pub type GSocketPrivate = _GSocketPrivate;
pub type GSocket = _GSocket;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketControlMessage {
    pub parent_instance: GObject,
    pub priv_0: *mut GSocketControlMessagePrivate,
}
pub type GSocketControlMessagePrivate = _GSocketControlMessagePrivate;
pub type GSocketControlMessage = _GSocketControlMessage;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketConnection {
    pub parent_instance: GIOStream,
    pub priv_0: *mut GSocketConnectionPrivate,
}
pub type GSocketConnectionPrivate = _GSocketConnectionPrivate;
pub type GSocketConnection = _GSocketConnection;
pub type GTask = _GTask;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInputVector {
    pub buffer: gpointer,
    pub size: gsize,
}
pub type GInputVector = _GInputVector;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GOutputVector {
    pub buffer: gconstpointer,
    pub size: gsize,
}
pub type GOutputVector = _GOutputVector;
pub type GCredentials = _GCredentials;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixCredentialsMessage {
    pub parent_instance: GSocketControlMessage,
    pub priv_0: *mut GUnixCredentialsMessagePrivate,
}
pub type GUnixCredentialsMessagePrivate = _GUnixCredentialsMessagePrivate;
pub type GUnixCredentialsMessage = _GUnixCredentialsMessage;
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
pub struct _GSocketConnectionClass {
    pub parent_class: GIOStreamClass,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved6: Option<unsafe extern "C" fn() -> ()>,
}
pub type GSocketConnectionClass = _GSocketConnectionClass;
pub type GTaskThreadFunc =
    Option<unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixConnection {
    pub parent_instance: GSocketConnection,
    pub priv_0: *mut GUnixConnectionPrivate,
}
pub type GUnixConnectionPrivate = _GUnixConnectionPrivate;
pub type GUnixConnection = _GUnixConnection;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixConnectionClass {
    pub parent_class: GSocketConnectionClass,
}
pub type GUnixConnectionClass = _GUnixConnectionClass;
pub type GUnixFDMessage = _GUnixFDMessage;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixFDMessage {
    pub parent_instance: GSocketControlMessage,
    pub priv_0: *mut GUnixFDMessagePrivate,
}
pub type GUnixFDMessagePrivate = _GUnixFDMessagePrivate;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SOL_SOCKET: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SO_PASSCRED: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_unix_connection_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_socket_connection_get_type(),
        g_intern_static_string(b"GUnixConnection\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GUnixConnectionClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_unix_connection_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GUnixConnection>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GUnixConnection) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_unix_connection_init
                    as unsafe extern "C" fn(*mut GUnixConnection) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    g_socket_connection_factory_register_type(
        g_define_type_id,
        G_SOCKET_FAMILY_UNIX,
        G_SOCKET_TYPE_STREAM,
        G_SOCKET_PROTOCOL_DEFAULT as ::core::ffi::c_int as gint,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_connection_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_unix_connection_get_type_once();
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
static mut safe_c2rust_GUnixConnection_private_offset: gint = 0;
static mut safe_c2rust_g_unix_connection_parent_class: gpointer = NULL;
unsafe extern "C" fn safe_c2rust_g_unix_connection_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_unix_connection_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GUnixConnection_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GUnixConnection_private_offset,
        );
    }
    safe_c2rust_g_unix_connection_class_init(klass as *mut GUnixConnectionClass);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_connection_send_fd(
    mut connection: *mut GUnixConnection,
    mut fd: gint,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut scm: *mut GSocketControlMessage = ::core::ptr::null_mut::<GSocketControlMessage>();
    let mut socket: *mut GSocket = ::core::ptr::null_mut::<GSocket>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_unix_connection_get_type();
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
            b"G_IS_UNIX_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if fd >= 0 as ::core::ffi::c_int {
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
            b"fd >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    scm = g_unix_fd_message_new();
    if g_unix_fd_message_append_fd(
        scm as *mut ::core::ffi::c_void as *mut GUnixFDMessage,
        fd,
        error,
    ) == 0
    {
        g_object_unref(scm as gpointer);
        return FALSE;
    }
    g_object_get(
        connection as gpointer,
        b"socket\0" as *const u8 as *const gchar,
        &raw mut socket,
        NULL,
    );
    if g_socket_send_message(
        socket,
        ::core::ptr::null_mut::<GSocketAddress>(),
        ::core::ptr::null_mut::<GOutputVector>(),
        0 as gint,
        &raw mut scm,
        1 as gint,
        0 as gint,
        cancellable,
        error,
    ) != 1 as gssize
    {
        g_object_unref(socket as gpointer);
        g_object_unref(scm as gpointer);
        return FALSE;
    }
    g_object_unref(socket as gpointer);
    g_object_unref(scm as gpointer);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_connection_receive_fd(
    mut connection: *mut GUnixConnection,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gint {
    let mut scms: *mut *mut GSocketControlMessage =
        ::core::ptr::null_mut::<*mut GSocketControlMessage>();
    let mut fds: *mut gint = ::core::ptr::null_mut::<gint>();
    let mut nfd: gint = 0;
    let mut fd: gint = 0;
    let mut nscm: gint = 0;
    let mut fdmsg: *mut GUnixFDMessage = ::core::ptr::null_mut::<GUnixFDMessage>();
    let mut socket: *mut GSocket = ::core::ptr::null_mut::<GSocket>();
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_unix_connection_get_type();
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
            b"G_IS_UNIX_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    g_object_get(
        connection as gpointer,
        b"socket\0" as *const u8 as *const gchar,
        &raw mut socket,
        NULL,
    );
    if g_socket_receive_message(
        socket,
        ::core::ptr::null_mut::<*mut GSocketAddress>(),
        ::core::ptr::null_mut::<GInputVector>(),
        0 as gint,
        &raw mut scms,
        &raw mut nscm,
        ::core::ptr::null_mut::<gint>(),
        cancellable,
        error,
    ) != 1 as gssize
    {
        g_object_unref(socket as gpointer);
        return -(1 as gint);
    }
    g_object_unref(socket as gpointer);
    if nscm != 1 as ::core::ffi::c_int {
        let mut i: gint = 0;
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
            g_dngettext(
                ::core::ptr::null::<gchar>(),
                b"Expecting 1 control message, got %d\0" as *const u8 as *const gchar,
                b"Expecting 1 control message, got %d\0" as *const u8 as *const gchar,
                nscm as gulong,
            ),
            nscm,
        );
        i = 0 as ::core::ffi::c_int as gint;
        while i < nscm {
            g_object_unref(*scms.offset(i as isize) as gpointer);
            i += 1;
        }
        g_free(scms as gpointer);
        return -(1 as gint);
    }
    if ({
        let mut __inst: *mut GTypeInstance =
            *scms.offset(0 as ::core::ffi::c_int as isize) as *mut GTypeInstance;
        let mut __t: GType = g_unix_fd_message_get_type();
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
            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(b"Unexpected type of ancillary data\0" as *const u8 as *const gchar),
        );
        g_object_unref(*scms.offset(0 as ::core::ffi::c_int as isize) as gpointer);
        g_free(scms as gpointer);
        return -(1 as gint);
    }
    fdmsg = *scms.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void
        as *mut GUnixFDMessage;
    g_free(scms as gpointer);
    fds = g_unix_fd_message_steal_fds(fdmsg, &raw mut nfd);
    g_object_unref(fdmsg as gpointer);
    if nfd != 1 as ::core::ffi::c_int {
        let mut i_0: gint = 0;
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
            g_dngettext(
                ::core::ptr::null::<gchar>(),
                b"Expecting one fd, but got %d\n\0" as *const u8 as *const gchar,
                b"Expecting one fd, but got %d\n\0" as *const u8 as *const gchar,
                nfd as gulong,
            ),
            nfd,
        );
        i_0 = 0 as ::core::ffi::c_int as gint;
        while i_0 < nfd {
            close(*fds.offset(i_0 as isize) as ::core::ffi::c_int);
            i_0 += 1;
        }
        g_free(fds as gpointer);
        return -(1 as gint);
    }
    fd = *fds;
    g_free(fds as gpointer);
    if fd < 0 as ::core::ffi::c_int {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(b"Received invalid fd\0" as *const u8 as *const gchar),
        );
        fd = -(1 as ::core::ffi::c_int) as gint;
    }
    return fd;
}
unsafe extern "C" fn safe_c2rust_g_unix_connection_init(mut connection: *mut GUnixConnection) {}
unsafe extern "C" fn safe_c2rust_g_unix_connection_class_init(
    mut class: *mut GUnixConnectionClass,
) {
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_connection_send_credentials(
    mut connection: *mut GUnixConnection,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut credentials: *mut GCredentials = ::core::ptr::null_mut::<GCredentials>();
    let mut scm: *mut GSocketControlMessage = ::core::ptr::null_mut::<GSocketControlMessage>();
    let mut socket: *mut GSocket = ::core::ptr::null_mut::<GSocket>();
    let mut ret: gboolean = 0;
    let mut vector: GOutputVector = _GOutputVector {
        buffer: ::core::ptr::null::<::core::ffi::c_void>(),
        size: 0,
    };
    let mut nul_byte: [guchar; 1] = ['\0' as i32 as guchar];
    let mut num_messages: gint = 0;
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_unix_connection_get_type();
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
            b"G_IS_UNIX_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    ret = FALSE as gboolean;
    credentials = g_credentials_new();
    vector.buffer = &raw mut nul_byte as gconstpointer;
    vector.size = 1 as gsize;
    if g_unix_credentials_message_is_supported() != 0 {
        scm = g_unix_credentials_message_new_with_credentials(credentials);
        num_messages = 1 as ::core::ffi::c_int as gint;
    } else {
        scm = ::core::ptr::null_mut::<GSocketControlMessage>();
        num_messages = 0 as ::core::ffi::c_int as gint;
    }
    g_object_get(
        connection as gpointer,
        b"socket\0" as *const u8 as *const gchar,
        &raw mut socket,
        NULL,
    );
    if g_socket_send_message(
        socket,
        ::core::ptr::null_mut::<GSocketAddress>(),
        &raw mut vector,
        1 as gint,
        &raw mut scm,
        num_messages,
        G_SOCKET_MSG_NONE as ::core::ffi::c_int as gint,
        cancellable,
        error,
    ) != 1 as gssize
    {
        g_prefix_error(
            error,
            glib_gettext(b"Error sending credentials: \0" as *const u8 as *const gchar),
        );
    } else {
        ret = TRUE as gboolean;
    }
    g_object_unref(socket as gpointer);
    if !scm.is_null() {
        g_object_unref(scm as gpointer);
    }
    g_object_unref(credentials as gpointer);
    return ret;
}
unsafe extern "C" fn safe_c2rust_send_credentials_async_thread(
    mut task: *mut GTask,
    mut source_object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if safe_c2rust_g_unix_connection_send_credentials(
        source_object as *mut GUnixConnection,
        cancellable,
        &raw mut error,
    ) != 0
    {
        g_task_return_boolean(task, TRUE);
    } else {
        g_task_return_error(task, error);
    }
    g_object_unref(task as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_connection_send_credentials_async(
    mut connection: *mut GUnixConnection,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    task = g_task_new(connection as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GUnixConnection,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_unix_connection_send_credentials_async
                as unsafe extern "C" fn(
                    *mut GUnixConnection,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"g_unix_connection_send_credentials_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_send_credentials_async_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_connection_send_credentials_finish(
    mut connection: *mut GUnixConnection,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, connection as gpointer) != 0 {
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
            b"g_task_is_valid (result, connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_connection_receive_credentials(
    mut connection: *mut GUnixConnection,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GCredentials {
    let mut current_block: u64;
    let mut ret: *mut GCredentials = ::core::ptr::null_mut::<GCredentials>();
    let mut scms: *mut *mut GSocketControlMessage =
        ::core::ptr::null_mut::<*mut GSocketControlMessage>();
    let mut nscm: gint = 0;
    let mut socket: *mut GSocket = ::core::ptr::null_mut::<GSocket>();
    let mut n: gint = 0;
    let mut num_bytes_read: gssize = 0;
    let mut turn_off_so_passcreds: gboolean = 0;
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_unix_connection_get_type();
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
            b"G_IS_UNIX_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GCredentials>();
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GCredentials>();
    }
    ret = ::core::ptr::null_mut::<GCredentials>();
    scms = ::core::ptr::null_mut::<*mut GSocketControlMessage>();
    g_object_get(
        connection as gpointer,
        b"socket\0" as *const u8 as *const gchar,
        &raw mut socket,
        NULL,
    );
    let mut opt_val: gint = 0;
    turn_off_so_passcreds = FALSE as gboolean;
    opt_val = 0 as ::core::ffi::c_int as gint;
    if g_socket_get_option(
        socket,
        SOL_SOCKET,
        SO_PASSCRED,
        &raw mut opt_val,
        ::core::ptr::null_mut::<*mut GError>(),
    ) == 0
    {
        let mut errsv: ::core::ffi::c_int = *__errno_location();
        g_set_error(
            error,
            g_io_error_quark(),
            g_io_error_from_errno(errsv as gint) as gint,
            glib_gettext(
                b"Error checking if SO_PASSCRED is enabled for socket: %s\0" as *const u8
                    as *const gchar,
            ),
            g_strerror(errsv as gint),
        );
    } else {
        if opt_val == 0 as ::core::ffi::c_int {
            if g_socket_set_option(
                socket,
                SOL_SOCKET,
                SO_PASSCRED,
                TRUE,
                ::core::ptr::null_mut::<*mut GError>(),
            ) == 0
            {
                let mut errsv_0: ::core::ffi::c_int = *__errno_location();
                g_set_error(
                    error,
                    g_io_error_quark(),
                    g_io_error_from_errno(errsv_0 as gint) as gint,
                    glib_gettext(b"Error enabling SO_PASSCRED: %s\0" as *const u8 as *const gchar),
                    g_strerror(errsv_0 as gint),
                );
                current_block = 9097995880671819619;
            } else {
                turn_off_so_passcreds = TRUE as gboolean;
                current_block = 17478428563724192186;
            }
        } else {
            current_block = 17478428563724192186;
        }
        match current_block {
            9097995880671819619 => {}
            _ => {
                g_type_ensure(g_unix_credentials_message_get_type());
                num_bytes_read = g_socket_receive_message(
                    socket,
                    ::core::ptr::null_mut::<*mut GSocketAddress>(),
                    ::core::ptr::null_mut::<GInputVector>(),
                    0 as gint,
                    &raw mut scms,
                    &raw mut nscm,
                    ::core::ptr::null_mut::<gint>(),
                    cancellable,
                    error,
                );
                if num_bytes_read != 1 as gssize {
                    if num_bytes_read == 0 as gssize && !error.is_null() && (*error).is_null() {
                        g_set_error_literal(
                            error,
                            g_io_error_quark(),
                            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                            glib_gettext(
                                b"Expecting to read a single byte for receiving credentials but read zero bytes\0"
                                    as *const u8 as *const gchar,
                            ),
                        );
                    }
                } else if g_unix_credentials_message_is_supported() != 0
                    && nscm > 0 as ::core::ffi::c_int
                {
                    if nscm != 1 as ::core::ffi::c_int {
                        g_set_error(
                            error,
                            g_io_error_quark(),
                            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                            g_dngettext(
                                ::core::ptr::null::<gchar>(),
                                b"Expecting 1 control message, got %d\0" as *const u8
                                    as *const gchar,
                                b"Expecting 1 control message, got %d\0" as *const u8
                                    as *const gchar,
                                nscm as gulong,
                            ),
                            nscm,
                        );
                    } else if ({
                        let mut __inst: *mut GTypeInstance =
                            *scms.offset(0 as ::core::ffi::c_int as isize) as *mut GTypeInstance;
                        let mut __t: GType = g_unix_credentials_message_get_type();
                        let mut __r: gboolean = 0;
                        if __inst.is_null() {
                            __r = FALSE as gboolean;
                        } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t
                        {
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
                            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                            glib_gettext(
                                b"Unexpected type of ancillary data\0" as *const u8 as *const gchar,
                            ),
                        );
                    } else {
                        ret = g_unix_credentials_message_get_credentials(
                            *scms.offset(0 as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_void
                                as *mut GUnixCredentialsMessage,
                        );
                        g_object_ref(ret as gpointer);
                    }
                } else if nscm != 0 as ::core::ffi::c_int {
                    g_set_error(
                        error,
                        g_io_error_quark(),
                        G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                        glib_gettext(
                            b"Not expecting control message, but got %d\0" as *const u8
                                as *const gchar,
                        ),
                        nscm,
                    );
                } else {
                    ret = g_socket_get_credentials(socket, error);
                }
            }
        }
    }
    while turn_off_so_passcreds != 0 {
        if !(g_socket_set_option(
            socket,
            SOL_SOCKET,
            SO_PASSCRED,
            FALSE,
            ::core::ptr::null_mut::<*mut GError>(),
        ) == 0)
        {
            break;
        }
        let mut errsv_1: ::core::ffi::c_int = *__errno_location();
        g_set_error(
            error,
            g_io_error_quark(),
            g_io_error_from_errno(errsv_1 as gint) as gint,
            glib_gettext(b"Error while disabling SO_PASSCRED: %s\0" as *const u8 as *const gchar),
            g_strerror(errsv_1 as gint),
        );
    }
    if !scms.is_null() {
        n = 0 as ::core::ffi::c_int as gint;
        while n < nscm {
            g_object_unref(*scms.offset(n as isize) as gpointer);
            n += 1;
        }
        g_free(scms as gpointer);
    }
    g_object_unref(socket as gpointer);
    return ret;
}
unsafe extern "C" fn safe_c2rust_receive_credentials_async_thread(
    mut task: *mut GTask,
    mut source_object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut creds: *mut GCredentials = ::core::ptr::null_mut::<GCredentials>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    creds = safe_c2rust_g_unix_connection_receive_credentials(
        source_object as *mut GUnixConnection,
        cancellable,
        &raw mut error,
    );
    if !creds.is_null() {
        g_task_return_pointer(
            task,
            creds as gpointer,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    } else {
        g_task_return_error(task, error);
    }
    g_object_unref(task as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_connection_receive_credentials_async(
    mut connection: *mut GUnixConnection,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    task = g_task_new(connection as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GUnixConnection,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_unix_connection_receive_credentials_async
                as unsafe extern "C" fn(
                    *mut GUnixConnection,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"g_unix_connection_receive_credentials_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_receive_credentials_async_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_connection_receive_credentials_finish(
    mut connection: *mut GUnixConnection,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GCredentials {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, connection as gpointer) != 0 {
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
            b"g_task_is_valid (result, connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GCredentials>();
    }
    return g_task_propagate_pointer(result as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GCredentials;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
