use ::c2rust_asm_casts;
use ::c2rust_asm_casts::AsmCastTrait;
use ::c2rust_bitfields;
use ::core::arch::asm;
use ::libc;
extern "C" {
    pub type _GBytes;
    pub type _GData;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GCancellablePrivate;
    pub type _GDatagramBased;
    pub type _GInetAddressPrivate;
    pub type _GInetSocketAddressPrivate;
    pub type _GInitable;
    pub type _GSocketControlMessagePrivate;
    pub type _GCredentials;
    pub type sockaddr_x25;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_ptr_array_new() -> *mut GPtrArray;
    fn g_ptr_array_free(array: *mut GPtrArray, free_seg: gboolean) -> *mut gpointer;
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_error_copy(error: *const GError) -> *mut GError;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_propagate_error(dest: *mut *mut GError, src: *mut GError);
    fn g_clear_error(err: *mut *mut GError);
    fn g_prefix_error(err: *mut *mut GError, format: *const gchar, ...);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn signal(__sig: ::core::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn g_bytes_new_take(data: gpointer, size: gsize) -> *mut GBytes;
    fn g_bytes_new_from_bytes(bytes: *mut GBytes, offset: gsize, length: gsize) -> *mut GBytes;
    fn g_bytes_unref(bytes: *mut GBytes);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_poll(fds: *mut GPollFD, nfds: guint, timeout: gint) -> gint;
    fn g_source_new(source_funcs: *mut GSourceFuncs, struct_size: guint) -> *mut GSource;
    fn g_source_unref(source: *mut GSource);
    fn g_source_set_static_name(source: *mut GSource, name: *const ::core::ffi::c_char);
    fn g_source_set_ready_time(source: *mut GSource, ready_time: gint64);
    fn g_source_get_ready_time(source: *mut GSource) -> gint64;
    fn g_source_add_unix_fd(source: *mut GSource, fd: gint, events: GIOCondition) -> gpointer;
    fn g_source_remove_unix_fd(source: *mut GSource, tag: gpointer);
    fn g_source_query_unix_fd(source: *mut GSource, tag: gpointer) -> GIOCondition;
    fn g_source_add_child_source(source: *mut GSource, child_source: *mut GSource);
    fn g_source_get_time(source: *mut GSource) -> gint64;
    fn g_get_monotonic_time() -> gint64;
    fn g_strerror(errnum: gint) -> *const gchar;
    fn g_memdup2(mem: gconstpointer, byte_size: gsize) -> gpointer;
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
    fn g_string_insert_c(string: *mut GString, pos: gssize, c: gchar) -> *mut GString;
    fn g_string_append_printf(string: *mut GString, format: *const gchar, ...);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn __errno_location() -> *mut ::core::ffi::c_int;
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
    fn g_type_ensure(type_0: GType);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_value_init(value: *mut GValue, g_type: GType) -> *mut GValue;
    fn g_value_unset(value: *mut GValue);
    fn g_closure_invoke(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
    );
    fn g_io_condition_get_type() -> GType;
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_value_set_object(value: *mut GValue, v_object: gpointer);
    fn g_value_take_object(value: *mut GValue, v_object: gpointer);
    fn g_value_set_enum(value: *mut GValue, v_enum: gint);
    fn g_value_get_enum(value: *const GValue) -> gint;
    fn g_value_set_flags(value: *mut GValue, v_flags: guint);
    fn g_param_spec_boolean(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: gboolean,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_int(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        minimum: gint,
        maximum: gint,
        default_value: gint,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_uint(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        minimum: guint,
        maximum: guint,
        default_value: guint,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_enum(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        enum_type: GType,
        default_value: gint,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_object(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        object_type: GType,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_source_set_dummy_callback(source: *mut GSource);
    fn g_value_set_boolean(value: *mut GValue, v_boolean: gboolean);
    fn g_value_get_boolean(value: *const GValue) -> gboolean;
    fn g_value_set_int(value: *mut GValue, v_int: gint);
    fn g_value_get_int(value: *const GValue) -> gint;
    fn g_value_set_uint(value: *mut GValue, v_uint: guint);
    fn g_value_get_uint(value: *const GValue) -> guint;
    fn fcntl(__fd: ::core::ffi::c_int, __cmd: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    fn g_unix_set_fd_nonblocking(fd: gint, nonblock: gboolean, error: *mut *mut GError)
        -> gboolean;
    fn ioctl(__fd: ::core::ffi::c_int, __request: ::core::ffi::c_ulong, ...) -> ::core::ffi::c_int;
    fn socket(
        __domain: ::core::ffi::c_int,
        __type: ::core::ffi::c_int,
        __protocol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn bind(
        __fd: ::core::ffi::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> ::core::ffi::c_int;
    fn getsockname(
        __fd: ::core::ffi::c_int,
        __addr: __SOCKADDR_ARG,
        __len: *mut socklen_t,
    ) -> ::core::ffi::c_int;
    fn connect(
        __fd: ::core::ffi::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> ::core::ffi::c_int;
    fn getpeername(
        __fd: ::core::ffi::c_int,
        __addr: __SOCKADDR_ARG,
        __len: *mut socklen_t,
    ) -> ::core::ffi::c_int;
    fn send(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __n: size_t,
        __flags: ::core::ffi::c_int,
    ) -> ssize_t;
    fn recv(
        __fd: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_void,
        __n: size_t,
        __flags: ::core::ffi::c_int,
    ) -> ssize_t;
    fn sendmsg(
        __fd: ::core::ffi::c_int,
        __message: *const msghdr,
        __flags: ::core::ffi::c_int,
    ) -> ssize_t;
    fn sendmmsg(
        __fd: ::core::ffi::c_int,
        __vmessages: *mut mmsghdr,
        __vlen: ::core::ffi::c_uint,
        __flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn recvmsg(
        __fd: ::core::ffi::c_int,
        __message: *mut msghdr,
        __flags: ::core::ffi::c_int,
    ) -> ssize_t;
    fn recvmmsg(
        __fd: ::core::ffi::c_int,
        __vmessages: *mut mmsghdr,
        __vlen: ::core::ffi::c_uint,
        __flags: ::core::ffi::c_int,
        __tmo: *mut timespec,
    ) -> ::core::ffi::c_int;
    fn getsockopt(
        __fd: ::core::ffi::c_int,
        __level: ::core::ffi::c_int,
        __optname: ::core::ffi::c_int,
        __optval: *mut ::core::ffi::c_void,
        __optlen: *mut socklen_t,
    ) -> ::core::ffi::c_int;
    fn setsockopt(
        __fd: ::core::ffi::c_int,
        __level: ::core::ffi::c_int,
        __optname: ::core::ffi::c_int,
        __optval: *const ::core::ffi::c_void,
        __optlen: socklen_t,
    ) -> ::core::ffi::c_int;
    fn listen(__fd: ::core::ffi::c_int, __n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn accept(
        __fd: ::core::ffi::c_int,
        __addr: __SOCKADDR_ARG,
        __addr_len: *mut socklen_t,
    ) -> ::core::ffi::c_int;
    fn accept4(
        __fd: ::core::ffi::c_int,
        __addr: __SOCKADDR_ARG,
        __addr_len: *mut socklen_t,
        __flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn shutdown(__fd: ::core::ffi::c_int, __how: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn if_nametoindex(__ifname: *const ::core::ffi::c_char) -> ::core::ffi::c_uint;
    fn g_cancellable_get_type() -> GType;
    fn g_cancellable_set_error_if_cancelled(
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_cancellable_make_pollfd(cancellable: *mut GCancellable, pollfd: *mut GPollFD) -> gboolean;
    fn g_cancellable_release_fd(cancellable: *mut GCancellable);
    fn g_cancellable_source_new(cancellable: *mut GCancellable) -> *mut GSource;
    fn g_datagram_based_get_type() -> GType;
    fn g_socket_family_get_type() -> GType;
    fn g_socket_type_get_type() -> GType;
    fn g_socket_protocol_get_type() -> GType;
    fn g_inet_address_get_type() -> GType;
    fn g_inet_address_to_string(address: *mut GInetAddress) -> *mut gchar;
    fn g_inet_address_to_bytes(address: *mut GInetAddress) -> *const guint8;
    fn g_inet_address_get_native_size(address: *mut GInetAddress) -> gsize;
    fn g_inet_address_get_family(address: *mut GInetAddress) -> GSocketFamily;
    fn g_socket_address_get_type() -> GType;
    fn g_socket_address_new_from_native(native: gpointer, len: gsize) -> *mut GSocketAddress;
    fn g_socket_address_to_native(
        address: *mut GSocketAddress,
        dest: gpointer,
        destlen: gsize,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_socket_address_get_native_size(address: *mut GSocketAddress) -> gssize;
    fn g_inet_socket_address_get_type() -> GType;
    fn g_inet_socket_address_new(address: *mut GInetAddress, port: guint16) -> *mut GSocketAddress;
    fn g_inet_socket_address_get_address(address: *mut GInetSocketAddress) -> *mut GInetAddress;
    fn g_inet_socket_address_get_port(address: *mut GInetSocketAddress) -> guint16;
    fn g_inet_socket_address_get_scope_id(address: *mut GInetSocketAddress) -> guint32;
    fn g_initable_get_type() -> GType;
    fn g_initable_new(
        object_type: GType,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
        first_property_name: *const gchar,
        ...
    ) -> gpointer;
    fn g_io_error_quark() -> GQuark;
    fn g_io_error_from_errno(err_no: gint) -> GIOErrorEnum;
    fn g_networking_init();
    fn g_socket_control_message_get_size(message: *mut GSocketControlMessage) -> gsize;
    fn g_socket_control_message_get_level(
        message: *mut GSocketControlMessage,
    ) -> ::core::ffi::c_int;
    fn g_socket_control_message_get_msg_type(
        message: *mut GSocketControlMessage,
    ) -> ::core::ffi::c_int;
    fn g_socket_control_message_serialize(message: *mut GSocketControlMessage, data: gpointer);
    fn g_socket_control_message_deserialize(
        level: ::core::ffi::c_int,
        type_0: ::core::ffi::c_int,
        size: gsize,
        data: gpointer,
    ) -> *mut GSocketControlMessage;
    fn g_credentials_new() -> *mut GCredentials;
    fn g_credentials_set_native(
        credentials: *mut GCredentials,
        native_type: GCredentialsType,
        native: gpointer,
    );
    fn glib_gettext(str: *const gchar) -> *const gchar;
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
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __time_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type __caddr_t = *mut ::core::ffi::c_char;
pub type __socklen_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guchar = ::core::ffi::c_uchar;
pub type gushort = ::core::ffi::c_ushort;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gfloat = ::core::ffi::c_float;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GBytes = _GBytes;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPtrArray {
    pub pdata: *mut gpointer,
    pub len: guint,
}
pub type GPtrArray = _GPtrArray;
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GError = _GError;
pub type ssize_t = isize;
pub type __sighandler_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
pub type socklen_t = __socklen_t;
pub type GData = _GData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPollFD {
    pub fd: gint,
    pub events: gushort,
    pub revents: gushort,
}
pub type GPollFD = _GPollFD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type GIOCondition = ::core::ffi::c_uint;
pub const G_IO_NVAL: GIOCondition = 32;
pub const G_IO_HUP: GIOCondition = 16;
pub const G_IO_ERR: GIOCondition = 8;
pub const G_IO_PRI: GIOCondition = 2;
pub const G_IO_OUT: GIOCondition = 4;
pub const G_IO_IN: GIOCondition = 1;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub type GString = _GString;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GClosure {
    #[bitfield(name = "ref_count", ty = "guint", bits = "0..=14")]
    #[bitfield(name = "meta_marshal_nouse", ty = "guint", bits = "15..=15")]
    #[bitfield(name = "n_guards", ty = "guint", bits = "16..=16")]
    #[bitfield(name = "n_fnotifiers", ty = "guint", bits = "17..=18")]
    #[bitfield(name = "n_inotifiers", ty = "guint", bits = "19..=26")]
    #[bitfield(name = "in_inotify", ty = "guint", bits = "27..=27")]
    #[bitfield(name = "floating", ty = "guint", bits = "28..=28")]
    #[bitfield(name = "derivative_flag", ty = "guint", bits = "29..=29")]
    #[bitfield(name = "in_marshal", ty = "guint", bits = "30..=30")]
    #[bitfield(name = "is_invalid", ty = "guint", bits = "31..=31")]
    pub ref_count_meta_marshal_nouse_n_guards_n_fnotifiers_n_inotifiers_in_inotify_floating_derivative_flag_in_marshal_is_invalid:
        [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
    pub marshal: Option<
        unsafe extern "C" fn(
            *mut GClosure,
            *mut GValue,
            guint,
            *const GValue,
            gpointer,
            gpointer,
        ) -> (),
    >,
    pub data: gpointer,
    pub notifiers: *mut GClosureNotifyData,
}
pub type GClosureNotifyData = _GClosureNotifyData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GClosureNotifyData {
    pub data: gpointer,
    pub notify: GClosureNotify,
}
pub type GClosureNotify = Option<unsafe extern "C" fn(gpointer, *mut GClosure) -> ()>;
pub type GClosure = _GClosure;
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
pub type GSocketProtocol = ::core::ffi::c_int;
pub const G_SOCKET_PROTOCOL_SCTP: GSocketProtocol = 132;
pub const G_SOCKET_PROTOCOL_UDP: GSocketProtocol = 17;
pub const G_SOCKET_PROTOCOL_TCP: GSocketProtocol = 6;
pub const G_SOCKET_PROTOCOL_DEFAULT: GSocketProtocol = 0;
pub const G_SOCKET_PROTOCOL_UNKNOWN: GSocketProtocol = -1;
pub type GCredentialsType = ::core::ffi::c_uint;
pub const G_CREDENTIALS_TYPE_WIN32_PID: GCredentialsType = 7;
pub const G_CREDENTIALS_TYPE_APPLE_XUCRED: GCredentialsType = 6;
pub const G_CREDENTIALS_TYPE_NETBSD_UNPCBID: GCredentialsType = 5;
pub const G_CREDENTIALS_TYPE_SOLARIS_UCRED: GCredentialsType = 4;
pub const G_CREDENTIALS_TYPE_OPENBSD_SOCKPEERCRED: GCredentialsType = 3;
pub const G_CREDENTIALS_TYPE_FREEBSD_CMSGCRED: GCredentialsType = 2;
pub const G_CREDENTIALS_TYPE_LINUX_UCRED: GCredentialsType = 1;
pub const G_CREDENTIALS_TYPE_INVALID: GCredentialsType = 0;
pub type GPollableReturn = ::core::ffi::c_int;
pub const G_POLLABLE_RETURN_WOULD_BLOCK: GPollableReturn = -27;
pub const G_POLLABLE_RETURN_OK: GPollableReturn = 1;
pub const G_POLLABLE_RETURN_FAILED: GPollableReturn = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
pub type GDatagramBased = _GDatagramBased;
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
pub type GInitable = _GInitable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocket {
    pub parent_instance: GObject,
    pub priv_0: *mut GSocketPrivate,
}
pub type GSocketPrivate = _GSocketPrivate;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GSocketPrivate {
    pub family: GSocketFamily,
    pub type_0: GSocketType,
    pub protocol: GSocketProtocol,
    pub fd: gint,
    pub listen_backlog: gint,
    pub timeout: guint,
    pub construct_error: *mut GError,
    pub remote_address: *mut GSocketAddress,
    #[bitfield(name = "inited", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "blocking", ty = "guint", bits = "1..=1")]
    #[bitfield(name = "keepalive", ty = "guint", bits = "2..=2")]
    #[bitfield(name = "closed", ty = "guint", bits = "3..=3")]
    #[bitfield(name = "connected_read", ty = "guint", bits = "4..=4")]
    #[bitfield(name = "connected_write", ty = "guint", bits = "5..=5")]
    #[bitfield(name = "listening", ty = "guint", bits = "6..=6")]
    #[bitfield(name = "timed_out", ty = "guint", bits = "7..=7")]
    #[bitfield(name = "connect_pending", ty = "guint", bits = "8..=8")]
    pub inited_blocking_keepalive_closed_connected_read_connected_write_listening_timed_out_connect_pending:
        [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 6],
    pub recv_addr_cache: [C2RustUnnamed_0; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub addr: *mut GSocketAddress,
    pub native: *mut sockaddr,
    pub native_len: gsize,
    pub last_used: guint64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [::core::ffi::c_char; 14],
}
pub type sa_family_t = ::core::ffi::c_ushort;
pub type GSocket = _GSocket;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketControlMessage {
    pub parent_instance: GObject,
    pub priv_0: *mut GSocketControlMessagePrivate,
}
pub type GSocketControlMessagePrivate = _GSocketControlMessagePrivate;
pub type GSocketControlMessage = _GSocketControlMessage;
pub type GSocketSourceFunc =
    Option<unsafe extern "C" fn(*mut GSocket, GIOCondition, gpointer) -> gboolean>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInputVector {
    pub buffer: gpointer,
    pub size: gsize,
}
pub type GInputVector = _GInputVector;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInputMessage {
    pub address: *mut *mut GSocketAddress,
    pub vectors: *mut GInputVector,
    pub num_vectors: guint,
    pub bytes_received: gsize,
    pub flags: gint,
    pub control_messages: *mut *mut *mut GSocketControlMessage,
    pub num_control_messages: *mut guint,
}
pub type GInputMessage = _GInputMessage;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GOutputVector {
    pub buffer: gconstpointer,
    pub size: gsize,
}
pub type GOutputVector = _GOutputVector;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GOutputMessage {
    pub address: *mut GSocketAddress,
    pub vectors: *mut GOutputVector,
    pub num_vectors: guint,
    pub bytes_sent: guint,
    pub control_messages: *mut *mut GSocketControlMessage,
    pub num_control_messages: guint,
}
pub type GOutputMessage = _GOutputMessage;
pub type GCredentials = _GCredentials;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketClass {
    pub parent_class: GObjectClass,
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
pub type GSocketClass = _GSocketClass;
pub const PROP_MULTICAST_TTL: C2RustUnnamed_12 = 14;
pub const PROP_MULTICAST_LOOPBACK: C2RustUnnamed_12 = 13;
pub const PROP_TTL: C2RustUnnamed_12 = 11;
pub const PROP_BROADCAST: C2RustUnnamed_12 = 12;
pub const PROP_TIMEOUT: C2RustUnnamed_12 = 10;
pub const PROP_REMOTE_ADDRESS: C2RustUnnamed_12 = 9;
pub const PROP_LOCAL_ADDRESS: C2RustUnnamed_12 = 8;
pub const PROP_KEEPALIVE: C2RustUnnamed_12 = 7;
pub const PROP_LISTEN_BACKLOG: C2RustUnnamed_12 = 6;
pub const PROP_BLOCKING: C2RustUnnamed_12 = 5;
pub const PROP_FD: C2RustUnnamed_12 = 4;
pub const PROP_PROTOCOL: C2RustUnnamed_12 = 3;
pub const PROP_TYPE: C2RustUnnamed_12 = 2;
pub const PROP_FAMILY: C2RustUnnamed_12 = 1;
pub const IPPROTO_IPV6: C2RustUnnamed_11 = 41;
pub const IPPROTO_IP: C2RustUnnamed_11 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub storage: sockaddr_storage,
    pub sa: sockaddr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [::core::ffi::c_char; 118],
    pub __ss_align: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __SOCKADDR_ARG {
    pub __sockaddr__: *mut sockaddr,
    pub __sockaddr_at__: *mut sockaddr_at,
    pub __sockaddr_ax25__: *mut sockaddr_ax25,
    pub __sockaddr_dl__: *mut sockaddr_dl,
    pub __sockaddr_eon__: *mut sockaddr_eon,
    pub __sockaddr_in__: *mut sockaddr_in,
    pub __sockaddr_in6__: *mut sockaddr_in6,
    pub __sockaddr_inarp__: *mut sockaddr_inarp,
    pub __sockaddr_ipx__: *mut sockaddr_ipx,
    pub __sockaddr_iso__: *mut sockaddr_iso,
    pub __sockaddr_ns__: *mut sockaddr_ns,
    pub __sockaddr_un__: *mut sockaddr_un,
    pub __sockaddr_x25__: *mut sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [::core::ffi::c_char; 108],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type uint16_t = __uint16_t;
pub type uint8_t = __uint8_t;
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [::core::ffi::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub storage: sockaddr_storage,
    pub sa: sockaddr,
}
pub const IPPROTO_TCP: C2RustUnnamed_11 = 6;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub storage: sockaddr_storage,
    pub sa: sockaddr,
}
pub type GDatagramBasedInterface = _GDatagramBasedInterface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDatagramBasedInterface {
    pub g_iface: GTypeInterface,
    pub receive_messages: Option<
        unsafe extern "C" fn(
            *mut GDatagramBased,
            *mut GInputMessage,
            guint,
            gint,
            gint64,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gint,
    >,
    pub send_messages: Option<
        unsafe extern "C" fn(
            *mut GDatagramBased,
            *mut GOutputMessage,
            guint,
            gint,
            gint64,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gint,
    >,
    pub create_source: Option<
        unsafe extern "C" fn(*mut GDatagramBased, GIOCondition, *mut GCancellable) -> *mut GSource,
    >,
    pub condition_check:
        Option<unsafe extern "C" fn(*mut GDatagramBased, GIOCondition) -> GIOCondition>,
    pub condition_wait: Option<
        unsafe extern "C" fn(
            *mut GDatagramBased,
            GIOCondition,
            gint64,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gboolean,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GSocketSource {
    pub source: GSource,
    pub fd_tag: gpointer,
    pub socket: *mut GSocket,
    pub condition: GIOCondition,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mmsghdr {
    pub msg_hdr: msghdr,
    pub msg_len: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msghdr {
    pub msg_name: *mut ::core::ffi::c_void,
    pub msg_namelen: socklen_t,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: size_t,
    pub msg_control: *mut ::core::ffi::c_void,
    pub msg_controllen: size_t,
    pub msg_flags: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut ::core::ffi::c_void,
    pub iov_len: size_t,
}
pub const MSG_NOSIGNAL: C2RustUnnamed_9 = 16384;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmsghdr {
    pub cmsg_len: size_t,
    pub cmsg_level: ::core::ffi::c_int,
    pub cmsg_type: ::core::ffi::c_int,
    pub __cmsg_data: [::core::ffi::c_uchar; 0],
}
pub const MSG_CMSG_CLOEXEC: C2RustUnnamed_9 = 1073741824;
pub type GInitableIface = _GInitableIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInitableIface {
    pub g_iface: GTypeInterface,
    pub init: Option<
        unsafe extern "C" fn(*mut GInitable, *mut GCancellable, *mut *mut GError) -> gboolean,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __CONST_SOCKADDR_ARG {
    pub __sockaddr__: *const sockaddr,
    pub __sockaddr_at__: *const sockaddr_at,
    pub __sockaddr_ax25__: *const sockaddr_ax25,
    pub __sockaddr_dl__: *const sockaddr_dl,
    pub __sockaddr_eon__: *const sockaddr_eon,
    pub __sockaddr_in__: *const sockaddr_in,
    pub __sockaddr_in6__: *const sockaddr_in6,
    pub __sockaddr_inarp__: *const sockaddr_inarp,
    pub __sockaddr_ipx__: *const sockaddr_ipx,
    pub __sockaddr_iso__: *const sockaddr_iso,
    pub __sockaddr_ns__: *const sockaddr_ns,
    pub __sockaddr_un__: *const sockaddr_un,
    pub __sockaddr_x25__: *const sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub storage: sockaddr_storage,
    pub sa: sockaddr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipv6_mreq {
    pub ipv6mr_multiaddr: in6_addr,
    pub ipv6mr_interface: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ip_mreqn {
    pub imr_multiaddr: in_addr,
    pub imr_address: in_addr,
    pub imr_ifindex: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group_source_req {
    pub gsr_interface: uint32_t,
    pub gsr_group: sockaddr_storage,
    pub gsr_source: sockaddr_storage,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ip_mreq_source {
    pub imr_multiaddr: in_addr,
    pub imr_interface: in_addr,
    pub imr_sourceaddr: in_addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub ifru_addr: sockaddr,
    pub ifru_dstaddr: sockaddr,
    pub ifru_broadaddr: sockaddr,
    pub ifru_netmask: sockaddr,
    pub ifru_hwaddr: sockaddr,
    pub ifru_flags: ::core::ffi::c_short,
    pub ifru_ivalue: ::core::ffi::c_int,
    pub ifru_mtu: ::core::ffi::c_int,
    pub ifru_map: ifmap,
    pub ifru_slave: [::core::ffi::c_char; 16],
    pub ifru_newname: [::core::ffi::c_char; 16],
    pub ifru_data: __caddr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifmap {
    pub mem_start: ::core::ffi::c_ulong,
    pub mem_end: ::core::ffi::c_ulong,
    pub base_addr: ::core::ffi::c_ushort,
    pub irq: ::core::ffi::c_uchar,
    pub dma: ::core::ffi::c_uchar,
    pub port: ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifreq {
    pub ifr_ifrn: C2RustUnnamed_7,
    pub ifr_ifru: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub ifrn_name: [::core::ffi::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub storage: sockaddr_storage,
    pub sa: sockaddr,
}
pub const MSG_PEEK: C2RustUnnamed_9 = 2;
pub const SHUT_WR: C2RustUnnamed_10 = 1;
pub const SHUT_RD: C2RustUnnamed_10 = 0;
pub const SHUT_RDWR: C2RustUnnamed_10 = 2;
pub type __socket_type = ::core::ffi::c_uint;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub type C2RustUnnamed_9 = ::core::ffi::c_uint;
pub const MSG_FASTOPEN: C2RustUnnamed_9 = 536870912;
pub const MSG_ZEROCOPY: C2RustUnnamed_9 = 67108864;
pub const MSG_BATCH: C2RustUnnamed_9 = 262144;
pub const MSG_WAITFORONE: C2RustUnnamed_9 = 65536;
pub const MSG_MORE: C2RustUnnamed_9 = 32768;
pub const MSG_ERRQUEUE: C2RustUnnamed_9 = 8192;
pub const MSG_RST: C2RustUnnamed_9 = 4096;
pub const MSG_CONFIRM: C2RustUnnamed_9 = 2048;
pub const MSG_SYN: C2RustUnnamed_9 = 1024;
pub const MSG_FIN: C2RustUnnamed_9 = 512;
pub const MSG_WAITALL: C2RustUnnamed_9 = 256;
pub const MSG_EOR: C2RustUnnamed_9 = 128;
pub const MSG_DONTWAIT: C2RustUnnamed_9 = 64;
pub const MSG_TRUNC: C2RustUnnamed_9 = 32;
pub const MSG_PROXY: C2RustUnnamed_9 = 16;
pub const MSG_CTRUNC: C2RustUnnamed_9 = 8;
pub const MSG_TRYHARD: C2RustUnnamed_9 = 4;
pub const MSG_DONTROUTE: C2RustUnnamed_9 = 4;
pub const MSG_OOB: C2RustUnnamed_9 = 1;
pub type C2RustUnnamed_10 = ::core::ffi::c_uint;
pub type C2RustUnnamed_11 = ::core::ffi::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_11 = 263;
pub const IPPROTO_MPTCP: C2RustUnnamed_11 = 262;
pub const IPPROTO_RAW: C2RustUnnamed_11 = 255;
pub const IPPROTO_ETHERNET: C2RustUnnamed_11 = 143;
pub const IPPROTO_MPLS: C2RustUnnamed_11 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_11 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_11 = 132;
pub const IPPROTO_L2TP: C2RustUnnamed_11 = 115;
pub const IPPROTO_COMP: C2RustUnnamed_11 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_11 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_11 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_11 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_11 = 92;
pub const IPPROTO_AH: C2RustUnnamed_11 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_11 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_11 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_11 = 46;
pub const IPPROTO_DCCP: C2RustUnnamed_11 = 33;
pub const IPPROTO_TP: C2RustUnnamed_11 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_11 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_11 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_11 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_11 = 8;
pub const IPPROTO_IPIP: C2RustUnnamed_11 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_11 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_11 = 1;
pub type C2RustUnnamed_12 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_12 = 0;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const LONG_MAX: ::core::ffi::c_long = __LONG_MAX__;
pub const INT_MIN: ::core::ffi::c_int = -__INT_MAX__ - 1 as ::core::ffi::c_int;
pub const UINT_MAX: ::core::ffi::c_uint = (__INT_MAX__ as ::core::ffi::c_uint)
    .wrapping_mul(2 as ::core::ffi::c_uint)
    .wrapping_add(1 as ::core::ffi::c_uint);
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const IOV_MAX: ::core::ffi::c_int = __IOV_MAX;
pub const __IOV_MAX: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const G_MININT: ::core::ffi::c_int = INT_MIN;
pub const G_MAXINT: ::core::ffi::c_int = INT_MAX;
pub const G_MAXUINT: ::core::ffi::c_uint = UINT_MAX;
pub const G_MAXLONG: ::core::ffi::c_long = LONG_MAX;
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const G_MAXSSIZE: ::core::ffi::c_long = G_MAXLONG;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_MAXUINT64: ::core::ffi::c_ulong = 0xffffffffffffffff as ::core::ffi::c_ulong;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const SIGPIPE: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const ENOSYS: ::core::ffi::c_int = 38 as ::core::ffi::c_int;
pub const EWOULDBLOCK: ::core::ffi::c_int = EAGAIN;
pub const EPROTOTYPE: ::core::ffi::c_int = 91 as ::core::ffi::c_int;
pub const EINPROGRESS: ::core::ffi::c_int = 115 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn safe_c2rust_g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    ref_0 = *ptr;
    *ptr = NULL as gpointer;
    return ref_0;
}
#[inline(always)]
unsafe extern "C" fn safe_c2rust_g_string_append_c_inline(
    mut gstring: *mut GString,
    mut c: gchar,
) -> *mut GString {
    if ({
        let mut _g_boolean_var_3: ::core::ffi::c_int = 0;
        if !gstring.is_null() && (*gstring).len.wrapping_add(1 as gsize) < (*gstring).allocated_len
        {
            _g_boolean_var_3 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_3 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_3
    }) as ::core::ffi::c_long
        != 0
    {
        let fresh0 = (*gstring).len;
        (*gstring).len = (*gstring).len.wrapping_add(1);
        *(*gstring).str_0.offset(fresh0 as isize) = c;
        *(*gstring).str_0.offset((*gstring).len as isize) = 0 as gchar;
    } else {
        g_string_insert_c(gstring, -(1 as ::core::ffi::c_int) as gssize, c);
    }
    return gstring;
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
pub const FIONREAD: ::core::ffi::c_int = 0x541b as ::core::ffi::c_int;
pub const SIOCGIFADDR: ::core::ffi::c_int = 0x8915 as ::core::ffi::c_int;
pub const G_USEC_PER_SEC: ::core::ffi::c_int = 1000000 as ::core::ffi::c_int;
pub const SOL_SOCKET: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SO_REUSEADDR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SO_TYPE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SO_ERROR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SO_BROADCAST: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const SO_KEEPALIVE: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SO_REUSEPORT: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const SO_PEERCRED: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const SO_DOMAIN: ::core::ffi::c_int = 39 as ::core::ffi::c_int;
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_BOOLEAN: GType = ((5 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const F_GETFD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const F_SETFD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FD_CLOEXEC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SOMAXCONN: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn safe_c2rust___cmsg_nxthdr(
    mut __mhdr: *mut msghdr,
    mut __cmsg: *mut cmsghdr,
) -> *mut cmsghdr {
    let mut __msg_control_ptr: *mut ::core::ffi::c_uchar =
        (*__mhdr).msg_control as *mut ::core::ffi::c_uchar;
    let mut __cmsg_ptr: *mut ::core::ffi::c_uchar = __cmsg as *mut ::core::ffi::c_uchar;
    let mut __size_needed: size_t = (::core::mem::size_of::<cmsghdr>() as size_t).wrapping_add(
        (::core::mem::size_of::<size_t>() as size_t).wrapping_sub(
            (*__cmsg).cmsg_len
                & (::core::mem::size_of::<size_t>() as size_t).wrapping_sub(1 as size_t),
        ) & (::core::mem::size_of::<size_t>() as size_t).wrapping_sub(1 as size_t),
    );
    if (*__cmsg).cmsg_len < ::core::mem::size_of::<cmsghdr>() as usize {
        return ::core::ptr::null_mut::<cmsghdr>();
    }
    if (__msg_control_ptr
        .offset((*__mhdr).msg_controllen as isize)
        .offset_from(__cmsg_ptr) as ::core::ffi::c_long as size_t)
        < __size_needed
        || (__msg_control_ptr
            .offset((*__mhdr).msg_controllen as isize)
            .offset_from(__cmsg_ptr) as ::core::ffi::c_long as size_t)
            .wrapping_sub(__size_needed)
            < (*__cmsg).cmsg_len
    {
        return ::core::ptr::null_mut::<cmsghdr>();
    }
    __cmsg = (__cmsg as *mut ::core::ffi::c_uchar).offset(
        ((*__cmsg)
            .cmsg_len
            .wrapping_add(::core::mem::size_of::<size_t>() as size_t)
            .wrapping_sub(1 as size_t)
            & !(::core::mem::size_of::<size_t>() as usize).wrapping_sub(1 as usize))
            as isize,
    ) as *mut cmsghdr;
    return __cmsg;
}
pub const IP_TTL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MCAST_JOIN_SOURCE_GROUP: ::core::ffi::c_int = 46 as ::core::ffi::c_int;
pub const MCAST_LEAVE_SOURCE_GROUP: ::core::ffi::c_int = 47 as ::core::ffi::c_int;
pub const IP_MULTICAST_TTL: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
pub const IP_MULTICAST_LOOP: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const IP_ADD_MEMBERSHIP: ::core::ffi::c_int = 35 as ::core::ffi::c_int;
pub const IP_DROP_MEMBERSHIP: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const IP_ADD_SOURCE_MEMBERSHIP: ::core::ffi::c_int = 39 as ::core::ffi::c_int;
pub const IP_DROP_SOURCE_MEMBERSHIP: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const IPV6_UNICAST_HOPS: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const IPV6_MULTICAST_HOPS: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const IPV6_MULTICAST_LOOP: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const IPV6_JOIN_GROUP: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const IPV6_LEAVE_GROUP: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const IPV6_V6ONLY: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
pub const TCP_NODELAY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const G_IOV_MAX: ::core::ffi::c_int = IOV_MAX;
pub const RECV_ADDR_CACHE_SIZE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
static mut safe_c2rust_GSocket_private_offset: gint = 0;
static mut safe_c2rust_g_socket_parent_class: gpointer =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_socket_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GSocket\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GSocketClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_socket_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GSocket>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GSocket) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_socket_init as unsafe extern "C" fn(*mut GSocket) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GSocket_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GSocketPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GInitableIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_socket_initable_iface_init
                as unsafe extern "C" fn(*mut GInitableIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_initable_get_type(),
        &raw const g_implement_interface_info,
    );
    let g_implement_interface_info_0: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GDatagramBasedInterface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_socket_datagram_based_iface_init
                as unsafe extern "C" fn(*mut GDatagramBasedInterface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_datagram_based_get_type(),
        &raw const g_implement_interface_info_0,
    );
    return g_define_type_id;
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_socket_get_instance_private(
    mut self_0: *mut GSocket,
) -> gpointer {
    return (self_0 as *mut guint8).offset(safe_c2rust_GSocket_private_offset as glong as isize)
        as gpointer;
}
unsafe extern "C" fn safe_c2rust_g_socket_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_socket_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GSocket_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut safe_c2rust_GSocket_private_offset);
    }
    safe_c2rust_g_socket_class_init(klass as *mut GSocketClass);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_get_type() -> GType {
    static mut safe_c2rust_static_g_define_type_id: GType = 0 as GType;
    g_type_ensure(g_socket_family_get_type());
    g_type_ensure(g_socket_type_get_type());
    g_type_ensure(g_socket_protocol_get_type());
    g_type_ensure(g_socket_address_get_type());
    g_networking_init();
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
        let mut g_define_type_id: GType = safe_c2rust_g_socket_get_type_once();
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
unsafe extern "C" fn safe_c2rust_get_socket_errno() -> ::core::ffi::c_int {
    return *__errno_location();
}
unsafe extern "C" fn safe_c2rust_socket_io_error_from_errno(
    mut err: ::core::ffi::c_int,
) -> GIOErrorEnum {
    return g_io_error_from_errno(err as gint);
}
unsafe extern "C" fn safe_c2rust_socket_strerror(
    mut err: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    return g_strerror(err as gint) as *const ::core::ffi::c_char;
}
unsafe extern "C" fn safe_c2rust_address_to_string(mut address: *mut GSocketAddress) -> *mut gchar {
    let mut ret: *mut GString = g_string_new(b"\0" as *const u8 as *const gchar);
    if ({
        let mut __inst: *mut GTypeInstance = address as *mut GTypeInstance;
        let mut __t: GType = g_inet_socket_address_get_type();
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
        let mut isa: *mut GInetSocketAddress =
            address as *mut ::core::ffi::c_void as *mut GInetSocketAddress;
        let mut ia: *mut GInetAddress = g_inet_socket_address_get_address(isa);
        let mut family: GSocketFamily = g_inet_address_get_family(ia);
        let mut tmp: *mut gchar = ::core::ptr::null_mut::<gchar>();
        if family as ::core::ffi::c_uint
            == G_SOCKET_FAMILY_IPV6 as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            safe_c2rust_g_string_append_c_inline(ret, '[' as i32 as gchar);
        }
        tmp = g_inet_address_to_string(ia);
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = tmp;
                safe_c2rust_g_string_append_len_inline(
                    ret,
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
                        strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                            as gssize
                    } else {
                        -(1 as ::core::ffi::c_int) as gssize
                    },
                );
            });
        } else {
            safe_c2rust_g_string_append_len_inline(ret, tmp, -(1 as ::core::ffi::c_int) as gssize);
        };
        g_free(tmp as gpointer);
        if family as ::core::ffi::c_uint
            == G_SOCKET_FAMILY_IPV6 as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut scope: guint32 = g_inet_socket_address_get_scope_id(isa);
            if scope != 0 as guint32 {
                g_string_append_printf(ret, b"%%%u\0" as *const u8 as *const gchar, scope);
            }
            safe_c2rust_g_string_append_c_inline(ret, ']' as i32 as gchar);
        }
        safe_c2rust_g_string_append_c_inline(ret, ':' as i32 as gchar);
        g_string_append_printf(
            ret,
            b"%u\0" as *const u8 as *const gchar,
            g_inet_socket_address_get_port(isa) as ::core::ffi::c_int,
        );
    } else {
        g_string_append_printf(
            ret,
            b"(%s)\0" as *const u8 as *const gchar,
            g_type_name((*(*(address as *mut GTypeInstance)).g_class).g_type),
        );
    }
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(ret, 0 as gboolean)
        } else {
            g_string_free_and_steal(ret)
        }
    } else {
        g_string_free(ret, 0 as gboolean)
    };
}
unsafe extern "C" fn safe_c2rust_check_socket(
    mut socket_0: *mut GSocket,
    mut error: *mut *mut GError,
) -> gboolean {
    if (*(*socket_0).priv_0).inited() == 0 {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_INITIALIZED as ::core::ffi::c_int as gint,
            glib_gettext(b"Invalid socket, not initialized\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    if !(*(*socket_0).priv_0).construct_error.is_null() {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_INITIALIZED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Invalid socket, initialization failed due to: %s\0" as *const u8 as *const gchar,
            ),
            (*(*(*socket_0).priv_0).construct_error).message,
        );
        return FALSE;
    }
    if (*(*socket_0).priv_0).closed() != 0 {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_CLOSED as ::core::ffi::c_int as gint,
            glib_gettext(b"Socket is already closed\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_check_timeout(
    mut socket_0: *mut GSocket,
    mut error: *mut *mut GError,
) -> gboolean {
    if (*(*socket_0).priv_0).timed_out() != 0 {
        (*(*socket_0).priv_0).set_timed_out(FALSE as guint as guint);
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_TIMED_OUT as ::core::ffi::c_int as gint,
            glib_gettext(b"Socket I/O timed out\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_socket_details_from_fd(mut socket_0: *mut GSocket) {
    let mut current_block: u64;
    let mut address: C2RustUnnamed_4 = C2RustUnnamed_4 {
        storage: sockaddr_storage {
            ss_family: 0,
            __ss_padding: [0; 118],
            __ss_align: 0,
        },
    };
    let mut fd: gint = 0;
    let mut addrlen: socklen_t = 0;
    let mut value: ::core::ffi::c_int = 0;
    let mut family: ::core::ffi::c_int = 0;
    let mut errsv: ::core::ffi::c_int = 0;
    memset(
        &raw mut address as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<C2RustUnnamed_4>() as size_t,
    );
    fd = (*(*socket_0).priv_0).fd;
    if safe_c2rust_g_socket_get_option(
        socket_0,
        SOL_SOCKET,
        SO_TYPE,
        &raw mut value,
        ::core::ptr::null_mut::<*mut GError>(),
    ) == 0
    {
        errsv = safe_c2rust_get_socket_errno();
    } else {
        match value {
            1 => {
                (*(*socket_0).priv_0).type_0 = G_SOCKET_TYPE_STREAM;
            }
            2 => {
                (*(*socket_0).priv_0).type_0 = G_SOCKET_TYPE_DATAGRAM;
            }
            5 => {
                (*(*socket_0).priv_0).type_0 = G_SOCKET_TYPE_SEQPACKET;
            }
            _ => {
                (*(*socket_0).priv_0).type_0 = G_SOCKET_TYPE_INVALID;
            }
        }
        addrlen = ::core::mem::size_of::<C2RustUnnamed_4>() as socklen_t;
        if getsockname(
            fd as ::core::ffi::c_int,
            __SOCKADDR_ARG {
                __sockaddr__: &raw mut address.sa,
            },
            &raw mut addrlen,
        ) != 0 as ::core::ffi::c_int
        {
            errsv = safe_c2rust_get_socket_errno();
        } else {
            if addrlen > 0 as socklen_t {
                if ({
                    let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
                    if 0 as ::core::ffi::c_ulong as glong
                        + ::core::mem::size_of::<sa_family_t>() as socklen_t as glong
                        <= addrlen as glong
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
                    g_assertion_message_expr(
                        G_LOG_DOMAIN.as_ptr(),
                        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocket.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        522 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"G_STRUCT_OFFSET (struct sockaddr, sa_family) + (socklen_t) sizeof address.storage.ss_family <= addrlen\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                family = address.storage.ss_family as ::core::ffi::c_int;
                current_block = 11298138898191919651;
            } else if safe_c2rust_g_socket_get_option(
                socket_0,
                SOL_SOCKET,
                SO_DOMAIN,
                &raw mut family,
                ::core::ptr::null_mut::<*mut GError>(),
            ) == 0
            {
                errsv = safe_c2rust_get_socket_errno();
                current_block = 2088001745508175907;
            } else {
                current_block = 11298138898191919651;
            }
            match current_block {
                2088001745508175907 => {}
                _ => {
                    match family {
                        2 | 10 => {
                            (*(*socket_0).priv_0).family =
                                address.storage.ss_family as GSocketFamily;
                            match (*(*socket_0).priv_0).type_0 as ::core::ffi::c_uint {
                                1 => {
                                    (*(*socket_0).priv_0).protocol = G_SOCKET_PROTOCOL_TCP;
                                }
                                2 => {
                                    (*(*socket_0).priv_0).protocol = G_SOCKET_PROTOCOL_UDP;
                                }
                                3 => {
                                    (*(*socket_0).priv_0).protocol = G_SOCKET_PROTOCOL_SCTP;
                                }
                                _ => {}
                            }
                        }
                        1 => {
                            (*(*socket_0).priv_0).family = G_SOCKET_FAMILY_UNIX;
                            (*(*socket_0).priv_0).protocol = G_SOCKET_PROTOCOL_DEFAULT;
                        }
                        _ => {
                            (*(*socket_0).priv_0).family = G_SOCKET_FAMILY_INVALID;
                        }
                    }
                    if (*(*socket_0).priv_0).family as ::core::ffi::c_uint
                        != G_SOCKET_FAMILY_INVALID as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        addrlen = ::core::mem::size_of::<C2RustUnnamed_4>() as socklen_t;
                        if getpeername(
                            fd as ::core::ffi::c_int,
                            __SOCKADDR_ARG {
                                __sockaddr__: &raw mut address.sa,
                            },
                            &raw mut addrlen,
                        ) >= 0 as ::core::ffi::c_int
                        {
                            (*(*socket_0).priv_0).set_connected_read(TRUE as guint as guint);
                            (*(*socket_0).priv_0).set_connected_write(TRUE as guint as guint);
                        }
                    }
                    if safe_c2rust_g_socket_get_option(
                        socket_0,
                        SOL_SOCKET,
                        SO_KEEPALIVE,
                        &raw mut value,
                        ::core::ptr::null_mut::<*mut GError>(),
                    ) != 0
                    {
                        (*(*socket_0).priv_0)
                            .set_keepalive((value != 0) as ::core::ffi::c_int as guint as guint);
                    } else {
                        (*(*socket_0).priv_0).set_keepalive(FALSE as guint as guint);
                    }
                    return;
                }
            }
        }
    }
    g_set_error(
        &raw mut (*(*socket_0).priv_0).construct_error,
        g_io_error_quark(),
        safe_c2rust_socket_io_error_from_errno(errsv) as gint,
        glib_gettext(b"creating GSocket from fd: %s\0" as *const u8 as *const gchar),
        safe_c2rust_socket_strerror(errsv),
    );
}
unsafe extern "C" fn safe_c2rust_socket_set_nonblock(mut fd: ::core::ffi::c_int) {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if g_unix_set_fd_nonblocking(fd as gint, TRUE, &raw mut error) == 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Error setting socket to nonblocking mode: %s\0" as *const u8 as *const gchar,
            (*error).message,
        );
        g_clear_error(&raw mut error);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket(
    mut domain: gint,
    mut type_0: gint,
    mut protocol: gint,
    mut error: *mut *mut GError,
) -> gint {
    let mut fd: ::core::ffi::c_int = 0;
    let mut errsv: ::core::ffi::c_int = 0;
    fd = socket(
        domain as ::core::ffi::c_int,
        type_0 as ::core::ffi::c_int
            | SOCK_CLOEXEC as ::core::ffi::c_int
            | SOCK_NONBLOCK as ::core::ffi::c_int,
        protocol as ::core::ffi::c_int,
    );
    errsv = *__errno_location();
    if fd != -(1 as ::core::ffi::c_int) {
        return fd as gint;
    }
    if fd < 0 as ::core::ffi::c_int && (errsv == EINVAL || errsv == EPROTOTYPE) {
        fd = socket(
            domain as ::core::ffi::c_int,
            type_0 as ::core::ffi::c_int,
            protocol as ::core::ffi::c_int,
        );
    }
    if fd < 0 as ::core::ffi::c_int {
        errsv = safe_c2rust_get_socket_errno();
        g_set_error(
            error,
            g_io_error_quark(),
            safe_c2rust_socket_io_error_from_errno(errsv) as gint,
            glib_gettext(b"Unable to create socket: %s\0" as *const u8 as *const gchar),
            safe_c2rust_socket_strerror(errsv),
        );
        *__errno_location() = errsv;
        return -(1 as gint);
    }
    let mut flags: ::core::ffi::c_int = 0;
    flags = fcntl(fd, F_GETFD, 0 as ::core::ffi::c_int);
    if flags != -(1 as ::core::ffi::c_int) && flags & FD_CLOEXEC == 0 as ::core::ffi::c_int {
        flags |= FD_CLOEXEC;
        fcntl(fd, F_SETFD, flags);
    }
    safe_c2rust_socket_set_nonblock(fd);
    return fd as gint;
}
unsafe extern "C" fn safe_c2rust_g_socket_create_socket(
    mut family: GSocketFamily,
    mut type_0: GSocketType,
    mut protocol: ::core::ffi::c_int,
    mut error: *mut *mut GError,
) -> gint {
    let mut native_type: gint = 0;
    match type_0 as ::core::ffi::c_uint {
        1 => {
            native_type = SOCK_STREAM as ::core::ffi::c_int as gint;
        }
        2 => {
            native_type = SOCK_DGRAM as ::core::ffi::c_int as gint;
        }
        3 => {
            native_type = SOCK_SEQPACKET as ::core::ffi::c_int as gint;
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocket.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                724 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    }
    if family as ::core::ffi::c_uint <= 0 as ::core::ffi::c_uint {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(b"Unable to create socket: %s\0" as *const u8 as *const gchar),
            glib_gettext(b"Unknown family was specified\0" as *const u8 as *const gchar),
        );
        return -(1 as gint);
    }
    if protocol == -(1 as ::core::ffi::c_int) {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(b"Unable to create socket: %s\0" as *const u8 as *const gchar),
            glib_gettext(b"Unknown protocol was specified\0" as *const u8 as *const gchar),
        );
        return -(1 as gint);
    }
    return safe_c2rust_g_socket(family as gint, native_type, protocol as gint, error);
}
unsafe extern "C" fn safe_c2rust_g_socket_constructed(mut object: *mut GObject) {
    let mut socket_0: *mut GSocket = object as *mut ::core::ffi::c_void as *mut GSocket;
    if (*(*socket_0).priv_0).fd >= 0 as ::core::ffi::c_int {
        safe_c2rust_g_socket_details_from_fd(socket_0);
        safe_c2rust_socket_set_nonblock((*(*socket_0).priv_0).fd as ::core::ffi::c_int);
    } else {
        (*(*socket_0).priv_0).fd = safe_c2rust_g_socket_create_socket(
            (*(*socket_0).priv_0).family,
            (*(*socket_0).priv_0).type_0,
            (*(*socket_0).priv_0).protocol as ::core::ffi::c_int,
            &raw mut (*(*socket_0).priv_0).construct_error,
        );
    }
    if (*(*socket_0).priv_0).fd != -(1 as ::core::ffi::c_int) {
        if (*(*socket_0).priv_0).type_0 as ::core::ffi::c_uint
            == G_SOCKET_TYPE_STREAM as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            safe_c2rust_g_socket_set_option(
                socket_0,
                IPPROTO_TCP as ::core::ffi::c_int as gint,
                TCP_NODELAY,
                TRUE,
                ::core::ptr::null_mut::<*mut GError>(),
            );
        }
    }
}
unsafe extern "C" fn safe_c2rust_g_socket_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut socket_0: *mut GSocket = object as *mut ::core::ffi::c_void as *mut GSocket;
    let mut address: *mut GSocketAddress = ::core::ptr::null_mut::<GSocketAddress>();
    match prop_id {
        1 => {
            g_value_set_enum(value, (*(*socket_0).priv_0).family as gint);
        }
        2 => {
            g_value_set_enum(value, (*(*socket_0).priv_0).type_0 as gint);
        }
        3 => {
            g_value_set_enum(value, (*(*socket_0).priv_0).protocol as gint);
        }
        4 => {
            g_value_set_int(value, (*(*socket_0).priv_0).fd);
        }
        5 => {
            g_value_set_boolean(value, (*(*socket_0).priv_0).blocking() as gboolean);
        }
        6 => {
            g_value_set_int(value, (*(*socket_0).priv_0).listen_backlog);
        }
        7 => {
            g_value_set_boolean(value, (*(*socket_0).priv_0).keepalive() as gboolean);
        }
        8 => {
            address = safe_c2rust_g_socket_get_local_address(
                socket_0,
                ::core::ptr::null_mut::<*mut GError>(),
            );
            g_value_take_object(value, address as gpointer);
        }
        9 => {
            address = safe_c2rust_g_socket_get_remote_address(
                socket_0,
                ::core::ptr::null_mut::<*mut GError>(),
            );
            g_value_take_object(value, address as gpointer);
        }
        10 => {
            g_value_set_uint(value, (*(*socket_0).priv_0).timeout);
        }
        11 => {
            g_value_set_uint(value, safe_c2rust_g_socket_get_ttl(socket_0));
        }
        12 => {
            g_value_set_boolean(value, safe_c2rust_g_socket_get_broadcast(socket_0));
        }
        13 => {
            g_value_set_boolean(value, safe_c2rust_g_socket_get_multicast_loopback(socket_0));
        }
        14 => {
            g_value_set_uint(value, safe_c2rust_g_socket_get_multicast_ttl(socket_0));
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocket.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                845 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_socket_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut socket_0: *mut GSocket = object as *mut ::core::ffi::c_void as *mut GSocket;
    match prop_id {
        1 => {
            (*(*socket_0).priv_0).family = g_value_get_enum(value) as GSocketFamily;
        }
        2 => {
            (*(*socket_0).priv_0).type_0 = g_value_get_enum(value) as GSocketType;
        }
        3 => {
            (*(*socket_0).priv_0).protocol = g_value_get_enum(value) as GSocketProtocol;
        }
        4 => {
            (*(*socket_0).priv_0).fd = g_value_get_int(value);
        }
        5 => {
            safe_c2rust_g_socket_set_blocking(socket_0, g_value_get_boolean(value));
        }
        6 => {
            safe_c2rust_g_socket_set_listen_backlog(socket_0, g_value_get_int(value));
        }
        7 => {
            safe_c2rust_g_socket_set_keepalive(socket_0, g_value_get_boolean(value));
        }
        10 => {
            safe_c2rust_g_socket_set_timeout(socket_0, g_value_get_uint(value));
        }
        11 => {
            safe_c2rust_g_socket_set_ttl(socket_0, g_value_get_uint(value));
        }
        12 => {
            safe_c2rust_g_socket_set_broadcast(socket_0, g_value_get_boolean(value));
        }
        13 => {
            safe_c2rust_g_socket_set_multicast_loopback(socket_0, g_value_get_boolean(value));
        }
        14 => {
            safe_c2rust_g_socket_set_multicast_ttl(socket_0, g_value_get_uint(value));
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocket.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                908 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_socket_finalize(mut object: *mut GObject) {
    let mut socket_0: *mut GSocket = object as *mut ::core::ffi::c_void as *mut GSocket;
    let mut i: gint = 0;
    g_clear_error(&raw mut (*(*socket_0).priv_0).construct_error);
    if (*(*socket_0).priv_0).fd != -(1 as ::core::ffi::c_int) && (*(*socket_0).priv_0).closed() == 0
    {
        safe_c2rust_g_socket_close(socket_0, ::core::ptr::null_mut::<*mut GError>());
    }
    if !(*(*socket_0).priv_0).remote_address.is_null() {
        g_object_unref((*(*socket_0).priv_0).remote_address as gpointer);
    }
    i = 0 as ::core::ffi::c_int as gint;
    while i < RECV_ADDR_CACHE_SIZE {
        if !(*(*socket_0).priv_0).recv_addr_cache[i as usize]
            .addr
            .is_null()
        {
            g_object_unref((*(*socket_0).priv_0).recv_addr_cache[i as usize].addr as gpointer);
            g_free((*(*socket_0).priv_0).recv_addr_cache[i as usize].native as gpointer);
        }
        i += 1;
    }
    if (*(safe_c2rust_g_socket_parent_class as *mut GObjectClass))
        .finalize
        .is_some()
    {
        Some(
            (*(safe_c2rust_g_socket_parent_class as *mut GObjectClass))
                .finalize
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(object);
    }
}
unsafe extern "C" fn safe_c2rust_g_socket_class_init(mut klass: *mut GSocketClass) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    signal(
        SIGPIPE,
        ::core::mem::transmute::<::libc::intptr_t, __sighandler_t>(
            1 as ::core::ffi::c_int as ::libc::intptr_t,
        ),
    );
    (*gobject_class).finalize =
        Some(safe_c2rust_g_socket_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).constructed =
        Some(safe_c2rust_g_socket_constructed as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_socket_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_socket_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    g_object_class_install_property(
        gobject_class,
        PROP_FAMILY as ::core::ffi::c_int as guint,
        g_param_spec_enum(
            b"family\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_socket_family_get_type(),
            G_SOCKET_FAMILY_INVALID as ::core::ffi::c_int as gint,
            (G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_TYPE as ::core::ffi::c_int as guint,
        g_param_spec_enum(
            b"type\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_socket_type_get_type(),
            G_SOCKET_TYPE_STREAM as ::core::ffi::c_int as gint,
            (G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_PROTOCOL as ::core::ffi::c_int as guint,
        g_param_spec_enum(
            b"protocol\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_socket_protocol_get_type(),
            G_SOCKET_PROTOCOL_UNKNOWN as ::core::ffi::c_int as gint,
            (G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_FD as ::core::ffi::c_int as guint,
        g_param_spec_int(
            b"fd\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            G_MININT,
            G_MAXINT,
            -(1 as gint),
            (G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_BLOCKING as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"blocking\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            TRUE,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_LISTEN_BACKLOG as ::core::ffi::c_int as guint,
        g_param_spec_int(
            b"listen-backlog\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            0 as gint,
            SOMAXCONN,
            10 as gint,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_KEEPALIVE as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"keepalive\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_LOCAL_ADDRESS as ::core::ffi::c_int as guint,
        g_param_spec_object(
            b"local-address\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_socket_address_get_type(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_REMOTE_ADDRESS as ::core::ffi::c_int as guint,
        g_param_spec_object(
            b"remote-address\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_socket_address_get_type(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_TIMEOUT as ::core::ffi::c_int as guint,
        g_param_spec_uint(
            b"timeout\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            0 as guint,
            G_MAXUINT,
            0 as guint,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_BROADCAST as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"broadcast\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_TTL as ::core::ffi::c_int as guint,
        g_param_spec_uint(
            b"ttl\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            0 as guint,
            G_MAXUINT,
            0 as guint,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_MULTICAST_LOOPBACK as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"multicast-loopback\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            TRUE,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_MULTICAST_TTL as ::core::ffi::c_int as guint,
        g_param_spec_uint(
            b"multicast-ttl\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            0 as guint,
            G_MAXUINT,
            1 as guint,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
unsafe extern "C" fn safe_c2rust_g_socket_initable_iface_init(mut iface: *mut GInitableIface) {
    (*iface).init = Some(
        safe_c2rust_g_socket_initable_init
            as unsafe extern "C" fn(
                *mut GInitable,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut GInitable, *mut GCancellable, *mut *mut GError) -> gboolean,
        >;
}
unsafe extern "C" fn safe_c2rust_g_socket_datagram_based_iface_init(
    mut iface: *mut GDatagramBasedInterface,
) {
    (*iface).receive_messages = Some(
        safe_c2rust_g_socket_datagram_based_receive_messages
            as unsafe extern "C" fn(
                *mut GDatagramBased,
                *mut GInputMessage,
                guint,
                gint,
                gint64,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gint,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GDatagramBased,
                *mut GInputMessage,
                guint,
                gint,
                gint64,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gint,
        >;
    (*iface).send_messages = Some(
        safe_c2rust_g_socket_datagram_based_send_messages
            as unsafe extern "C" fn(
                *mut GDatagramBased,
                *mut GOutputMessage,
                guint,
                gint,
                gint64,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gint,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GDatagramBased,
                *mut GOutputMessage,
                guint,
                gint,
                gint64,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gint,
        >;
    (*iface).create_source = Some(
        safe_c2rust_g_socket_datagram_based_create_source
            as unsafe extern "C" fn(
                *mut GDatagramBased,
                GIOCondition,
                *mut GCancellable,
            ) -> *mut GSource,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GDatagramBased,
                GIOCondition,
                *mut GCancellable,
            ) -> *mut GSource,
        >;
    (*iface).condition_check = Some(
        safe_c2rust_g_socket_datagram_based_condition_check
            as unsafe extern "C" fn(*mut GDatagramBased, GIOCondition) -> GIOCondition,
    )
        as Option<unsafe extern "C" fn(*mut GDatagramBased, GIOCondition) -> GIOCondition>;
    (*iface).condition_wait = Some(
        safe_c2rust_g_socket_datagram_based_condition_wait
            as unsafe extern "C" fn(
                *mut GDatagramBased,
                GIOCondition,
                gint64,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GDatagramBased,
                GIOCondition,
                gint64,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
        >;
}
unsafe extern "C" fn safe_c2rust_g_socket_init(mut socket_0: *mut GSocket) {
    (*socket_0).priv_0 = safe_c2rust_g_socket_get_instance_private(socket_0) as *mut GSocketPrivate;
    (*(*socket_0).priv_0).fd = -(1 as ::core::ffi::c_int) as gint;
    (*(*socket_0).priv_0).set_blocking(TRUE as guint as guint);
    (*(*socket_0).priv_0).listen_backlog = 10 as ::core::ffi::c_int as gint;
    (*(*socket_0).priv_0).construct_error = ::core::ptr::null_mut::<GError>();
}
unsafe extern "C" fn safe_c2rust_g_socket_initable_init(
    mut initable: *mut GInitable,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut socket_0: *mut GSocket = ::core::ptr::null_mut::<GSocket>();
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = initable as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (initable)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    socket_0 = initable as *mut ::core::ffi::c_void as *mut GSocket;
    if !cancellable.is_null() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Cancellable initialization not supported\0" as *const u8 as *const gchar,
            ),
        );
        return FALSE;
    }
    (*(*socket_0).priv_0).set_inited(TRUE as guint as guint);
    if !(*(*socket_0).priv_0).construct_error.is_null() {
        if !error.is_null() {
            *error = g_error_copy((*(*socket_0).priv_0).construct_error);
        }
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_check_datagram_based(
    mut self_0: *mut GDatagramBased,
    mut error: *mut *mut GError,
) -> gboolean {
    match safe_c2rust_g_socket_get_socket_type(self_0 as *mut ::core::ffi::c_void as *mut GSocket)
        as ::core::ffi::c_uint
    {
        0 | 1 => {
            g_set_error(
                error,
                g_io_error_quark(),
                G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Cannot use datagram operations on a non-datagram socket.\0" as *const u8
                        as *const gchar,
                ),
            );
            return FALSE;
        }
        2 | 3 | _ => {}
    }
    if safe_c2rust_g_socket_get_timeout(self_0 as *mut ::core::ffi::c_void as *mut GSocket)
        != 0 as guint
    {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Cannot use datagram operations on a socket with a timeout set.\0" as *const u8
                    as *const gchar,
            ),
        );
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_socket_datagram_based_receive_messages(
    mut self_0: *mut GDatagramBased,
    mut messages: *mut GInputMessage,
    mut num_messages: guint,
    mut flags: gint,
    mut timeout_us: gint64,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gint {
    if safe_c2rust_check_datagram_based(self_0, error) == 0 {
        return FALSE;
    }
    return safe_c2rust_g_socket_receive_messages_with_timeout(
        self_0 as *mut ::core::ffi::c_void as *mut GSocket,
        messages,
        num_messages,
        flags,
        timeout_us,
        cancellable,
        error,
    );
}
unsafe extern "C" fn safe_c2rust_g_socket_datagram_based_send_messages(
    mut self_0: *mut GDatagramBased,
    mut messages: *mut GOutputMessage,
    mut num_messages: guint,
    mut flags: gint,
    mut timeout_us: gint64,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gint {
    if safe_c2rust_check_datagram_based(self_0, error) == 0 {
        return FALSE;
    }
    return safe_c2rust_g_socket_send_messages_with_timeout(
        self_0 as *mut ::core::ffi::c_void as *mut GSocket,
        messages,
        num_messages,
        flags,
        timeout_us,
        cancellable,
        error,
    );
}
unsafe extern "C" fn safe_c2rust_g_socket_datagram_based_create_source(
    mut self_0: *mut GDatagramBased,
    mut condition: GIOCondition,
    mut cancellable: *mut GCancellable,
) -> *mut GSource {
    if safe_c2rust_check_datagram_based(self_0, ::core::ptr::null_mut::<*mut GError>()) == 0 {
        return ::core::ptr::null_mut::<GSource>();
    }
    return safe_c2rust_g_socket_create_source(
        self_0 as *mut ::core::ffi::c_void as *mut GSocket,
        condition,
        cancellable,
    );
}
unsafe extern "C" fn safe_c2rust_g_socket_datagram_based_condition_check(
    mut datagram_based: *mut GDatagramBased,
    mut condition: GIOCondition,
) -> GIOCondition {
    if safe_c2rust_check_datagram_based(datagram_based, ::core::ptr::null_mut::<*mut GError>()) == 0
    {
        return G_IO_ERR;
    }
    return safe_c2rust_g_socket_condition_check(
        datagram_based as *mut ::core::ffi::c_void as *mut GSocket,
        condition,
    );
}
unsafe extern "C" fn safe_c2rust_g_socket_datagram_based_condition_wait(
    mut datagram_based: *mut GDatagramBased,
    mut condition: GIOCondition,
    mut timeout_us: gint64,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    if safe_c2rust_check_datagram_based(datagram_based, error) == 0 {
        return FALSE;
    }
    return safe_c2rust_g_socket_condition_timed_wait(
        datagram_based as *mut ::core::ffi::c_void as *mut GSocket,
        condition,
        timeout_us,
        cancellable,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_new(
    mut family: GSocketFamily,
    mut type_0: GSocketType,
    mut protocol: GSocketProtocol,
    mut error: *mut *mut GError,
) -> *mut GSocket {
    return g_initable_new(
        safe_c2rust_g_socket_get_type(),
        ::core::ptr::null_mut::<GCancellable>(),
        error,
        b"family\0" as *const u8 as *const gchar,
        family as ::core::ffi::c_uint,
        b"type\0" as *const u8 as *const ::core::ffi::c_char,
        type_0 as ::core::ffi::c_uint,
        b"protocol\0" as *const u8 as *const ::core::ffi::c_char,
        protocol as ::core::ffi::c_int,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut GSocket;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_new_from_fd(
    mut fd: gint,
    mut error: *mut *mut GError,
) -> *mut GSocket {
    return g_initable_new(
        safe_c2rust_g_socket_get_type(),
        ::core::ptr::null_mut::<GCancellable>(),
        error,
        b"fd\0" as *const u8 as *const gchar,
        fd,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut GSocket;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_set_blocking(
    mut socket_0: *mut GSocket,
    mut blocking: gboolean,
) {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    blocking = (blocking != 0) as ::core::ffi::c_int as gboolean;
    if (*(*socket_0).priv_0).blocking() as ::core::ffi::c_int == blocking {
        return;
    }
    (*(*socket_0).priv_0).set_blocking(blocking as guint as guint);
    g_object_notify(
        socket_0 as *mut ::core::ffi::c_void as *mut GObject,
        b"blocking\0" as *const u8 as *const gchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_get_blocking(mut socket_0: *mut GSocket) -> gboolean {
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*(*socket_0).priv_0).blocking() as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_set_keepalive(
    mut socket_0: *mut GSocket,
    mut keepalive: gboolean,
) {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    keepalive = (keepalive != 0) as ::core::ffi::c_int as gboolean;
    if (*(*socket_0).priv_0).keepalive() as ::core::ffi::c_int == keepalive {
        return;
    }
    if safe_c2rust_g_socket_set_option(
        socket_0,
        SOL_SOCKET,
        SO_KEEPALIVE,
        keepalive as gint,
        &raw mut error,
    ) == 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"error setting keepalive: %s\0" as *const u8 as *const gchar,
            (*error).message,
        );
        g_error_free(error);
        return;
    }
    (*(*socket_0).priv_0).set_keepalive(keepalive as guint as guint);
    g_object_notify(
        socket_0 as *mut ::core::ffi::c_void as *mut GObject,
        b"keepalive\0" as *const u8 as *const gchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_get_keepalive(
    mut socket_0: *mut GSocket,
) -> gboolean {
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*(*socket_0).priv_0).keepalive() as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_get_listen_backlog(
    mut socket_0: *mut GSocket,
) -> gint {
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    return (*(*socket_0).priv_0).listen_backlog;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_set_listen_backlog(
    mut socket_0: *mut GSocket,
    mut backlog: gint,
) {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if (*(*socket_0).priv_0).listening() == 0 {
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
            b"!socket->priv->listening\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if backlog != (*(*socket_0).priv_0).listen_backlog {
        (*(*socket_0).priv_0).listen_backlog = backlog;
        g_object_notify(
            socket_0 as *mut ::core::ffi::c_void as *mut GObject,
            b"listen-backlog\0" as *const u8 as *const gchar,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_get_timeout(mut socket_0: *mut GSocket) -> guint {
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    return (*(*socket_0).priv_0).timeout;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_set_timeout(
    mut socket_0: *mut GSocket,
    mut timeout: guint,
) {
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if timeout != (*(*socket_0).priv_0).timeout {
        (*(*socket_0).priv_0).timeout = timeout;
        g_object_notify(
            socket_0 as *mut ::core::ffi::c_void as *mut GObject,
            b"timeout\0" as *const u8 as *const gchar,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_get_ttl(mut socket_0: *mut GSocket) -> guint {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut value: gint = 0;
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if (*(*socket_0).priv_0).family as ::core::ffi::c_uint
        == G_SOCKET_FAMILY_IPV4 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        safe_c2rust_g_socket_get_option(
            socket_0,
            IPPROTO_IP as ::core::ffi::c_int as gint,
            IP_TTL,
            &raw mut value,
            &raw mut error,
        );
    } else if (*(*socket_0).priv_0).family as ::core::ffi::c_uint
        == G_SOCKET_FAMILY_IPV6 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        safe_c2rust_g_socket_get_option(
            socket_0,
            IPPROTO_IPV6 as ::core::ffi::c_int as gint,
            IPV6_UNICAST_HOPS,
            &raw mut value,
            &raw mut error,
        );
    } else {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"file %s: line %d (%s): should not be reached\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocket.c\0" as *const u8
                as *const ::core::ffi::c_char,
            1666 as ::core::ffi::c_int,
            G_STRFUNC,
        );
        return 0 as guint;
    }
    if !error.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"error getting unicast ttl: %s\0" as *const u8 as *const gchar,
            (*error).message,
        );
        g_error_free(error);
        return 0 as guint;
    }
    return value as guint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_set_ttl(mut socket_0: *mut GSocket, mut ttl: guint) {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*(*socket_0).priv_0).family as ::core::ffi::c_uint
        == G_SOCKET_FAMILY_IPV4 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        safe_c2rust_g_socket_set_option(
            socket_0,
            IPPROTO_IP as ::core::ffi::c_int as gint,
            IP_TTL,
            ttl as gint,
            &raw mut error,
        );
    } else if (*(*socket_0).priv_0).family as ::core::ffi::c_uint
        == G_SOCKET_FAMILY_IPV6 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        safe_c2rust_g_socket_set_option(
            socket_0,
            IPPROTO_IP as ::core::ffi::c_int as gint,
            IP_TTL,
            ttl as gint,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        safe_c2rust_g_socket_set_option(
            socket_0,
            IPPROTO_IPV6 as ::core::ffi::c_int as gint,
            IPV6_UNICAST_HOPS,
            ttl as gint,
            &raw mut error,
        );
    } else {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"file %s: line %d (%s): should not be reached\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocket.c\0" as *const u8
                as *const ::core::ffi::c_char,
            1709 as ::core::ffi::c_int,
            G_STRFUNC,
        );
        return;
    }
    if !error.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"error setting unicast ttl: %s\0" as *const u8 as *const gchar,
            (*error).message,
        );
        g_error_free(error);
        return;
    }
    g_object_notify(
        socket_0 as *mut ::core::ffi::c_void as *mut GObject,
        b"ttl\0" as *const u8 as *const gchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_get_broadcast(
    mut socket_0: *mut GSocket,
) -> gboolean {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut value: gint = 0;
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if safe_c2rust_g_socket_get_option(
        socket_0,
        SOL_SOCKET,
        SO_BROADCAST,
        &raw mut value,
        &raw mut error,
    ) == 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"error getting broadcast: %s\0" as *const u8 as *const gchar,
            (*error).message,
        );
        g_error_free(error);
        return FALSE;
    }
    return (value != 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_set_broadcast(
    mut socket_0: *mut GSocket,
    mut broadcast: gboolean,
) {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    broadcast = (broadcast != 0) as ::core::ffi::c_int as gboolean;
    if safe_c2rust_g_socket_set_option(
        socket_0,
        SOL_SOCKET,
        SO_BROADCAST,
        broadcast as gint,
        &raw mut error,
    ) == 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"error setting broadcast: %s\0" as *const u8 as *const gchar,
            (*error).message,
        );
        g_error_free(error);
        return;
    }
    g_object_notify(
        socket_0 as *mut ::core::ffi::c_void as *mut GObject,
        b"broadcast\0" as *const u8 as *const gchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_get_multicast_loopback(
    mut socket_0: *mut GSocket,
) -> gboolean {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut value: gint = 0;
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if (*(*socket_0).priv_0).family as ::core::ffi::c_uint
        == G_SOCKET_FAMILY_IPV4 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        safe_c2rust_g_socket_get_option(
            socket_0,
            IPPROTO_IP as ::core::ffi::c_int as gint,
            IP_MULTICAST_LOOP,
            &raw mut value,
            &raw mut error,
        );
    } else if (*(*socket_0).priv_0).family as ::core::ffi::c_uint
        == G_SOCKET_FAMILY_IPV6 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        safe_c2rust_g_socket_get_option(
            socket_0,
            IPPROTO_IPV6 as ::core::ffi::c_int as gint,
            IPV6_MULTICAST_LOOP,
            &raw mut value,
            &raw mut error,
        );
    } else {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"file %s: line %d (%s): should not be reached\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocket.c\0" as *const u8
                as *const ::core::ffi::c_char,
            1815 as ::core::ffi::c_int,
            G_STRFUNC,
        );
        return 0 as gboolean;
    }
    if !error.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"error getting multicast loopback: %s\0" as *const u8 as *const gchar,
            (*error).message,
        );
        g_error_free(error);
        return FALSE;
    }
    return (value != 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_set_multicast_loopback(
    mut socket_0: *mut GSocket,
    mut loopback: gboolean,
) {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    loopback = (loopback != 0) as ::core::ffi::c_int as gboolean;
    if (*(*socket_0).priv_0).family as ::core::ffi::c_uint
        == G_SOCKET_FAMILY_IPV4 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        safe_c2rust_g_socket_set_option(
            socket_0,
            IPPROTO_IP as ::core::ffi::c_int as gint,
            IP_MULTICAST_LOOP,
            loopback as gint,
            &raw mut error,
        );
    } else if (*(*socket_0).priv_0).family as ::core::ffi::c_uint
        == G_SOCKET_FAMILY_IPV6 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        safe_c2rust_g_socket_set_option(
            socket_0,
            IPPROTO_IP as ::core::ffi::c_int as gint,
            IP_MULTICAST_LOOP,
            loopback as gint,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        safe_c2rust_g_socket_set_option(
            socket_0,
            IPPROTO_IPV6 as ::core::ffi::c_int as gint,
            IPV6_MULTICAST_LOOP,
            loopback as gint,
            &raw mut error,
        );
    } else {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"file %s: line %d (%s): should not be reached\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocket.c\0" as *const u8
                as *const ::core::ffi::c_char,
            1862 as ::core::ffi::c_int,
            G_STRFUNC,
        );
        return;
    }
    if !error.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"error setting multicast loopback: %s\0" as *const u8 as *const gchar,
            (*error).message,
        );
        g_error_free(error);
        return;
    }
    g_object_notify(
        socket_0 as *mut ::core::ffi::c_void as *mut GObject,
        b"multicast-loopback\0" as *const u8 as *const gchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_get_multicast_ttl(
    mut socket_0: *mut GSocket,
) -> guint {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut value: gint = 0;
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if (*(*socket_0).priv_0).family as ::core::ffi::c_uint
        == G_SOCKET_FAMILY_IPV4 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        safe_c2rust_g_socket_get_option(
            socket_0,
            IPPROTO_IP as ::core::ffi::c_int as gint,
            IP_MULTICAST_TTL,
            &raw mut value,
            &raw mut error,
        );
    } else if (*(*socket_0).priv_0).family as ::core::ffi::c_uint
        == G_SOCKET_FAMILY_IPV6 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        safe_c2rust_g_socket_get_option(
            socket_0,
            IPPROTO_IPV6 as ::core::ffi::c_int as gint,
            IPV6_MULTICAST_HOPS,
            &raw mut value,
            &raw mut error,
        );
    } else {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"file %s: line %d (%s): should not be reached\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocket.c\0" as *const u8
                as *const ::core::ffi::c_char,
            1904 as ::core::ffi::c_int,
            G_STRFUNC,
        );
        return 0 as guint;
    }
    if !error.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"error getting multicast ttl: %s\0" as *const u8 as *const gchar,
            (*error).message,
        );
        g_error_free(error);
        return FALSE as guint;
    }
    return value as guint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_set_multicast_ttl(
    mut socket_0: *mut GSocket,
    mut ttl: guint,
) {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*(*socket_0).priv_0).family as ::core::ffi::c_uint
        == G_SOCKET_FAMILY_IPV4 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        safe_c2rust_g_socket_set_option(
            socket_0,
            IPPROTO_IP as ::core::ffi::c_int as gint,
            IP_MULTICAST_TTL,
            ttl as gint,
            &raw mut error,
        );
    } else if (*(*socket_0).priv_0).family as ::core::ffi::c_uint
        == G_SOCKET_FAMILY_IPV6 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        safe_c2rust_g_socket_set_option(
            socket_0,
            IPPROTO_IP as ::core::ffi::c_int as gint,
            IP_MULTICAST_TTL,
            ttl as gint,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        safe_c2rust_g_socket_set_option(
            socket_0,
            IPPROTO_IPV6 as ::core::ffi::c_int as gint,
            IPV6_MULTICAST_HOPS,
            ttl as gint,
            &raw mut error,
        );
    } else {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"file %s: line %d (%s): should not be reached\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocket.c\0" as *const u8
                as *const ::core::ffi::c_char,
            1948 as ::core::ffi::c_int,
            G_STRFUNC,
        );
        return;
    }
    if !error.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"error setting multicast ttl: %s\0" as *const u8 as *const gchar,
            (*error).message,
        );
        g_error_free(error);
        return;
    }
    g_object_notify(
        socket_0 as *mut ::core::ffi::c_void as *mut GObject,
        b"multicast-ttl\0" as *const u8 as *const gchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_get_family(
    mut socket_0: *mut GSocket,
) -> GSocketFamily {
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_SOCKET_FAMILY_INVALID;
    }
    return (*(*socket_0).priv_0).family;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_get_socket_type(
    mut socket_0: *mut GSocket,
) -> GSocketType {
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_SOCKET_TYPE_INVALID;
    }
    return (*(*socket_0).priv_0).type_0;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_get_protocol(
    mut socket_0: *mut GSocket,
) -> GSocketProtocol {
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_SOCKET_PROTOCOL_UNKNOWN;
    }
    return (*(*socket_0).priv_0).protocol;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_get_fd(
    mut socket_0: *mut GSocket,
) -> ::core::ffi::c_int {
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    return (*(*socket_0).priv_0).fd as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_get_local_address(
    mut socket_0: *mut GSocket,
    mut error: *mut *mut GError,
) -> *mut GSocketAddress {
    let mut buffer: C2RustUnnamed_3 = C2RustUnnamed_3 {
        storage: sockaddr_storage {
            ss_family: 0,
            __ss_padding: [0; 118],
            __ss_align: 0,
        },
    };
    let mut len: socklen_t = ::core::mem::size_of::<C2RustUnnamed_3>() as socklen_t;
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSocketAddress>();
    }
    if getsockname(
        (*(*socket_0).priv_0).fd as ::core::ffi::c_int,
        __SOCKADDR_ARG {
            __sockaddr__: &raw mut buffer.sa,
        },
        &raw mut len,
    ) < 0 as ::core::ffi::c_int
    {
        let mut errsv: ::core::ffi::c_int = safe_c2rust_get_socket_errno();
        g_set_error(
            error,
            g_io_error_quark(),
            safe_c2rust_socket_io_error_from_errno(errsv) as gint,
            glib_gettext(b"could not get local address: %s\0" as *const u8 as *const gchar),
            safe_c2rust_socket_strerror(errsv),
        );
        return ::core::ptr::null_mut::<GSocketAddress>();
    }
    return g_socket_address_new_from_native(&raw mut buffer.storage as gpointer, len as gsize);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_get_remote_address(
    mut socket_0: *mut GSocket,
    mut error: *mut *mut GError,
) -> *mut GSocketAddress {
    let mut buffer: C2RustUnnamed_1 = C2RustUnnamed_1 {
        storage: sockaddr_storage {
            ss_family: 0,
            __ss_padding: [0; 118],
            __ss_align: 0,
        },
    };
    let mut len: socklen_t = ::core::mem::size_of::<C2RustUnnamed_1>() as socklen_t;
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSocketAddress>();
    }
    if (*(*socket_0).priv_0).connect_pending() != 0 {
        if safe_c2rust_g_socket_check_connect_result(socket_0, error) == 0 {
            return ::core::ptr::null_mut::<GSocketAddress>();
        } else {
            (*(*socket_0).priv_0).set_connect_pending(FALSE as guint as guint);
        }
    }
    if (*(*socket_0).priv_0).remote_address.is_null() {
        if getpeername(
            (*(*socket_0).priv_0).fd as ::core::ffi::c_int,
            __SOCKADDR_ARG {
                __sockaddr__: &raw mut buffer.sa,
            },
            &raw mut len,
        ) < 0 as ::core::ffi::c_int
        {
            let mut errsv: ::core::ffi::c_int = safe_c2rust_get_socket_errno();
            g_set_error(
                error,
                g_io_error_quark(),
                safe_c2rust_socket_io_error_from_errno(errsv) as gint,
                glib_gettext(b"could not get remote address: %s\0" as *const u8 as *const gchar),
                safe_c2rust_socket_strerror(errsv),
            );
            return ::core::ptr::null_mut::<GSocketAddress>();
        }
        (*(*socket_0).priv_0).remote_address =
            g_socket_address_new_from_native(&raw mut buffer.storage as gpointer, len as gsize);
    }
    return g_object_ref((*(*socket_0).priv_0).remote_address as gpointer) as *mut GSocketAddress;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_is_connected(mut socket_0: *mut GSocket) -> gboolean {
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return ((*(*socket_0).priv_0).connected_read() as ::core::ffi::c_int != 0
        || (*(*socket_0).priv_0).connected_write() as ::core::ffi::c_int != 0)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_listen(
    mut socket_0: *mut GSocket,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if safe_c2rust_check_socket(socket_0, error) == 0 {
        return FALSE;
    }
    if listen(
        (*(*socket_0).priv_0).fd as ::core::ffi::c_int,
        (*(*socket_0).priv_0).listen_backlog as ::core::ffi::c_int,
    ) < 0 as ::core::ffi::c_int
    {
        let mut errsv: ::core::ffi::c_int = safe_c2rust_get_socket_errno();
        g_set_error(
            error,
            g_io_error_quark(),
            safe_c2rust_socket_io_error_from_errno(errsv) as gint,
            glib_gettext(b"could not listen: %s\0" as *const u8 as *const gchar),
            safe_c2rust_socket_strerror(errsv),
        );
        return FALSE;
    }
    (*(*socket_0).priv_0).set_listening(TRUE as guint as guint);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_bind(
    mut socket_0: *mut GSocket,
    mut address: *mut GSocketAddress,
    mut reuse_address: gboolean,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut addr: C2RustUnnamed_5 = C2RustUnnamed_5 {
        storage: sockaddr_storage {
            ss_family: 0,
            __ss_padding: [0; 118],
            __ss_align: 0,
        },
    };
    let mut so_reuseaddr: gboolean = 0;
    let mut so_reuseport: gboolean = 0;
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            && ({
                let mut __inst: *mut GTypeInstance = address as *mut GTypeInstance;
                let mut __t: GType = g_socket_address_get_type();
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
            b"G_IS_SOCKET (socket) && G_IS_SOCKET_ADDRESS (address)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if safe_c2rust_check_socket(socket_0, error) == 0 {
        return FALSE;
    }
    if g_socket_address_to_native(
        address,
        &raw mut addr.storage as gpointer,
        ::core::mem::size_of::<C2RustUnnamed_5>() as gsize,
        error,
    ) == 0
    {
        return FALSE;
    }
    so_reuseaddr = (reuse_address != 0) as ::core::ffi::c_int as gboolean;
    so_reuseport = (reuse_address != 0
        && (*(*socket_0).priv_0).type_0 as ::core::ffi::c_uint
            == G_SOCKET_TYPE_DATAGRAM as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int as gboolean;
    safe_c2rust_g_socket_set_option(
        socket_0,
        SOL_SOCKET,
        SO_REUSEADDR,
        so_reuseaddr as gint,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    safe_c2rust_g_socket_set_option(
        socket_0,
        SOL_SOCKET,
        SO_REUSEPORT,
        so_reuseport as gint,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if bind(
        (*(*socket_0).priv_0).fd as ::core::ffi::c_int,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: &raw mut addr.sa,
        },
        g_socket_address_get_native_size(address) as socklen_t,
    ) < 0 as ::core::ffi::c_int
    {
        let mut errsv: ::core::ffi::c_int = safe_c2rust_get_socket_errno();
        let mut address_string: *mut gchar = safe_c2rust_address_to_string(address);
        g_set_error(
            error,
            g_io_error_quark(),
            safe_c2rust_socket_io_error_from_errno(errsv) as gint,
            glib_gettext(b"Error binding to address %s: %s\0" as *const u8 as *const gchar),
            address_string,
            safe_c2rust_socket_strerror(errsv),
        );
        g_free(address_string as gpointer);
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_socket_multicast_group_operation(
    mut socket_0: *mut GSocket,
    mut group: *mut GInetAddress,
    mut source_specific: gboolean,
    mut iface: *const gchar,
    mut join_group: gboolean,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut native_addr: *const guint8 = ::core::ptr::null::<guint8>();
    let mut optname: gint = 0;
    let mut result: gint = 0;
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if (*(*socket_0).priv_0).type_0 as ::core::ffi::c_uint
            == G_SOCKET_TYPE_DATAGRAM as ::core::ffi::c_int as ::core::ffi::c_uint
        {
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
            b"socket->priv->type == G_SOCKET_TYPE_DATAGRAM\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = group as *mut GTypeInstance;
            let mut __t: GType = g_inet_address_get_type();
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
            b"G_IS_INET_ADDRESS (group)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if safe_c2rust_check_socket(socket_0, error) == 0 {
        return FALSE;
    }
    native_addr = g_inet_address_to_bytes(group);
    if g_inet_address_get_family(group) as ::core::ffi::c_uint
        == G_SOCKET_FAMILY_IPV4 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut mc_req: ip_mreqn = ip_mreqn {
            imr_multiaddr: in_addr { s_addr: 0 },
            imr_address: in_addr { s_addr: 0 },
            imr_ifindex: 0,
        };
        memset(
            &raw mut mc_req as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<ip_mreqn>() as size_t,
        );
        memcpy(
            &raw mut mc_req.imr_multiaddr as *mut ::core::ffi::c_void,
            native_addr as *const ::core::ffi::c_void,
            ::core::mem::size_of::<in_addr>() as size_t,
        );
        if !iface.is_null() {
            mc_req.imr_ifindex =
                if_nametoindex(iface as *const ::core::ffi::c_char) as ::core::ffi::c_int;
        } else {
            mc_req.imr_ifindex = 0 as ::core::ffi::c_int;
        }
        if source_specific != 0 {
            optname = (if join_group != 0 {
                IP_ADD_SOURCE_MEMBERSHIP
            } else {
                IP_DROP_SOURCE_MEMBERSHIP
            }) as gint;
        } else {
            optname = (if join_group != 0 {
                IP_ADD_MEMBERSHIP
            } else {
                IP_DROP_MEMBERSHIP
            }) as gint;
        }
        result = setsockopt(
            (*(*socket_0).priv_0).fd as ::core::ffi::c_int,
            IPPROTO_IP as ::core::ffi::c_int,
            optname as ::core::ffi::c_int,
            &raw mut mc_req as *const ::core::ffi::c_void,
            ::core::mem::size_of::<ip_mreqn>() as socklen_t,
        ) as gint;
    } else if g_inet_address_get_family(group) as ::core::ffi::c_uint
        == G_SOCKET_FAMILY_IPV6 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut mc_req_ipv6: ipv6_mreq = ipv6_mreq {
            ipv6mr_multiaddr: in6_addr {
                __in6_u: C2RustUnnamed_2 {
                    __u6_addr8: [0; 16],
                },
            },
            ipv6mr_interface: 0,
        };
        memset(
            &raw mut mc_req_ipv6 as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<ipv6_mreq>() as size_t,
        );
        memcpy(
            &raw mut mc_req_ipv6.ipv6mr_multiaddr as *mut ::core::ffi::c_void,
            native_addr as *const ::core::ffi::c_void,
            ::core::mem::size_of::<in6_addr>() as size_t,
        );
        if !iface.is_null() {
            mc_req_ipv6.ipv6mr_interface = if_nametoindex(iface as *const ::core::ffi::c_char);
        } else {
            mc_req_ipv6.ipv6mr_interface = 0 as ::core::ffi::c_uint;
        }
        optname = (if join_group != 0 {
            IPV6_JOIN_GROUP
        } else {
            IPV6_LEAVE_GROUP
        }) as gint;
        result = setsockopt(
            (*(*socket_0).priv_0).fd as ::core::ffi::c_int,
            IPPROTO_IPV6 as ::core::ffi::c_int,
            optname as ::core::ffi::c_int,
            &raw mut mc_req_ipv6 as *const ::core::ffi::c_void,
            ::core::mem::size_of::<ipv6_mreq>() as socklen_t,
        ) as gint;
    } else {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"file %s: line %d (%s): should not be reached\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocket.c\0" as *const u8
                as *const ::core::ffi::c_char,
            2487 as ::core::ffi::c_int,
            G_STRFUNC,
        );
        return 0 as gboolean;
    }
    if result < 0 as ::core::ffi::c_int {
        let mut errsv: ::core::ffi::c_int = safe_c2rust_get_socket_errno();
        g_set_error(
            error,
            g_io_error_quark(),
            safe_c2rust_socket_io_error_from_errno(errsv) as gint,
            if join_group != 0 {
                glib_gettext(b"Error joining multicast group: %s\0" as *const u8 as *const gchar)
            } else {
                glib_gettext(b"Error leaving multicast group: %s\0" as *const u8 as *const gchar)
            },
            safe_c2rust_socket_strerror(errsv),
        );
        return FALSE;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_join_multicast_group(
    mut socket_0: *mut GSocket,
    mut group: *mut GInetAddress,
    mut source_specific: gboolean,
    mut iface: *const gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    return safe_c2rust_g_socket_multicast_group_operation(
        socket_0,
        group,
        source_specific,
        iface,
        TRUE,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_leave_multicast_group(
    mut socket_0: *mut GSocket,
    mut group: *mut GInetAddress,
    mut source_specific: gboolean,
    mut iface: *const gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    return safe_c2rust_g_socket_multicast_group_operation(
        socket_0,
        group,
        source_specific,
        iface,
        FALSE,
        error,
    );
}
unsafe extern "C" fn safe_c2rust_g_socket_multicast_group_operation_ssm(
    mut socket_0: *mut GSocket,
    mut group: *mut GInetAddress,
    mut source_specific: *mut GInetAddress,
    mut iface: *const gchar,
    mut join_group: gboolean,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut result: gint = 0;
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if (*(*socket_0).priv_0).type_0 as ::core::ffi::c_uint
            == G_SOCKET_TYPE_DATAGRAM as ::core::ffi::c_int as ::core::ffi::c_uint
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
            b"socket->priv->type == G_SOCKET_TYPE_DATAGRAM\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = group as *mut GTypeInstance;
            let mut __t: GType = g_inet_address_get_type();
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
            b"G_IS_INET_ADDRESS (group)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if iface.is_null() || *iface as ::core::ffi::c_int != '\0' as i32 {
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
            b"iface == NULL || *iface != '\\0'\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if source_specific.is_null() {
        return safe_c2rust_g_socket_multicast_group_operation(
            socket_0, group, FALSE, iface, join_group, error,
        );
    }
    if safe_c2rust_check_socket(socket_0, error) == 0 {
        return FALSE;
    }
    match g_inet_address_get_family(group) as ::core::ffi::c_uint {
        0 | 1 => {
            g_set_error(
                error,
                g_io_error_quark(),
                G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
                if join_group != 0 {
                    glib_gettext(
                        b"Error joining multicast group: %s\0" as *const u8 as *const gchar,
                    )
                } else {
                    glib_gettext(
                        b"Error leaving multicast group: %s\0" as *const u8 as *const gchar,
                    )
                },
                glib_gettext(b"Unsupported socket family\0" as *const u8 as *const gchar),
            );
            return FALSE;
        }
        2 => {
            let mut optname: gint = 0;
            let mut mc_req_src: ip_mreq_source = ip_mreq_source {
                imr_multiaddr: in_addr { s_addr: 0 },
                imr_interface: in_addr { s_addr: 0 },
                imr_sourceaddr: in_addr { s_addr: 0 },
            };
            if g_inet_address_get_family(source_specific) as ::core::ffi::c_uint
                != G_SOCKET_FAMILY_IPV4 as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                g_set_error(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
                    if join_group != 0 {
                        glib_gettext(
                            b"Error joining multicast group: %s\0" as *const u8 as *const gchar,
                        )
                    } else {
                        glib_gettext(
                            b"Error leaving multicast group: %s\0" as *const u8 as *const gchar,
                        )
                    },
                    glib_gettext(
                        b"source-specific not an IPv4 address\0" as *const u8 as *const gchar,
                    ),
                );
                return FALSE;
            }
            memset(
                &raw mut mc_req_src as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<ip_mreq_source>() as size_t,
            );
            mc_req_src.imr_interface.s_addr = ({
                let mut __v: guint32 = 0;
                let mut __x: guint32 = 0 as ::core::ffi::c_int as in_addr_t as guint32;
                if 0 != 0 {
                    __v = (__x & 0xff as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int
                        | (__x & 0xff00 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int
                        | (__x & 0xff0000 as ::core::ffi::c_uint) >> 8 as ::core::ffi::c_int
                        | (__x & 0xff000000 as ::core::ffi::c_uint) >> 24 as ::core::ffi::c_int;
                } else {
                    let fresh3 = &mut __v;
                    let fresh4;
                    let fresh5 = __x;
                    asm!(
                        "bswapl {0:e}\n", inlateout(reg)
                        c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5) => fresh4,
                        options(preserves_flags, pure, readonly, att_syntax)
                    );
                    c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
                }
                __v
            }) as in_addr_t;
            if !iface.is_null() {
                let mut ret: ::core::ffi::c_int = 0;
                let mut ifr: ifreq = ifreq {
                    ifr_ifrn: C2RustUnnamed_7 { ifrn_name: [0; 16] },
                    ifr_ifru: C2RustUnnamed_6 {
                        ifru_addr: sockaddr {
                            sa_family: 0,
                            sa_data: [0; 14],
                        },
                    },
                };
                let mut iface_addr: *mut sockaddr_in = ::core::ptr::null_mut::<sockaddr_in>();
                let mut if_name_len: size_t = strlen(iface as *const ::core::ffi::c_char);
                memset(
                    &raw mut ifr as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<ifreq>() as size_t,
                );
                if if_name_len >= ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as usize {
                    g_set_error(
                        error,
                        g_io_error_quark(),
                        G_IO_ERROR_FILENAME_TOO_LONG as ::core::ffi::c_int as gint,
                        glib_gettext(b"Interface name too long\0" as *const u8 as *const gchar),
                    );
                    return FALSE;
                }
                memcpy(
                    &raw mut ifr.ifr_ifrn.ifrn_name as *mut ::core::ffi::c_char
                        as *mut ::core::ffi::c_void,
                    iface as *const ::core::ffi::c_void,
                    if_name_len,
                );
                ret = ioctl(
                    (*(*socket_0).priv_0).fd as ::core::ffi::c_int,
                    SIOCGIFADDR as ::core::ffi::c_ulong,
                    &raw mut ifr,
                );
                if ret < 0 as ::core::ffi::c_int {
                    let mut errsv: ::core::ffi::c_int = *__errno_location();
                    g_set_error(
                        error,
                        g_io_error_quark(),
                        g_io_error_from_errno(errsv as gint) as gint,
                        glib_gettext(b"Interface not found: %s\0" as *const u8 as *const gchar),
                        g_strerror(errsv as gint),
                    );
                    return FALSE;
                }
                iface_addr = &raw mut ifr.ifr_ifru.ifru_addr as *mut sockaddr_in;
                mc_req_src.imr_interface.s_addr = (*iface_addr).sin_addr.s_addr;
            }
            if ({
                let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
                if g_inet_address_get_native_size(group) as usize
                    == ::core::mem::size_of::<in_addr>() as usize
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
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocket.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    2678 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"g_inet_address_get_native_size (group) == sizeof (mc_req_src.imr_multiaddr)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
            memcpy(
                &raw mut mc_req_src.imr_multiaddr as *mut ::core::ffi::c_void,
                g_inet_address_to_bytes(group) as *const ::core::ffi::c_void,
                g_inet_address_get_native_size(group) as size_t,
            );
            if ({
                let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
                if g_inet_address_get_native_size(source_specific) as usize
                    == ::core::mem::size_of::<in_addr>() as usize
                {
                    _g_boolean_var_48 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_48 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_48
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocket.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    2682 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"g_inet_address_get_native_size (source_specific) == sizeof (mc_req_src.imr_sourceaddr)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
            memcpy(
                &raw mut mc_req_src.imr_sourceaddr as *mut ::core::ffi::c_void,
                g_inet_address_to_bytes(source_specific) as *const ::core::ffi::c_void,
                g_inet_address_get_native_size(source_specific) as size_t,
            );
            optname = (if join_group != 0 {
                IP_ADD_SOURCE_MEMBERSHIP
            } else {
                IP_DROP_SOURCE_MEMBERSHIP
            }) as gint;
            result = setsockopt(
                (*(*socket_0).priv_0).fd as ::core::ffi::c_int,
                IPPROTO_IP as ::core::ffi::c_int,
                optname as ::core::ffi::c_int,
                &raw mut mc_req_src as *const ::core::ffi::c_void,
                ::core::mem::size_of::<ip_mreq_source>() as socklen_t,
            ) as gint;
        }
        10 => {
            let mut res: gboolean = 0;
            let mut optname_0: gint = 0;
            let mut mc_req_src_0: group_source_req = group_source_req {
                gsr_interface: 0,
                gsr_group: sockaddr_storage {
                    ss_family: 0,
                    __ss_padding: [0; 118],
                    __ss_align: 0,
                },
                gsr_source: sockaddr_storage {
                    ss_family: 0,
                    __ss_padding: [0; 118],
                    __ss_align: 0,
                },
            };
            let mut saddr_group: *mut GSocketAddress = ::core::ptr::null_mut::<GSocketAddress>();
            let mut saddr_source_specific: *mut GSocketAddress =
                ::core::ptr::null_mut::<GSocketAddress>();
            let mut iface_index: guint = 0 as guint;
            if !iface.is_null() {
                iface_index = if_nametoindex(iface as *const ::core::ffi::c_char) as guint;
                if iface_index == 0 as guint {
                    let mut errsv_0: ::core::ffi::c_int = *__errno_location();
                    g_set_error(
                        error,
                        g_io_error_quark(),
                        g_io_error_from_errno(errsv_0 as gint) as gint,
                        glib_gettext(b"Interface not found: %s\0" as *const u8 as *const gchar),
                        g_strerror(errsv_0 as gint),
                    );
                    return FALSE;
                }
            }
            mc_req_src_0.gsr_interface = iface_index as uint32_t;
            saddr_group = g_inet_socket_address_new(group, 0 as guint16);
            res = g_socket_address_to_native(
                saddr_group,
                &raw mut mc_req_src_0.gsr_group as gpointer,
                ::core::mem::size_of::<sockaddr_storage>() as gsize,
                error,
            );
            g_object_unref(saddr_group as gpointer);
            if res == 0 {
                return FALSE;
            }
            saddr_source_specific = g_inet_socket_address_new(source_specific, 0 as guint16);
            res = g_socket_address_to_native(
                saddr_source_specific,
                &raw mut mc_req_src_0.gsr_source as gpointer,
                ::core::mem::size_of::<sockaddr_storage>() as gsize,
                error,
            );
            g_object_unref(saddr_source_specific as gpointer);
            if res == 0 {
                return FALSE;
            }
            optname_0 = (if join_group != 0 {
                MCAST_JOIN_SOURCE_GROUP
            } else {
                MCAST_LEAVE_SOURCE_GROUP
            }) as gint;
            result = setsockopt(
                (*(*socket_0).priv_0).fd as ::core::ffi::c_int,
                IPPROTO_IPV6 as ::core::ffi::c_int,
                optname_0 as ::core::ffi::c_int,
                &raw mut mc_req_src_0 as *const ::core::ffi::c_void,
                ::core::mem::size_of::<group_source_req>() as socklen_t,
            ) as gint;
        }
        _ => {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"file %s: line %d (%s): should not be reached\0" as *const u8 as *const gchar,
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocket.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                2764 as ::core::ffi::c_int,
                G_STRFUNC,
            );
            return 0 as gboolean;
        }
    }
    if result < 0 as ::core::ffi::c_int {
        let mut errsv_1: ::core::ffi::c_int = safe_c2rust_get_socket_errno();
        g_set_error(
            error,
            g_io_error_quark(),
            safe_c2rust_socket_io_error_from_errno(errsv_1) as gint,
            if join_group != 0 {
                glib_gettext(b"Error joining multicast group: %s\0" as *const u8 as *const gchar)
            } else {
                glib_gettext(b"Error leaving multicast group: %s\0" as *const u8 as *const gchar)
            },
            safe_c2rust_socket_strerror(errsv_1),
        );
        return FALSE;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_join_multicast_group_ssm(
    mut socket_0: *mut GSocket,
    mut group: *mut GInetAddress,
    mut source_specific: *mut GInetAddress,
    mut iface: *const gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    return safe_c2rust_g_socket_multicast_group_operation_ssm(
        socket_0,
        group,
        source_specific,
        iface,
        TRUE,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_leave_multicast_group_ssm(
    mut socket_0: *mut GSocket,
    mut group: *mut GInetAddress,
    mut source_specific: *mut GInetAddress,
    mut iface: *const gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    return safe_c2rust_g_socket_multicast_group_operation_ssm(
        socket_0,
        group,
        source_specific,
        iface,
        FALSE,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_speaks_ipv4(mut socket_0: *mut GSocket) -> gboolean {
    match (*(*socket_0).priv_0).family as ::core::ffi::c_uint {
        2 => return TRUE,
        10 => {
            let mut v6_only: gint = 0;
            if safe_c2rust_g_socket_get_option(
                socket_0,
                IPPROTO_IPV6 as ::core::ffi::c_int as gint,
                IPV6_V6ONLY,
                &raw mut v6_only,
                ::core::ptr::null_mut::<*mut GError>(),
            ) == 0
            {
                return FALSE;
            }
            return (v6_only == 0) as ::core::ffi::c_int;
        }
        _ => return FALSE,
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_accept(
    mut socket_0: *mut GSocket,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GSocket {
    let mut try_accept4: gboolean = TRUE;
    let mut new_socket: *mut GSocket = ::core::ptr::null_mut::<GSocket>();
    let mut ret: gint = 0;
    if ({
        let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSocket>();
    }
    if safe_c2rust_check_socket(socket_0, error) == 0 {
        return ::core::ptr::null_mut::<GSocket>();
    }
    if safe_c2rust_check_timeout(socket_0, error) == 0 {
        return ::core::ptr::null_mut::<GSocket>();
    }
    while FALSE == 0 {
        let mut try_accept: gboolean = TRUE;
        if try_accept4 != 0 {
            ret = accept4(
                (*(*socket_0).priv_0).fd as ::core::ffi::c_int,
                __SOCKADDR_ARG {
                    __sockaddr__: ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut sockaddr,
                },
                ::core::ptr::null_mut::<socklen_t>(),
                SOCK_CLOEXEC as ::core::ffi::c_int,
            ) as gint;
            if ret < 0 as ::core::ffi::c_int && *__errno_location() == ENOSYS {
                try_accept4 = FALSE as gboolean;
            } else {
                try_accept = FALSE as gboolean;
            }
        }
        if ({
            let mut _g_boolean_var_50: ::core::ffi::c_int = 0;
            if try_accept4 != 0 || try_accept != 0 {
                _g_boolean_var_50 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_50 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_50
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocket.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                2959 as ::core::ffi::c_int,
                G_STRFUNC,
                b"try_accept4 || try_accept\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if try_accept != 0 {
            ret = accept(
                (*(*socket_0).priv_0).fd as ::core::ffi::c_int,
                __SOCKADDR_ARG {
                    __sockaddr__: ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut sockaddr,
                },
                ::core::ptr::null_mut::<socklen_t>(),
            ) as gint;
        }
        if !(ret < 0 as ::core::ffi::c_int) {
            break;
        }
        let mut errsv: ::core::ffi::c_int = safe_c2rust_get_socket_errno();
        if errsv == EINTR {
            continue;
        }
        if errsv == EWOULDBLOCK || errsv == EAGAIN {
            if (*(*socket_0).priv_0).blocking() != 0 {
                if safe_c2rust_g_socket_condition_wait(socket_0, G_IO_IN, cancellable, error) == 0 {
                    return ::core::ptr::null_mut::<GSocket>();
                }
                continue;
            }
        }
        let mut __err: *mut *mut GError = error;
        let mut __errsv: ::core::ffi::c_int = errsv;
        if !__err.is_null() {
            let mut __code: ::core::ffi::c_int =
                safe_c2rust_socket_io_error_from_errno(__errsv) as ::core::ffi::c_int;
            let mut __strerr: *const ::core::ffi::c_char = safe_c2rust_socket_strerror(__errsv);
            if __code == G_IO_ERROR_WOULD_BLOCK as ::core::ffi::c_int {
                g_set_error_literal(
                    __err,
                    g_io_error_quark(),
                    __code as gint,
                    __strerr as *const gchar,
                );
            } else {
                g_set_error(
                    __err,
                    g_io_error_quark(),
                    __code as gint,
                    glib_gettext(b"Error accepting connection: %s\0" as *const u8 as *const gchar),
                    __strerr,
                );
            }
        }
        return ::core::ptr::null_mut::<GSocket>();
    }
    let mut flags: ::core::ffi::c_int = 0;
    flags = fcntl(ret as ::core::ffi::c_int, F_GETFD, 0 as ::core::ffi::c_int);
    if flags != -(1 as ::core::ffi::c_int) && flags & FD_CLOEXEC == 0 as ::core::ffi::c_int {
        flags |= FD_CLOEXEC;
        fcntl(ret as ::core::ffi::c_int, F_SETFD, flags);
    }
    new_socket = safe_c2rust_g_socket_new_from_fd(ret, error);
    if new_socket.is_null() {
        close(ret as ::core::ffi::c_int);
    } else {
        (*(*new_socket).priv_0).protocol = (*(*socket_0).priv_0).protocol;
    }
    return new_socket;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_connect(
    mut socket_0: *mut GSocket,
    mut address: *mut GSocketAddress,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut buffer: C2RustUnnamed_8 = C2RustUnnamed_8 {
        storage: sockaddr_storage {
            ss_family: 0,
            __ss_padding: [0; 118],
            __ss_align: 0,
        },
    };
    if ({
        let mut _g_boolean_var_51: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            && ({
                let mut __inst: *mut GTypeInstance = address as *mut GTypeInstance;
                let mut __t: GType = g_socket_address_get_type();
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
            b"G_IS_SOCKET (socket) && G_IS_SOCKET_ADDRESS (address)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if safe_c2rust_check_socket(socket_0, error) == 0 {
        return FALSE;
    }
    if g_socket_address_to_native(
        address,
        &raw mut buffer.storage as gpointer,
        ::core::mem::size_of::<C2RustUnnamed_8>() as gsize,
        error,
    ) == 0
    {
        return FALSE;
    }
    if !(*(*socket_0).priv_0).remote_address.is_null() {
        g_object_unref((*(*socket_0).priv_0).remote_address as gpointer);
    }
    (*(*socket_0).priv_0).remote_address =
        g_object_ref(address as gpointer) as *mut GSocketAddress as *mut GSocketAddress;
    while connect(
        (*(*socket_0).priv_0).fd as ::core::ffi::c_int,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: &raw mut buffer.sa,
        },
        g_socket_address_get_native_size(address) as socklen_t,
    ) < 0 as ::core::ffi::c_int
    {
        let mut errsv: ::core::ffi::c_int = safe_c2rust_get_socket_errno();
        if errsv == EINTR {
            continue;
        }
        if errsv == EINPROGRESS {
            if (*(*socket_0).priv_0).blocking() != 0 {
                if safe_c2rust_g_socket_condition_wait(socket_0, G_IO_OUT, cancellable, error) != 0
                {
                    if safe_c2rust_g_socket_check_connect_result(socket_0, error) != 0 {
                        break;
                    }
                }
            } else {
                g_set_error_literal(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_PENDING as ::core::ffi::c_int as gint,
                    glib_gettext(b"Connection in progress\0" as *const u8 as *const gchar),
                );
                (*(*socket_0).priv_0).set_connect_pending(TRUE as guint as guint);
            }
        } else {
            g_set_error_literal(
                error,
                g_io_error_quark(),
                safe_c2rust_socket_io_error_from_errno(errsv) as gint,
                safe_c2rust_socket_strerror(errsv) as *const gchar,
            );
        }
        return FALSE;
    }
    (*(*socket_0).priv_0).set_connected_read(TRUE as guint as guint);
    (*(*socket_0).priv_0).set_connected_write(TRUE as guint as guint);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_check_connect_result(
    mut socket_0: *mut GSocket,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut value: ::core::ffi::c_int = 0;
    if ({
        let mut _g_boolean_var_52: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if safe_c2rust_check_socket(socket_0, error) == 0 {
        return FALSE;
    }
    if safe_c2rust_check_timeout(socket_0, error) == 0 {
        return FALSE;
    }
    if safe_c2rust_g_socket_get_option(socket_0, SOL_SOCKET, SO_ERROR, &raw mut value, error) == 0 {
        g_prefix_error(
            error,
            glib_gettext(b"Unable to get pending error: \0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    if value != 0 as ::core::ffi::c_int {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            safe_c2rust_socket_io_error_from_errno(value) as gint,
            safe_c2rust_socket_strerror(value) as *const gchar,
        );
        if !(*(*socket_0).priv_0).remote_address.is_null() {
            g_object_unref((*(*socket_0).priv_0).remote_address as gpointer);
            (*(*socket_0).priv_0).remote_address = ::core::ptr::null_mut::<GSocketAddress>();
        }
        return FALSE;
    }
    (*(*socket_0).priv_0).set_connected_read(TRUE as guint as guint);
    (*(*socket_0).priv_0).set_connected_write(TRUE as guint as guint);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_get_available_bytes(
    mut socket_0: *mut GSocket,
) -> gssize {
    let bufsize: gint = 64 as gint * 1024 as gint;
    static mut safe_c2rust_buf: *mut guchar = ::core::ptr::null::<guchar>() as *mut guchar;
    let mut avail: gint = 0;
    if ({
        let mut _g_boolean_var_53: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            _g_boolean_var_53 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_53 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_53
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    if safe_c2rust_check_socket(socket_0, ::core::ptr::null_mut::<*mut GError>()) == 0 {
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    if (*(*socket_0).priv_0).type_0 as ::core::ffi::c_uint
        == G_SOCKET_TYPE_DATAGRAM as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if ({
            let mut _g_boolean_var_54: ::core::ffi::c_int = 0;
            if ({
                if 0 as ::core::ffi::c_int != 0 {
                    safe_c2rust_buf;
                } else {
                };
                (({
                    let mut gapg_temp_newval: *mut guchar = ::core::ptr::null_mut::<guchar>();
                    let mut gapg_temp_atomic: *mut *mut guchar = &raw mut safe_c2rust_buf;
                    *&raw mut gapg_temp_newval =
                        crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
                    gapg_temp_newval
                })
                .is_null()
                    && g_once_init_enter_pointer(
                        &raw mut safe_c2rust_buf as *mut ::core::ffi::c_void,
                    ) != 0) as ::core::ffi::c_int
            }) != 0
            {
                _g_boolean_var_54 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_54 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_54
        }) as ::core::ffi::c_long
            != 0
        {
            if 0 as ::core::ffi::c_int != 0 {
                safe_c2rust_buf = g_malloc(bufsize as gsize) as *mut guchar;
            } else {
            };
            g_once_init_leave_pointer(
                &raw mut safe_c2rust_buf as *mut ::core::ffi::c_void,
                g_malloc(bufsize as gsize) as guintptr as gpointer,
            );
        }
        avail = recv(
            (*(*socket_0).priv_0).fd as ::core::ffi::c_int,
            safe_c2rust_buf as *mut ::core::ffi::c_void,
            bufsize as size_t,
            MSG_PEEK as ::core::ffi::c_int,
        ) as gint;
        if avail == -(1 as ::core::ffi::c_int) {
            let mut errsv: ::core::ffi::c_int = safe_c2rust_get_socket_errno();
            if errsv == EWOULDBLOCK || errsv == EAGAIN {
                avail = 0 as ::core::ffi::c_int as gint;
            }
        }
    } else if ioctl(
        (*(*socket_0).priv_0).fd as ::core::ffi::c_int,
        FIONREAD as ::core::ffi::c_ulong,
        &raw mut avail,
    ) < 0 as ::core::ffi::c_int
    {
        avail = -(1 as ::core::ffi::c_int) as gint;
    }
    return avail as gssize;
}
unsafe extern "C" fn safe_c2rust_block_on_timeout(
    mut socket_0: *mut GSocket,
    mut condition: GIOCondition,
    mut timeout_us: gint64,
    mut start_time: gint64,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut wait_timeout: gint64 = -(1 as ::core::ffi::c_int) as gint64;
    if ({
        let mut _g_boolean_var_55: ::core::ffi::c_int = 0;
        if timeout_us != 0 as gint64 {
            _g_boolean_var_55 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_55 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_55
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"timeout_us != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    if timeout_us >= 0 as gint64 {
        let mut elapsed: gint64 = g_get_monotonic_time() - start_time;
        if elapsed >= timeout_us {
            g_set_error_literal(
                error,
                g_io_error_quark(),
                G_IO_ERROR_TIMED_OUT as ::core::ffi::c_int as gint,
                glib_gettext(b"Socket I/O timed out\0" as *const u8 as *const gchar),
            );
            return FALSE;
        }
        wait_timeout = timeout_us - elapsed;
    }
    return safe_c2rust_g_socket_condition_timed_wait(
        socket_0,
        condition,
        wait_timeout,
        cancellable,
        error,
    );
}
unsafe extern "C" fn safe_c2rust_g_socket_receive_with_timeout(
    mut socket_0: *mut GSocket,
    mut buffer: *mut guint8,
    mut size: gsize,
    mut timeout_us: gint64,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    let mut ret: gssize = 0;
    let mut start_time: gint64 = 0;
    if ({
        let mut _g_boolean_var_56: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            && !buffer.is_null()
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
            b"G_IS_SOCKET (socket) && buffer != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    start_time = g_get_monotonic_time();
    if safe_c2rust_check_socket(socket_0, error) == 0 {
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    if safe_c2rust_check_timeout(socket_0, error) == 0 {
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    loop {
        ret = recv(
            (*(*socket_0).priv_0).fd as ::core::ffi::c_int,
            buffer as *mut ::core::ffi::c_void,
            size as size_t,
            0 as ::core::ffi::c_int,
        ) as gssize;
        if !(ret < 0 as gssize) {
            break;
        }
        let mut errsv: ::core::ffi::c_int = safe_c2rust_get_socket_errno();
        if errsv == EINTR {
            continue;
        }
        if errsv == EWOULDBLOCK || errsv == EAGAIN {
            if timeout_us != 0 as gint64 {
                if safe_c2rust_block_on_timeout(
                    socket_0,
                    G_IO_IN,
                    timeout_us,
                    start_time,
                    cancellable,
                    error,
                ) == 0
                {
                    return -(1 as ::core::ffi::c_int) as gssize;
                }
                continue;
            }
        }
        let mut __err: *mut *mut GError = error;
        let mut __errsv: ::core::ffi::c_int = errsv;
        if !__err.is_null() {
            let mut __code: ::core::ffi::c_int =
                safe_c2rust_socket_io_error_from_errno(__errsv) as ::core::ffi::c_int;
            let mut __strerr: *const ::core::ffi::c_char = safe_c2rust_socket_strerror(__errsv);
            if __code == G_IO_ERROR_WOULD_BLOCK as ::core::ffi::c_int {
                g_set_error_literal(
                    __err,
                    g_io_error_quark(),
                    __code as gint,
                    __strerr as *const gchar,
                );
            } else {
                g_set_error(
                    __err,
                    g_io_error_quark(),
                    __code as gint,
                    glib_gettext(b"Error receiving data: %s\0" as *const u8 as *const gchar),
                    __strerr,
                );
            }
        }
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_receive_bytes(
    mut socket_0: *mut GSocket,
    mut size: gsize,
    mut timeout_us: gint64,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GBytes {
    let mut data: *mut guint8 = ::core::ptr::null_mut::<guint8>();
    let mut res: gssize = 0;
    let mut buf: *mut GBytes = ::core::ptr::null_mut::<GBytes>();
    if ({
        let mut _g_boolean_var_57: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBytes>();
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
        return ::core::ptr::null_mut::<GBytes>();
    }
    if ({
        let mut _g_boolean_var_59: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBytes>();
    }
    data = ({
        let mut __n: gsize = size;
        let mut __s: gsize = ::core::mem::size_of::<guint8>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut guint8;
    res = safe_c2rust_g_socket_receive_with_timeout(
        socket_0,
        data,
        size,
        timeout_us,
        cancellable,
        error,
    );
    if res < 0 as gssize {
        g_free(data as gpointer);
        return ::core::ptr::null_mut::<GBytes>();
    }
    if res as gsize == size {
        buf = g_bytes_new_take(
            safe_c2rust_g_steal_pointer(&raw mut data as gpointer) as *mut guint8 as gpointer,
            res as gsize,
        );
    } else {
        let mut sub_buf: *mut GBytes = ::core::ptr::null_mut::<GBytes>();
        buf = g_bytes_new_take(
            safe_c2rust_g_steal_pointer(&raw mut data as gpointer) as *mut guint8 as gpointer,
            size,
        );
        sub_buf = g_bytes_new_from_bytes(buf, 0 as gsize, res as gsize);
        g_bytes_unref(buf);
        buf =
            safe_c2rust_g_steal_pointer(&raw mut sub_buf as gpointer) as *mut GBytes as *mut GBytes;
    }
    return safe_c2rust_g_steal_pointer(&raw mut buf as gpointer) as *mut GBytes;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_receive(
    mut socket_0: *mut GSocket,
    mut buffer: *mut gchar,
    mut size: gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    return safe_c2rust_g_socket_receive_with_timeout(
        socket_0,
        buffer as *mut guint8,
        size,
        (if (*(*socket_0).priv_0).blocking() as ::core::ffi::c_int != 0 {
            -(1 as ::core::ffi::c_int)
        } else {
            0 as ::core::ffi::c_int
        }) as gint64,
        cancellable,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_receive_with_blocking(
    mut socket_0: *mut GSocket,
    mut buffer: *mut gchar,
    mut size: gsize,
    mut blocking: gboolean,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    return safe_c2rust_g_socket_receive_with_timeout(
        socket_0,
        buffer as *mut guint8,
        size,
        (if blocking != 0 {
            -(1 as ::core::ffi::c_int)
        } else {
            0 as ::core::ffi::c_int
        }) as gint64,
        cancellable,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_receive_bytes_from(
    mut socket_0: *mut GSocket,
    mut address: *mut *mut GSocketAddress,
    mut size: gsize,
    mut timeout_us: gint64,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GBytes {
    let mut v: GInputVector = _GInputVector {
        buffer: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        size: 0,
    };
    let mut res: gssize = 0;
    let mut buf: *mut GBytes = ::core::ptr::null_mut::<GBytes>();
    if ({
        let mut _g_boolean_var_60: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBytes>();
    }
    if ({
        let mut _g_boolean_var_61: ::core::ffi::c_int = 0;
        if address.is_null() || (*address).is_null() {
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
            b"address == NULL || *address == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBytes>();
    }
    if ({
        let mut _g_boolean_var_62: ::core::ffi::c_int = 0;
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
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBytes>();
    }
    if ({
        let mut _g_boolean_var_63: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBytes>();
    }
    v.buffer = ({
        let mut __n: gsize = size;
        let mut __s: gsize = ::core::mem::size_of::<guint8>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut guint8 as gpointer;
    v.size = size;
    res = safe_c2rust_g_socket_receive_message_with_timeout(
        socket_0,
        address,
        &raw mut v,
        1 as gint,
        ::core::ptr::null_mut::<*mut *mut GSocketControlMessage>(),
        ::core::ptr::null_mut::<gint>(),
        ::core::ptr::null_mut::<gint>(),
        timeout_us,
        cancellable,
        error,
    );
    if res < 0 as gssize {
        g_free(v.buffer);
        return ::core::ptr::null_mut::<GBytes>();
    }
    if res as gsize == size {
        buf = g_bytes_new_take(
            safe_c2rust_g_steal_pointer(&raw mut v.buffer as gpointer),
            res as gsize,
        );
    } else {
        let mut sub_buf: *mut GBytes = ::core::ptr::null_mut::<GBytes>();
        buf = g_bytes_new_take(
            safe_c2rust_g_steal_pointer(&raw mut v.buffer as gpointer),
            size,
        );
        sub_buf = g_bytes_new_from_bytes(buf, 0 as gsize, res as gsize);
        g_bytes_unref(buf);
        buf =
            safe_c2rust_g_steal_pointer(&raw mut sub_buf as gpointer) as *mut GBytes as *mut GBytes;
    }
    return safe_c2rust_g_steal_pointer(&raw mut buf as gpointer) as *mut GBytes;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_receive_from(
    mut socket_0: *mut GSocket,
    mut address: *mut *mut GSocketAddress,
    mut buffer: *mut gchar,
    mut size: gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    let mut v: GInputVector = _GInputVector {
        buffer: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        size: 0,
    };
    v.buffer = buffer as gpointer;
    v.size = size;
    return safe_c2rust_g_socket_receive_message(
        socket_0,
        address,
        &raw mut v,
        1 as gint,
        ::core::ptr::null_mut::<*mut *mut GSocketControlMessage>(),
        ::core::ptr::null_mut::<gint>(),
        ::core::ptr::null_mut::<gint>(),
        cancellable,
        error,
    );
}
unsafe extern "C" fn safe_c2rust_g_socket_send_with_timeout(
    mut socket_0: *mut GSocket,
    mut buffer: *const guint8,
    mut size: gsize,
    mut timeout_us: gint64,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    let mut ret: gssize = 0;
    let mut start_time: gint64 = 0;
    if ({
        let mut _g_boolean_var_64: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            && !buffer.is_null()
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
            b"G_IS_SOCKET (socket) && buffer != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    start_time = g_get_monotonic_time();
    if safe_c2rust_check_socket(socket_0, error) == 0 {
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    if safe_c2rust_check_timeout(socket_0, error) == 0 {
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    loop {
        ret = send(
            (*(*socket_0).priv_0).fd as ::core::ffi::c_int,
            buffer as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            size as size_t,
            MSG_NOSIGNAL as ::core::ffi::c_int,
        ) as gssize;
        if !(ret < 0 as gssize) {
            break;
        }
        let mut errsv: ::core::ffi::c_int = safe_c2rust_get_socket_errno();
        if errsv == EINTR {
            continue;
        }
        if errsv == EWOULDBLOCK || errsv == EAGAIN {
            if timeout_us != 0 as gint64 {
                if safe_c2rust_block_on_timeout(
                    socket_0,
                    G_IO_OUT,
                    timeout_us,
                    start_time,
                    cancellable,
                    error,
                ) == 0
                {
                    return -(1 as ::core::ffi::c_int) as gssize;
                }
                continue;
            }
        }
        let mut __err: *mut *mut GError = error;
        let mut __errsv: ::core::ffi::c_int = errsv;
        if !__err.is_null() {
            let mut __code: ::core::ffi::c_int =
                safe_c2rust_socket_io_error_from_errno(__errsv) as ::core::ffi::c_int;
            let mut __strerr: *const ::core::ffi::c_char = safe_c2rust_socket_strerror(__errsv);
            if __code == G_IO_ERROR_WOULD_BLOCK as ::core::ffi::c_int {
                g_set_error_literal(
                    __err,
                    g_io_error_quark(),
                    __code as gint,
                    __strerr as *const gchar,
                );
            } else {
                g_set_error(
                    __err,
                    g_io_error_quark(),
                    __code as gint,
                    glib_gettext(b"Error sending data: %s\0" as *const u8 as *const gchar),
                    __strerr,
                );
            }
        }
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_send(
    mut socket_0: *mut GSocket,
    mut buffer: *const gchar,
    mut size: gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    return safe_c2rust_g_socket_send_with_blocking(
        socket_0,
        buffer,
        size,
        (*(*socket_0).priv_0).blocking() as gboolean,
        cancellable,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_send_with_blocking(
    mut socket_0: *mut GSocket,
    mut buffer: *const gchar,
    mut size: gsize,
    mut blocking: gboolean,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    return safe_c2rust_g_socket_send_with_timeout(
        socket_0,
        buffer as *const guint8,
        size,
        (if blocking != 0 {
            -(1 as ::core::ffi::c_int)
        } else {
            0 as ::core::ffi::c_int
        }) as gint64,
        cancellable,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_send_to(
    mut socket_0: *mut GSocket,
    mut address: *mut GSocketAddress,
    mut buffer: *const gchar,
    mut size: gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    let mut v: GOutputVector = _GOutputVector {
        buffer: ::core::ptr::null::<::core::ffi::c_void>(),
        size: 0,
    };
    v.buffer = buffer as gconstpointer;
    v.size = size;
    return safe_c2rust_g_socket_send_message(
        socket_0,
        address,
        &raw mut v,
        1 as gint,
        ::core::ptr::null_mut::<*mut GSocketControlMessage>(),
        0 as gint,
        0 as gint,
        cancellable,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_shutdown(
    mut socket_0: *mut GSocket,
    mut shutdown_read: gboolean,
    mut shutdown_write: gboolean,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut how: ::core::ffi::c_int = 0;
    if ({
        let mut _g_boolean_var_65: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    if safe_c2rust_check_socket(socket_0, error) == 0 {
        return FALSE;
    }
    if shutdown_read == 0 && shutdown_write == 0 {
        return TRUE;
    }
    if shutdown_read != 0 && shutdown_write != 0 {
        how = SHUT_RDWR as ::core::ffi::c_int;
    } else if shutdown_read != 0 {
        how = SHUT_RD as ::core::ffi::c_int;
    } else {
        how = SHUT_WR as ::core::ffi::c_int;
    }
    if shutdown((*(*socket_0).priv_0).fd as ::core::ffi::c_int, how) != 0 as ::core::ffi::c_int {
        let mut errsv: ::core::ffi::c_int = safe_c2rust_get_socket_errno();
        g_set_error(
            error,
            g_io_error_quark(),
            safe_c2rust_socket_io_error_from_errno(errsv) as gint,
            glib_gettext(b"Unable to shutdown socket: %s\0" as *const u8 as *const gchar),
            safe_c2rust_socket_strerror(errsv),
        );
        return FALSE;
    }
    if shutdown_read != 0 {
        (*(*socket_0).priv_0).set_connected_read(FALSE as guint as guint);
    }
    if shutdown_write != 0 {
        (*(*socket_0).priv_0).set_connected_write(FALSE as guint as guint);
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_close(
    mut socket_0: *mut GSocket,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut res: ::core::ffi::c_int = 0;
    if ({
        let mut _g_boolean_var_66: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    if (*(*socket_0).priv_0).closed() != 0 {
        return TRUE;
    }
    if safe_c2rust_check_socket(socket_0, error) == 0 {
        return FALSE;
    }
    loop {
        res = close((*(*socket_0).priv_0).fd as ::core::ffi::c_int);
        if !(res == -(1 as ::core::ffi::c_int)) {
            break;
        }
        let mut errsv: ::core::ffi::c_int = safe_c2rust_get_socket_errno();
        if errsv == EINTR {
            continue;
        }
        g_set_error(
            error,
            g_io_error_quark(),
            safe_c2rust_socket_io_error_from_errno(errsv) as gint,
            glib_gettext(b"Error closing socket: %s\0" as *const u8 as *const gchar),
            safe_c2rust_socket_strerror(errsv),
        );
        return FALSE;
    }
    (*(*socket_0).priv_0).fd = -(1 as ::core::ffi::c_int) as gint;
    (*(*socket_0).priv_0).set_connected_read(FALSE as guint as guint);
    (*(*socket_0).priv_0).set_connected_write(FALSE as guint as guint);
    (*(*socket_0).priv_0).set_closed(TRUE as guint as guint);
    if !(*(*socket_0).priv_0).remote_address.is_null() {
        g_object_unref((*(*socket_0).priv_0).remote_address as gpointer);
        (*(*socket_0).priv_0).remote_address = ::core::ptr::null_mut::<GSocketAddress>();
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_is_closed(mut socket_0: *mut GSocket) -> gboolean {
    return (*(*socket_0).priv_0).closed() as gboolean;
}
unsafe extern "C" fn safe_c2rust_broken_dispatch(
    mut source: *mut GSource,
    mut callback: GSourceFunc,
    mut user_data: gpointer,
) -> gboolean {
    return TRUE;
}
static mut safe_c2rust_broken_funcs: GSourceFuncs = unsafe {
    _GSourceFuncs {
        prepare: None,
        check: None,
        dispatch: Some(
            safe_c2rust_broken_dispatch
                as unsafe extern "C" fn(*mut GSource, GSourceFunc, gpointer) -> gboolean,
        ),
        finalize: None,
        closure_callback: None,
        closure_marshal: None,
    }
};
unsafe extern "C" fn safe_c2rust_socket_source_prepare(
    mut source: *mut GSource,
    mut timeout: *mut gint,
) -> gboolean {
    let mut socket_source: *mut GSocketSource = source as *mut GSocketSource;
    return (safe_c2rust_g_socket_is_closed((*socket_source).socket) != 0
        && !(*socket_source).fd_tag.is_null()) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_socket_source_dispatch(
    mut source: *mut GSource,
    mut callback: GSourceFunc,
    mut user_data: gpointer,
) -> gboolean {
    let mut func: GSocketSourceFunc =
        ::core::mem::transmute::<GSourceFunc, GSocketSourceFunc>(callback);
    let mut socket_source: *mut GSocketSource = source as *mut GSocketSource;
    let mut socket_0: *mut GSocket = (*socket_source).socket;
    let mut timeout: gint64 = 0;
    let mut events: guint = 0;
    let mut ret: gboolean = 0;
    if safe_c2rust_g_socket_is_closed((*socket_source).socket) != 0 {
        if !(*socket_source).fd_tag.is_null() {
            g_source_remove_unix_fd(source, (*socket_source).fd_tag);
        }
        (*socket_source).fd_tag = ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
        events = G_IO_NVAL as ::core::ffi::c_int as guint;
    } else {
        events = g_source_query_unix_fd(source, (*socket_source).fd_tag) as guint;
    }
    timeout = g_source_get_ready_time(source);
    if timeout >= 0 as gint64
        && timeout <= g_source_get_time(source)
        && safe_c2rust_g_socket_is_closed((*socket_source).socket) == 0
    {
        (*(*socket_0).priv_0).set_timed_out(TRUE as guint as guint);
        events |= (G_IO_IN as ::core::ffi::c_int | G_IO_OUT as ::core::ffi::c_int) as guint;
    }
    ret = Some(func.expect("non-null function pointer")).expect("non-null function pointer")(
        socket_0,
        (events as ::core::ffi::c_uint & (*socket_source).condition as ::core::ffi::c_uint)
            as GIOCondition,
        user_data,
    );
    if (*(*socket_0).priv_0).timeout != 0
        && safe_c2rust_g_socket_is_closed((*socket_source).socket) == 0
    {
        g_source_set_ready_time(
            source,
            g_get_monotonic_time()
                + (*(*socket_0).priv_0)
                    .timeout
                    .wrapping_mul(1000000 as ::core::ffi::c_int as guint)
                    as gint64,
        );
    } else {
        g_source_set_ready_time(source, -(1 as ::core::ffi::c_int) as gint64);
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust_socket_source_finalize(mut source: *mut GSource) {
    let mut socket_source: *mut GSocketSource = source as *mut GSocketSource;
    let mut socket_0: *mut GSocket = ::core::ptr::null_mut::<GSocket>();
    socket_0 = (*socket_source).socket;
    g_object_unref(socket_0 as gpointer);
}
unsafe extern "C" fn safe_c2rust_socket_source_closure_callback(
    mut socket_0: *mut GSocket,
    mut condition: GIOCondition,
    mut data: gpointer,
) -> gboolean {
    let mut closure: *mut GClosure = data as *mut GClosure;
    let mut params: [GValue; 2] = [
        _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed { v_int: 0 },
            ],
        },
        _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed { v_int: 0 },
            ],
        },
    ];
    let mut result_value: GValue = _GValue {
        g_type: 0 as GType,
        data: [
            C2RustUnnamed {
                v_int: 0 as ::core::ffi::c_int,
            },
            C2RustUnnamed { v_int: 0 },
        ],
    };
    let mut result: gboolean = 0;
    g_value_init(&raw mut result_value, G_TYPE_BOOLEAN);
    g_value_init(
        (&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        safe_c2rust_g_socket_get_type(),
    );
    g_value_set_object(
        (&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        socket_0 as gpointer,
    );
    g_value_init(
        (&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize) as *mut GValue,
        g_io_condition_get_type(),
    );
    g_value_set_flags(
        (&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize) as *mut GValue,
        condition as guint,
    );
    g_closure_invoke(
        closure,
        &raw mut result_value,
        2 as guint,
        &raw mut params as *mut GValue,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    result = g_value_get_boolean(&raw mut result_value);
    g_value_unset(&raw mut result_value);
    g_value_unset(
        (&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
    );
    g_value_unset(
        (&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize) as *mut GValue,
    );
    return result;
}
static mut safe_c2rust_socket_source_funcs: GSourceFuncs = unsafe {
    _GSourceFuncs {
        prepare: Some(
            safe_c2rust_socket_source_prepare
                as unsafe extern "C" fn(*mut GSource, *mut gint) -> gboolean,
        ),
        check: None,
        dispatch: Some(
            safe_c2rust_socket_source_dispatch
                as unsafe extern "C" fn(*mut GSource, GSourceFunc, gpointer) -> gboolean,
        ),
        finalize: Some(
            safe_c2rust_socket_source_finalize as unsafe extern "C" fn(*mut GSource) -> (),
        ),
        closure_callback: ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GSocket, GIOCondition, gpointer) -> gboolean>,
            GSourceFunc,
        >(Some(
            safe_c2rust_socket_source_closure_callback
                as unsafe extern "C" fn(*mut GSocket, GIOCondition, gpointer) -> gboolean,
        )),
        closure_marshal: None,
    }
};
unsafe extern "C" fn safe_c2rust_socket_source_new(
    mut socket_0: *mut GSocket,
    mut condition: GIOCondition,
    mut cancellable: *mut GCancellable,
) -> *mut GSource {
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    let mut socket_source: *mut GSocketSource = ::core::ptr::null_mut::<GSocketSource>();
    if safe_c2rust_check_socket(socket_0, ::core::ptr::null_mut::<*mut GError>()) == 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Socket check failed\0" as *const u8 as *const gchar,
        );
        return g_source_new(
            &raw mut safe_c2rust_broken_funcs,
            ::core::mem::size_of::<GSource>() as guint,
        );
    }
    condition = ::core::mem::transmute::<::core::ffi::c_uint, GIOCondition>(
        condition as ::core::ffi::c_uint
            | (G_IO_HUP as ::core::ffi::c_int
                | G_IO_ERR as ::core::ffi::c_int
                | G_IO_NVAL as ::core::ffi::c_int) as ::core::ffi::c_uint,
    );
    source = g_source_new(
        &raw mut safe_c2rust_socket_source_funcs,
        ::core::mem::size_of::<GSocketSource>() as guint,
    );
    g_source_set_static_name(
        source,
        b"GSocket\0" as *const u8 as *const ::core::ffi::c_char,
    );
    socket_source = source as *mut GSocketSource;
    (*socket_source).socket = g_object_ref(socket_0 as gpointer) as *mut GSocket as *mut GSocket;
    (*socket_source).condition = condition;
    if !cancellable.is_null() {
        let mut cancellable_source: *mut GSource = ::core::ptr::null_mut::<GSource>();
        cancellable_source = g_cancellable_source_new(cancellable);
        g_source_add_child_source(source, cancellable_source);
        g_source_set_dummy_callback(cancellable_source);
        g_source_unref(cancellable_source);
    }
    (*socket_source).fd_tag = g_source_add_unix_fd(source, (*(*socket_0).priv_0).fd, condition);
    if (*(*socket_0).priv_0).timeout != 0 {
        g_source_set_ready_time(
            source,
            g_get_monotonic_time()
                + (*(*socket_0).priv_0)
                    .timeout
                    .wrapping_mul(1000000 as ::core::ffi::c_int as guint)
                    as gint64,
        );
    } else {
        g_source_set_ready_time(source, -(1 as ::core::ffi::c_int) as gint64);
    }
    return source;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_create_source(
    mut socket_0: *mut GSocket,
    mut condition: GIOCondition,
    mut cancellable: *mut GCancellable,
) -> *mut GSource {
    if ({
        let mut _g_boolean_var_67: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            && (cancellable.is_null()
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
                }) != 0)
        {
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
            b"G_IS_SOCKET (socket) && (cancellable == NULL || G_IS_CANCELLABLE (cancellable))\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSource>();
    }
    return safe_c2rust_socket_source_new(socket_0, condition, cancellable);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_condition_check(
    mut socket_0: *mut GSocket,
    mut condition: GIOCondition,
) -> GIOCondition {
    if ({
        let mut _g_boolean_var_68: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GIOCondition;
    }
    if safe_c2rust_check_socket(socket_0, ::core::ptr::null_mut::<*mut GError>()) == 0 {
        return 0 as GIOCondition;
    }
    let mut poll_fd: GPollFD = _GPollFD {
        fd: 0,
        events: 0,
        revents: 0,
    };
    let mut result: gint = 0;
    poll_fd.fd = (*(*socket_0).priv_0).fd;
    poll_fd.events = condition as gushort;
    poll_fd.revents = 0 as gushort;
    loop {
        result = g_poll(&raw mut poll_fd, 1 as guint, 0 as gint);
        if !(result == -(1 as ::core::ffi::c_int) && safe_c2rust_get_socket_errno() == EINTR) {
            break;
        }
    }
    return poll_fd.revents as GIOCondition;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_condition_wait(
    mut socket_0: *mut GSocket,
    mut condition: GIOCondition,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_69: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return safe_c2rust_g_socket_condition_timed_wait(
        socket_0,
        condition,
        -(1 as ::core::ffi::c_int) as gint64,
        cancellable,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_condition_timed_wait(
    mut socket_0: *mut GSocket,
    mut condition: GIOCondition,
    mut timeout_us: gint64,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut start_time: gint64 = 0;
    let mut timeout_ms: gint64 = 0;
    if ({
        let mut _g_boolean_var_70: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if safe_c2rust_check_socket(socket_0, error) == 0 {
        return FALSE;
    }
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return FALSE;
    }
    if (*(*socket_0).priv_0).timeout != 0
        && (timeout_us < 0 as gint64
            || ((*(*socket_0).priv_0).timeout as gint64) < timeout_us / G_USEC_PER_SEC as gint64)
    {
        timeout_ms = (*(*socket_0).priv_0).timeout as gint64 * 1000 as gint64;
    } else if timeout_us != -(1 as ::core::ffi::c_int) as gint64 {
        timeout_ms = timeout_us / 1000 as gint64;
    } else {
        timeout_ms = -(1 as ::core::ffi::c_int) as gint64;
    }
    start_time = g_get_monotonic_time();
    let mut poll_fd: [GPollFD; 2] = [_GPollFD {
        fd: 0,
        events: 0,
        revents: 0,
    }; 2];
    let mut result: gint = 0;
    let mut num: gint = 0;
    poll_fd[0 as ::core::ffi::c_int as usize].fd = (*(*socket_0).priv_0).fd;
    poll_fd[0 as ::core::ffi::c_int as usize].events = condition as gushort;
    num = 1 as ::core::ffi::c_int as gint;
    if g_cancellable_make_pollfd(
        cancellable,
        (&raw mut poll_fd as *mut GPollFD).offset(1 as ::core::ffi::c_int as isize) as *mut GPollFD,
    ) != 0
    {
        num += 1;
    }
    while FALSE == 0 {
        let mut errsv: ::core::ffi::c_int = 0;
        result = g_poll(
            &raw mut poll_fd as *mut GPollFD,
            num as guint,
            timeout_ms as gint,
        );
        errsv = *__errno_location();
        if result != -(1 as ::core::ffi::c_int) || errsv != EINTR {
            break;
        }
        if timeout_ms != -(1 as ::core::ffi::c_int) as gint64 {
            timeout_ms -= (g_get_monotonic_time() - start_time) / 1000 as gint64;
            if timeout_ms < 0 as gint64 {
                timeout_ms = 0 as gint64;
            }
        }
    }
    if num > 1 as ::core::ffi::c_int {
        g_cancellable_release_fd(cancellable);
    }
    if result == 0 as ::core::ffi::c_int {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_TIMED_OUT as ::core::ffi::c_int as gint,
            glib_gettext(b"Socket I/O timed out\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    return (g_cancellable_set_error_if_cancelled(cancellable, error) == 0) as ::core::ffi::c_int;
}
pub const G_SOCKET_CONTROL_BUFFER_SIZE_BYTES: ::core::ffi::c_int = 2048 as ::core::ffi::c_int;
unsafe extern "C" fn safe_c2rust_input_message_from_msghdr(
    mut msg: *const msghdr,
    mut message: *mut GInputMessage,
    mut socket_0: *mut GSocket,
) {
    if !(*message).address.is_null() {
        *(*message).address = safe_c2rust_cache_recv_address(
            socket_0,
            (*msg).msg_name as *mut sockaddr,
            (*msg).msg_namelen as size_t,
        );
    }
    let mut my_messages: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    let mut cmsg: *mut cmsghdr = ::core::ptr::null_mut::<cmsghdr>();
    if (*msg).msg_controllen >= ::core::mem::size_of::<cmsghdr>() as socklen_t as size_t {
        if ({
            let mut _g_boolean_var_71: ::core::ffi::c_int = 0;
            if !(*message).control_messages.is_null() {
                _g_boolean_var_71 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_71 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_71
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocket.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                4914 as ::core::ffi::c_int,
                G_STRFUNC,
                b"message->control_messages != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        cmsg = if (*msg).msg_controllen >= ::core::mem::size_of::<cmsghdr>() as usize {
            (*msg).msg_control as *mut cmsghdr
        } else {
            ::core::ptr::null_mut::<cmsghdr>()
        };
        while !cmsg.is_null() {
            let mut control_message: *mut GSocketControlMessage =
                ::core::ptr::null_mut::<GSocketControlMessage>();
            control_message = g_socket_control_message_deserialize(
                (*cmsg).cmsg_level,
                (*cmsg).cmsg_type,
                ((*cmsg).cmsg_len as gsize).wrapping_sub(
                    (&raw mut (*cmsg).__cmsg_data as *mut ::core::ffi::c_uchar
                        as *mut ::core::ffi::c_char)
                        .offset_from(cmsg as *mut ::core::ffi::c_char)
                        as ::core::ffi::c_long as gsize,
                ),
                &raw mut (*cmsg).__cmsg_data as *mut ::core::ffi::c_uchar as gpointer,
            );
            if !control_message.is_null() {
                if my_messages.is_null() {
                    my_messages = g_ptr_array_new();
                }
                g_ptr_array_add(my_messages, control_message as gpointer);
            }
            cmsg = safe_c2rust___cmsg_nxthdr(msg as *mut msghdr, cmsg);
        }
    }
    if !(*message).num_control_messages.is_null() {
        *(*message).num_control_messages = if !my_messages.is_null() {
            (*my_messages).len
        } else {
            0 as guint
        };
    }
    if !(*message).control_messages.is_null() {
        if my_messages.is_null() {
            *(*message).control_messages = ::core::ptr::null_mut::<*mut GSocketControlMessage>();
        } else {
            g_ptr_array_add(my_messages, ::core::ptr::null_mut::<::core::ffi::c_void>());
            *(*message).control_messages =
                g_ptr_array_free(my_messages, FALSE) as *mut *mut GSocketControlMessage;
        }
    } else if ({
        let mut _g_boolean_var_72: ::core::ffi::c_int = 0;
        if my_messages.is_null() {
            _g_boolean_var_72 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_72 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_72
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocket.c\0" as *const u8
                as *const ::core::ffi::c_char,
            4953 as ::core::ffi::c_int,
            G_STRFUNC,
            b"my_messages == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*message).flags = (*msg).msg_flags as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_send_message(
    mut socket_0: *mut GSocket,
    mut address: *mut GSocketAddress,
    mut vectors: *mut GOutputVector,
    mut num_vectors: gint,
    mut messages: *mut *mut GSocketControlMessage,
    mut num_messages: gint,
    mut flags: gint,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    let mut res: GPollableReturn = G_POLLABLE_RETURN_FAILED;
    let mut bytes_written: gsize = 0 as gsize;
    let mut vectors_size: gsize = 0 as gsize;
    if num_vectors != -(1 as ::core::ffi::c_int) {
        let mut i: gint = 0 as gint;
        while i < num_vectors {
            if vectors_size > vectors_size.wrapping_add((*vectors.offset(i as isize)).size) {
                g_set_error(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                    glib_gettext(b"Unable to send message: %s\0" as *const u8 as *const gchar),
                    glib_gettext(b"Message vectors too large\0" as *const u8 as *const gchar),
                );
                return -(1 as ::core::ffi::c_int) as gssize;
            }
            vectors_size = vectors_size.wrapping_add((*vectors.offset(i as isize)).size);
            i += 1;
        }
    } else {
        let mut i_0: gsize = 0 as gsize;
        while !(*vectors.offset(i_0 as isize)).buffer.is_null() {
            if vectors_size > vectors_size.wrapping_add((*vectors.offset(i_0 as isize)).size) {
                g_set_error(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                    glib_gettext(b"Unable to send message: %s\0" as *const u8 as *const gchar),
                    glib_gettext(b"Message vectors too large\0" as *const u8 as *const gchar),
                );
                return -(1 as ::core::ffi::c_int) as gssize;
            }
            vectors_size = vectors_size.wrapping_add((*vectors.offset(i_0 as isize)).size);
            i_0 = i_0.wrapping_add(1);
        }
    }
    if vectors_size > G_MAXSSIZE as gsize {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(b"Unable to send message: %s\0" as *const u8 as *const gchar),
            glib_gettext(b"Message vectors too large\0" as *const u8 as *const gchar),
        );
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    res = safe_c2rust_g_socket_send_message_with_timeout(
        socket_0,
        address,
        vectors,
        num_vectors,
        messages,
        num_messages,
        flags,
        (if (*(*socket_0).priv_0).blocking() as ::core::ffi::c_int != 0 {
            -(1 as ::core::ffi::c_int)
        } else {
            0 as ::core::ffi::c_int
        }) as gint64,
        &raw mut bytes_written,
        cancellable,
        error,
    );
    if ({
        let mut _g_boolean_var_73: ::core::ffi::c_int = 0;
        if res as ::core::ffi::c_int != G_POLLABLE_RETURN_OK as ::core::ffi::c_int
            || bytes_written <= 9223372036854775807 as ::core::ffi::c_long as gsize
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
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocket.c\0" as *const u8
                as *const ::core::ffi::c_char,
            5088 as ::core::ffi::c_int,
            G_STRFUNC,
            b"res != G_POLLABLE_RETURN_OK || bytes_written <= G_MAXSSIZE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if res as ::core::ffi::c_int == G_POLLABLE_RETURN_WOULD_BLOCK as ::core::ffi::c_int {
        let mut __err: *mut *mut GError = error;
        let mut __errsv: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
        if !__err.is_null() {
            let mut __code: ::core::ffi::c_int =
                safe_c2rust_socket_io_error_from_errno(__errsv) as ::core::ffi::c_int;
            let mut __strerr: *const ::core::ffi::c_char = safe_c2rust_socket_strerror(__errsv);
            if __code == G_IO_ERROR_WOULD_BLOCK as ::core::ffi::c_int {
                g_set_error_literal(
                    __err,
                    g_io_error_quark(),
                    __code as gint,
                    __strerr as *const gchar,
                );
            } else {
                g_set_error(
                    __err,
                    g_io_error_quark(),
                    __code as gint,
                    glib_gettext(b"Error sending message: %s\0" as *const u8 as *const gchar),
                    __strerr,
                );
            }
        }
    }
    return if res as ::core::ffi::c_int == G_POLLABLE_RETURN_OK as ::core::ffi::c_int {
        bytes_written as gssize
    } else {
        -(1 as ::core::ffi::c_int) as gssize
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_send_message_with_timeout(
    mut socket_0: *mut GSocket,
    mut address: *mut GSocketAddress,
    mut vectors: *const GOutputVector,
    mut num_vectors: gint,
    mut messages: *mut *mut GSocketControlMessage,
    mut num_messages: gint,
    mut flags: gint,
    mut timeout_us: gint64,
    mut bytes_written: *mut gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> GPollableReturn {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut one_vector: GOutputVector = _GOutputVector {
        buffer: ::core::ptr::null::<::core::ffi::c_void>(),
        size: 0,
    };
    let mut zero: ::core::ffi::c_char = 0;
    let mut start_time: gint64 = 0;
    if !bytes_written.is_null() {
        *bytes_written = 0 as gsize;
    }
    if ({
        let mut _g_boolean_var_74: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_POLLABLE_RETURN_FAILED;
    }
    if ({
        let mut _g_boolean_var_75: ::core::ffi::c_int = 0;
        if address.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = address as *mut GTypeInstance;
                let mut __t: GType = g_socket_address_get_type();
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
            b"address == NULL || G_IS_SOCKET_ADDRESS (address)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return G_POLLABLE_RETURN_FAILED;
    }
    if ({
        let mut _g_boolean_var_76: ::core::ffi::c_int = 0;
        if num_vectors == 0 as ::core::ffi::c_int || !vectors.is_null() {
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
            b"num_vectors == 0 || vectors != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_POLLABLE_RETURN_FAILED;
    }
    if ({
        let mut _g_boolean_var_77: ::core::ffi::c_int = 0;
        if num_messages == 0 as ::core::ffi::c_int || !messages.is_null() {
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
            b"num_messages == 0 || messages != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_POLLABLE_RETURN_FAILED;
    }
    if ({
        let mut _g_boolean_var_78: ::core::ffi::c_int = 0;
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
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return G_POLLABLE_RETURN_FAILED;
    }
    if ({
        let mut _g_boolean_var_79: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_POLLABLE_RETURN_FAILED;
    }
    start_time = g_get_monotonic_time();
    if safe_c2rust_check_socket(socket_0, error) == 0 {
        return G_POLLABLE_RETURN_FAILED;
    }
    if safe_c2rust_check_timeout(socket_0, error) == 0 {
        return G_POLLABLE_RETURN_FAILED;
    }
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return G_POLLABLE_RETURN_FAILED;
    }
    if num_vectors == -(1 as ::core::ffi::c_int) {
        num_vectors = 0 as ::core::ffi::c_int as gint;
        while !(*vectors.offset(num_vectors as isize)).buffer.is_null() {
            num_vectors += 1;
        }
    }
    if num_messages == -(1 as ::core::ffi::c_int) {
        num_messages = 0 as ::core::ffi::c_int as gint;
        while !messages.is_null() && !(*messages.offset(num_messages as isize)).is_null() {
            num_messages += 1;
        }
    }
    if num_vectors == 0 as ::core::ffi::c_int {
        zero = '\0' as i32 as ::core::ffi::c_char;
        one_vector.buffer = &raw mut zero as gconstpointer;
        one_vector.size = 1 as gsize;
        num_vectors = 1 as ::core::ffi::c_int as gint;
        vectors = &raw mut one_vector;
    }
    let mut output_message: GOutputMessage = _GOutputMessage {
        address: ::core::ptr::null_mut::<GSocketAddress>(),
        vectors: ::core::ptr::null_mut::<GOutputVector>(),
        num_vectors: 0,
        bytes_sent: 0,
        control_messages: ::core::ptr::null_mut::<*mut GSocketControlMessage>(),
        num_control_messages: 0,
    };
    let mut msg: msghdr = msghdr {
        msg_name: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        msg_namelen: 0,
        msg_iov: ::core::ptr::null_mut::<iovec>(),
        msg_iovlen: 0,
        msg_control: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        msg_controllen: 0,
        msg_flags: 0,
    };
    let mut result: gssize = 0;
    let mut child_error: *mut GError = ::core::ptr::null_mut::<GError>();
    output_message.address = address;
    output_message.vectors = vectors as *mut GOutputVector;
    output_message.num_vectors = num_vectors as guint;
    output_message.bytes_sent = 0 as guint;
    output_message.control_messages = messages;
    output_message.num_control_messages = num_messages as guint;
    let mut current_block_118: u64;
    let mut _message: *const GOutputMessage = &raw mut output_message;
    let mut _prev_message: *const GOutputMessage = ::core::ptr::null::<GOutputMessage>();
    let mut _msg: *mut msghdr = &raw mut msg;
    let mut _prev_msg: *const msghdr = ::core::ptr::null::<msghdr>();
    let mut _error: *mut *mut GError = &raw mut child_error;
    (*_msg).msg_flags = 0 as ::core::ffi::c_int;
    if !_prev_message.is_null() && (*_prev_message).address == (*_message).address {
        (*_msg).msg_name = (*_prev_msg).msg_name;
        (*_msg).msg_namelen = (*_prev_msg).msg_namelen;
        current_block_118 = 13910774313357589740;
    } else if !(*_message).address.is_null() {
        (*_msg).msg_namelen = g_socket_address_get_native_size((*_message).address) as socklen_t;
        alloca_allocations.push(::std::vec::from_elem(
            0,
            (*_msg).msg_namelen as ::core::ffi::c_ulong as usize,
        ));
        (*_msg).msg_name = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_void;
        if g_socket_address_to_native(
            (*_message).address,
            (*_msg).msg_name as gpointer,
            (*_msg).msg_namelen as gsize,
            _error,
        ) == 0
        {
            current_block_118 = 3217137713928741134;
        } else {
            current_block_118 = 13910774313357589740;
        }
    } else {
        (*_msg).msg_name = ::core::ptr::null_mut::<::core::ffi::c_void>();
        (*_msg).msg_namelen = 0 as socklen_t;
        current_block_118 = 13910774313357589740;
    }
    match current_block_118 {
        13910774313357589740 => {
            if ::core::mem::size_of::<iovec>() as usize
                == ::core::mem::size_of::<GOutputVector>() as usize
                && ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize
                    == ::core::mem::size_of::<gconstpointer>() as usize
                && 0 as ::core::ffi::c_ulong as glong == 0 as ::core::ffi::c_ulong as glong
                && ::core::mem::size_of::<size_t>() as usize
                    == ::core::mem::size_of::<gsize>() as usize
                && 8 as ::core::ffi::c_ulong as glong == 8 as ::core::ffi::c_ulong as glong
            {
                (*_msg).msg_iov = (*_message).vectors as *mut iovec;
                (*_msg).msg_iovlen = (*_message).num_vectors as size_t;
            } else {
                let mut i: guint = 0;
                alloca_allocations.push(::std::vec::from_elem(
                    0,
                    (::core::mem::size_of::<iovec>() as usize)
                        .wrapping_mul((*_message).num_vectors as usize)
                        as usize,
                ));
                (*_msg).msg_iov = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_void as *mut iovec;
                i = 0 as guint;
                while i < (*_message).num_vectors {
                    let ref mut fresh7 = (*(*_msg).msg_iov.offset(i as isize)).iov_base;
                    *fresh7 = (*(*_message).vectors.offset(i as isize)).buffer
                        as *mut ::core::ffi::c_void;
                    (*(*_msg).msg_iov.offset(i as isize)).iov_len =
                        (*(*_message).vectors.offset(i as isize)).size as size_t;
                    i = i.wrapping_add(1);
                }
                (*_msg).msg_iovlen = (*_message).num_vectors as size_t;
            }
            let mut cmsg: *mut cmsghdr = ::core::ptr::null_mut::<cmsghdr>();
            let mut i_0: guint = 0;
            (*_msg).msg_controllen = 0 as size_t;
            i_0 = 0 as guint;
            while i_0 < (*_message).num_control_messages {
                (*_msg).msg_controllen = ((*_msg).msg_controllen as ::core::ffi::c_ulong)
                    .wrapping_add(
                        ((g_socket_control_message_get_size(
                            *(*_message).control_messages.offset(i_0 as isize),
                        ) as usize)
                            .wrapping_add(::core::mem::size_of::<size_t>() as usize)
                            .wrapping_sub(1 as usize)
                            & !(::core::mem::size_of::<size_t>() as usize).wrapping_sub(1 as usize))
                        .wrapping_add(
                            (::core::mem::size_of::<cmsghdr>() as usize)
                                .wrapping_add(::core::mem::size_of::<size_t>() as usize)
                                .wrapping_sub(1 as usize)
                                & !(::core::mem::size_of::<size_t>() as usize)
                                    .wrapping_sub(1 as usize),
                        ) as ::core::ffi::c_ulong,
                    ) as size_t as size_t;
                i_0 = i_0.wrapping_add(1);
            }
            if (*_msg).msg_controllen == 0 as size_t {
                (*_msg).msg_control = ::core::ptr::null_mut::<::core::ffi::c_void>();
            } else {
                (*_msg).msg_control = if (*_msg).msg_controllen == 0 as size_t {
                    ::core::ptr::null_mut::<::core::ffi::c_void>()
                } else {
                    alloca_allocations
                        .push(::std::vec::from_elem(0, (*_msg).msg_controllen as usize));
                    memset(
                        alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        (*_msg).msg_controllen,
                    )
                };
            }
            cmsg = if (*_msg).msg_controllen >= ::core::mem::size_of::<cmsghdr>() as usize {
                (*_msg).msg_control as *mut cmsghdr
            } else {
                ::core::ptr::null_mut::<cmsghdr>()
            };
            i_0 = 0 as guint;
            while i_0 < (*_message).num_control_messages {
                (*cmsg).cmsg_level = g_socket_control_message_get_level(
                    *(*_message).control_messages.offset(i_0 as isize),
                );
                (*cmsg).cmsg_type = g_socket_control_message_get_msg_type(
                    *(*_message).control_messages.offset(i_0 as isize),
                );
                (*cmsg).cmsg_len = ((::core::mem::size_of::<cmsghdr>() as usize)
                    .wrapping_add(::core::mem::size_of::<size_t>() as usize)
                    .wrapping_sub(1 as usize)
                    & !(::core::mem::size_of::<size_t>() as usize).wrapping_sub(1 as usize))
                .wrapping_add(g_socket_control_message_get_size(
                    *(*_message).control_messages.offset(i_0 as isize),
                ) as usize) as size_t;
                g_socket_control_message_serialize(
                    *(*_message).control_messages.offset(i_0 as isize),
                    &raw mut (*cmsg).__cmsg_data as *mut ::core::ffi::c_uchar as gpointer,
                );
                cmsg = safe_c2rust___cmsg_nxthdr(_msg, cmsg);
                i_0 = i_0.wrapping_add(1);
            }
            if ({
                let mut _g_boolean_var_80: ::core::ffi::c_int = 0;
                if cmsg.is_null() {
                    _g_boolean_var_80 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_80 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_80
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocket.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    5210 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"cmsg == NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
        }
        _ => {}
    }
    if !child_error.is_null() {
        g_propagate_error(error, child_error);
        return G_POLLABLE_RETURN_FAILED;
    }
    loop {
        result = sendmsg(
            (*(*socket_0).priv_0).fd as ::core::ffi::c_int,
            &raw mut msg,
            flags as ::core::ffi::c_int | MSG_NOSIGNAL as ::core::ffi::c_int,
        ) as gssize;
        if !(result < 0 as gssize) {
            break;
        }
        let mut errsv: ::core::ffi::c_int = safe_c2rust_get_socket_errno();
        if errsv == EINTR {
            continue;
        }
        if errsv == EWOULDBLOCK || errsv == EAGAIN {
            if timeout_us != 0 as gint64 {
                if safe_c2rust_block_on_timeout(
                    socket_0,
                    G_IO_OUT,
                    timeout_us,
                    start_time,
                    cancellable,
                    error,
                ) == 0
                {
                    return G_POLLABLE_RETURN_FAILED;
                }
            } else {
                return G_POLLABLE_RETURN_WOULD_BLOCK;
            }
        } else {
            let mut __err: *mut *mut GError = error;
            let mut __errsv: ::core::ffi::c_int = errsv;
            if !__err.is_null() {
                let mut __code: ::core::ffi::c_int =
                    safe_c2rust_socket_io_error_from_errno(__errsv) as ::core::ffi::c_int;
                let mut __strerr: *const ::core::ffi::c_char = safe_c2rust_socket_strerror(__errsv);
                if __code == G_IO_ERROR_WOULD_BLOCK as ::core::ffi::c_int {
                    g_set_error_literal(
                        __err,
                        g_io_error_quark(),
                        __code as gint,
                        __strerr as *const gchar,
                    );
                } else {
                    g_set_error(
                        __err,
                        g_io_error_quark(),
                        __code as gint,
                        glib_gettext(b"Error sending message: %s\0" as *const u8 as *const gchar),
                        __strerr,
                    );
                }
            }
            return G_POLLABLE_RETURN_FAILED;
        }
    }
    if !bytes_written.is_null() {
        *bytes_written = result as gsize;
    }
    return G_POLLABLE_RETURN_OK;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_send_messages(
    mut socket_0: *mut GSocket,
    mut messages: *mut GOutputMessage,
    mut num_messages: guint,
    mut flags: gint,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gint {
    return safe_c2rust_g_socket_send_messages_with_timeout(
        socket_0,
        messages,
        num_messages,
        flags,
        (if (*(*socket_0).priv_0).blocking() as ::core::ffi::c_int != 0 {
            -(1 as ::core::ffi::c_int)
        } else {
            0 as ::core::ffi::c_int
        }) as gint64,
        cancellable,
        error,
    );
}
unsafe extern "C" fn safe_c2rust_g_socket_send_messages_with_timeout(
    mut socket_0: *mut GSocket,
    mut messages: *mut GOutputMessage,
    mut num_messages: guint,
    mut flags: gint,
    mut timeout_us: gint64,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gint {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut start_time: gint64 = 0;
    if ({
        let mut _g_boolean_var_81: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    if ({
        let mut _g_boolean_var_82: ::core::ffi::c_int = 0;
        if num_messages == 0 as guint || !messages.is_null() {
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
            b"num_messages == 0 || messages != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    if ({
        let mut _g_boolean_var_83: ::core::ffi::c_int = 0;
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
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    if ({
        let mut _g_boolean_var_84: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    start_time = g_get_monotonic_time();
    if safe_c2rust_check_socket(socket_0, error) == 0 {
        return -(1 as gint);
    }
    if safe_c2rust_check_timeout(socket_0, error) == 0 {
        return -(1 as gint);
    }
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return -(1 as gint);
    }
    if num_messages == 0 as guint {
        return 0 as gint;
    }
    let mut msgvec: *mut mmsghdr = ::core::ptr::null_mut::<mmsghdr>();
    let mut i: guint = 0;
    let mut num_sent: guint = 0;
    if num_messages > G_IOV_MAX as guint {
        num_messages = G_IOV_MAX as guint;
    }
    alloca_allocations.push(::std::vec::from_elem(
        0,
        (::core::mem::size_of::<mmsghdr>() as usize).wrapping_mul(num_messages as usize) as usize,
    ));
    msgvec = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_void as *mut mmsghdr;
    i = 0 as guint;
    while i < num_messages {
        let mut msg: *mut GOutputMessage = messages.offset(i as isize) as *mut GOutputMessage;
        let mut msg_hdr: *mut msghdr = &raw mut (*msgvec.offset(i as isize)).msg_hdr;
        let mut child_error: *mut GError = ::core::ptr::null_mut::<GError>();
        (*msgvec.offset(i as isize)).msg_len = 0 as ::core::ffi::c_uint;
        let mut current_block_86: u64;
        let mut _message: *const GOutputMessage = msg;
        let mut _prev_message: *const GOutputMessage = if i > 0 as guint {
            messages.offset(i.wrapping_sub(1 as guint) as isize) as *mut GOutputMessage
        } else {
            ::core::ptr::null_mut::<GOutputMessage>()
        };
        let mut _msg: *mut msghdr = msg_hdr;
        let mut _prev_msg: *const msghdr = if i > 0 as guint {
            &raw mut (*msgvec.offset(i.wrapping_sub(1 as guint) as isize)).msg_hdr
        } else {
            ::core::ptr::null_mut::<msghdr>()
        };
        let mut _error: *mut *mut GError = &raw mut child_error;
        (*_msg).msg_flags = 0 as ::core::ffi::c_int;
        if !_prev_message.is_null() && (*_prev_message).address == (*_message).address {
            (*_msg).msg_name = (*_prev_msg).msg_name;
            (*_msg).msg_namelen = (*_prev_msg).msg_namelen;
            current_block_86 = 313581471991351815;
        } else if !(*_message).address.is_null() {
            (*_msg).msg_namelen =
                g_socket_address_get_native_size((*_message).address) as socklen_t;
            alloca_allocations.push(::std::vec::from_elem(
                0,
                (*_msg).msg_namelen as ::core::ffi::c_ulong as usize,
            ));
            (*_msg).msg_name = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_void;
            if g_socket_address_to_native(
                (*_message).address,
                (*_msg).msg_name as gpointer,
                (*_msg).msg_namelen as gsize,
                _error,
            ) == 0
            {
                current_block_86 = 15514718523126015390;
            } else {
                current_block_86 = 313581471991351815;
            }
        } else {
            (*_msg).msg_name = ::core::ptr::null_mut::<::core::ffi::c_void>();
            (*_msg).msg_namelen = 0 as socklen_t;
            current_block_86 = 313581471991351815;
        }
        match current_block_86 {
            313581471991351815 => {
                if ::core::mem::size_of::<iovec>() as usize
                    == ::core::mem::size_of::<GOutputVector>() as usize
                    && ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize
                        == ::core::mem::size_of::<gconstpointer>() as usize
                    && 0 as ::core::ffi::c_ulong as glong == 0 as ::core::ffi::c_ulong as glong
                    && ::core::mem::size_of::<size_t>() as usize
                        == ::core::mem::size_of::<gsize>() as usize
                    && 8 as ::core::ffi::c_ulong as glong == 8 as ::core::ffi::c_ulong as glong
                {
                    (*_msg).msg_iov = (*_message).vectors as *mut iovec;
                    (*_msg).msg_iovlen = (*_message).num_vectors as size_t;
                } else {
                    let mut i_0: guint = 0;
                    alloca_allocations.push(::std::vec::from_elem(
                        0,
                        (::core::mem::size_of::<iovec>() as usize)
                            .wrapping_mul((*_message).num_vectors as usize)
                            as usize,
                    ));
                    (*_msg).msg_iov =
                        alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_void as *mut iovec;
                    i_0 = 0 as guint;
                    while i_0 < (*_message).num_vectors {
                        let ref mut fresh1 = (*(*_msg).msg_iov.offset(i_0 as isize)).iov_base;
                        *fresh1 = (*(*_message).vectors.offset(i_0 as isize)).buffer
                            as *mut ::core::ffi::c_void;
                        (*(*_msg).msg_iov.offset(i_0 as isize)).iov_len =
                            (*(*_message).vectors.offset(i_0 as isize)).size as size_t;
                        i_0 = i_0.wrapping_add(1);
                    }
                    (*_msg).msg_iovlen = (*_message).num_vectors as size_t;
                }
                let mut cmsg: *mut cmsghdr = ::core::ptr::null_mut::<cmsghdr>();
                let mut i_1: guint = 0;
                (*_msg).msg_controllen = 0 as size_t;
                i_1 = 0 as guint;
                while i_1 < (*_message).num_control_messages {
                    (*_msg).msg_controllen = ((*_msg).msg_controllen as ::core::ffi::c_ulong)
                        .wrapping_add(
                            ((g_socket_control_message_get_size(
                                *(*_message).control_messages.offset(i_1 as isize),
                            ) as usize)
                                .wrapping_add(::core::mem::size_of::<size_t>() as usize)
                                .wrapping_sub(1 as usize)
                                & !(::core::mem::size_of::<size_t>() as usize)
                                    .wrapping_sub(1 as usize))
                            .wrapping_add(
                                (::core::mem::size_of::<cmsghdr>() as usize)
                                    .wrapping_add(::core::mem::size_of::<size_t>() as usize)
                                    .wrapping_sub(1 as usize)
                                    & !(::core::mem::size_of::<size_t>() as usize)
                                        .wrapping_sub(1 as usize),
                            ) as ::core::ffi::c_ulong,
                        ) as size_t as size_t;
                    i_1 = i_1.wrapping_add(1);
                }
                if (*_msg).msg_controllen == 0 as size_t {
                    (*_msg).msg_control = ::core::ptr::null_mut::<::core::ffi::c_void>();
                } else {
                    (*_msg).msg_control = if (*_msg).msg_controllen == 0 as size_t {
                        ::core::ptr::null_mut::<::core::ffi::c_void>()
                    } else {
                        alloca_allocations
                            .push(::std::vec::from_elem(0, (*_msg).msg_controllen as usize));
                        memset(
                            alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            (*_msg).msg_controllen,
                        )
                    };
                }
                cmsg = if (*_msg).msg_controllen >= ::core::mem::size_of::<cmsghdr>() as usize {
                    (*_msg).msg_control as *mut cmsghdr
                } else {
                    ::core::ptr::null_mut::<cmsghdr>()
                };
                i_1 = 0 as guint;
                while i_1 < (*_message).num_control_messages {
                    (*cmsg).cmsg_level = g_socket_control_message_get_level(
                        *(*_message).control_messages.offset(i_1 as isize),
                    );
                    (*cmsg).cmsg_type = g_socket_control_message_get_msg_type(
                        *(*_message).control_messages.offset(i_1 as isize),
                    );
                    (*cmsg).cmsg_len = ((::core::mem::size_of::<cmsghdr>() as usize)
                        .wrapping_add(::core::mem::size_of::<size_t>() as usize)
                        .wrapping_sub(1 as usize)
                        & !(::core::mem::size_of::<size_t>() as usize).wrapping_sub(1 as usize))
                    .wrapping_add(g_socket_control_message_get_size(
                        *(*_message).control_messages.offset(i_1 as isize),
                    ) as usize) as size_t;
                    g_socket_control_message_serialize(
                        *(*_message).control_messages.offset(i_1 as isize),
                        &raw mut (*cmsg).__cmsg_data as *mut ::core::ffi::c_uchar as gpointer,
                    );
                    cmsg = safe_c2rust___cmsg_nxthdr(_msg, cmsg);
                    i_1 = i_1.wrapping_add(1);
                }
                if ({
                    let mut _g_boolean_var_85: ::core::ffi::c_int = 0;
                    if cmsg.is_null() {
                        _g_boolean_var_85 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_85 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_85
                }) as ::core::ffi::c_long
                    != 0
                {
                } else {
                    g_assertion_message_expr(
                        G_LOG_DOMAIN.as_ptr(),
                        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocket.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        5459 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"cmsg == NULL\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            }
            _ => {}
        }
        if !child_error.is_null() {
            g_propagate_error(error, child_error);
            return -(1 as gint);
        }
        i = i.wrapping_add(1);
    }
    num_sent = 0 as guint;
    while num_sent < num_messages {
        let mut ret: gint = 0;
        ret = sendmmsg(
            (*(*socket_0).priv_0).fd as ::core::ffi::c_int,
            msgvec.offset(num_sent as isize),
            (num_messages as ::core::ffi::c_uint).wrapping_sub(num_sent as ::core::ffi::c_uint),
            flags as ::core::ffi::c_int | MSG_NOSIGNAL as ::core::ffi::c_int,
        ) as gint;
        if ret < 0 as ::core::ffi::c_int {
            let mut errsv: ::core::ffi::c_int = safe_c2rust_get_socket_errno();
            if errsv == EINTR {
                continue;
            }
            if timeout_us != 0 as gint64 && (errsv == EWOULDBLOCK || errsv == EAGAIN) {
                if !(safe_c2rust_block_on_timeout(
                    socket_0,
                    G_IO_OUT,
                    timeout_us,
                    start_time,
                    cancellable,
                    error,
                ) == 0)
                {
                    continue;
                }
                if num_sent > 0 as guint {
                    g_clear_error(error);
                    break;
                } else {
                    return -(1 as gint);
                }
            } else {
                if num_sent > 0 as guint {
                    break;
                }
                let mut __err: *mut *mut GError = error;
                let mut __errsv: ::core::ffi::c_int = errsv;
                if !__err.is_null() {
                    let mut __code: ::core::ffi::c_int =
                        safe_c2rust_socket_io_error_from_errno(__errsv) as ::core::ffi::c_int;
                    let mut __strerr: *const ::core::ffi::c_char =
                        safe_c2rust_socket_strerror(__errsv);
                    if __code == G_IO_ERROR_WOULD_BLOCK as ::core::ffi::c_int {
                        g_set_error_literal(
                            __err,
                            g_io_error_quark(),
                            __code as gint,
                            __strerr as *const gchar,
                        );
                    } else {
                        g_set_error(
                            __err,
                            g_io_error_quark(),
                            __code as gint,
                            glib_gettext(
                                b"Error sending message: %s\0" as *const u8 as *const gchar,
                            ),
                            __strerr,
                        );
                    }
                }
                return -(1 as gint);
            }
        } else {
            num_sent = num_sent.wrapping_add(ret as guint);
        }
    }
    i = 0 as guint;
    while i < num_sent {
        (*messages.offset(i as isize)).bytes_sent = (*msgvec.offset(i as isize)).msg_len as guint;
        i = i.wrapping_add(1);
    }
    return num_sent as gint;
}
unsafe extern "C" fn safe_c2rust_cache_recv_address(
    mut socket_0: *mut GSocket,
    mut native: *mut sockaddr,
    mut native_len: size_t,
) -> *mut GSocketAddress {
    let mut saddr: *mut GSocketAddress = ::core::ptr::null_mut::<GSocketAddress>();
    let mut i: gint = 0;
    let mut oldest_time: guint64 = G_MAXUINT64;
    let mut oldest_index: gint = 0 as gint;
    if native_len == 0 as size_t {
        return ::core::ptr::null_mut::<GSocketAddress>();
    }
    saddr = ::core::ptr::null_mut::<GSocketAddress>();
    i = 0 as ::core::ffi::c_int as gint;
    while i < RECV_ADDR_CACHE_SIZE {
        let mut tmp: *mut GSocketAddress = (*(*socket_0).priv_0).recv_addr_cache[i as usize].addr;
        let mut tmp_native: gpointer =
            (*(*socket_0).priv_0).recv_addr_cache[i as usize].native as gpointer;
        let mut tmp_native_len: gsize =
            (*(*socket_0).priv_0).recv_addr_cache[i as usize].native_len;
        if !tmp.is_null() {
            if !(tmp_native_len as size_t != native_len) {
                if memcmp(
                    tmp_native as *const ::core::ffi::c_void,
                    native as *const ::core::ffi::c_void,
                    native_len,
                ) == 0 as ::core::ffi::c_int
                {
                    saddr =
                        g_object_ref(tmp as gpointer) as *mut GSocketAddress as *mut GSocketAddress;
                    (*(*socket_0).priv_0).recv_addr_cache[i as usize].last_used =
                        g_get_monotonic_time() as guint64;
                    return saddr;
                }
                if (*(*socket_0).priv_0).recv_addr_cache[i as usize].last_used < oldest_time {
                    oldest_time = (*(*socket_0).priv_0).recv_addr_cache[i as usize].last_used;
                    oldest_index = i;
                }
            }
        }
        i += 1;
    }
    saddr = g_socket_address_new_from_native(native as gpointer, native_len as gsize);
    if !(*(*socket_0).priv_0).recv_addr_cache[oldest_index as usize]
        .addr
        .is_null()
    {
        g_object_unref(
            (*(*socket_0).priv_0).recv_addr_cache[oldest_index as usize].addr as gpointer,
        );
        g_free((*(*socket_0).priv_0).recv_addr_cache[oldest_index as usize].native as gpointer);
    }
    (*(*socket_0).priv_0).recv_addr_cache[oldest_index as usize].native =
        g_memdup2(native as gconstpointer, native_len as gsize) as *mut sockaddr;
    (*(*socket_0).priv_0).recv_addr_cache[oldest_index as usize].native_len = native_len as gsize;
    (*(*socket_0).priv_0).recv_addr_cache[oldest_index as usize].addr =
        g_object_ref(saddr as gpointer) as *mut GSocketAddress as *mut GSocketAddress;
    (*(*socket_0).priv_0).recv_addr_cache[oldest_index as usize].last_used =
        g_get_monotonic_time() as guint64;
    return saddr;
}
unsafe extern "C" fn safe_c2rust_g_socket_receive_message_with_timeout(
    mut socket_0: *mut GSocket,
    mut address: *mut *mut GSocketAddress,
    mut vectors: *mut GInputVector,
    mut num_vectors: gint,
    mut messages: *mut *mut *mut GSocketControlMessage,
    mut num_messages: *mut gint,
    mut flags: *mut gint,
    mut timeout_us: gint64,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut one_vector: GInputVector = _GInputVector {
        buffer: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        size: 0,
    };
    let mut one_byte: ::core::ffi::c_char = 0;
    let mut start_time: gint64 = 0;
    if ({
        let mut _g_boolean_var_86: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    start_time = g_get_monotonic_time();
    if safe_c2rust_check_socket(socket_0, error) == 0 {
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    if safe_c2rust_check_timeout(socket_0, error) == 0 {
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    if num_vectors == -(1 as ::core::ffi::c_int) {
        num_vectors = 0 as ::core::ffi::c_int as gint;
        while !(*vectors.offset(num_vectors as isize)).buffer.is_null() {
            num_vectors += 1;
        }
    }
    if num_vectors == 0 as ::core::ffi::c_int {
        one_vector.buffer = &raw mut one_byte as gpointer;
        one_vector.size = 1 as gsize;
        num_vectors = 1 as ::core::ffi::c_int as gint;
        vectors = &raw mut one_vector;
    }
    let mut input_message: GInputMessage = _GInputMessage {
        address: ::core::ptr::null_mut::<*mut GSocketAddress>(),
        vectors: ::core::ptr::null_mut::<GInputVector>(),
        num_vectors: 0,
        bytes_received: 0,
        flags: 0,
        control_messages: ::core::ptr::null_mut::<*mut *mut GSocketControlMessage>(),
        num_control_messages: ::core::ptr::null_mut::<guint>(),
    };
    let mut msg: msghdr = msghdr {
        msg_name: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        msg_namelen: 0,
        msg_iov: ::core::ptr::null_mut::<iovec>(),
        msg_iovlen: 0,
        msg_control: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        msg_controllen: 0,
        msg_flags: 0,
    };
    let mut result: gssize = 0;
    input_message.address = address;
    input_message.vectors = vectors;
    input_message.num_vectors = num_vectors as guint;
    input_message.bytes_received = 0 as gsize;
    input_message.flags = (if !flags.is_null() {
        *flags
    } else {
        0 as ::core::ffi::c_int
    }) as gint;
    input_message.control_messages = messages;
    input_message.num_control_messages = num_messages as *mut guint;
    input_message.flags |= MSG_CMSG_CLOEXEC as ::core::ffi::c_int;
    let mut _message: *const GInputMessage = &raw mut input_message;
    let mut _msg: *mut msghdr = &raw mut msg;
    if !(*_message).address.is_null() {
        (*_msg).msg_namelen = ::core::mem::size_of::<sockaddr_storage>() as socklen_t;
        alloca_allocations.push(::std::vec::from_elem(
            0,
            (*_msg).msg_namelen as ::core::ffi::c_ulong as usize,
        ));
        (*_msg).msg_name = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_void;
    } else {
        (*_msg).msg_name = ::core::ptr::null_mut::<::core::ffi::c_void>();
        (*_msg).msg_namelen = 0 as socklen_t;
    }
    if ::core::mem::size_of::<iovec>() as usize == ::core::mem::size_of::<GInputVector>() as usize
        && ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize
            == ::core::mem::size_of::<gpointer>() as usize
        && 0 as ::core::ffi::c_ulong as glong == 0 as ::core::ffi::c_ulong as glong
        && ::core::mem::size_of::<size_t>() as usize == ::core::mem::size_of::<gsize>() as usize
        && 8 as ::core::ffi::c_ulong as glong == 8 as ::core::ffi::c_ulong as glong
    {
        (*_msg).msg_iov = (*_message).vectors as *mut iovec;
        (*_msg).msg_iovlen = (*_message).num_vectors as size_t;
    } else {
        let mut i: guint = 0;
        alloca_allocations.push(::std::vec::from_elem(
            0,
            (::core::mem::size_of::<iovec>() as usize)
                .wrapping_mul((*_message).num_vectors as usize) as usize,
        ));
        (*_msg).msg_iov = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_void as *mut iovec;
        i = 0 as guint;
        while i < (*_message).num_vectors {
            let ref mut fresh6 = (*(*_msg).msg_iov.offset(i as isize)).iov_base;
            *fresh6 = (*(*_message).vectors.offset(i as isize)).buffer as *mut ::core::ffi::c_void;
            (*(*_msg).msg_iov.offset(i as isize)).iov_len =
                (*(*_message).vectors.offset(i as isize)).size as size_t;
            i = i.wrapping_add(1);
        }
        (*_msg).msg_iovlen = (*_message).num_vectors as size_t;
    }
    if (*_message).control_messages.is_null() {
        (*_msg).msg_controllen = 0 as size_t;
        (*_msg).msg_control = ::core::ptr::null_mut::<::core::ffi::c_void>();
    } else {
        (*_msg).msg_controllen = G_SOCKET_CONTROL_BUFFER_SIZE_BYTES as size_t;
        alloca_allocations.push(::std::vec::from_elem(0, (*_msg).msg_controllen as usize));
        (*_msg).msg_control = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_void;
    }
    (*_msg).msg_flags = (*_message).flags as ::core::ffi::c_int;
    loop {
        result = recvmsg(
            (*(*socket_0).priv_0).fd as ::core::ffi::c_int,
            &raw mut msg,
            msg.msg_flags,
        ) as gssize;
        if result < 0 as gssize && safe_c2rust_get_socket_errno() == EINVAL {
            msg.msg_flags &= !(MSG_CMSG_CLOEXEC as ::core::ffi::c_int);
            result = recvmsg(
                (*(*socket_0).priv_0).fd as ::core::ffi::c_int,
                &raw mut msg,
                msg.msg_flags,
            ) as gssize;
        }
        if !(result < 0 as gssize) {
            break;
        }
        let mut errsv: ::core::ffi::c_int = safe_c2rust_get_socket_errno();
        if errsv == EINTR {
            continue;
        }
        if timeout_us != 0 as gint64 && (errsv == EWOULDBLOCK || errsv == EAGAIN) {
            if safe_c2rust_block_on_timeout(
                socket_0,
                G_IO_IN,
                timeout_us,
                start_time,
                cancellable,
                error,
            ) == 0
            {
                return -(1 as ::core::ffi::c_int) as gssize;
            }
        } else {
            let mut __err: *mut *mut GError = error;
            let mut __errsv: ::core::ffi::c_int = errsv;
            if !__err.is_null() {
                let mut __code: ::core::ffi::c_int =
                    safe_c2rust_socket_io_error_from_errno(__errsv) as ::core::ffi::c_int;
                let mut __strerr: *const ::core::ffi::c_char = safe_c2rust_socket_strerror(__errsv);
                if __code == G_IO_ERROR_WOULD_BLOCK as ::core::ffi::c_int {
                    g_set_error_literal(
                        __err,
                        g_io_error_quark(),
                        __code as gint,
                        __strerr as *const gchar,
                    );
                } else {
                    g_set_error(
                        __err,
                        g_io_error_quark(),
                        __code as gint,
                        glib_gettext(b"Error receiving message: %s\0" as *const u8 as *const gchar),
                        __strerr,
                    );
                }
            }
            return -(1 as ::core::ffi::c_int) as gssize;
        }
    }
    safe_c2rust_input_message_from_msghdr(&raw mut msg, &raw mut input_message, socket_0);
    if !flags.is_null() {
        *flags = input_message.flags;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_receive_messages(
    mut socket_0: *mut GSocket,
    mut messages: *mut GInputMessage,
    mut num_messages: guint,
    mut flags: gint,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gint {
    if safe_c2rust_check_socket(socket_0, error) == 0
        || safe_c2rust_check_timeout(socket_0, error) == 0
    {
        return -(1 as gint);
    }
    return safe_c2rust_g_socket_receive_messages_with_timeout(
        socket_0,
        messages,
        num_messages,
        flags,
        (if (*(*socket_0).priv_0).blocking() as ::core::ffi::c_int != 0 {
            -(1 as ::core::ffi::c_int)
        } else {
            0 as ::core::ffi::c_int
        }) as gint64,
        cancellable,
        error,
    );
}
unsafe extern "C" fn safe_c2rust_g_socket_receive_messages_with_timeout(
    mut socket_0: *mut GSocket,
    mut messages: *mut GInputMessage,
    mut num_messages: guint,
    mut flags: gint,
    mut timeout_us: gint64,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gint {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut start_time: gint64 = 0;
    if ({
        let mut _g_boolean_var_87: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    if ({
        let mut _g_boolean_var_88: ::core::ffi::c_int = 0;
        if num_messages == 0 as guint || !messages.is_null() {
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
            b"num_messages == 0 || messages != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    if ({
        let mut _g_boolean_var_89: ::core::ffi::c_int = 0;
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
            _g_boolean_var_89 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_89 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_89
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
        return -(1 as gint);
    }
    if ({
        let mut _g_boolean_var_90: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_90 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_90 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_90
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    start_time = g_get_monotonic_time();
    if safe_c2rust_check_socket(socket_0, error) == 0 {
        return -(1 as gint);
    }
    if safe_c2rust_check_timeout(socket_0, error) == 0 {
        return -(1 as gint);
    }
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return -(1 as gint);
    }
    if num_messages == 0 as guint {
        return 0 as gint;
    }
    let mut msgvec: *mut mmsghdr = ::core::ptr::null_mut::<mmsghdr>();
    let mut i: guint = 0;
    let mut num_received: guint = 0;
    if num_messages > G_IOV_MAX as guint {
        num_messages = G_IOV_MAX as guint;
    }
    alloca_allocations.push(::std::vec::from_elem(
        0,
        (::core::mem::size_of::<mmsghdr>() as usize).wrapping_mul(num_messages as usize) as usize,
    ));
    msgvec = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_void as *mut mmsghdr;
    i = 0 as guint;
    while i < num_messages {
        let mut msg: *mut GInputMessage = messages.offset(i as isize) as *mut GInputMessage;
        let mut msg_hdr: *mut msghdr = &raw mut (*msgvec.offset(i as isize)).msg_hdr;
        let mut _message: *const GInputMessage = msg;
        let mut _msg: *mut msghdr = msg_hdr;
        if !(*_message).address.is_null() {
            (*_msg).msg_namelen = ::core::mem::size_of::<sockaddr_storage>() as socklen_t;
            alloca_allocations.push(::std::vec::from_elem(
                0,
                (*_msg).msg_namelen as ::core::ffi::c_ulong as usize,
            ));
            (*_msg).msg_name = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_void;
        } else {
            (*_msg).msg_name = ::core::ptr::null_mut::<::core::ffi::c_void>();
            (*_msg).msg_namelen = 0 as socklen_t;
        }
        if ::core::mem::size_of::<iovec>() as usize
            == ::core::mem::size_of::<GInputVector>() as usize
            && ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize
                == ::core::mem::size_of::<gpointer>() as usize
            && 0 as ::core::ffi::c_ulong as glong == 0 as ::core::ffi::c_ulong as glong
            && ::core::mem::size_of::<size_t>() as usize == ::core::mem::size_of::<gsize>() as usize
            && 8 as ::core::ffi::c_ulong as glong == 8 as ::core::ffi::c_ulong as glong
        {
            (*_msg).msg_iov = (*_message).vectors as *mut iovec;
            (*_msg).msg_iovlen = (*_message).num_vectors as size_t;
        } else {
            let mut i_0: guint = 0;
            alloca_allocations.push(::std::vec::from_elem(
                0,
                (::core::mem::size_of::<iovec>() as usize)
                    .wrapping_mul((*_message).num_vectors as usize) as usize,
            ));
            (*_msg).msg_iov = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_void as *mut iovec;
            i_0 = 0 as guint;
            while i_0 < (*_message).num_vectors {
                let ref mut fresh2 = (*(*_msg).msg_iov.offset(i_0 as isize)).iov_base;
                *fresh2 =
                    (*(*_message).vectors.offset(i_0 as isize)).buffer as *mut ::core::ffi::c_void;
                (*(*_msg).msg_iov.offset(i_0 as isize)).iov_len =
                    (*(*_message).vectors.offset(i_0 as isize)).size as size_t;
                i_0 = i_0.wrapping_add(1);
            }
            (*_msg).msg_iovlen = (*_message).num_vectors as size_t;
        }
        if (*_message).control_messages.is_null() {
            (*_msg).msg_controllen = 0 as size_t;
            (*_msg).msg_control = ::core::ptr::null_mut::<::core::ffi::c_void>();
        } else {
            (*_msg).msg_controllen = G_SOCKET_CONTROL_BUFFER_SIZE_BYTES as size_t;
            alloca_allocations.push(::std::vec::from_elem(0, (*_msg).msg_controllen as usize));
            (*_msg).msg_control = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_void;
        }
        (*_msg).msg_flags = (*_message).flags as ::core::ffi::c_int;
        (*msgvec.offset(i as isize)).msg_len = 0 as ::core::ffi::c_uint;
        i = i.wrapping_add(1);
    }
    flags |= MSG_CMSG_CLOEXEC as ::core::ffi::c_int;
    num_received = 0 as guint;
    while num_received < num_messages {
        let mut ret: gint = 0;
        ret = recvmmsg(
            (*(*socket_0).priv_0).fd as ::core::ffi::c_int,
            msgvec.offset(num_received as isize),
            (num_messages as ::core::ffi::c_uint).wrapping_sub(num_received as ::core::ffi::c_uint),
            flags as ::core::ffi::c_int | MSG_NOSIGNAL as ::core::ffi::c_int,
            ::core::ptr::null_mut::<timespec>(),
        ) as gint;
        if ret < 0 as ::core::ffi::c_int && safe_c2rust_get_socket_errno() == EINVAL {
            flags &= !(MSG_CMSG_CLOEXEC as ::core::ffi::c_int);
            ret = recvmmsg(
                (*(*socket_0).priv_0).fd as ::core::ffi::c_int,
                msgvec.offset(num_received as isize),
                (num_messages as ::core::ffi::c_uint)
                    .wrapping_sub(num_received as ::core::ffi::c_uint),
                flags as ::core::ffi::c_int | MSG_NOSIGNAL as ::core::ffi::c_int,
                ::core::ptr::null_mut::<timespec>(),
            ) as gint;
        }
        if ret < 0 as ::core::ffi::c_int {
            let mut errsv: ::core::ffi::c_int = safe_c2rust_get_socket_errno();
            if errsv == EINTR {
                continue;
            }
            if timeout_us != 0 as gint64 && (errsv == EWOULDBLOCK || errsv == EAGAIN) {
                if !(safe_c2rust_block_on_timeout(
                    socket_0,
                    G_IO_IN,
                    timeout_us,
                    start_time,
                    cancellable,
                    error,
                ) == 0)
                {
                    continue;
                }
                if num_received > 0 as guint {
                    g_clear_error(error);
                    break;
                } else {
                    return -(1 as gint);
                }
            } else {
                if num_received > 0 as guint {
                    break;
                }
                let mut __err: *mut *mut GError = error;
                let mut __errsv: ::core::ffi::c_int = errsv;
                if !__err.is_null() {
                    let mut __code: ::core::ffi::c_int =
                        safe_c2rust_socket_io_error_from_errno(__errsv) as ::core::ffi::c_int;
                    let mut __strerr: *const ::core::ffi::c_char =
                        safe_c2rust_socket_strerror(__errsv);
                    if __code == G_IO_ERROR_WOULD_BLOCK as ::core::ffi::c_int {
                        g_set_error_literal(
                            __err,
                            g_io_error_quark(),
                            __code as gint,
                            __strerr as *const gchar,
                        );
                    } else {
                        g_set_error(
                            __err,
                            g_io_error_quark(),
                            __code as gint,
                            glib_gettext(
                                b"Error receiving message: %s\0" as *const u8 as *const gchar,
                            ),
                            __strerr,
                        );
                    }
                }
                return -(1 as gint);
            }
        } else {
            if ret == 0 as ::core::ffi::c_int {
                break;
            }
            num_received = num_received.wrapping_add(ret as guint);
        }
    }
    i = 0 as guint;
    while i < num_received {
        safe_c2rust_input_message_from_msghdr(
            &raw mut (*msgvec.offset(i as isize)).msg_hdr,
            messages.offset(i as isize) as *mut GInputMessage,
            socket_0,
        );
        (*messages.offset(i as isize)).bytes_received =
            (*msgvec.offset(i as isize)).msg_len as gsize;
        i = i.wrapping_add(1);
    }
    return num_received as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_receive_message(
    mut socket_0: *mut GSocket,
    mut address: *mut *mut GSocketAddress,
    mut vectors: *mut GInputVector,
    mut num_vectors: gint,
    mut messages: *mut *mut *mut GSocketControlMessage,
    mut num_messages: *mut gint,
    mut flags: *mut gint,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    return safe_c2rust_g_socket_receive_message_with_timeout(
        socket_0,
        address,
        vectors,
        num_vectors,
        messages,
        num_messages,
        flags,
        (if (*(*socket_0).priv_0).blocking() as ::core::ffi::c_int != 0 {
            -(1 as ::core::ffi::c_int)
        } else {
            0 as ::core::ffi::c_int
        }) as gint64,
        cancellable,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_get_credentials(
    mut socket_0: *mut GSocket,
    mut error: *mut *mut GError,
) -> *mut GCredentials {
    let mut ret: *mut GCredentials = ::core::ptr::null_mut::<GCredentials>();
    if ({
        let mut _g_boolean_var_91: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            _g_boolean_var_91 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_91 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_91
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GCredentials>();
    }
    if ({
        let mut _g_boolean_var_92: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_92 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_92 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_92
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
    if safe_c2rust_check_socket(socket_0, error) == 0 {
        return ::core::ptr::null_mut::<GCredentials>();
    }
    ret = ::core::ptr::null_mut::<GCredentials>();
    let mut native_creds_buf: [guint8; 12] = [0; 12];
    let mut optlen: socklen_t = ::core::mem::size_of::<[guint8; 12]>() as socklen_t;
    if getsockopt(
        (*(*socket_0).priv_0).fd as ::core::ffi::c_int,
        SOL_SOCKET,
        SO_PEERCRED,
        &raw mut native_creds_buf as *mut guint8 as *mut ::core::ffi::c_void,
        &raw mut optlen,
    ) == 0 as ::core::ffi::c_int
    {
        ret = g_credentials_new();
        g_credentials_set_native(
            ret,
            G_CREDENTIALS_TYPE_LINUX_UCRED,
            &raw mut native_creds_buf as *mut guint8 as gpointer,
        );
    }
    if ret.is_null() {
        let mut errsv: ::core::ffi::c_int = safe_c2rust_get_socket_errno();
        g_set_error(
            error,
            g_io_error_quark(),
            safe_c2rust_socket_io_error_from_errno(errsv) as gint,
            glib_gettext(b"Unable to read socket credentials: %s\0" as *const u8 as *const gchar),
            safe_c2rust_socket_strerror(errsv),
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_get_option(
    mut socket_0: *mut GSocket,
    mut level: gint,
    mut optname: gint,
    mut value: *mut gint,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut size: socklen_t = 0;
    if ({
        let mut _g_boolean_var_93: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            _g_boolean_var_93 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_93 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_93
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if (*(*socket_0).priv_0).inited() as ::core::ffi::c_int != 0
        && safe_c2rust_check_socket(socket_0, error) == 0
    {
        return FALSE;
    }
    *value = 0 as ::core::ffi::c_int as gint;
    size = ::core::mem::size_of::<gint>() as socklen_t;
    if getsockopt(
        (*(*socket_0).priv_0).fd as ::core::ffi::c_int,
        level as ::core::ffi::c_int,
        optname as ::core::ffi::c_int,
        value as *mut ::core::ffi::c_void,
        &raw mut size,
    ) != 0 as ::core::ffi::c_int
    {
        let mut errsv: ::core::ffi::c_int = safe_c2rust_get_socket_errno();
        g_set_error_literal(
            error,
            g_io_error_quark(),
            safe_c2rust_socket_io_error_from_errno(errsv) as gint,
            safe_c2rust_socket_strerror(errsv) as *const gchar,
        );
        *__errno_location() = errsv;
        return FALSE;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_set_option(
    mut socket_0: *mut GSocket,
    mut level: gint,
    mut optname: gint,
    mut value: gint,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut errsv: gint = 0;
    if ({
        let mut _g_boolean_var_94: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_get_type();
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
            _g_boolean_var_94 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_94 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_94
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if (*(*socket_0).priv_0).inited() as ::core::ffi::c_int != 0
        && safe_c2rust_check_socket(socket_0, error) == 0
    {
        return FALSE;
    }
    if setsockopt(
        (*(*socket_0).priv_0).fd as ::core::ffi::c_int,
        level as ::core::ffi::c_int,
        optname as ::core::ffi::c_int,
        &raw mut value as *const ::core::ffi::c_void,
        ::core::mem::size_of::<gint>() as socklen_t,
    ) == 0 as ::core::ffi::c_int
    {
        return TRUE;
    }
    errsv = safe_c2rust_get_socket_errno() as gint;
    g_set_error_literal(
        error,
        g_io_error_quark(),
        safe_c2rust_socket_io_error_from_errno(errsv as ::core::ffi::c_int) as gint,
        safe_c2rust_socket_strerror(errsv as ::core::ffi::c_int) as *const gchar,
    );
    *__errno_location() = errsv as ::core::ffi::c_int;
    return FALSE;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
