extern "C" {
    pub type _GData;
    pub type _GSocketConnectable;
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
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_byte_array_sized_new(reserved_size: guint) -> *mut GByteArray;
    fn g_byte_array_unref(array: *mut GByteArray);
    fn g_byte_array_append(
        array: *mut GByteArray,
        data: *const guint8,
        len: guint,
    ) -> *mut GByteArray;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    static safe_c2rust_g_ascii_table: *const guint16;
    fn g_strlcpy(dest: *mut gchar, src: *const gchar, dest_size: gsize) -> gsize;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_string_sized_new(dfl_size: gsize) -> *mut GString;
    fn g_string_free(string: *mut GString, free_segment: gboolean) -> *mut gchar;
    fn g_string_free_and_steal(string: *mut GString) -> *mut gchar;
    fn g_string_insert_c(string: *mut GString, pos: gssize, c: gchar) -> *mut GString;
    fn g_string_append_printf(string: *mut GString, format: *const gchar, ...);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
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
    fn g_byte_array_get_type() -> GType;
    fn g_value_take_boxed(value: *mut GValue, v_boxed: gconstpointer);
    fn g_value_get_boxed(value: *const GValue) -> gpointer;
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_value_set_enum(value: *mut GValue, v_enum: gint);
    fn g_value_get_enum(value: *const GValue) -> gint;
    fn g_param_spec_boolean(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: gboolean,
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
    fn g_param_spec_string(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: *const gchar,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_boxed(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        boxed_type: GType,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_value_set_boolean(value: *mut GValue, v_boolean: gboolean);
    fn g_value_get_boolean(value: *const GValue) -> gboolean;
    fn g_value_set_string(value: *mut GValue, v_string: *const gchar);
    fn g_value_get_string(value: *const GValue) -> *const gchar;
    fn g_io_error_quark() -> GQuark;
    fn g_socket_address_get_type() -> GType;
    fn g_unix_socket_address_type_get_type() -> GType;
    fn g_socket_connectable_get_type() -> GType;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GByteArray {
    pub data: *mut guint8,
    pub len: guint,
}
pub type GByteArray = _GByteArray;
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
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const G_ASCII_XDIGIT: C2RustUnnamed = 1024;
pub const G_ASCII_UPPER: C2RustUnnamed = 512;
pub const G_ASCII_SPACE: C2RustUnnamed = 256;
pub const G_ASCII_PUNCT: C2RustUnnamed = 128;
pub const G_ASCII_PRINT: C2RustUnnamed = 64;
pub const G_ASCII_LOWER: C2RustUnnamed = 32;
pub const G_ASCII_GRAPH: C2RustUnnamed = 16;
pub const G_ASCII_DIGIT: C2RustUnnamed = 8;
pub const G_ASCII_CNTRL: C2RustUnnamed = 4;
pub const G_ASCII_ALPHA: C2RustUnnamed = 2;
pub const G_ASCII_ALNUM: C2RustUnnamed = 1;
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
    pub data: [C2RustUnnamed_0; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const G_IO_ERROR_DESTINATION_UNSET: C2RustUnnamed_1 = 48;
pub const G_IO_ERROR_NO_SUCH_DEVICE: C2RustUnnamed_1 = 47;
pub const G_IO_ERROR_MESSAGE_TOO_LARGE: C2RustUnnamed_1 = 46;
pub const G_IO_ERROR_NOT_CONNECTED: C2RustUnnamed_1 = 45;
pub const G_IO_ERROR_CONNECTION_CLOSED: C2RustUnnamed_1 = 44;
pub const G_IO_ERROR_BROKEN_PIPE: C2RustUnnamed_1 = 44;
pub const G_IO_ERROR_PROXY_NOT_ALLOWED: C2RustUnnamed_1 = 43;
pub const G_IO_ERROR_PROXY_NEED_AUTH: C2RustUnnamed_1 = 42;
pub const G_IO_ERROR_PROXY_AUTH_FAILED: C2RustUnnamed_1 = 41;
pub const G_IO_ERROR_PROXY_FAILED: C2RustUnnamed_1 = 40;
pub const G_IO_ERROR_CONNECTION_REFUSED: C2RustUnnamed_1 = 39;
pub const G_IO_ERROR_NETWORK_UNREACHABLE: C2RustUnnamed_1 = 38;
pub const G_IO_ERROR_HOST_UNREACHABLE: C2RustUnnamed_1 = 37;
pub const G_IO_ERROR_DBUS_ERROR: C2RustUnnamed_1 = 36;
pub const G_IO_ERROR_INVALID_DATA: C2RustUnnamed_1 = 35;
pub const G_IO_ERROR_PARTIAL_INPUT: C2RustUnnamed_1 = 34;
pub const G_IO_ERROR_ADDRESS_IN_USE: C2RustUnnamed_1 = 33;
pub const G_IO_ERROR_NOT_INITIALIZED: C2RustUnnamed_1 = 32;
pub const G_IO_ERROR_TOO_MANY_OPEN_FILES: C2RustUnnamed_1 = 31;
pub const G_IO_ERROR_FAILED_HANDLED: C2RustUnnamed_1 = 30;
pub const G_IO_ERROR_WOULD_MERGE: C2RustUnnamed_1 = 29;
pub const G_IO_ERROR_HOST_NOT_FOUND: C2RustUnnamed_1 = 28;
pub const G_IO_ERROR_WOULD_BLOCK: C2RustUnnamed_1 = 27;
pub const G_IO_ERROR_BUSY: C2RustUnnamed_1 = 26;
pub const G_IO_ERROR_WOULD_RECURSE: C2RustUnnamed_1 = 25;
pub const G_IO_ERROR_TIMED_OUT: C2RustUnnamed_1 = 24;
pub const G_IO_ERROR_WRONG_ETAG: C2RustUnnamed_1 = 23;
pub const G_IO_ERROR_CANT_CREATE_BACKUP: C2RustUnnamed_1 = 22;
pub const G_IO_ERROR_READ_ONLY: C2RustUnnamed_1 = 21;
pub const G_IO_ERROR_PENDING: C2RustUnnamed_1 = 20;
pub const G_IO_ERROR_CANCELLED: C2RustUnnamed_1 = 19;
pub const G_IO_ERROR_CLOSED: C2RustUnnamed_1 = 18;
pub const G_IO_ERROR_ALREADY_MOUNTED: C2RustUnnamed_1 = 17;
pub const G_IO_ERROR_NOT_MOUNTED: C2RustUnnamed_1 = 16;
pub const G_IO_ERROR_NOT_SUPPORTED: C2RustUnnamed_1 = 15;
pub const G_IO_ERROR_PERMISSION_DENIED: C2RustUnnamed_1 = 14;
pub const G_IO_ERROR_INVALID_ARGUMENT: C2RustUnnamed_1 = 13;
pub const G_IO_ERROR_NO_SPACE: C2RustUnnamed_1 = 12;
pub const G_IO_ERROR_TOO_MANY_LINKS: C2RustUnnamed_1 = 11;
pub const G_IO_ERROR_INVALID_FILENAME: C2RustUnnamed_1 = 10;
pub const G_IO_ERROR_FILENAME_TOO_LONG: C2RustUnnamed_1 = 9;
pub const G_IO_ERROR_NOT_MOUNTABLE_FILE: C2RustUnnamed_1 = 8;
pub const G_IO_ERROR_NOT_SYMBOLIC_LINK: C2RustUnnamed_1 = 7;
pub const G_IO_ERROR_NOT_REGULAR_FILE: C2RustUnnamed_1 = 6;
pub const G_IO_ERROR_NOT_EMPTY: C2RustUnnamed_1 = 5;
pub const G_IO_ERROR_NOT_DIRECTORY: C2RustUnnamed_1 = 4;
pub const G_IO_ERROR_IS_DIRECTORY: C2RustUnnamed_1 = 3;
pub const G_IO_ERROR_EXISTS: C2RustUnnamed_1 = 2;
pub const G_IO_ERROR_NOT_FOUND: C2RustUnnamed_1 = 1;
pub const G_IO_ERROR_FAILED: C2RustUnnamed_1 = 0;
pub type GSocketFamily = ::core::ffi::c_uint;
pub const G_SOCKET_FAMILY_IPV6: GSocketFamily = 10;
pub const G_SOCKET_FAMILY_IPV4: GSocketFamily = 2;
pub const G_SOCKET_FAMILY_UNIX: GSocketFamily = 1;
pub const G_SOCKET_FAMILY_INVALID: GSocketFamily = 0;
pub type GUnixSocketAddressType = ::core::ffi::c_uint;
pub const G_UNIX_SOCKET_ADDRESS_ABSTRACT_PADDED: GUnixSocketAddressType = 4;
pub const G_UNIX_SOCKET_ADDRESS_ABSTRACT: GUnixSocketAddressType = 3;
pub const G_UNIX_SOCKET_ADDRESS_PATH: GUnixSocketAddressType = 2;
pub const G_UNIX_SOCKET_ADDRESS_ANONYMOUS: GUnixSocketAddressType = 1;
pub const G_UNIX_SOCKET_ADDRESS_INVALID: GUnixSocketAddressType = 0;
pub type GSocketAddress = _GSocketAddress;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketAddress {
    pub parent_instance: GObject,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketAddressEnumerator {
    pub parent_instance: GObject,
}
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
pub struct _GSocketConnectableIface {
    pub g_iface: GTypeInterface,
    pub enumerate:
        Option<unsafe extern "C" fn(*mut GSocketConnectable) -> *mut GSocketAddressEnumerator>,
    pub proxy_enumerate:
        Option<unsafe extern "C" fn(*mut GSocketConnectable) -> *mut GSocketAddressEnumerator>,
    pub to_string: Option<unsafe extern "C" fn(*mut GSocketConnectable) -> *mut gchar>,
}
pub type GSocketConnectableIface = _GSocketConnectableIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixSocketAddress {
    pub parent_instance: GSocketAddress,
    pub priv_0: *mut GUnixSocketAddressPrivate,
}
pub type GUnixSocketAddressPrivate = _GUnixSocketAddressPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixSocketAddressPrivate {
    pub path: [::core::ffi::c_char; 108],
    pub path_len: gsize,
    pub address_type: GUnixSocketAddressType,
}
pub type GUnixSocketAddress = _GUnixSocketAddress;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixSocketAddressClass {
    pub parent_class: GSocketAddressClass,
}
pub type GUnixSocketAddressClass = _GUnixSocketAddressClass;
pub const PROP_ADDRESS_TYPE: C2RustUnnamed_2 = 4;
pub const PROP_ABSTRACT: C2RustUnnamed_2 = 3;
pub const PROP_PATH_AS_ARRAY: C2RustUnnamed_2 = 2;
pub const PROP_PATH: C2RustUnnamed_2 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [::core::ffi::c_char; 108],
}
pub type sa_family_t = ::core::ffi::c_ushort;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_2 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline(always)]
unsafe extern "C" fn safe_c2rust_g_strdup_inline(
    mut str: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if 0 != 0 && str.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if 0 != 0 && !str.is_null() && 0 != 0 {
        let len: size_t = (strlen(str) as size_t).wrapping_add(1 as size_t);
        let mut dup_str: *mut ::core::ffi::c_char =
            g_malloc(len as gsize) as *mut ::core::ffi::c_char;
        return memcpy(
            dup_str as *mut ::core::ffi::c_void,
            str as *const ::core::ffi::c_void,
            len,
        ) as *mut ::core::ffi::c_char;
    }
    return g_strdup(str as *const gchar) as *mut ::core::ffi::c_char;
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
pub const PF_LOCAL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PF_UNIX: ::core::ffi::c_int = PF_LOCAL;
pub const AF_UNIX: ::core::ffi::c_int = PF_UNIX;
static mut safe_c2rust_GUnixSocketAddress_private_offset: gint = 0;
#[inline]
unsafe extern "C" fn safe_c2rust_g_unix_socket_address_get_instance_private(
    mut self_0: *mut GUnixSocketAddress,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GUnixSocketAddress_private_offset as glong as isize)
        as gpointer;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_socket_address_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_unix_socket_address_get_type_once();
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
static mut safe_c2rust_g_unix_socket_address_parent_class: gpointer = NULL_0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_unix_socket_address_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_socket_address_get_type(),
        g_intern_static_string(b"GUnixSocketAddress\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GUnixSocketAddressClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_unix_socket_address_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GUnixSocketAddress>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GUnixSocketAddress) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_unix_socket_address_init
                    as unsafe extern "C" fn(*mut GUnixSocketAddress) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GUnixSocketAddress_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GUnixSocketAddressPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GSocketConnectableIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_unix_socket_address_connectable_iface_init
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
unsafe extern "C" fn safe_c2rust_g_unix_socket_address_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_unix_socket_address_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GUnixSocketAddress_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GUnixSocketAddress_private_offset,
        );
    }
    safe_c2rust_g_unix_socket_address_class_init(klass as *mut GUnixSocketAddressClass);
}
unsafe extern "C" fn safe_c2rust_g_unix_socket_address_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut address: *mut GUnixSocketAddress =
        object as *mut ::core::ffi::c_void as *mut GUnixSocketAddress;
    let mut str: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut array: *mut GByteArray = ::core::ptr::null_mut::<GByteArray>();
    let mut len: gsize = 0;
    match prop_id {
        1 => {
            str = g_value_get_string(value) as *const ::core::ffi::c_char;
            if !str.is_null() {
                g_strlcpy(
                    &raw mut (*(*address).priv_0).path as *mut gchar,
                    str as *const gchar,
                    ::core::mem::size_of::<[::core::ffi::c_char; 108]>() as gsize,
                );
                (*(*address).priv_0).path_len =
                    strlen(&raw mut (*(*address).priv_0).path as *mut ::core::ffi::c_char) as gsize;
            }
        }
        2 => {
            array = g_value_get_boxed(value) as *mut GByteArray;
            if !array.is_null() {
                len = (if ((*array).len as usize)
                    < (::core::mem::size_of::<[::core::ffi::c_char; 108]>() as usize)
                        .wrapping_sub(1 as usize)
                {
                    (*array).len as usize
                } else {
                    (::core::mem::size_of::<[::core::ffi::c_char; 108]>() as usize)
                        .wrapping_sub(1 as usize)
                }) as gsize;
                if len != 0 as gsize {
                    memcpy(
                        &raw mut (*(*address).priv_0).path as *mut ::core::ffi::c_char
                            as *mut ::core::ffi::c_void,
                        (*array).data as *const ::core::ffi::c_void,
                        len as size_t,
                    );
                }
                (*(*address).priv_0).path[len as usize] = 0 as ::core::ffi::c_char;
                (*(*address).priv_0).path_len = len;
            }
        }
        3 => {
            if g_value_get_boolean(value) != 0 {
                (*(*address).priv_0).address_type = G_UNIX_SOCKET_ADDRESS_ABSTRACT_PADDED;
            }
        }
        4 => {
            if g_value_get_enum(value) != G_UNIX_SOCKET_ADDRESS_PATH as ::core::ffi::c_int {
                (*(*address).priv_0).address_type =
                    g_value_get_enum(value) as GUnixSocketAddressType;
            }
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gunixsocketaddress.c\0"
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
unsafe extern "C" fn safe_c2rust_g_unix_socket_address_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut address: *mut GUnixSocketAddress =
        object as *mut ::core::ffi::c_void as *mut GUnixSocketAddress;
    let mut array: *mut GByteArray = ::core::ptr::null_mut::<GByteArray>();
    match prop_id {
        1 => {
            g_value_set_string(
                value,
                &raw mut (*(*address).priv_0).path as *mut ::core::ffi::c_char,
            );
        }
        2 => {
            array = g_byte_array_sized_new((*(*address).priv_0).path_len as guint);
            g_byte_array_append(
                array,
                &raw mut (*(*address).priv_0).path as *mut ::core::ffi::c_char as *mut guint8,
                (*(*address).priv_0).path_len as guint,
            );
            g_value_take_boxed(value, array as gconstpointer);
        }
        3 => {
            g_value_set_boolean(
                value,
                ((*(*address).priv_0).address_type as ::core::ffi::c_uint
                    == G_UNIX_SOCKET_ADDRESS_ABSTRACT as ::core::ffi::c_int as ::core::ffi::c_uint
                    || (*(*address).priv_0).address_type as ::core::ffi::c_uint
                        == G_UNIX_SOCKET_ADDRESS_ABSTRACT_PADDED as ::core::ffi::c_int
                            as ::core::ffi::c_uint) as ::core::ffi::c_int,
            );
        }
        4 => {
            g_value_set_enum(value, (*(*address).priv_0).address_type as gint);
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gunixsocketaddress.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                179 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_unix_socket_address_get_family(
    mut address: *mut GSocketAddress,
) -> GSocketFamily {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if 1 as ::core::ffi::c_int == G_SOCKET_FAMILY_UNIX as ::core::ffi::c_int {
            _g_boolean_var_10 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_10 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_10
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gunixsocketaddress.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            186 as ::core::ffi::c_int,
            G_STRFUNC,
            b"PF_UNIX == G_SOCKET_FAMILY_UNIX\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return G_SOCKET_FAMILY_UNIX;
}
unsafe extern "C" fn safe_c2rust_g_unix_socket_address_get_native_size(
    mut address: *mut GSocketAddress,
) -> gssize {
    let mut addr: *mut GUnixSocketAddress =
        address as *mut ::core::ffi::c_void as *mut GUnixSocketAddress;
    match (*(*addr).priv_0).address_type as ::core::ffi::c_uint {
        1 => return 2 as ::core::ffi::c_ulong as gssize,
        3 => {
            return (2 as ::core::ffi::c_ulong as glong as gsize)
                .wrapping_add((*(*addr).priv_0).path_len)
                .wrapping_add(1 as gsize) as gssize;
        }
        _ => return ::core::mem::size_of::<sockaddr_un>() as gssize,
    };
}
unsafe extern "C" fn safe_c2rust_g_unix_socket_address_to_native(
    mut address: *mut GSocketAddress,
    mut dest: gpointer,
    mut destlen: gsize,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut addr: *mut GUnixSocketAddress =
        address as *mut ::core::ffi::c_void as *mut GUnixSocketAddress;
    let mut sock: *mut sockaddr_un = ::core::ptr::null_mut::<sockaddr_un>();
    let mut socklen: gssize = 0;
    socklen = safe_c2rust_g_unix_socket_address_get_native_size(address);
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if socklen >= 0 as gssize {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gunixsocketaddress.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            218 as ::core::ffi::c_int,
            G_STRFUNC,
            b"socklen >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if destlen < socklen as gsize {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NO_SPACE as ::core::ffi::c_int as gint,
            glib_gettext(b"Not enough space for socket address\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    sock = dest as *mut sockaddr_un;
    memset(
        sock as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        socklen as size_t,
    );
    (*sock).sun_family = AF_UNIX as sa_family_t;
    match (*(*addr).priv_0).address_type as ::core::ffi::c_uint {
        2 => {
            strcpy(
                &raw mut (*sock).sun_path as *mut ::core::ffi::c_char,
                &raw mut (*(*addr).priv_0).path as *mut ::core::ffi::c_char,
            );
        }
        3 | 4 => {
            if safe_c2rust_g_unix_socket_address_abstract_names_supported() == 0 {
                g_set_error_literal(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Abstract UNIX domain socket addresses not supported on this system\0"
                            as *const u8 as *const gchar,
                    ),
                );
                return FALSE;
            }
            (*sock).sun_path[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
            memcpy(
                (&raw mut (*sock).sun_path as *mut ::core::ffi::c_char)
                    .offset(1 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_void,
                &raw mut (*(*addr).priv_0).path as *mut ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                (*(*addr).priv_0).path_len as size_t,
            );
        }
        0 | 1 | _ => {}
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_unix_socket_address_class_init(
    mut klass: *mut GUnixSocketAddressClass,
) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    let mut gsocketaddress_class: *mut GSocketAddressClass =
        klass as *mut ::core::ffi::c_void as *mut GSocketAddressClass;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_unix_socket_address_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_unix_socket_address_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*gsocketaddress_class).get_family = Some(
        safe_c2rust_g_unix_socket_address_get_family
            as unsafe extern "C" fn(*mut GSocketAddress) -> GSocketFamily,
    )
        as Option<unsafe extern "C" fn(*mut GSocketAddress) -> GSocketFamily>;
    (*gsocketaddress_class).to_native = Some(
        safe_c2rust_g_unix_socket_address_to_native
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
        safe_c2rust_g_unix_socket_address_get_native_size
            as unsafe extern "C" fn(*mut GSocketAddress) -> gssize,
    )
        as Option<unsafe extern "C" fn(*mut GSocketAddress) -> gssize>;
    g_object_class_install_property(
        gobject_class,
        PROP_PATH as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"path\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_PATH_AS_ARRAY as ::core::ffi::c_int as guint,
        g_param_spec_boxed(
            b"path-as-array\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_byte_array_get_type(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_ABSTRACT as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"abstract\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_ADDRESS_TYPE as ::core::ffi::c_int as guint,
        g_param_spec_enum(
            b"address-type\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_unix_socket_address_type_get_type(),
            G_UNIX_SOCKET_ADDRESS_PATH as ::core::ffi::c_int as gint,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
unsafe extern "C" fn safe_c2rust_g_unix_socket_address_connectable_iface_init(
    mut iface: *mut GSocketConnectableIface,
) {
    let mut parent_iface: *mut GSocketConnectableIface =
        g_type_interface_peek_parent(iface as gpointer) as *mut GSocketConnectableIface;
    (*iface).enumerate = (*parent_iface).enumerate;
    (*iface).proxy_enumerate = (*parent_iface).proxy_enumerate;
    (*iface).to_string = Some(
        safe_c2rust_g_unix_socket_address_connectable_to_string
            as unsafe extern "C" fn(*mut GSocketConnectable) -> *mut gchar,
    )
        as Option<unsafe extern "C" fn(*mut GSocketConnectable) -> *mut gchar>;
}
unsafe extern "C" fn safe_c2rust_g_unix_socket_address_connectable_to_string(
    mut connectable: *mut GSocketConnectable,
) -> *mut gchar {
    let mut ua: *mut GUnixSocketAddress = ::core::ptr::null_mut::<GUnixSocketAddress>();
    let mut out: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut path: *const gchar = ::core::ptr::null::<gchar>();
    let mut path_len: gsize = 0;
    let mut i: gsize = 0;
    ua = connectable as *mut ::core::ffi::c_void as *mut GUnixSocketAddress;
    if (*(*ua).priv_0).address_type as ::core::ffi::c_uint
        == G_UNIX_SOCKET_ADDRESS_ANONYMOUS as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return safe_c2rust_g_strdup_inline(
            b"anonymous\0" as *const u8 as *const ::core::ffi::c_char,
        ) as *mut gchar;
    }
    path = safe_c2rust_g_unix_socket_address_get_path(ua) as *const gchar;
    path_len = safe_c2rust_g_unix_socket_address_get_path_len(ua);
    out = g_string_sized_new(path_len);
    i = 0 as gsize;
    while i < path_len {
        let mut c: guint8 = *path.offset(i as isize) as guint8;
        if *safe_c2rust_g_ascii_table.offset(*path.offset(i as isize) as guchar as isize)
            as ::core::ffi::c_int
            & G_ASCII_PRINT as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            safe_c2rust_g_string_append_c_inline(out, c as gchar);
        } else {
            g_string_append_printf(out, b"\\x%02x\0" as *const u8 as *const gchar, c as guint);
        }
        i = i.wrapping_add(1);
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
unsafe extern "C" fn safe_c2rust_g_unix_socket_address_init(mut address: *mut GUnixSocketAddress) {
    (*address).priv_0 = safe_c2rust_g_unix_socket_address_get_instance_private(address)
        as *mut GUnixSocketAddressPrivate;
    memset(
        &raw mut (*(*address).priv_0).path as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[::core::ffi::c_char; 108]>() as size_t,
    );
    (*(*address).priv_0).path_len = -(1 as ::core::ffi::c_int) as gsize;
    (*(*address).priv_0).address_type = G_UNIX_SOCKET_ADDRESS_PATH;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_socket_address_new(
    mut path: *const gchar,
) -> *mut GSocketAddress {
    return g_object_new(
        safe_c2rust_g_unix_socket_address_get_type(),
        b"path\0" as *const u8 as *const gchar,
        path,
        b"abstract\0" as *const u8 as *const ::core::ffi::c_char,
        FALSE,
        NULL_0,
    ) as *mut GSocketAddress;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_socket_address_new_abstract(
    mut path: *const gchar,
    mut path_len: gint,
) -> *mut GSocketAddress {
    return safe_c2rust_g_unix_socket_address_new_with_type(
        path,
        path_len,
        G_UNIX_SOCKET_ADDRESS_ABSTRACT_PADDED,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_socket_address_new_with_type(
    mut path: *const gchar,
    mut path_len: gint,
    mut type_0: GUnixSocketAddressType,
) -> *mut GSocketAddress {
    let mut address: *mut GSocketAddress = ::core::ptr::null_mut::<GSocketAddress>();
    let mut array: *mut GByteArray = ::core::ptr::null_mut::<GByteArray>();
    if type_0 as ::core::ffi::c_uint
        == G_UNIX_SOCKET_ADDRESS_ANONYMOUS as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        path_len = 0 as ::core::ffi::c_int as gint;
    } else if path_len == -(1 as ::core::ffi::c_int) {
        path_len = strlen(path as *const ::core::ffi::c_char) as gint;
    }
    array = g_byte_array_sized_new(path_len as guint);
    g_byte_array_append(array, path as *mut guint8, path_len as guint);
    address = g_object_new(
        safe_c2rust_g_unix_socket_address_get_type(),
        b"path-as-array\0" as *const u8 as *const gchar,
        array,
        b"address-type\0" as *const u8 as *const ::core::ffi::c_char,
        type_0 as ::core::ffi::c_uint,
        NULL_0,
    ) as *mut GSocketAddress;
    g_byte_array_unref(array);
    return address;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_socket_address_get_path(
    mut address: *mut GUnixSocketAddress,
) -> *const ::core::ffi::c_char {
    return &raw mut (*(*address).priv_0).path as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_socket_address_get_path_len(
    mut address: *mut GUnixSocketAddress,
) -> gsize {
    return (*(*address).priv_0).path_len;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_socket_address_get_address_type(
    mut address: *mut GUnixSocketAddress,
) -> GUnixSocketAddressType {
    return (*(*address).priv_0).address_type;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_socket_address_get_is_abstract(
    mut address: *mut GUnixSocketAddress,
) -> gboolean {
    return ((*(*address).priv_0).address_type as ::core::ffi::c_uint
        == G_UNIX_SOCKET_ADDRESS_ABSTRACT as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*(*address).priv_0).address_type as ::core::ffi::c_uint
            == G_UNIX_SOCKET_ADDRESS_ABSTRACT_PADDED as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_socket_address_abstract_names_supported() -> gboolean {
    return TRUE;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
