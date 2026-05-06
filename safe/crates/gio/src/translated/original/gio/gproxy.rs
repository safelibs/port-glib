extern "C" {
    pub type _GData;
    pub type _GAsyncResult;
    pub type _GCancellable;
    pub type _GIOExtensionPoint;
    pub type _GIOExtension;
    pub type _GIOStream;
    pub type _GProxy;
    pub type _GProxyAddress;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
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
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_io_extension_point_lookup(name: *const ::core::ffi::c_char) -> *mut GIOExtensionPoint;
    fn g_io_extension_point_get_extension_by_name(
        extension_point: *mut GIOExtensionPoint,
        name: *const ::core::ffi::c_char,
    ) -> *mut GIOExtension;
    fn g_io_extension_get_type(extension: *mut GIOExtension) -> GType;
    fn _g_io_modules_ensure_loaded();
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
pub type GAsyncResult = _GAsyncResult;
pub type GCancellable = _GCancellable;
pub type GIOExtensionPoint = _GIOExtensionPoint;
pub type GIOExtension = _GIOExtension;
pub type GIOStream = _GIOStream;
pub type GProxy = _GProxy;
pub type GProxyAddress = _GProxyAddress;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
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
pub type GProxyInterface = _GProxyInterface;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_INVALID: GType = ((0 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_INTERFACE: GType =
    ((2 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_PROXY_EXTENSION_POINT_NAME: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"gio-proxy\0") };
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_proxy_get_type() -> GType {
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
            g_intern_static_string(b"GProxy\0" as *const u8 as *const gchar),
            ::core::mem::size_of::<GProxyInterface>() as guint,
            ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GProxyInterface) -> ()>,
                    Option<unsafe extern "C" fn() -> ()>,
                >(Some(
                    safe_c2rust_g_proxy_default_init
                        as unsafe extern "C" fn(*mut GProxyInterface) -> (),
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
unsafe extern "C" fn safe_c2rust_g_proxy_default_init(mut iface: *mut GProxyInterface) {}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_proxy_get_default_for_protocol(
    mut protocol: *const gchar,
) -> *mut GProxy {
    let mut ep: *mut GIOExtensionPoint = ::core::ptr::null_mut::<GIOExtensionPoint>();
    let mut extension: *mut GIOExtension = ::core::ptr::null_mut::<GIOExtension>();
    _g_io_modules_ensure_loaded();
    ep = g_io_extension_point_lookup(G_PROXY_EXTENSION_POINT_NAME.as_ptr());
    extension =
        g_io_extension_point_get_extension_by_name(ep, protocol as *const ::core::ffi::c_char);
    if !extension.is_null() {
        return g_object_new(
            g_io_extension_get_type(extension),
            ::core::ptr::null::<gchar>(),
        ) as *mut GProxy;
    }
    return ::core::ptr::null_mut::<GProxy>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_proxy_connect(
    mut proxy: *mut GProxy,
    mut connection: *mut GIOStream,
    mut proxy_address: *mut GProxyAddress,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GIOStream {
    let mut iface: *mut GProxyInterface = ::core::ptr::null_mut::<GProxyInterface>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = proxy as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_proxy_get_type();
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
            b"G_IS_PROXY (proxy)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GIOStream>();
    }
    iface = g_type_interface_peek(
        (*(proxy as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_proxy_get_type(),
    ) as *mut GProxyInterface;
    return Some((*iface).connect.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        proxy, connection, proxy_address, cancellable, error
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_proxy_connect_async(
    mut proxy: *mut GProxy,
    mut connection: *mut GIOStream,
    mut proxy_address: *mut GProxyAddress,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GProxyInterface = ::core::ptr::null_mut::<GProxyInterface>();
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = proxy as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_proxy_get_type();
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
            b"G_IS_PROXY (proxy)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(proxy as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_proxy_get_type(),
    ) as *mut GProxyInterface;
    Some((*iface).connect_async.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        proxy,
        connection,
        proxy_address,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_proxy_connect_finish(
    mut proxy: *mut GProxy,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GIOStream {
    let mut iface: *mut GProxyInterface = ::core::ptr::null_mut::<GProxyInterface>();
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = proxy as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_proxy_get_type();
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
            b"G_IS_PROXY (proxy)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GIOStream>();
    }
    iface = g_type_interface_peek(
        (*(proxy as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_proxy_get_type(),
    ) as *mut GProxyInterface;
    return Some((*iface).connect_finish.expect("non-null function pointer"))
        .expect("non-null function pointer")(proxy, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_proxy_supports_hostname(mut proxy: *mut GProxy) -> gboolean {
    let mut iface: *mut GProxyInterface = ::core::ptr::null_mut::<GProxyInterface>();
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = proxy as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_proxy_get_type();
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
            b"G_IS_PROXY (proxy)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    iface = g_type_interface_peek(
        (*(proxy as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_proxy_get_type(),
    ) as *mut GProxyInterface;
    return Some(
        (*iface)
            .supports_hostname
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(proxy);
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
