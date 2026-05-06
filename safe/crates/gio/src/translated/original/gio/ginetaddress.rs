use ::c2rust_asm_casts;
use ::c2rust_asm_casts::AsmCastTrait;
use ::core::arch::asm;
extern "C" {
    pub type _GData;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
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
    fn g_type_add_instance_private(class_type: GType, private_size: gsize) -> gint;
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
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
    fn g_param_spec_pointer(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_value_set_boolean(value: *mut GValue, v_boolean: gboolean);
    fn g_value_set_pointer(value: *mut GValue, v_pointer: gpointer);
    fn g_value_get_pointer(value: *const GValue) -> gpointer;
    fn g_socket_family_get_type() -> GType;
    static safe_c2rust_in6addr_any: in6_addr;
    static safe_c2rust_in6addr_loopback: in6_addr;
    fn inet_pton(
        __af: ::core::ffi::c_int,
        __cp: *const ::core::ffi::c_char,
        __buf: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn inet_ntop(
        __af: ::core::ffi::c_int,
        __cp: *const ::core::ffi::c_void,
        __buf: *mut ::core::ffi::c_char,
        __len: socklen_t,
    ) -> *const ::core::ffi::c_char;
    fn g_networking_init();
}
pub type size_t = usize;
pub type guint8 = ::core::ffi::c_uchar;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
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
pub type socklen_t = __socklen_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInetAddressPrivate {
    pub family: GSocketFamily,
    pub addr: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ipv4: in_addr,
    pub ipv6: in6_addr,
}
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
pub type uint32_t = __uint32_t;
pub type uint16_t = __uint16_t;
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
pub type GInetAddress = _GInetAddress;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInetAddressClass {
    pub parent_class: GObjectClass,
    pub to_string: Option<unsafe extern "C" fn(*mut GInetAddress) -> *mut gchar>,
    pub to_bytes: Option<unsafe extern "C" fn(*mut GInetAddress) -> *const guint8>,
}
pub type GInetAddressClass = _GInetAddressClass;
pub const PROP_IS_MC_SITE_LOCAL: C2RustUnnamed_2 = 12;
pub const PROP_IS_MC_ORG_LOCAL: C2RustUnnamed_2 = 11;
pub const PROP_IS_MC_NODE_LOCAL: C2RustUnnamed_2 = 10;
pub const PROP_IS_MC_LINK_LOCAL: C2RustUnnamed_2 = 9;
pub const PROP_IS_MC_GLOBAL: C2RustUnnamed_2 = 8;
pub const PROP_IS_MULTICAST: C2RustUnnamed_2 = 7;
pub const PROP_IS_SITE_LOCAL: C2RustUnnamed_2 = 6;
pub const PROP_IS_LOOPBACK: C2RustUnnamed_2 = 4;
pub const PROP_IS_LINK_LOCAL: C2RustUnnamed_2 = 5;
pub const PROP_IS_ANY: C2RustUnnamed_2 = 3;
pub const PROP_BYTES: C2RustUnnamed_2 = 2;
pub const PROP_FAMILY: C2RustUnnamed_2 = 1;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_2 = 0;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn safe_c2rust___bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as __uint32_t) >> 24 as ::core::ffi::c_int
        | (__bsx & 0xff0000 as __uint32_t) >> 8 as ::core::ffi::c_int
        | (__bsx & 0xff00 as __uint32_t) << 8 as ::core::ffi::c_int
        | (__bsx & 0xff as __uint32_t) << 24 as ::core::ffi::c_int;
}
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
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PF_INET6: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const AF_INET6: ::core::ffi::c_int = PF_INET6;
pub const INADDR_ANY: in_addr_t = 0 as ::core::ffi::c_int as in_addr_t;
static mut safe_c2rust_g_inet_address_parent_class: gpointer = NULL_0;
unsafe extern "C" fn safe_c2rust_g_inet_address_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_inet_address_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GInetAddress_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut safe_c2rust_GInetAddress_private_offset);
    }
    safe_c2rust_g_inet_address_class_init(klass as *mut GInetAddressClass);
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_inet_address_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GInetAddress\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GInetAddressClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_inet_address_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GInetAddress>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GInetAddress) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_inet_address_init as unsafe extern "C" fn(*mut GInetAddress) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GInetAddress_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GInetAddressPrivate>() as gsize,
    );
    g_networking_init();
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_inet_address_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_inet_address_get_type_once();
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
#[inline]
unsafe extern "C" fn safe_c2rust_g_inet_address_get_instance_private(
    mut self_0: *mut GInetAddress,
) -> gpointer {
    return (self_0 as *mut guint8).offset(safe_c2rust_GInetAddress_private_offset as glong as isize)
        as gpointer;
}
static mut safe_c2rust_GInetAddress_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_inet_address_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut address: *mut GInetAddress = object as *mut ::core::ffi::c_void as *mut GInetAddress;
    match prop_id {
        1 => {
            (*(*address).priv_0).family = g_value_get_enum(value) as GSocketFamily;
        }
        2 => {
            memcpy(
                &raw mut (*(*address).priv_0).addr as *mut ::core::ffi::c_void,
                g_value_get_pointer(value) as *const ::core::ffi::c_void,
                if (*(*address).priv_0).family as ::core::ffi::c_uint
                    == AF_INET as ::core::ffi::c_uint
                {
                    ::core::mem::size_of::<in_addr>() as size_t
                } else {
                    ::core::mem::size_of::<in6_addr>() as size_t
                },
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/ginetaddress.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                111 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_inet_address_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut address: *mut GInetAddress = object as *mut ::core::ffi::c_void as *mut GInetAddress;
    match prop_id {
        1 => {
            g_value_set_enum(value, (*(*address).priv_0).family as gint);
        }
        2 => {
            g_value_set_pointer(value, &raw mut (*(*address).priv_0).addr as gpointer);
        }
        3 => {
            g_value_set_boolean(value, safe_c2rust_g_inet_address_get_is_any(address));
        }
        4 => {
            g_value_set_boolean(value, safe_c2rust_g_inet_address_get_is_loopback(address));
        }
        5 => {
            g_value_set_boolean(value, safe_c2rust_g_inet_address_get_is_link_local(address));
        }
        6 => {
            g_value_set_boolean(value, safe_c2rust_g_inet_address_get_is_site_local(address));
        }
        7 => {
            g_value_set_boolean(value, safe_c2rust_g_inet_address_get_is_multicast(address));
        }
        8 => {
            g_value_set_boolean(value, safe_c2rust_g_inet_address_get_is_mc_global(address));
        }
        9 => {
            g_value_set_boolean(
                value,
                safe_c2rust_g_inet_address_get_is_mc_link_local(address),
            );
        }
        10 => {
            g_value_set_boolean(
                value,
                safe_c2rust_g_inet_address_get_is_mc_node_local(address),
            );
        }
        11 => {
            g_value_set_boolean(
                value,
                safe_c2rust_g_inet_address_get_is_mc_org_local(address),
            );
        }
        12 => {
            g_value_set_boolean(
                value,
                safe_c2rust_g_inet_address_get_is_mc_site_local(address),
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/ginetaddress.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                176 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_inet_address_class_init(mut klass: *mut GInetAddressClass) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_inet_address_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_inet_address_get_property
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
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_BYTES as ::core::ffi::c_int as guint,
        g_param_spec_pointer(
            b"bytes\0" as *const u8 as *const gchar,
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
        PROP_IS_ANY as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"is-any\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_IS_LINK_LOCAL as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"is-link-local\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_IS_LOOPBACK as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"is-loopback\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_IS_SITE_LOCAL as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"is-site-local\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_IS_MULTICAST as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"is-multicast\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_IS_MC_GLOBAL as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"is-mc-global\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_IS_MC_LINK_LOCAL as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"is-mc-link-local\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_IS_MC_NODE_LOCAL as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"is-mc-node-local\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_IS_MC_ORG_LOCAL as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"is-mc-org-local\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_IS_MC_SITE_LOCAL as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"is-mc-site-local\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
unsafe extern "C" fn safe_c2rust_g_inet_address_init(mut address: *mut GInetAddress) {
    (*address).priv_0 =
        safe_c2rust_g_inet_address_get_instance_private(address) as *mut GInetAddressPrivate;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_inet_address_new_from_string(
    mut string: *const gchar,
) -> *mut GInetAddress {
    let mut in_addr: in_addr = in_addr { s_addr: 0 };
    let mut in6_addr: in6_addr = in6_addr {
        __in6_u: C2RustUnnamed_1 {
            __u6_addr8: [0; 16],
        },
    };
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !string.is_null() {
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
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GInetAddress>();
    }
    g_networking_init();
    if inet_pton(
        AF_INET,
        string as *const ::core::ffi::c_char,
        &raw mut in_addr as *mut ::core::ffi::c_void,
    ) > 0 as ::core::ffi::c_int
    {
        return safe_c2rust_g_inet_address_new_from_bytes(
            &raw mut in_addr as *mut guint8,
            G_SOCKET_FAMILY_IPV4,
        );
    } else if inet_pton(
        AF_INET6,
        string as *const ::core::ffi::c_char,
        &raw mut in6_addr as *mut ::core::ffi::c_void,
    ) > 0 as ::core::ffi::c_int
    {
        return safe_c2rust_g_inet_address_new_from_bytes(
            &raw mut in6_addr as *mut guint8,
            G_SOCKET_FAMILY_IPV6,
        );
    }
    return ::core::ptr::null_mut::<GInetAddress>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_inet_address_new_from_bytes(
    mut bytes: *const guint8,
    mut family: GSocketFamily,
) -> *mut GInetAddress {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if family as ::core::ffi::c_uint == 2 as ::core::ffi::c_uint
            || family as ::core::ffi::c_uint == 10 as ::core::ffi::c_uint
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
            b"G_INET_ADDRESS_FAMILY_IS_VALID (family)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GInetAddress>();
    }
    return g_object_new(
        safe_c2rust_g_inet_address_get_type(),
        b"family\0" as *const u8 as *const gchar,
        family as ::core::ffi::c_uint,
        b"bytes\0" as *const u8 as *const ::core::ffi::c_char,
        bytes,
        NULL_0,
    ) as *mut GInetAddress;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_inet_address_new_loopback(
    mut family: GSocketFamily,
) -> *mut GInetAddress {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if family as ::core::ffi::c_uint == 2 as ::core::ffi::c_uint
            || family as ::core::ffi::c_uint == 10 as ::core::ffi::c_uint
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
            b"G_INET_ADDRESS_FAMILY_IS_VALID (family)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GInetAddress>();
    }
    if family as ::core::ffi::c_uint == AF_INET as ::core::ffi::c_uint {
        let mut addr: [guint8; 4] = [
            127 as ::core::ffi::c_int as guint8,
            0 as ::core::ffi::c_int as guint8,
            0 as ::core::ffi::c_int as guint8,
            1 as ::core::ffi::c_int as guint8,
        ];
        return safe_c2rust_g_inet_address_new_from_bytes(&raw mut addr as *mut guint8, family);
    } else {
        return safe_c2rust_g_inet_address_new_from_bytes(
            &raw const safe_c2rust_in6addr_loopback.__in6_u.__u6_addr8 as *const guint8,
            family,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_inet_address_new_any(
    mut family: GSocketFamily,
) -> *mut GInetAddress {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if family as ::core::ffi::c_uint == 2 as ::core::ffi::c_uint
            || family as ::core::ffi::c_uint == 10 as ::core::ffi::c_uint
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
            b"G_INET_ADDRESS_FAMILY_IS_VALID (family)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GInetAddress>();
    }
    if family as ::core::ffi::c_uint == AF_INET as ::core::ffi::c_uint {
        let mut addr: [guint8; 4] = [
            0 as ::core::ffi::c_int as guint8,
            0 as ::core::ffi::c_int as guint8,
            0 as ::core::ffi::c_int as guint8,
            0 as ::core::ffi::c_int as guint8,
        ];
        return safe_c2rust_g_inet_address_new_from_bytes(&raw mut addr as *mut guint8, family);
    } else {
        return safe_c2rust_g_inet_address_new_from_bytes(
            &raw const safe_c2rust_in6addr_any.__in6_u.__u6_addr8 as *const guint8,
            family,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_inet_address_to_string(
    mut address: *mut GInetAddress,
) -> *mut gchar {
    let mut buffer: [gchar; 46] = [0; 46];
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = address as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_inet_address_get_type();
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
            b"G_IS_INET_ADDRESS (address)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if (*(*address).priv_0).family as ::core::ffi::c_uint == AF_INET as ::core::ffi::c_uint {
        inet_ntop(
            AF_INET,
            &raw mut (*(*address).priv_0).addr.ipv4 as *const ::core::ffi::c_void,
            &raw mut buffer as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[gchar; 46]>() as socklen_t,
        );
        return safe_c2rust_g_strdup_inline(&raw mut buffer as *mut gchar) as *mut gchar;
    } else {
        inet_ntop(
            AF_INET6,
            &raw mut (*(*address).priv_0).addr.ipv6 as *const ::core::ffi::c_void,
            &raw mut buffer as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[gchar; 46]>() as socklen_t,
        );
        return safe_c2rust_g_strdup_inline(&raw mut buffer as *mut gchar) as *mut gchar;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_inet_address_to_bytes(
    mut address: *mut GInetAddress,
) -> *const guint8 {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = address as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_inet_address_get_type();
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
            b"G_IS_INET_ADDRESS (address)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<guint8>();
    }
    return &raw mut (*(*address).priv_0).addr as *mut guint8;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_inet_address_get_native_size(
    mut address: *mut GInetAddress,
) -> gsize {
    if (*(*address).priv_0).family as ::core::ffi::c_uint == AF_INET as ::core::ffi::c_uint {
        return ::core::mem::size_of::<in_addr>() as gsize;
    }
    return ::core::mem::size_of::<in6_addr>() as gsize;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_inet_address_get_family(
    mut address: *mut GInetAddress,
) -> GSocketFamily {
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = address as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_inet_address_get_type();
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
            b"G_IS_INET_ADDRESS (address)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_SOCKET_FAMILY_INVALID;
    }
    return (*(*address).priv_0).family;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_inet_address_get_is_any(
    mut address: *mut GInetAddress,
) -> gboolean {
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = address as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_inet_address_get_type();
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
            b"G_IS_INET_ADDRESS (address)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if (*(*address).priv_0).family as ::core::ffi::c_uint == AF_INET as ::core::ffi::c_uint {
        let mut addr4: guint32 = ({
            let mut __v: guint32 = 0;
            let mut __x: guint32 = (*(*address).priv_0).addr.ipv4.s_addr as guint32;
            if 0 != 0 {
                __v = (__x & 0xff as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int
                    | (__x & 0xff00 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int
                    | (__x & 0xff0000 as ::core::ffi::c_uint) >> 8 as ::core::ffi::c_int
                    | (__x & 0xff000000 as ::core::ffi::c_uint) >> 24 as ::core::ffi::c_int;
            } else {
                let fresh12 = &mut __v;
                let fresh13;
                let fresh14 = __x;
                asm!(
                    "bswapl {0:e}\n", inlateout(reg)
                    c2rust_asm_casts::AsmCast::cast_in(fresh12, fresh14) => fresh13,
                    options(preserves_flags, pure, readonly, att_syntax)
                );
                c2rust_asm_casts::AsmCast::cast_out(fresh12, fresh14, fresh13);
            }
            __v
        });
        return (addr4 as in_addr_t == INADDR_ANY) as ::core::ffi::c_int;
    } else {
        return ({
            let mut __a: *const in6_addr =
                &raw mut (*(*address).priv_0).addr.ipv6 as *const in6_addr;
            ((*__a).__in6_u.__u6_addr32[0 as ::core::ffi::c_int as usize] == 0 as uint32_t
                && (*__a).__in6_u.__u6_addr32[1 as ::core::ffi::c_int as usize] == 0 as uint32_t
                && (*__a).__in6_u.__u6_addr32[2 as ::core::ffi::c_int as usize] == 0 as uint32_t
                && (*__a).__in6_u.__u6_addr32[3 as ::core::ffi::c_int as usize] == 0 as uint32_t)
                as ::core::ffi::c_int
        });
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_inet_address_get_is_loopback(
    mut address: *mut GInetAddress,
) -> gboolean {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = address as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_inet_address_get_type();
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
            b"G_IS_INET_ADDRESS (address)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if (*(*address).priv_0).family as ::core::ffi::c_uint == AF_INET as ::core::ffi::c_uint {
        let mut addr4: guint32 = ({
            let mut __v: guint32 = 0;
            let mut __x: guint32 = (*(*address).priv_0).addr.ipv4.s_addr as guint32;
            if 0 != 0 {
                __v = (__x & 0xff as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int
                    | (__x & 0xff00 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int
                    | (__x & 0xff0000 as ::core::ffi::c_uint) >> 8 as ::core::ffi::c_int
                    | (__x & 0xff000000 as ::core::ffi::c_uint) >> 24 as ::core::ffi::c_int;
            } else {
                let fresh9 = &mut __v;
                let fresh10;
                let fresh11 = __x;
                asm!(
                    "bswapl {0:e}\n", inlateout(reg)
                    c2rust_asm_casts::AsmCast::cast_in(fresh9, fresh11) => fresh10,
                    options(preserves_flags, pure, readonly, att_syntax)
                );
                c2rust_asm_casts::AsmCast::cast_out(fresh9, fresh11, fresh10);
            }
            __v
        });
        return (addr4 as ::core::ffi::c_uint & 0xff000000 as ::core::ffi::c_uint
            == 0x7f000000 as ::core::ffi::c_int as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
    } else {
        return ({
            let mut __a: *const in6_addr =
                &raw mut (*(*address).priv_0).addr.ipv6 as *const in6_addr;
            ((*__a).__in6_u.__u6_addr32[0 as ::core::ffi::c_int as usize] == 0 as uint32_t
                && (*__a).__in6_u.__u6_addr32[1 as ::core::ffi::c_int as usize] == 0 as uint32_t
                && (*__a).__in6_u.__u6_addr32[2 as ::core::ffi::c_int as usize] == 0 as uint32_t
                && (*__a).__in6_u.__u6_addr32[3 as ::core::ffi::c_int as usize]
                    == safe_c2rust___bswap_32(1 as __uint32_t)) as ::core::ffi::c_int
        });
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_inet_address_get_is_link_local(
    mut address: *mut GInetAddress,
) -> gboolean {
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = address as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_inet_address_get_type();
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
            b"G_IS_INET_ADDRESS (address)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if (*(*address).priv_0).family as ::core::ffi::c_uint == AF_INET as ::core::ffi::c_uint {
        let mut addr4: guint32 = ({
            let mut __v: guint32 = 0;
            let mut __x: guint32 = (*(*address).priv_0).addr.ipv4.s_addr as guint32;
            if 0 != 0 {
                __v = (__x & 0xff as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int
                    | (__x & 0xff00 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int
                    | (__x & 0xff0000 as ::core::ffi::c_uint) >> 8 as ::core::ffi::c_int
                    | (__x & 0xff000000 as ::core::ffi::c_uint) >> 24 as ::core::ffi::c_int;
            } else {
                let fresh6 = &mut __v;
                let fresh7;
                let fresh8 = __x;
                asm!(
                    "bswapl {0:e}\n", inlateout(reg)
                    c2rust_asm_casts::AsmCast::cast_in(fresh6, fresh8) => fresh7,
                    options(preserves_flags, pure, readonly, att_syntax)
                );
                c2rust_asm_casts::AsmCast::cast_out(fresh6, fresh8, fresh7);
            }
            __v
        });
        return (addr4 as ::core::ffi::c_uint & 0xffff0000 as ::core::ffi::c_uint
            == 0xa9fe0000 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    } else {
        return ({
            let mut __a: *const in6_addr =
                &raw mut (*(*address).priv_0).addr.ipv6 as *const in6_addr;
            ((*__a).__in6_u.__u6_addr32[0 as ::core::ffi::c_int as usize]
                & safe_c2rust___bswap_32(0xffc00000 as __uint32_t) as uint32_t
                == safe_c2rust___bswap_32(0xfe800000 as __uint32_t))
                as ::core::ffi::c_int
        });
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_inet_address_get_is_site_local(
    mut address: *mut GInetAddress,
) -> gboolean {
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = address as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_inet_address_get_type();
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
            b"G_IS_INET_ADDRESS (address)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if (*(*address).priv_0).family as ::core::ffi::c_uint == AF_INET as ::core::ffi::c_uint {
        let mut addr4: guint32 = ({
            let mut __v: guint32 = 0;
            let mut __x: guint32 = (*(*address).priv_0).addr.ipv4.s_addr as guint32;
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
        });
        return (addr4 as ::core::ffi::c_uint & 0xff000000 as ::core::ffi::c_uint
            == 0xa000000 as ::core::ffi::c_int as ::core::ffi::c_uint
            || addr4 as ::core::ffi::c_uint & 0xfff00000 as ::core::ffi::c_uint
                == 0xac100000 as ::core::ffi::c_uint
            || addr4 as ::core::ffi::c_uint & 0xffff0000 as ::core::ffi::c_uint
                == 0xc0a80000 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    } else {
        return ({
            let mut __a: *const in6_addr =
                &raw mut (*(*address).priv_0).addr.ipv6 as *const in6_addr;
            ((*__a).__in6_u.__u6_addr32[0 as ::core::ffi::c_int as usize]
                & safe_c2rust___bswap_32(0xffc00000 as __uint32_t) as uint32_t
                == safe_c2rust___bswap_32(0xfec00000 as __uint32_t))
                as ::core::ffi::c_int
        });
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_inet_address_get_is_multicast(
    mut address: *mut GInetAddress,
) -> gboolean {
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = address as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_inet_address_get_type();
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
            b"G_IS_INET_ADDRESS (address)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if (*(*address).priv_0).family as ::core::ffi::c_uint == AF_INET as ::core::ffi::c_uint {
        let mut addr4: guint32 = ({
            let mut __v: guint32 = 0;
            let mut __x: guint32 = (*(*address).priv_0).addr.ipv4.s_addr as guint32;
            if 0 != 0 {
                __v = (__x & 0xff as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int
                    | (__x & 0xff00 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int
                    | (__x & 0xff0000 as ::core::ffi::c_uint) >> 8 as ::core::ffi::c_int
                    | (__x & 0xff000000 as ::core::ffi::c_uint) >> 24 as ::core::ffi::c_int;
            } else {
                let fresh0 = &mut __v;
                let fresh1;
                let fresh2 = __x;
                asm!(
                    "bswapl {0:e}\n", inlateout(reg)
                    c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2) => fresh1,
                    options(preserves_flags, pure, readonly, att_syntax)
                );
                c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
            }
            __v
        });
        return (addr4 as in_addr_t & 0xf0000000 as in_addr_t == 0xe0000000 as in_addr_t)
            as ::core::ffi::c_int;
    } else {
        return (*(&raw mut (*(*address).priv_0).addr.ipv6 as *const uint8_t)
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0xff as ::core::ffi::c_int) as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_inet_address_get_is_mc_global(
    mut address: *mut GInetAddress,
) -> gboolean {
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = address as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_inet_address_get_type();
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
            b"G_IS_INET_ADDRESS (address)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if (*(*address).priv_0).family as ::core::ffi::c_uint == AF_INET as ::core::ffi::c_uint {
        return FALSE;
    } else {
        return (*(&raw mut (*(*address).priv_0).addr.ipv6 as *const uint8_t)
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0xff as ::core::ffi::c_int
            && *(&raw mut (*(*address).priv_0).addr.ipv6 as *const uint8_t)
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int
                == 0xe as ::core::ffi::c_int) as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_inet_address_get_is_mc_link_local(
    mut address: *mut GInetAddress,
) -> gboolean {
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = address as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_inet_address_get_type();
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
            b"G_IS_INET_ADDRESS (address)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if (*(*address).priv_0).family as ::core::ffi::c_uint == AF_INET as ::core::ffi::c_uint {
        return FALSE;
    } else {
        return (*(&raw mut (*(*address).priv_0).addr.ipv6 as *const uint8_t)
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0xff as ::core::ffi::c_int
            && *(&raw mut (*(*address).priv_0).addr.ipv6 as *const uint8_t)
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int
                == 0x2 as ::core::ffi::c_int) as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_inet_address_get_is_mc_node_local(
    mut address: *mut GInetAddress,
) -> gboolean {
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = address as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_inet_address_get_type();
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
            b"G_IS_INET_ADDRESS (address)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if (*(*address).priv_0).family as ::core::ffi::c_uint == AF_INET as ::core::ffi::c_uint {
        return FALSE;
    } else {
        return (*(&raw mut (*(*address).priv_0).addr.ipv6 as *const uint8_t)
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0xff as ::core::ffi::c_int
            && *(&raw mut (*(*address).priv_0).addr.ipv6 as *const uint8_t)
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int
                == 0x1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_inet_address_get_is_mc_org_local(
    mut address: *mut GInetAddress,
) -> gboolean {
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = address as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_inet_address_get_type();
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
            b"G_IS_INET_ADDRESS (address)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if (*(*address).priv_0).family as ::core::ffi::c_uint == AF_INET as ::core::ffi::c_uint {
        return FALSE;
    } else {
        return (*(&raw mut (*(*address).priv_0).addr.ipv6 as *const uint8_t)
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0xff as ::core::ffi::c_int
            && *(&raw mut (*(*address).priv_0).addr.ipv6 as *const uint8_t)
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int
                == 0x8 as ::core::ffi::c_int) as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_inet_address_get_is_mc_site_local(
    mut address: *mut GInetAddress,
) -> gboolean {
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = address as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_inet_address_get_type();
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
            b"G_IS_INET_ADDRESS (address)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if (*(*address).priv_0).family as ::core::ffi::c_uint == AF_INET as ::core::ffi::c_uint {
        return FALSE;
    } else {
        return (*(&raw mut (*(*address).priv_0).addr.ipv6 as *const uint8_t)
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0xff as ::core::ffi::c_int
            && *(&raw mut (*(*address).priv_0).addr.ipv6 as *const uint8_t)
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int
                == 0x5 as ::core::ffi::c_int) as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_inet_address_equal(
    mut address: *mut GInetAddress,
    mut other_address: *mut GInetAddress,
) -> gboolean {
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = address as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_inet_address_get_type();
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
            b"G_IS_INET_ADDRESS (address)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = other_address as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_inet_address_get_type();
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
            b"G_IS_INET_ADDRESS (other_address)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if safe_c2rust_g_inet_address_get_family(address) as ::core::ffi::c_uint
        != safe_c2rust_g_inet_address_get_family(other_address) as ::core::ffi::c_uint
    {
        return FALSE;
    }
    if memcmp(
        safe_c2rust_g_inet_address_to_bytes(address) as *const ::core::ffi::c_void,
        safe_c2rust_g_inet_address_to_bytes(other_address) as *const ::core::ffi::c_void,
        safe_c2rust_g_inet_address_get_native_size(address) as size_t,
    ) != 0 as ::core::ffi::c_int
    {
        return FALSE;
    }
    return TRUE;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
