extern "C" {
    pub type _GData;
    pub type _GInetAddressPrivate;
    pub type _GSocketAddressEnumerator;
    pub type _GSocketConnectable;
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
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
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
    fn g_free(mem: gpointer);
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
    fn g_string_prepend(string: *mut GString, val: *const gchar) -> *mut GString;
    fn g_string_append_printf(string: *mut GString, format: *const gchar, ...);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
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
    fn g_type_name(type_0: GType) -> *const gchar;
    fn g_type_class_peek_parent(g_class: gpointer) -> gpointer;
    fn g_type_interface_peek_parent(g_iface: gpointer) -> gpointer;
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
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_value_set_object(value: *mut GValue, v_object: gpointer);
    fn g_value_get_object(value: *const GValue) -> gpointer;
    fn g_param_spec_uint(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        minimum: guint,
        maximum: guint,
        default_value: guint,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_object(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        object_type: GType,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_value_set_uint(value: *mut GValue, v_uint: guint);
    fn g_value_get_uint(value: *const GValue) -> guint;
    fn g_socket_address_get_type() -> GType;
    fn g_socket_address_new_from_native(native: gpointer, len: gsize) -> *mut GSocketAddress;
    fn g_inet_address_get_type() -> GType;
    fn g_inet_address_new_from_string(string: *const gchar) -> *mut GInetAddress;
    fn g_inet_address_to_string(address: *mut GInetAddress) -> *mut gchar;
    fn g_inet_address_to_bytes(address: *mut GInetAddress) -> *const guint8;
    fn g_inet_address_get_family(address: *mut GInetAddress) -> GSocketFamily;
    fn getaddrinfo(
        __name: *const ::core::ffi::c_char,
        __service: *const ::core::ffi::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> ::core::ffi::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn g_socket_connectable_get_type() -> GType;
    fn g_io_error_quark() -> GQuark;
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
pub type __socklen_t = ::core::ffi::c_uint;
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
pub type socklen_t = __socklen_t;
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
pub type GSocketFamily = ::core::ffi::c_uint;
pub const G_SOCKET_FAMILY_IPV6: GSocketFamily = 10;
pub const G_SOCKET_FAMILY_IPV4: GSocketFamily = 2;
pub const G_SOCKET_FAMILY_UNIX: GSocketFamily = 1;
pub const G_SOCKET_FAMILY_INVALID: GSocketFamily = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInetSocketAddressPrivate {
    pub address: *mut GInetAddress,
    pub port: guint16,
    pub flowinfo: guint32,
    pub scope_id: guint32,
}
pub type GSocketAddress = _GSocketAddress;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketAddress {
    pub parent_instance: GObject,
}
pub type GInetSocketAddress = _GInetSocketAddress;
pub type GSocketAddressEnumerator = _GSocketAddressEnumerator;
pub type GSocketConnectable = _GSocketConnectable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketAddressClass {
    pub parent_class: GObjectClass,
    pub get_family: Option<unsafe extern "C" fn(*mut GSocketAddress) -> GSocketFamily>,
    pub get_native_size: Option<unsafe extern "C" fn(*mut GSocketAddress) -> gssize>,
    pub to_native: Option<
        unsafe extern "C" fn(*mut GSocketAddress, gpointer, gsize, *mut *mut GError) -> gboolean,
    >,
}
pub type GSocketAddressClass = _GSocketAddressClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInetSocketAddressClass {
    pub parent_class: GSocketAddressClass,
}
pub type GInetSocketAddressClass = _GInetSocketAddressClass;
pub const PROP_SCOPE_ID: C2RustUnnamed_2 = 4;
pub const PROP_FLOWINFO: C2RustUnnamed_2 = 3;
pub const PROP_PORT: C2RustUnnamed_2 = 2;
pub const PROP_ADDRESS: C2RustUnnamed_2 = 1;
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
    pub __in6_u: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type uint16_t = __uint16_t;
pub type uint8_t = __uint8_t;
pub type in_port_t = uint16_t;
pub type sa_family_t = ::core::ffi::c_ushort;
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
pub type GSocketConnectableIface = _GSocketConnectableIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketConnectableIface {
    pub g_iface: GTypeInterface,
    pub enumerate:
        Option<unsafe extern "C" fn(*mut GSocketConnectable) -> *mut GSocketAddressEnumerator>,
    pub proxy_enumerate:
        Option<unsafe extern "C" fn(*mut GSocketConnectable) -> *mut GSocketAddressEnumerator>,
    pub to_string: Option<unsafe extern "C" fn(*mut GSocketConnectable) -> *mut gchar>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: ::core::ffi::c_int,
    pub ai_family: ::core::ffi::c_int,
    pub ai_socktype: ::core::ffi::c_int,
    pub ai_protocol: ::core::ffi::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut ::core::ffi::c_char,
    pub ai_next: *mut addrinfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [::core::ffi::c_char; 14],
}
pub const SOCK_STREAM: __socket_type = 1;
pub type __socket_type = ::core::ffi::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_2 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_MAXUINT32: guint32 = 0xffffffff as ::core::ffi::c_uint;
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
pub const PF_UNSPEC: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PF_INET6: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const AF_UNSPEC: ::core::ffi::c_int = PF_UNSPEC;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const AF_INET6: ::core::ffi::c_int = PF_INET6;
pub const AI_NUMERICHOST: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn safe_c2rust_g_inet_socket_address_get_instance_private(
    mut self_0: *mut GInetSocketAddress,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GInetSocketAddress_private_offset as glong as isize)
        as gpointer;
}
static mut safe_c2rust_g_inet_socket_address_parent_class: gpointer = NULL;
unsafe extern "C" fn safe_c2rust_g_inet_socket_address_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_inet_socket_address_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GInetSocketAddress_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GInetSocketAddress_private_offset,
        );
    }
    safe_c2rust_g_inet_socket_address_class_init(klass as *mut GInetSocketAddressClass);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_inet_socket_address_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_inet_socket_address_get_type_once();
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
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_inet_socket_address_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_socket_address_get_type(),
        g_intern_static_string(b"GInetSocketAddress\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GInetSocketAddressClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_inet_socket_address_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GInetSocketAddress>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GInetSocketAddress) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_inet_socket_address_init
                    as unsafe extern "C" fn(*mut GInetSocketAddress) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GInetSocketAddress_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GInetSocketAddressPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GSocketConnectableIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_inet_socket_address_connectable_iface_init
                as unsafe extern "C" fn(*mut GSocketConnectableIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_socket_connectable_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
static mut safe_c2rust_GInetSocketAddress_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_inet_socket_address_dispose(mut object: *mut GObject) {
    let mut address: *mut GInetSocketAddress =
        object as *mut ::core::ffi::c_void as *mut GInetSocketAddress;
    let mut _pp: *mut *mut GInetAddress = &raw mut (*(*address).priv_0).address;
    let mut _ptr: *mut GInetAddress = *_pp;
    *_pp = ::core::ptr::null_mut::<GInetAddress>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    (*(safe_c2rust_g_inet_socket_address_parent_class as *mut GObjectClass))
        .dispose
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_inet_socket_address_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut address: *mut GInetSocketAddress =
        object as *mut ::core::ffi::c_void as *mut GInetSocketAddress;
    match prop_id {
        1 => {
            g_value_set_object(value, (*(*address).priv_0).address as gpointer);
        }
        2 => {
            g_value_set_uint(value, (*(*address).priv_0).port as guint);
        }
        3 => {
            if ({
                let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
                if g_inet_address_get_family((*(*address).priv_0).address) as ::core::ffi::c_uint
                    == G_SOCKET_FAMILY_IPV6 as ::core::ffi::c_int as ::core::ffi::c_uint
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
                    b"g_inet_address_get_family (address->priv->address) == G_SOCKET_FAMILY_IPV6\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
                return;
            }
            g_value_set_uint(value, (*(*address).priv_0).flowinfo as guint);
        }
        4 => {
            if ({
                let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
                if g_inet_address_get_family((*(*address).priv_0).address) as ::core::ffi::c_uint
                    == G_SOCKET_FAMILY_IPV6 as ::core::ffi::c_int as ::core::ffi::c_uint
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
                    b"g_inet_address_get_family (address->priv->address) == G_SOCKET_FAMILY_IPV6\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
                return;
            }
            g_value_set_uint(value, (*(*address).priv_0).scope_id as guint);
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/ginetsocketaddress.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                109 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_inet_socket_address_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut address: *mut GInetSocketAddress =
        object as *mut ::core::ffi::c_void as *mut GInetSocketAddress;
    match prop_id {
        1 => {
            (*(*address).priv_0).address =
                g_object_ref(g_value_get_object(value)) as *mut GInetAddress;
        }
        2 => {
            (*(*address).priv_0).port = g_value_get_uint(value) as guint16;
        }
        3 => {
            (*(*address).priv_0).flowinfo = g_value_get_uint(value) as guint32;
        }
        4 => {
            (*(*address).priv_0).scope_id = g_value_get_uint(value) as guint32;
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/ginetsocketaddress.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                143 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_inet_socket_address_get_family(
    mut address: *mut GSocketAddress,
) -> GSocketFamily {
    let mut addr: *mut GInetSocketAddress = ::core::ptr::null_mut::<GInetSocketAddress>();
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = address as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_inet_socket_address_get_type();
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
            b"G_IS_INET_SOCKET_ADDRESS (address)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_SOCKET_FAMILY_INVALID;
    }
    addr = address as *mut ::core::ffi::c_void as *mut GInetSocketAddress;
    return g_inet_address_get_family((*(*addr).priv_0).address);
}
unsafe extern "C" fn safe_c2rust_g_inet_socket_address_get_native_size(
    mut address: *mut GSocketAddress,
) -> gssize {
    let mut addr: *mut GInetSocketAddress = ::core::ptr::null_mut::<GInetSocketAddress>();
    let mut family: GSocketFamily = G_SOCKET_FAMILY_INVALID;
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = address as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_inet_socket_address_get_type();
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
            b"G_IS_INET_SOCKET_ADDRESS (address)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gssize;
    }
    addr = address as *mut ::core::ffi::c_void as *mut GInetSocketAddress;
    family = g_inet_address_get_family((*(*addr).priv_0).address);
    if family as ::core::ffi::c_uint == AF_INET as ::core::ffi::c_uint {
        return ::core::mem::size_of::<sockaddr_in>() as gssize;
    } else if family as ::core::ffi::c_uint == AF_INET6 as ::core::ffi::c_uint {
        return ::core::mem::size_of::<sockaddr_in6>() as gssize;
    } else {
        return -(1 as ::core::ffi::c_int) as gssize;
    };
}
unsafe extern "C" fn safe_c2rust_g_inet_socket_address_to_native(
    mut address: *mut GSocketAddress,
    mut dest: gpointer,
    mut destlen: gsize,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut addr: *mut GInetSocketAddress = ::core::ptr::null_mut::<GInetSocketAddress>();
    let mut family: GSocketFamily = G_SOCKET_FAMILY_INVALID;
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = address as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_inet_socket_address_get_type();
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
            b"G_IS_INET_SOCKET_ADDRESS (address)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    addr = address as *mut ::core::ffi::c_void as *mut GInetSocketAddress;
    family = g_inet_address_get_family((*(*addr).priv_0).address);
    if family as ::core::ffi::c_uint == AF_INET as ::core::ffi::c_uint {
        let mut sock: *mut sockaddr_in = dest as *mut sockaddr_in;
        if (destlen as usize) < ::core::mem::size_of::<sockaddr_in>() as usize {
            g_set_error_literal(
                error,
                g_io_error_quark(),
                G_IO_ERROR_NO_SPACE as ::core::ffi::c_int as gint,
                glib_gettext(b"Not enough space for socket address\0" as *const u8 as *const gchar),
            );
            return FALSE;
        }
        (*sock).sin_family = AF_INET as sa_family_t;
        (*sock).sin_port = (((*(*addr).priv_0).port as ::core::ffi::c_int
            >> 8 as ::core::ffi::c_int) as guint16
            as ::core::ffi::c_int
            | (((*(*addr).priv_0).port as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as guint16
                as ::core::ffi::c_int) as guint16 as in_port_t;
        memcpy(
            &raw mut (*sock).sin_addr.s_addr as *mut ::core::ffi::c_void,
            g_inet_address_to_bytes((*(*addr).priv_0).address) as *const ::core::ffi::c_void,
            ::core::mem::size_of::<in_addr>() as size_t,
        );
        memset(
            &raw mut (*sock).sin_zero as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[::core::ffi::c_uchar; 8]>() as size_t,
        );
        return TRUE;
    } else if family as ::core::ffi::c_uint == AF_INET6 as ::core::ffi::c_uint {
        let mut sock_0: *mut sockaddr_in6 = dest as *mut sockaddr_in6;
        if (destlen as usize) < ::core::mem::size_of::<sockaddr_in6>() as usize {
            g_set_error_literal(
                error,
                g_io_error_quark(),
                G_IO_ERROR_NO_SPACE as ::core::ffi::c_int as gint,
                glib_gettext(b"Not enough space for socket address\0" as *const u8 as *const gchar),
            );
            return FALSE;
        }
        memset(
            sock_0 as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<sockaddr_in6>() as size_t,
        );
        (*sock_0).sin6_family = AF_INET6 as sa_family_t;
        (*sock_0).sin6_port = (((*(*addr).priv_0).port as ::core::ffi::c_int
            >> 8 as ::core::ffi::c_int) as guint16
            as ::core::ffi::c_int
            | (((*(*addr).priv_0).port as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as guint16
                as ::core::ffi::c_int) as guint16 as in_port_t;
        (*sock_0).sin6_flowinfo = (*(*addr).priv_0).flowinfo as uint32_t;
        (*sock_0).sin6_scope_id = (*(*addr).priv_0).scope_id as uint32_t;
        memcpy(
            &raw mut (*sock_0).sin6_addr.__in6_u.__u6_addr8 as *mut ::core::ffi::c_void,
            g_inet_address_to_bytes((*(*addr).priv_0).address) as *const ::core::ffi::c_void,
            ::core::mem::size_of::<in6_addr>() as size_t,
        );
        return TRUE;
    } else {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(b"Unsupported socket address\0" as *const u8 as *const gchar),
        );
        return FALSE;
    };
}
unsafe extern "C" fn safe_c2rust_g_inet_socket_address_class_init(
    mut klass: *mut GInetSocketAddressClass,
) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    let mut gsocketaddress_class: *mut GSocketAddressClass =
        klass as *mut ::core::ffi::c_void as *mut GSocketAddressClass;
    (*gobject_class).dispose =
        Some(safe_c2rust_g_inet_socket_address_dispose as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_inet_socket_address_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_inet_socket_address_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*gsocketaddress_class).get_family = Some(
        safe_c2rust_g_inet_socket_address_get_family
            as unsafe extern "C" fn(*mut GSocketAddress) -> GSocketFamily,
    )
        as Option<unsafe extern "C" fn(*mut GSocketAddress) -> GSocketFamily>;
    (*gsocketaddress_class).to_native = Some(
        safe_c2rust_g_inet_socket_address_to_native
            as unsafe extern "C" fn(
                *mut GSocketAddress,
                gpointer,
                gsize,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GSocketAddress,
                gpointer,
                gsize,
                *mut *mut GError,
            ) -> gboolean,
        >;
    (*gsocketaddress_class).get_native_size = Some(
        safe_c2rust_g_inet_socket_address_get_native_size
            as unsafe extern "C" fn(*mut GSocketAddress) -> gssize,
    )
        as Option<unsafe extern "C" fn(*mut GSocketAddress) -> gssize>;
    g_object_class_install_property(
        gobject_class,
        PROP_ADDRESS as ::core::ffi::c_int as guint,
        g_param_spec_object(
            b"address\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_inet_address_get_type(),
            (G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_PORT as ::core::ffi::c_int as guint,
        g_param_spec_uint(
            b"port\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            0 as guint,
            65535 as guint,
            0 as guint,
            (G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_FLOWINFO as ::core::ffi::c_int as guint,
        g_param_spec_uint(
            b"flowinfo\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            0 as guint,
            G_MAXUINT32,
            0 as guint,
            (G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_SCOPE_ID as ::core::ffi::c_int as guint,
        g_param_spec_uint(
            b"scope-id\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            0 as guint,
            G_MAXUINT32,
            0 as guint,
            (G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
unsafe extern "C" fn safe_c2rust_g_inet_socket_address_connectable_iface_init(
    mut iface: *mut GSocketConnectableIface,
) {
    let mut parent_iface: *mut GSocketConnectableIface =
        g_type_interface_peek_parent(iface as gpointer) as *mut GSocketConnectableIface;
    (*iface).enumerate = (*parent_iface).enumerate;
    (*iface).proxy_enumerate = (*parent_iface).proxy_enumerate;
    (*iface).to_string = Some(
        safe_c2rust_g_inet_socket_address_connectable_to_string
            as unsafe extern "C" fn(*mut GSocketConnectable) -> *mut gchar,
    )
        as Option<unsafe extern "C" fn(*mut GSocketConnectable) -> *mut gchar>;
}
unsafe extern "C" fn safe_c2rust_g_inet_socket_address_connectable_to_string(
    mut connectable: *mut GSocketConnectable,
) -> *mut gchar {
    let mut sa: *mut GInetSocketAddress = ::core::ptr::null_mut::<GInetSocketAddress>();
    let mut a: *mut GInetAddress = ::core::ptr::null_mut::<GInetAddress>();
    let mut a_string: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut out: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut port: guint16 = 0;
    sa = connectable as *mut ::core::ffi::c_void as *mut GInetSocketAddress;
    a = safe_c2rust_g_inet_socket_address_get_address(sa);
    out = g_string_new(b"\0" as *const u8 as *const gchar);
    a_string = g_inet_address_to_string(a);
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char = a_string;
            safe_c2rust_g_string_append_len_inline(
                out,
                __val,
                if ({
                    let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
                    if !__val.is_null() {
                        _g_boolean_var_15 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_15 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_15
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
        safe_c2rust_g_string_append_len_inline(out, a_string, -(1 as ::core::ffi::c_int) as gssize);
    };
    g_free(a_string as gpointer);
    if g_inet_address_get_family(a) as ::core::ffi::c_uint
        == G_SOCKET_FAMILY_IPV6 as ::core::ffi::c_int as ::core::ffi::c_uint
        && safe_c2rust_g_inet_socket_address_get_scope_id(sa) != 0 as guint32
    {
        g_string_append_printf(
            out,
            b"%%%u\0" as *const u8 as *const gchar,
            safe_c2rust_g_inet_socket_address_get_scope_id(sa),
        );
    }
    port = safe_c2rust_g_inet_socket_address_get_port(sa);
    if port as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        if g_inet_address_get_family(a) as ::core::ffi::c_uint
            == G_SOCKET_FAMILY_IPV6 as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            g_string_prepend(out, b"[\0" as *const u8 as *const gchar);
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char =
                        b"]\0" as *const u8 as *const ::core::ffi::c_char;
                    safe_c2rust_g_string_append_len_inline(
                        out,
                        __val,
                        if ({
                            let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
                            if !__val.is_null() {
                                _g_boolean_var_16 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_16 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_16
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
                safe_c2rust_g_string_append_len_inline(
                    out,
                    b"]\0" as *const u8 as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
        }
        g_string_append_printf(
            out,
            b":%u\0" as *const u8 as *const gchar,
            port as ::core::ffi::c_int,
        );
    }
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(out, 0 as gboolean)
        } else {
            g_string_free_and_steal(out)
        }
    } else {
        g_string_free(out, 0 as gboolean)
    };
}
unsafe extern "C" fn safe_c2rust_g_inet_socket_address_init(mut address: *mut GInetSocketAddress) {
    (*address).priv_0 = safe_c2rust_g_inet_socket_address_get_instance_private(address)
        as *mut GInetSocketAddressPrivate;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_inet_socket_address_new(
    mut address: *mut GInetAddress,
    mut port: guint16,
) -> *mut GSocketAddress {
    return g_object_new(
        safe_c2rust_g_inet_socket_address_get_type(),
        b"address\0" as *const u8 as *const gchar,
        address,
        b"port\0" as *const u8 as *const ::core::ffi::c_char,
        port as ::core::ffi::c_int,
        NULL,
    ) as *mut GSocketAddress;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_inet_socket_address_new_from_string(
    mut address: *const ::core::ffi::c_char,
    mut port: guint,
) -> *mut GSocketAddress {
    static mut safe_c2rust_hints: *mut addrinfo = ::core::ptr::null::<addrinfo>() as *mut addrinfo;
    static mut safe_c2rust_hints_struct: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: ::core::ptr::null::<sockaddr>() as *mut sockaddr,
        ai_canonname: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        ai_next: ::core::ptr::null::<addrinfo>() as *mut addrinfo,
    };
    let mut saddr: *mut GSocketAddress = ::core::ptr::null_mut::<GSocketAddress>();
    let mut iaddr: *mut GInetAddress = ::core::ptr::null_mut::<GInetAddress>();
    let mut res: *mut addrinfo = ::core::ptr::null_mut::<addrinfo>();
    let mut status: gint = 0;
    if !strchr(address, ':' as i32).is_null() {
        if ({
            let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
            if ({
                if 0 as ::core::ffi::c_int != 0 {
                    safe_c2rust_hints;
                } else {
                };
                (({
                    let mut gapg_temp_newval: *mut addrinfo = ::core::ptr::null_mut::<addrinfo>();
                    let mut gapg_temp_atomic: *mut *mut addrinfo = &raw mut safe_c2rust_hints;
                    *&raw mut gapg_temp_newval =
                        crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
                    gapg_temp_newval
                })
                .is_null()
                    && g_once_init_enter_pointer(
                        &raw mut safe_c2rust_hints as *mut ::core::ffi::c_void,
                    ) != 0) as ::core::ffi::c_int
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
            safe_c2rust_hints_struct.ai_family = AF_UNSPEC;
            safe_c2rust_hints_struct.ai_socktype = SOCK_STREAM as ::core::ffi::c_int;
            safe_c2rust_hints_struct.ai_protocol = 0 as ::core::ffi::c_int;
            safe_c2rust_hints_struct.ai_flags = AI_NUMERICHOST;
            if 0 as ::core::ffi::c_int != 0 {
                safe_c2rust_hints = &raw mut safe_c2rust_hints_struct;
            } else {
            };
            g_once_init_leave_pointer(
                &raw mut safe_c2rust_hints as *mut ::core::ffi::c_void,
                &raw mut safe_c2rust_hints_struct as guintptr as gpointer,
            );
        }
        status = getaddrinfo(
            address,
            ::core::ptr::null::<::core::ffi::c_char>(),
            safe_c2rust_hints,
            &raw mut res,
        ) as gint;
        if status != 0 as ::core::ffi::c_int {
            return ::core::ptr::null_mut::<GSocketAddress>();
        }
        if (*res).ai_family == AF_INET6
            && (*res).ai_addrlen as usize == ::core::mem::size_of::<sockaddr_in6>() as usize
        {
            (*((*res).ai_addr as *mut sockaddr_in6)).sin6_port =
                ((port as guint16 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as guint16
                    as ::core::ffi::c_int
                    | ((port as guint16 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                        as guint16 as ::core::ffi::c_int) as guint16 as in_port_t;
            saddr = g_socket_address_new_from_native(
                (*res).ai_addr as gpointer,
                (*res).ai_addrlen as gsize,
            );
        } else {
            saddr = ::core::ptr::null_mut::<GSocketAddress>();
        }
        freeaddrinfo(res);
    } else {
        iaddr = g_inet_address_new_from_string(address as *const gchar);
        if iaddr.is_null() {
            return ::core::ptr::null_mut::<GSocketAddress>();
        }
        if !(({
            let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
            if g_inet_address_get_family(iaddr) as ::core::ffi::c_uint
                == G_SOCKET_FAMILY_IPV4 as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                _g_boolean_var_18 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_18 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_18
        }) as ::core::ffi::c_long
            != 0)
        {
            g_warn_message(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/ginetsocketaddress.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                460 as ::core::ffi::c_int,
                G_STRFUNC,
                b"g_inet_address_get_family (iaddr) == G_SOCKET_FAMILY_IPV4\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        saddr = safe_c2rust_g_inet_socket_address_new(iaddr, port as guint16);
        g_object_unref(iaddr as gpointer);
    }
    return saddr;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_inet_socket_address_get_address(
    mut address: *mut GInetSocketAddress,
) -> *mut GInetAddress {
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = address as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_inet_socket_address_get_type();
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
            b"G_IS_INET_SOCKET_ADDRESS (address)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GInetAddress>();
    }
    return (*(*address).priv_0).address;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_inet_socket_address_get_port(
    mut address: *mut GInetSocketAddress,
) -> guint16 {
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = address as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_inet_socket_address_get_type();
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
            b"G_IS_INET_SOCKET_ADDRESS (address)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint16;
    }
    return (*(*address).priv_0).port;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_inet_socket_address_get_flowinfo(
    mut address: *mut GInetSocketAddress,
) -> guint32 {
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = address as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_inet_socket_address_get_type();
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
            b"G_IS_INET_SOCKET_ADDRESS (address)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint32;
    }
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if g_inet_address_get_family((*(*address).priv_0).address) as ::core::ffi::c_uint
            == G_SOCKET_FAMILY_IPV6 as ::core::ffi::c_int as ::core::ffi::c_uint
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
            b"g_inet_address_get_family (address->priv->address) == G_SOCKET_FAMILY_IPV6\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint32;
    }
    return (*(*address).priv_0).flowinfo;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_inet_socket_address_get_scope_id(
    mut address: *mut GInetSocketAddress,
) -> guint32 {
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = address as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_inet_socket_address_get_type();
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
            b"G_IS_INET_SOCKET_ADDRESS (address)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint32;
    }
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if g_inet_address_get_family((*(*address).priv_0).address) as ::core::ffi::c_uint
            == G_SOCKET_FAMILY_IPV6 as ::core::ffi::c_int as ::core::ffi::c_uint
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
            b"g_inet_address_get_family (address->priv->address) == G_SOCKET_FAMILY_IPV6\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint32;
    }
    return (*(*address).priv_0).scope_id;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
