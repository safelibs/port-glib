extern "C" {
    pub type _GData;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _GInetAddressPrivate;
    pub type _GSocketConnectable;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
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
    fn g_uri_join(
        flags: GUriFlags,
        scheme: *const gchar,
        userinfo: *const gchar,
        host: *const gchar,
        port: gint,
        path: *const gchar,
        query: *const gchar,
        fragment: *const gchar,
    ) -> *mut gchar;
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
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_get(object: gpointer, first_property_name: *const gchar, ...);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_value_set_enum(value: *mut GValue, v_enum: gint);
    fn g_param_spec_enum(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        enum_type: GType,
        default_value: gint,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_inet_address_new_from_bytes(
        bytes: *const guint8,
        family: GSocketFamily,
    ) -> *mut GInetAddress;
    fn g_inet_address_to_string(address: *mut GInetAddress) -> *mut gchar;
    fn g_inet_socket_address_get_type() -> GType;
    fn g_inet_socket_address_new(address: *mut GInetAddress, port: guint16) -> *mut GSocketAddress;
    fn g_native_socket_address_new(native: gpointer, len: gsize) -> *mut GSocketAddress;
    fn g_proxy_address_get_type() -> GType;
    fn g_socket_address_enumerator_get_type() -> GType;
    fn g_proxy_address_enumerator_get_type() -> GType;
    fn g_socket_connectable_get_type() -> GType;
    fn g_socket_family_get_type() -> GType;
    fn g_unix_socket_address_new(path: *const gchar) -> *mut GSocketAddress;
    fn g_unix_socket_address_new_with_type(
        path: *const gchar,
        path_len: gint,
        type_0: GUnixSocketAddressType,
    ) -> *mut GSocketAddress;
    fn g_unix_socket_address_abstract_names_supported() -> gboolean;
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
pub type gchar = ::core::ffi::c_char;
pub type gshort = ::core::ffi::c_short;
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
pub type GUriFlags = ::core::ffi::c_uint;
pub const G_URI_FLAGS_SCHEME_NORMALIZE: GUriFlags = 256;
pub const G_URI_FLAGS_ENCODED_FRAGMENT: GUriFlags = 128;
pub const G_URI_FLAGS_ENCODED_PATH: GUriFlags = 64;
pub const G_URI_FLAGS_ENCODED_QUERY: GUriFlags = 32;
pub const G_URI_FLAGS_NON_DNS: GUriFlags = 16;
pub const G_URI_FLAGS_ENCODED: GUriFlags = 8;
pub const G_URI_FLAGS_HAS_AUTH_PARAMS: GUriFlags = 4;
pub const G_URI_FLAGS_HAS_PASSWORD: GUriFlags = 2;
pub const G_URI_FLAGS_PARSE_RELAXED: GUriFlags = 1;
pub const G_URI_FLAGS_NONE: GUriFlags = 0;
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
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
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
pub const PROP_FAMILY: C2RustUnnamed_1 = 1;
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
pub struct GSocketAddressAddressEnumerator {
    pub parent_instance: GSocketAddressEnumerator,
    pub sockaddr: *mut GSocketAddress,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GSocketAddressAddressEnumeratorClass {
    pub parent_class: GSocketAddressEnumeratorClass,
}
pub type GSocketAddressEnumeratorClass = _GSocketAddressEnumeratorClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketAddressEnumeratorClass {
    pub parent_class: GObjectClass,
    pub next: Option<
        unsafe extern "C" fn(
            *mut GSocketAddressEnumerator,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GSocketAddress,
    >,
    pub next_async: Option<
        unsafe extern "C" fn(
            *mut GSocketAddressEnumerator,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub next_finish: Option<
        unsafe extern "C" fn(
            *mut GSocketAddressEnumerator,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> *mut GSocketAddress,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [::core::ffi::c_char; 108],
}
pub type sa_family_t = ::core::ffi::c_ushort;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type uint16_t = __uint16_t;
pub type uint8_t = __uint8_t;
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
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
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [::core::ffi::c_char; 14],
}
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const PROP_NONE: C2RustUnnamed_1 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn safe_c2rust___bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as __uint32_t) >> 24 as ::core::ffi::c_int
        | (__bsx & 0xff0000 as __uint32_t) >> 8 as ::core::ffi::c_int
        | (__bsx & 0xff00 as __uint32_t) << 8 as ::core::ffi::c_int
        | (__bsx & 0xff as __uint32_t) << 24 as ::core::ffi::c_int;
}
pub const PF_UNSPEC: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PF_LOCAL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PF_UNIX: ::core::ffi::c_int = PF_LOCAL;
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PF_INET6: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const AF_UNSPEC: ::core::ffi::c_int = PF_UNSPEC;
pub const AF_UNIX: ::core::ffi::c_int = PF_UNIX;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const AF_INET6: ::core::ffi::c_int = PF_INET6;
static mut safe_c2rust_GSocketAddress_private_offset: gint = 0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_address_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_socket_address_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_socket_address_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_socket_address_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GSocketAddress_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GSocketAddress_private_offset,
        );
    }
    safe_c2rust_g_socket_address_class_init(klass as *mut GSocketAddressClass);
}
static mut safe_c2rust_g_socket_address_parent_class: gpointer = NULL;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_socket_address_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GSocketAddress\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GSocketAddressClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_socket_address_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GSocketAddress>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GSocketAddress) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_socket_address_init
                    as unsafe extern "C" fn(*mut GSocketAddress) -> (),
            )),
        ),
        G_TYPE_FLAG_ABSTRACT,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GSocketConnectableIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_socket_address_connectable_iface_init
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_address_get_family(
    mut address: *mut GSocketAddress,
) -> GSocketFamily {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = address as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_address_get_type();
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
            b"G_IS_SOCKET_ADDRESS (address)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_SOCKET_FAMILY_INVALID;
    }
    return (*((*(address as *mut GTypeInstance)).g_class as *mut GSocketAddressClass))
        .get_family
        .expect("non-null function pointer")(address);
}
unsafe extern "C" fn safe_c2rust_g_socket_address_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut address: *mut GSocketAddress =
        object as *mut ::core::ffi::c_void as *mut GSocketAddress;
    match prop_id {
        1 => {
            g_value_set_enum(
                value,
                safe_c2rust_g_socket_address_get_family(address) as gint,
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocketaddress.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                101 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_socket_address_class_init(mut klass: *mut GSocketAddressClass) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_socket_address_get_property
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
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
unsafe extern "C" fn safe_c2rust_g_socket_address_connectable_iface_init(
    mut connectable_iface: *mut GSocketConnectableIface,
) {
    (*connectable_iface).enumerate = Some(
        safe_c2rust_g_socket_address_connectable_enumerate
            as unsafe extern "C" fn(*mut GSocketConnectable) -> *mut GSocketAddressEnumerator,
    )
        as Option<unsafe extern "C" fn(*mut GSocketConnectable) -> *mut GSocketAddressEnumerator>;
    (*connectable_iface).proxy_enumerate = Some(
        safe_c2rust_g_socket_address_connectable_proxy_enumerate
            as unsafe extern "C" fn(*mut GSocketConnectable) -> *mut GSocketAddressEnumerator,
    )
        as Option<unsafe extern "C" fn(*mut GSocketConnectable) -> *mut GSocketAddressEnumerator>;
}
unsafe extern "C" fn safe_c2rust_g_socket_address_init(mut address: *mut GSocketAddress) {}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_address_get_native_size(
    mut address: *mut GSocketAddress,
) -> gssize {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = address as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_address_get_type();
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
            b"G_IS_SOCKET_ADDRESS (address)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    return (*((*(address as *mut GTypeInstance)).g_class as *mut GSocketAddressClass))
        .get_native_size
        .expect("non-null function pointer")(address);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_address_to_native(
    mut address: *mut GSocketAddress,
    mut dest: gpointer,
    mut destlen: gsize,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = address as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_address_get_type();
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
            b"G_IS_SOCKET_ADDRESS (address)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*((*(address as *mut GTypeInstance)).g_class as *mut GSocketAddressClass))
        .to_native
        .expect("non-null function pointer")(address, dest, destlen, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_address_new_from_native(
    mut native: gpointer,
    mut len: gsize,
) -> *mut GSocketAddress {
    let mut family: gshort = 0;
    if (len as usize) < ::core::mem::size_of::<gshort>() as usize {
        return ::core::ptr::null_mut::<GSocketAddress>();
    }
    family = (*(native as *mut sockaddr)).sa_family as gshort;
    if family as ::core::ffi::c_int == AF_UNSPEC {
        return ::core::ptr::null_mut::<GSocketAddress>();
    }
    if family as ::core::ffi::c_int == AF_INET {
        let mut addr: *mut sockaddr_in = native as *mut sockaddr_in;
        let mut iaddr: *mut GInetAddress = ::core::ptr::null_mut::<GInetAddress>();
        let mut sockaddr: *mut GSocketAddress = ::core::ptr::null_mut::<GSocketAddress>();
        if (len as usize) < ::core::mem::size_of::<sockaddr_in>() as usize {
            return ::core::ptr::null_mut::<GSocketAddress>();
        }
        iaddr = g_inet_address_new_from_bytes(
            &raw mut (*addr).sin_addr as *mut guint8,
            G_SOCKET_FAMILY_IPV4,
        );
        sockaddr = g_inet_socket_address_new(
            iaddr,
            (((*addr).sin_port as guint16 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int)
                as guint16 as ::core::ffi::c_int
                | (((*addr).sin_port as guint16 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                    as guint16 as ::core::ffi::c_int) as guint16,
        );
        g_object_unref(iaddr as gpointer);
        return sockaddr;
    }
    if family as ::core::ffi::c_int == AF_INET6 {
        let mut addr_0: *mut sockaddr_in6 = native as *mut sockaddr_in6;
        let mut iaddr_0: *mut GInetAddress = ::core::ptr::null_mut::<GInetAddress>();
        let mut sockaddr_0: *mut GSocketAddress = ::core::ptr::null_mut::<GSocketAddress>();
        if (len as usize) < ::core::mem::size_of::<sockaddr_in6>() as usize {
            return ::core::ptr::null_mut::<GSocketAddress>();
        }
        if ({
            let mut __a: *const in6_addr = &raw mut (*addr_0).sin6_addr as *const in6_addr;
            ((*__a).__in6_u.__u6_addr32[0 as ::core::ffi::c_int as usize] == 0 as uint32_t
                && (*__a).__in6_u.__u6_addr32[1 as ::core::ffi::c_int as usize] == 0 as uint32_t
                && (*__a).__in6_u.__u6_addr32[2 as ::core::ffi::c_int as usize]
                    == safe_c2rust___bswap_32(0xffff as __uint32_t))
                as ::core::ffi::c_int
        }) != 0
        {
            let mut sin_addr: sockaddr_in = sockaddr_in {
                sin_family: 0,
                sin_port: 0,
                sin_addr: in_addr { s_addr: 0 },
                sin_zero: [0; 8],
            };
            sin_addr.sin_family = AF_INET as sa_family_t;
            sin_addr.sin_port = (*addr_0).sin6_port;
            memcpy(
                &raw mut sin_addr.sin_addr.s_addr as *mut ::core::ffi::c_void,
                (&raw mut (*addr_0).sin6_addr.__in6_u.__u6_addr8 as *mut uint8_t)
                    .offset(12 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_void,
                4 as size_t,
            );
            iaddr_0 = g_inet_address_new_from_bytes(
                &raw mut sin_addr.sin_addr as *mut guint8,
                G_SOCKET_FAMILY_IPV4,
            );
        } else {
            iaddr_0 = g_inet_address_new_from_bytes(
                &raw mut (*addr_0).sin6_addr as *mut guint8,
                G_SOCKET_FAMILY_IPV6,
            );
        }
        sockaddr_0 = g_object_new(
            g_inet_socket_address_get_type(),
            b"address\0" as *const u8 as *const gchar,
            iaddr_0,
            b"port\0" as *const u8 as *const ::core::ffi::c_char,
            (((*addr_0).sin6_port as guint16 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int)
                as guint16 as ::core::ffi::c_int
                | (((*addr_0).sin6_port as guint16 as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int) as guint16 as ::core::ffi::c_int)
                as guint16 as ::core::ffi::c_int,
            b"flowinfo\0" as *const u8 as *const ::core::ffi::c_char,
            (*addr_0).sin6_flowinfo,
            b"scope_id\0" as *const u8 as *const ::core::ffi::c_char,
            (*addr_0).sin6_scope_id,
            NULL,
        ) as *mut GSocketAddress;
        g_object_unref(iaddr_0 as gpointer);
        return sockaddr_0;
    }
    if family as ::core::ffi::c_int == AF_UNIX {
        let mut addr_1: *mut sockaddr_un = native as *mut sockaddr_un;
        let mut path_len: gint =
            len.wrapping_sub(2 as ::core::ffi::c_ulong as glong as gsize) as gint;
        if path_len == 0 as ::core::ffi::c_int {
            return g_unix_socket_address_new_with_type(
                b"\0" as *const u8 as *const gchar,
                0 as gint,
                G_UNIX_SOCKET_ADDRESS_ANONYMOUS,
            );
        } else if (*addr_1).sun_path[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            if g_unix_socket_address_abstract_names_supported() == 0 {
                return g_unix_socket_address_new_with_type(
                    b"\0" as *const u8 as *const gchar,
                    0 as gint,
                    G_UNIX_SOCKET_ADDRESS_ANONYMOUS,
                );
            } else if (len as usize) < ::core::mem::size_of::<sockaddr_un>() as usize {
                return g_unix_socket_address_new_with_type(
                    (&raw mut (*addr_1).sun_path as *mut ::core::ffi::c_char)
                        .offset(1 as ::core::ffi::c_int as isize),
                    path_len - 1 as gint,
                    G_UNIX_SOCKET_ADDRESS_ABSTRACT,
                );
            } else {
                return g_unix_socket_address_new_with_type(
                    (&raw mut (*addr_1).sun_path as *mut ::core::ffi::c_char)
                        .offset(1 as ::core::ffi::c_int as isize),
                    path_len - 1 as gint,
                    G_UNIX_SOCKET_ADDRESS_ABSTRACT_PADDED,
                );
            }
        } else {
            return g_unix_socket_address_new(
                &raw mut (*addr_1).sun_path as *mut ::core::ffi::c_char,
            );
        }
    }
    return g_native_socket_address_new(native, len);
}
unsafe extern "C" fn safe_c2rust__g_socket_address_address_enumerator_get_type() -> GType {
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
        let mut g_define_type_id: GType =
            safe_c2rust__g_socket_address_address_enumerator_get_type_once();
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
unsafe extern "C" fn safe_c2rust__g_socket_address_address_enumerator_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_socket_address_enumerator_get_type(),
        g_intern_static_string(b"GSocketAddressAddressEnumerator\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GSocketAddressAddressEnumeratorClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust__g_socket_address_address_enumerator_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GSocketAddressAddressEnumerator>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GSocketAddressAddressEnumerator) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust__g_socket_address_address_enumerator_init
                    as unsafe extern "C" fn(*mut GSocketAddressAddressEnumerator) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust__g_socket_address_address_enumerator_class_intern_init(
    mut klass: gpointer,
) {
    safe_c2rust__g_socket_address_address_enumerator_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GSocketAddressAddressEnumerator_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GSocketAddressAddressEnumerator_private_offset,
        );
    }
    safe_c2rust__g_socket_address_address_enumerator_class_init(
        klass as *mut GSocketAddressAddressEnumeratorClass,
    );
}
static mut safe_c2rust__g_socket_address_address_enumerator_parent_class: gpointer = NULL;
static mut safe_c2rust_GSocketAddressAddressEnumerator_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_socket_address_address_enumerator_finalize(
    mut object: *mut GObject,
) {
    let mut sockaddr_enum: *mut GSocketAddressAddressEnumerator =
        object as *mut ::core::ffi::c_void as *mut GSocketAddressAddressEnumerator;
    if !(*sockaddr_enum).sockaddr.is_null() {
        g_object_unref((*sockaddr_enum).sockaddr as gpointer);
    }
    (*(safe_c2rust__g_socket_address_address_enumerator_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_socket_address_address_enumerator_next(
    mut enumerator: *mut GSocketAddressEnumerator,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GSocketAddress {
    let mut sockaddr_enum: *mut GSocketAddressAddressEnumerator =
        enumerator as *mut ::core::ffi::c_void as *mut GSocketAddressAddressEnumerator;
    if !(*sockaddr_enum).sockaddr.is_null() {
        let mut ret: *mut GSocketAddress = (*sockaddr_enum).sockaddr;
        (*sockaddr_enum).sockaddr = ::core::ptr::null_mut::<GSocketAddress>();
        return ret;
    } else {
        return ::core::ptr::null_mut::<GSocketAddress>();
    };
}
unsafe extern "C" fn safe_c2rust__g_socket_address_address_enumerator_init(
    mut enumerator: *mut GSocketAddressAddressEnumerator,
) {
}
unsafe extern "C" fn safe_c2rust__g_socket_address_address_enumerator_class_init(
    mut sockaddrenum_class: *mut GSocketAddressAddressEnumeratorClass,
) {
    let mut object_class: *mut GObjectClass =
        sockaddrenum_class as *mut ::core::ffi::c_void as *mut GObjectClass;
    let mut enumerator_class: *mut GSocketAddressEnumeratorClass =
        sockaddrenum_class as *mut ::core::ffi::c_void as *mut GSocketAddressEnumeratorClass;
    (*enumerator_class).next = Some(
        safe_c2rust_g_socket_address_address_enumerator_next
            as unsafe extern "C" fn(
                *mut GSocketAddressEnumerator,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GSocketAddress,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GSocketAddressEnumerator,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GSocketAddress,
        >;
    (*object_class).finalize = Some(
        safe_c2rust_g_socket_address_address_enumerator_finalize
            as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
}
unsafe extern "C" fn safe_c2rust_g_socket_address_connectable_enumerate(
    mut connectable: *mut GSocketConnectable,
) -> *mut GSocketAddressEnumerator {
    let mut sockaddr_enum: *mut GSocketAddressAddressEnumerator =
        ::core::ptr::null_mut::<GSocketAddressAddressEnumerator>();
    sockaddr_enum = g_object_new(
        safe_c2rust__g_socket_address_address_enumerator_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GSocketAddressAddressEnumerator;
    (*sockaddr_enum).sockaddr =
        g_object_ref(connectable as *mut ::core::ffi::c_void as *mut GSocketAddress as gpointer)
            as *mut GSocketAddress as *mut GSocketAddress;
    return sockaddr_enum as *mut GSocketAddressEnumerator;
}
unsafe extern "C" fn safe_c2rust_g_socket_address_connectable_proxy_enumerate(
    mut connectable: *mut GSocketConnectable,
) -> *mut GSocketAddressEnumerator {
    let mut addr_enum: *mut GSocketAddressEnumerator =
        ::core::ptr::null_mut::<GSocketAddressEnumerator>();
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !connectable.is_null() {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsocketaddress.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            386 as ::core::ffi::c_int,
            G_STRFUNC,
            b"connectable != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut __inst: *mut GTypeInstance = connectable as *mut GTypeInstance;
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
        && ({
            let mut __inst: *mut GTypeInstance = connectable as *mut GTypeInstance;
            let mut __t: GType = g_proxy_address_get_type();
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
        let mut addr: *mut GInetAddress = ::core::ptr::null_mut::<GInetAddress>();
        let mut port: guint = 0;
        let mut uri: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut ip: *mut gchar = ::core::ptr::null_mut::<gchar>();
        g_object_get(
            connectable as gpointer,
            b"address\0" as *const u8 as *const gchar,
            &raw mut addr,
            b"port\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut port,
            NULL,
        );
        ip = g_inet_address_to_string(addr);
        uri = g_uri_join(
            G_URI_FLAGS_NONE,
            b"none\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ip,
            port as gint,
            b"\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
        );
        addr_enum = g_object_new(
            g_proxy_address_enumerator_get_type(),
            b"connectable\0" as *const u8 as *const gchar,
            connectable,
            b"uri\0" as *const u8 as *const ::core::ffi::c_char,
            uri,
            NULL,
        ) as *mut GSocketAddressEnumerator;
        g_object_unref(addr as gpointer);
        g_free(ip as gpointer);
        g_free(uri as gpointer);
    } else {
        addr_enum = safe_c2rust_g_socket_address_connectable_enumerate(connectable);
    }
    return addr_enum;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
