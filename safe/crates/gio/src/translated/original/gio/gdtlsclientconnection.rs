extern "C" {
    pub type _GData;
    pub type _GCancellable;
    pub type _GDatagramBased;
    pub type _GSocketConnectable;
    pub type _GDtlsClientConnection;
    pub type _GTlsBackend;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_type_register_static_simple(
        parent_type: GType,
        type_name: *const gchar,
        class_size: guint,
        class_init: GClassInitFunc,
        instance_size: guint,
        instance_init: GInstanceInitFunc,
        flags: GTypeFlags,
    ) -> GType;
    fn g_type_interface_add_prerequisite(interface_type: GType, prerequisite_type: GType);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_object_interface_install_property(g_iface: gpointer, pspec: *mut GParamSpec);
    fn g_object_set(object: gpointer, first_property_name: *const gchar, ...);
    fn g_object_get(object: gpointer, first_property_name: *const gchar, ...);
    fn g_object_unref(object: gpointer);
    fn g_param_spec_flags(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        flags_type: GType,
        default_value: guint,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_pointer(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_object(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        object_type: GType,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_dtls_connection_get_type() -> GType;
    fn g_initable_new(
        object_type: GType,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
        first_property_name: *const gchar,
        ...
    ) -> gpointer;
    fn g_tls_certificate_flags_get_type() -> GType;
    fn g_socket_connectable_get_type() -> GType;
    fn g_tls_backend_get_default() -> *mut GTlsBackend;
    fn g_tls_backend_get_dtls_client_connection_type(backend: *mut GTlsBackend) -> GType;
}
pub type guint32 = ::core::ffi::c_uint;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
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
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
pub type GList = _GList;
pub type GType = gsize;
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
pub type GTlsCertificateFlags = ::core::ffi::c_uint;
pub const G_TLS_CERTIFICATE_VALIDATE_ALL: GTlsCertificateFlags = 127;
pub const G_TLS_CERTIFICATE_GENERIC_ERROR: GTlsCertificateFlags = 64;
pub const G_TLS_CERTIFICATE_INSECURE: GTlsCertificateFlags = 32;
pub const G_TLS_CERTIFICATE_REVOKED: GTlsCertificateFlags = 16;
pub const G_TLS_CERTIFICATE_EXPIRED: GTlsCertificateFlags = 8;
pub const G_TLS_CERTIFICATE_NOT_ACTIVATED: GTlsCertificateFlags = 4;
pub const G_TLS_CERTIFICATE_BAD_IDENTITY: GTlsCertificateFlags = 2;
pub const G_TLS_CERTIFICATE_UNKNOWN_CA: GTlsCertificateFlags = 1;
pub const G_TLS_CERTIFICATE_NO_FLAGS: GTlsCertificateFlags = 0;
pub type GCancellable = _GCancellable;
pub type GDatagramBased = _GDatagramBased;
pub type GSocketConnectable = _GSocketConnectable;
pub type GDtlsClientConnection = _GDtlsClientConnection;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDtlsClientConnectionInterface {
    pub g_iface: GTypeInterface,
}
pub type GDtlsClientConnectionInterface = _GDtlsClientConnectionInterface;
pub type GTlsBackend = _GTlsBackend;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_INVALID: GType = ((0 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_INTERFACE: GType =
    ((2 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dtls_client_connection_get_type() -> GType {
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
        let mut g_define_type_id: GType = g_type_register_static_simple(
            G_TYPE_INTERFACE,
            g_intern_static_string(b"GDtlsClientConnection\0" as *const u8 as *const gchar),
            ::core::mem::size_of::<GDtlsClientConnectionInterface>() as guint,
            ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GDtlsClientConnectionInterface) -> ()>,
                    Option<unsafe extern "C" fn() -> ()>,
                >(Some(
                    safe_c2rust_g_dtls_client_connection_default_init
                        as unsafe extern "C" fn(*mut GDtlsClientConnectionInterface) -> (),
                )),
            ),
            0 as guint,
            ::core::mem::transmute::<*mut ::core::ffi::c_void, GInstanceInitFunc>(NULL),
            G_TYPE_FLAG_NONE,
        );
        if g_dtls_connection_get_type() != G_TYPE_INVALID {
            g_type_interface_add_prerequisite(g_define_type_id, g_dtls_connection_get_type());
        }
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
unsafe extern "C" fn safe_c2rust_g_dtls_client_connection_default_init(
    mut iface: *mut GDtlsClientConnectionInterface,
) {
    g_object_interface_install_property(
        iface as gpointer,
        g_param_spec_flags(
            b"validation-flags\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_tls_certificate_flags_get_type(),
            G_TLS_CERTIFICATE_VALIDATE_ALL as ::core::ffi::c_int as guint,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_interface_install_property(
        iface as gpointer,
        g_param_spec_object(
            b"server-identity\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_socket_connectable_get_type(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_interface_install_property(
        iface as gpointer,
        g_param_spec_pointer(
            b"accepted-cas\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dtls_client_connection_new(
    mut base_socket: *mut GDatagramBased,
    mut server_identity: *mut GSocketConnectable,
    mut error: *mut *mut GError,
) -> *mut GDatagramBased {
    let mut conn: *mut GObject = ::core::ptr::null_mut::<GObject>();
    let mut backend: *mut GTlsBackend = ::core::ptr::null_mut::<GTlsBackend>();
    backend = g_tls_backend_get_default();
    conn = g_initable_new(
        g_tls_backend_get_dtls_client_connection_type(backend),
        ::core::ptr::null_mut::<GCancellable>(),
        error,
        b"base-socket\0" as *const u8 as *const gchar,
        base_socket,
        b"server-identity\0" as *const u8 as *const ::core::ffi::c_char,
        server_identity,
        NULL,
    ) as *mut GObject;
    return conn as *mut ::core::ffi::c_void as *mut GDatagramBased;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dtls_client_connection_get_validation_flags(
    mut conn: *mut GDtlsClientConnection,
) -> GTlsCertificateFlags {
    let mut flags: GTlsCertificateFlags = G_TLS_CERTIFICATE_NO_FLAGS;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dtls_client_connection_get_type();
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
            b"G_IS_DTLS_CLIENT_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_TLS_CERTIFICATE_NO_FLAGS;
    }
    g_object_get(
        conn as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"validation-flags\0" as *const u8 as *const gchar,
        &raw mut flags,
        NULL,
    );
    return flags;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dtls_client_connection_set_validation_flags(
    mut conn: *mut GDtlsClientConnection,
    mut flags: GTlsCertificateFlags,
) {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dtls_client_connection_get_type();
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
            b"G_IS_DTLS_CLIENT_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_object_set(
        conn as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"validation-flags\0" as *const u8 as *const gchar,
        flags as ::core::ffi::c_uint,
        NULL,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dtls_client_connection_get_server_identity(
    mut conn: *mut GDtlsClientConnection,
) -> *mut GSocketConnectable {
    let mut identity: *mut GSocketConnectable = ::core::ptr::null_mut::<GSocketConnectable>();
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dtls_client_connection_get_type();
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
            b"G_IS_DTLS_CLIENT_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSocketConnectable>();
    }
    g_object_get(
        conn as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"server-identity\0" as *const u8 as *const gchar,
        &raw mut identity,
        NULL,
    );
    if !identity.is_null() {
        g_object_unref(identity as gpointer);
    }
    return identity;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dtls_client_connection_set_server_identity(
    mut conn: *mut GDtlsClientConnection,
    mut identity: *mut GSocketConnectable,
) {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dtls_client_connection_get_type();
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
            b"G_IS_DTLS_CLIENT_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_object_set(
        conn as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"server-identity\0" as *const u8 as *const gchar,
        identity,
        NULL,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dtls_client_connection_get_accepted_cas(
    mut conn: *mut GDtlsClientConnection,
) -> *mut GList {
    let mut accepted_cas: *mut GList = ::core::ptr::null_mut::<GList>();
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = conn as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dtls_client_connection_get_type();
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
            b"G_IS_DTLS_CLIENT_CONNECTION (conn)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    g_object_get(
        conn as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"accepted-cas\0" as *const u8 as *const gchar,
        &raw mut accepted_cas,
        NULL,
    );
    return accepted_cas;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
