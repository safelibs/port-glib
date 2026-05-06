extern "C" {
    pub type _GData;
    pub type _GTlsDatabasePrivate;
    pub type _GTlsBackend;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_type_interface_peek(instance_class: gpointer, iface_type: GType) -> gpointer;
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
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_tls_database_get_type() -> GType;
    fn _g_dummy_tls_backend_get_type() -> GType;
    fn _g_io_module_get_default(
        extension_point: *const gchar,
        envvar: *const gchar,
        verify_func: GIOModuleVerifyFunc,
    ) -> gpointer;
}
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GMutex = _GMutex;
pub type GData = _GData;
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
pub struct _GTlsDatabase {
    pub parent_instance: GObject,
    pub priv_0: *mut GTlsDatabasePrivate,
}
pub type GTlsDatabasePrivate = _GTlsDatabasePrivate;
pub type GTlsDatabase = _GTlsDatabase;
pub type GTlsBackend = _GTlsBackend;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTlsBackendInterface {
    pub g_iface: GTypeInterface,
    pub supports_tls: Option<unsafe extern "C" fn(*mut GTlsBackend) -> gboolean>,
    pub get_certificate_type: Option<unsafe extern "C" fn() -> GType>,
    pub get_client_connection_type: Option<unsafe extern "C" fn() -> GType>,
    pub get_server_connection_type: Option<unsafe extern "C" fn() -> GType>,
    pub get_file_database_type: Option<unsafe extern "C" fn() -> GType>,
    pub get_default_database: Option<unsafe extern "C" fn(*mut GTlsBackend) -> *mut GTlsDatabase>,
    pub supports_dtls: Option<unsafe extern "C" fn(*mut GTlsBackend) -> gboolean>,
    pub get_dtls_client_connection_type: Option<unsafe extern "C" fn() -> GType>,
    pub get_dtls_server_connection_type: Option<unsafe extern "C" fn() -> GType>,
}
pub type GTlsBackendInterface = _GTlsBackendInterface;
pub type GIOModuleVerifyFunc = Option<unsafe extern "C" fn(gpointer) -> gboolean>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub in_0: *mut ::core::ffi::c_char,
    pub out: *mut *mut GObject,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_INVALID: GType = ((0 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_INTERFACE: GType =
    ((2 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
#[inline]
unsafe extern "C" fn safe_c2rust_g_set_object(
    mut object_ptr: *mut *mut GObject,
    mut new_object: *mut GObject,
) -> gboolean {
    let mut old_object: *mut GObject = *object_ptr;
    if old_object == new_object {
        return FALSE;
    }
    if !new_object.is_null() {
        g_object_ref(new_object as gpointer);
    }
    *object_ptr = new_object;
    if !old_object.is_null() {
        g_object_unref(old_object as gpointer);
    }
    return TRUE;
}
pub const G_TLS_BACKEND_EXTENSION_POINT_NAME: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"gio-tls-backend\0") };
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_backend_get_type() -> GType {
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
            g_intern_static_string(b"GTlsBackend\0" as *const u8 as *const gchar),
            ::core::mem::size_of::<GTlsBackendInterface>() as guint,
            ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GTlsBackendInterface) -> ()>,
                    Option<unsafe extern "C" fn() -> ()>,
                >(Some(
                    safe_c2rust_g_tls_backend_default_init
                        as unsafe extern "C" fn(*mut GTlsBackendInterface) -> (),
                )),
            ),
            0 as guint,
            ::core::mem::transmute::<*mut ::core::ffi::c_void, GInstanceInitFunc>(NULL),
            G_TYPE_FLAG_NONE,
        );
        if ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType != G_TYPE_INVALID {
            g_type_interface_add_prerequisite(
                g_define_type_id,
                ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            );
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
static mut safe_c2rust_default_database: *mut GTlsDatabase =
    ::core::ptr::null::<GTlsDatabase>() as *mut GTlsDatabase;
static mut safe_c2rust_g__default_database_lock_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
unsafe extern "C" fn safe_c2rust_g_tls_backend_default_init(mut iface: *mut GTlsBackendInterface) {}
static mut safe_c2rust_tls_backend_default_singleton: *mut GTlsBackend =
    ::core::ptr::null::<GTlsBackend>() as *mut GTlsBackend;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_backend_get_default() -> *mut GTlsBackend {
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_tls_backend_default_singleton;
        } else {
        };
        (({
            let mut gapg_temp_newval: *mut GTlsBackend = ::core::ptr::null_mut::<GTlsBackend>();
            let mut gapg_temp_atomic: *mut *mut GTlsBackend =
                &raw mut safe_c2rust_tls_backend_default_singleton;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        })
        .is_null()
            && g_once_init_enter_pointer(
                &raw mut safe_c2rust_tls_backend_default_singleton as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut singleton: *mut GTlsBackend = ::core::ptr::null_mut::<GTlsBackend>();
        singleton = _g_io_module_get_default(
            G_TLS_BACKEND_EXTENSION_POINT_NAME.as_ptr() as *const gchar,
            b"GIO_USE_TLS\0" as *const u8 as *const gchar,
            None,
        ) as *mut GTlsBackend;
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_tls_backend_default_singleton = singleton;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut safe_c2rust_tls_backend_default_singleton as *mut ::core::ffi::c_void,
            singleton as guintptr as gpointer,
        );
    }
    return safe_c2rust_tls_backend_default_singleton;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_backend_supports_tls(
    mut backend: *mut GTlsBackend,
) -> gboolean {
    if (*(g_type_interface_peek(
        (*(backend as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_tls_backend_get_type(),
    ) as *mut GTlsBackendInterface))
        .supports_tls
        .is_some()
    {
        return (*(g_type_interface_peek(
            (*(backend as *mut GTypeInstance)).g_class as gpointer,
            safe_c2rust_g_tls_backend_get_type(),
        ) as *mut GTlsBackendInterface))
            .supports_tls
            .expect("non-null function pointer")(backend);
    } else if ({
        let mut __inst: *mut GTypeInstance = backend as *mut GTypeInstance;
        let mut __t: GType = _g_dummy_tls_backend_get_type();
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
        return FALSE;
    } else {
        return TRUE;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_backend_supports_dtls(
    mut backend: *mut GTlsBackend,
) -> gboolean {
    if (*(g_type_interface_peek(
        (*(backend as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_tls_backend_get_type(),
    ) as *mut GTlsBackendInterface))
        .supports_dtls
        .is_some()
    {
        return (*(g_type_interface_peek(
            (*(backend as *mut GTypeInstance)).g_class as gpointer,
            safe_c2rust_g_tls_backend_get_type(),
        ) as *mut GTlsBackendInterface))
            .supports_dtls
            .expect("non-null function pointer")(backend);
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_backend_get_default_database(
    mut backend: *mut GTlsBackend,
) -> *mut GTlsDatabase {
    let mut db: *mut GTlsDatabase = ::core::ptr::null_mut::<GTlsDatabase>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = backend as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_backend_get_type();
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
            b"G_IS_TLS_BACKEND (backend)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsDatabase>();
    }
    if (*(g_type_interface_peek(
        (*(backend as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_tls_backend_get_type(),
    ) as *mut GTlsBackendInterface))
        .get_default_database
        .is_none()
    {
        return ::core::ptr::null_mut::<GTlsDatabase>();
    }
    g_mutex_lock(&raw mut safe_c2rust_g__default_database_lock_lock);
    if safe_c2rust_default_database.is_null() {
        safe_c2rust_default_database = (*(g_type_interface_peek(
            (*(backend as *mut GTypeInstance)).g_class as gpointer,
            safe_c2rust_g_tls_backend_get_type(),
        ) as *mut GTlsBackendInterface))
            .get_default_database
            .expect("non-null function pointer")(backend);
    }
    db = (if !safe_c2rust_default_database.is_null() {
        g_object_ref(safe_c2rust_default_database as gpointer) as *mut GTlsDatabase
    } else {
        ::core::ptr::null_mut::<GTlsDatabase>()
    }) as *mut GTlsDatabase;
    g_mutex_unlock(&raw mut safe_c2rust_g__default_database_lock_lock);
    return db;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_backend_set_default_database(
    mut backend: *mut GTlsBackend,
    mut database: *mut GTlsDatabase,
) {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = backend as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_backend_get_type();
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
            b"G_IS_TLS_BACKEND (backend)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if database.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = database as *mut GTypeInstance;
                let mut __t: GType = g_tls_database_get_type();
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
            b"database == NULL || G_IS_TLS_DATABASE (database)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut safe_c2rust_g__default_database_lock_lock);
    let mut _object_ptr: C2RustUnnamed = C2RustUnnamed {
        in_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    _object_ptr.in_0 = &raw mut safe_c2rust_default_database as *mut ::core::ffi::c_char;
    if 0 as ::core::ffi::c_int != 0 {
        safe_c2rust_default_database = database;
    } else {
    };
    safe_c2rust_g_set_object(_object_ptr.out, database as *mut GObject);
    g_mutex_unlock(&raw mut safe_c2rust_g__default_database_lock_lock);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_backend_get_certificate_type(
    mut backend: *mut GTlsBackend,
) -> GType {
    return (*(g_type_interface_peek(
        (*(backend as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_tls_backend_get_type(),
    ) as *mut GTlsBackendInterface))
        .get_certificate_type
        .expect("non-null function pointer")();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_backend_get_client_connection_type(
    mut backend: *mut GTlsBackend,
) -> GType {
    return (*(g_type_interface_peek(
        (*(backend as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_tls_backend_get_type(),
    ) as *mut GTlsBackendInterface))
        .get_client_connection_type
        .expect("non-null function pointer")();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_backend_get_server_connection_type(
    mut backend: *mut GTlsBackend,
) -> GType {
    return (*(g_type_interface_peek(
        (*(backend as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_tls_backend_get_type(),
    ) as *mut GTlsBackendInterface))
        .get_server_connection_type
        .expect("non-null function pointer")();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_backend_get_dtls_client_connection_type(
    mut backend: *mut GTlsBackend,
) -> GType {
    let mut iface: *mut GTlsBackendInterface = ::core::ptr::null_mut::<GTlsBackendInterface>();
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = backend as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_backend_get_type();
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
            b"G_IS_TLS_BACKEND (backend)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ((0 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
    }
    iface = g_type_interface_peek(
        (*(backend as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_tls_backend_get_type(),
    ) as *mut GTlsBackendInterface;
    if (*iface).get_dtls_client_connection_type.is_none() {
        return G_TYPE_INVALID;
    }
    return (*iface)
        .get_dtls_client_connection_type
        .expect("non-null function pointer")();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_backend_get_dtls_server_connection_type(
    mut backend: *mut GTlsBackend,
) -> GType {
    let mut iface: *mut GTlsBackendInterface = ::core::ptr::null_mut::<GTlsBackendInterface>();
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = backend as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_backend_get_type();
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
            b"G_IS_TLS_BACKEND (backend)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ((0 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
    }
    iface = g_type_interface_peek(
        (*(backend as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_tls_backend_get_type(),
    ) as *mut GTlsBackendInterface;
    if (*iface).get_dtls_server_connection_type.is_none() {
        return G_TYPE_INVALID;
    }
    return (*iface)
        .get_dtls_server_connection_type
        .expect("non-null function pointer")();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_backend_get_file_database_type(
    mut backend: *mut GTlsBackend,
) -> GType {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = backend as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_backend_get_type();
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
            b"G_IS_TLS_BACKEND (backend)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    if (*(g_type_interface_peek(
        (*(backend as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_tls_backend_get_type(),
    ) as *mut GTlsBackendInterface))
        .get_file_database_type
        .is_none()
    {
        return 0 as GType;
    }
    return (*(g_type_interface_peek(
        (*(backend as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_tls_backend_get_type(),
    ) as *mut GTlsBackendInterface))
        .get_file_database_type
        .expect("non-null function pointer")();
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
