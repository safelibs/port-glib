extern "C" {
    pub type _GData;
    pub type _GInetAddress;
    pub type _GInetSocketAddressPrivate;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
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
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_param_spec_uint(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        minimum: guint,
        maximum: guint,
        default_value: guint,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_string(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: *const gchar,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_value_set_uint(value: *mut GValue, v_uint: guint);
    fn g_value_get_uint(value: *const GValue) -> guint;
    fn g_value_set_string(value: *mut GValue, v_string: *const gchar);
    fn g_value_dup_string(value: *const GValue) -> *mut gchar;
    fn g_inet_socket_address_get_type() -> GType;
}
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
pub struct _GProxyAddress {
    pub parent_instance: GInetSocketAddress,
    pub priv_0: *mut GProxyAddressPrivate,
}
pub type GProxyAddressPrivate = _GProxyAddressPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GProxyAddressPrivate {
    pub uri: *mut gchar,
    pub protocol: *mut gchar,
    pub username: *mut gchar,
    pub password: *mut gchar,
    pub dest_protocol: *mut gchar,
    pub dest_hostname: *mut gchar,
    pub dest_port: guint16,
}
pub type GProxyAddress = _GProxyAddress;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GProxyAddressClass {
    pub parent_class: GInetSocketAddressClass,
}
pub type GProxyAddressClass = _GProxyAddressClass;
pub const PROP_URI: C2RustUnnamed_0 = 7;
pub const PROP_DESTINATION_PORT: C2RustUnnamed_0 = 4;
pub const PROP_DESTINATION_HOSTNAME: C2RustUnnamed_0 = 3;
pub const PROP_DESTINATION_PROTOCOL: C2RustUnnamed_0 = 2;
pub const PROP_PASSWORD: C2RustUnnamed_0 = 6;
pub const PROP_USERNAME: C2RustUnnamed_0 = 5;
pub const PROP_PROTOCOL: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_0 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn safe_c2rust_g_proxy_address_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_proxy_address_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GProxyAddress_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GProxyAddress_private_offset,
        );
    }
    safe_c2rust_g_proxy_address_class_init(klass as *mut GProxyAddressClass);
}
static mut safe_c2rust_GProxyAddress_private_offset: gint = 0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_proxy_address_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_inet_socket_address_get_type(),
        g_intern_static_string(b"GProxyAddress\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GProxyAddressClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_proxy_address_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GProxyAddress>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GProxyAddress) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_proxy_address_init as unsafe extern "C" fn(*mut GProxyAddress) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GProxyAddress_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GProxyAddressPrivate>() as gsize,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_proxy_address_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_proxy_address_get_type_once();
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
static mut safe_c2rust_g_proxy_address_parent_class: gpointer = NULL;
#[inline]
unsafe extern "C" fn safe_c2rust_g_proxy_address_get_instance_private(
    mut self_0: *mut GProxyAddress,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GProxyAddress_private_offset as glong as isize) as gpointer;
}
unsafe extern "C" fn safe_c2rust_g_proxy_address_finalize(mut object: *mut GObject) {
    let mut proxy: *mut GProxyAddress = object as *mut ::core::ffi::c_void as *mut GProxyAddress;
    g_free((*(*proxy).priv_0).uri as gpointer);
    g_free((*(*proxy).priv_0).protocol as gpointer);
    g_free((*(*proxy).priv_0).username as gpointer);
    g_free((*(*proxy).priv_0).password as gpointer);
    g_free((*(*proxy).priv_0).dest_hostname as gpointer);
    g_free((*(*proxy).priv_0).dest_protocol as gpointer);
    (*(safe_c2rust_g_proxy_address_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_proxy_address_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut proxy: *mut GProxyAddress = object as *mut ::core::ffi::c_void as *mut GProxyAddress;
    match prop_id {
        1 => {
            g_free((*(*proxy).priv_0).protocol as gpointer);
            (*(*proxy).priv_0).protocol = g_value_dup_string(value);
        }
        2 => {
            g_free((*(*proxy).priv_0).dest_protocol as gpointer);
            (*(*proxy).priv_0).dest_protocol = g_value_dup_string(value);
        }
        3 => {
            g_free((*(*proxy).priv_0).dest_hostname as gpointer);
            (*(*proxy).priv_0).dest_hostname = g_value_dup_string(value);
        }
        4 => {
            (*(*proxy).priv_0).dest_port = g_value_get_uint(value) as guint16;
        }
        5 => {
            g_free((*(*proxy).priv_0).username as gpointer);
            (*(*proxy).priv_0).username = g_value_dup_string(value);
        }
        6 => {
            g_free((*(*proxy).priv_0).password as gpointer);
            (*(*proxy).priv_0).password = g_value_dup_string(value);
        }
        7 => {
            g_free((*(*proxy).priv_0).uri as gpointer);
            (*(*proxy).priv_0).uri = g_value_dup_string(value);
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gproxyaddress.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                133 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_proxy_address_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut proxy: *mut GProxyAddress = object as *mut ::core::ffi::c_void as *mut GProxyAddress;
    match prop_id {
        1 => {
            g_value_set_string(value, (*(*proxy).priv_0).protocol);
        }
        2 => {
            g_value_set_string(value, (*(*proxy).priv_0).dest_protocol);
        }
        3 => {
            g_value_set_string(value, (*(*proxy).priv_0).dest_hostname);
        }
        4 => {
            g_value_set_uint(value, (*(*proxy).priv_0).dest_port as guint);
        }
        5 => {
            g_value_set_string(value, (*(*proxy).priv_0).username);
        }
        6 => {
            g_value_set_string(value, (*(*proxy).priv_0).password);
        }
        7 => {
            g_value_set_string(value, (*(*proxy).priv_0).uri);
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gproxyaddress.c\0"
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
unsafe extern "C" fn safe_c2rust_g_proxy_address_class_init(mut klass: *mut GProxyAddressClass) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize =
        Some(safe_c2rust_g_proxy_address_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_proxy_address_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_proxy_address_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    g_object_class_install_property(
        gobject_class,
        PROP_PROTOCOL as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"protocol\0" as *const u8 as *const gchar,
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
        PROP_USERNAME as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"username\0" as *const u8 as *const gchar,
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
        PROP_PASSWORD as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"password\0" as *const u8 as *const gchar,
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
        PROP_DESTINATION_PROTOCOL as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"destination-protocol\0" as *const u8 as *const gchar,
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
        PROP_DESTINATION_HOSTNAME as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"destination-hostname\0" as *const u8 as *const gchar,
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
        PROP_DESTINATION_PORT as ::core::ffi::c_int as guint,
        g_param_spec_uint(
            b"destination-port\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            0 as guint,
            65535 as guint,
            0 as guint,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_URI as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"uri\0" as *const u8 as *const gchar,
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
}
unsafe extern "C" fn safe_c2rust_g_proxy_address_init(mut proxy: *mut GProxyAddress) {
    (*proxy).priv_0 =
        safe_c2rust_g_proxy_address_get_instance_private(proxy) as *mut GProxyAddressPrivate;
    (*(*proxy).priv_0).protocol = ::core::ptr::null_mut::<gchar>();
    (*(*proxy).priv_0).username = ::core::ptr::null_mut::<gchar>();
    (*(*proxy).priv_0).password = ::core::ptr::null_mut::<gchar>();
    (*(*proxy).priv_0).dest_hostname = ::core::ptr::null_mut::<gchar>();
    (*(*proxy).priv_0).dest_port = 0 as guint16;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_proxy_address_new(
    mut inetaddr: *mut GInetAddress,
    mut port: guint16,
    mut protocol: *const gchar,
    mut dest_hostname: *const gchar,
    mut dest_port: guint16,
    mut username: *const gchar,
    mut password: *const gchar,
) -> *mut GSocketAddress {
    return g_object_new(
        safe_c2rust_g_proxy_address_get_type(),
        b"address\0" as *const u8 as *const gchar,
        inetaddr,
        b"port\0" as *const u8 as *const ::core::ffi::c_char,
        port as ::core::ffi::c_int,
        b"protocol\0" as *const u8 as *const ::core::ffi::c_char,
        protocol,
        b"destination-hostname\0" as *const u8 as *const ::core::ffi::c_char,
        dest_hostname,
        b"destination-port\0" as *const u8 as *const ::core::ffi::c_char,
        dest_port as ::core::ffi::c_int,
        b"username\0" as *const u8 as *const ::core::ffi::c_char,
        username,
        b"password\0" as *const u8 as *const ::core::ffi::c_char,
        password,
        NULL,
    ) as *mut GSocketAddress;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_proxy_address_get_protocol(
    mut proxy: *mut GProxyAddress,
) -> *const gchar {
    return (*(*proxy).priv_0).protocol;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_proxy_address_get_destination_protocol(
    mut proxy: *mut GProxyAddress,
) -> *const gchar {
    return (*(*proxy).priv_0).dest_protocol;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_proxy_address_get_destination_hostname(
    mut proxy: *mut GProxyAddress,
) -> *const gchar {
    return (*(*proxy).priv_0).dest_hostname;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_proxy_address_get_destination_port(
    mut proxy: *mut GProxyAddress,
) -> guint16 {
    return (*(*proxy).priv_0).dest_port;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_proxy_address_get_username(
    mut proxy: *mut GProxyAddress,
) -> *const gchar {
    return (*(*proxy).priv_0).username;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_proxy_address_get_password(
    mut proxy: *mut GProxyAddress,
) -> *const gchar {
    return (*(*proxy).priv_0).password;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_proxy_address_get_uri(
    mut proxy: *mut GProxyAddress,
) -> *const gchar {
    return (*(*proxy).priv_0).uri;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
