extern "C" {
    pub type _GData;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _GIOExtension;
    pub type _GTask;
    pub type _GProxyResolver;
    pub type _GDBusProxyPrivate;
    pub type _GXdpProxyResolver;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_strfreev(str_array: *mut *mut gchar);
    fn g_strdupv(str_array: *mut *mut gchar) -> *mut *mut gchar;
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
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
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_dbus_proxy_get_name_owner(proxy: *mut GDBusProxy) -> *mut gchar;
    fn g_io_extension_point_implement(
        extension_point_name: *const ::core::ffi::c_char,
        type_0: GType,
        extension_name: *const ::core::ffi::c_char,
        priority: gint,
    ) -> *mut GIOExtension;
    fn g_proxy_resolver_get_type() -> GType;
    fn g_task_new(
        source_object: gpointer,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        callback_data: gpointer,
    ) -> *mut GTask;
    fn g_task_return_pointer(task: *mut GTask, result: gpointer, result_destroy: GDestroyNotify);
    fn g_task_return_error(task: *mut GTask, error: *mut GError);
    fn g_task_propagate_pointer(task: *mut GTask, error: *mut *mut GError) -> gpointer;
    fn gxdp_proxy_resolver_call_lookup(
        proxy: *mut GXdpProxyResolver,
        arg_uri: *const gchar,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn gxdp_proxy_resolver_call_lookup_finish(
        proxy: *mut GXdpProxyResolver,
        out_proxies: *mut *mut *mut gchar,
        res: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gboolean;
    fn gxdp_proxy_resolver_call_lookup_sync(
        proxy: *mut GXdpProxyResolver,
        arg_uri: *const gchar,
        out_proxies: *mut *mut *mut gchar,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn gxdp_proxy_resolver_proxy_new_for_bus_sync(
        bus_type: GBusType,
        flags: GDBusProxyFlags,
        name: *const gchar,
        object_path: *const gchar,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GXdpProxyResolver;
    fn _g_io_modules_ensure_extension_points_registered();
    fn glib_should_use_portal() -> gboolean;
    fn glib_network_available_in_sandbox() -> gboolean;
}
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
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
pub type GBusType = ::core::ffi::c_int;
pub const G_BUS_TYPE_SESSION: GBusType = 2;
pub const G_BUS_TYPE_SYSTEM: GBusType = 1;
pub const G_BUS_TYPE_NONE: GBusType = 0;
pub const G_BUS_TYPE_STARTER: GBusType = -1;
pub type GDBusProxyFlags = ::core::ffi::c_uint;
pub const G_DBUS_PROXY_FLAGS_NO_MATCH_RULE: GDBusProxyFlags = 32;
pub const G_DBUS_PROXY_FLAGS_DO_NOT_AUTO_START_AT_CONSTRUCTION: GDBusProxyFlags = 16;
pub const G_DBUS_PROXY_FLAGS_GET_INVALIDATED_PROPERTIES: GDBusProxyFlags = 8;
pub const G_DBUS_PROXY_FLAGS_DO_NOT_AUTO_START: GDBusProxyFlags = 4;
pub const G_DBUS_PROXY_FLAGS_DO_NOT_CONNECT_SIGNALS: GDBusProxyFlags = 2;
pub const G_DBUS_PROXY_FLAGS_DO_NOT_LOAD_PROPERTIES: GDBusProxyFlags = 1;
pub const G_DBUS_PROXY_FLAGS_NONE: GDBusProxyFlags = 0;
pub type GAsyncResult = _GAsyncResult;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
pub type GIOExtension = _GIOExtension;
pub type GTask = _GTask;
pub type GProxyResolver = _GProxyResolver;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusProxy {
    pub parent_instance: GObject,
    pub priv_0: *mut GDBusProxyPrivate,
}
pub type GDBusProxyPrivate = _GDBusProxyPrivate;
pub type GDBusProxy = _GDBusProxy;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GProxyResolverInterface {
    pub g_iface: GTypeInterface,
    pub is_supported: Option<unsafe extern "C" fn(*mut GProxyResolver) -> gboolean>,
    pub lookup: Option<
        unsafe extern "C" fn(
            *mut GProxyResolver,
            *const gchar,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut *mut gchar,
    >,
    pub lookup_async: Option<
        unsafe extern "C" fn(
            *mut GProxyResolver,
            *const gchar,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub lookup_finish: Option<
        unsafe extern "C" fn(
            *mut GProxyResolver,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> *mut *mut gchar,
    >,
}
pub type GProxyResolverInterface = _GProxyResolverInterface;
pub type GXdpProxyResolver = _GXdpProxyResolver;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GProxyResolverPortal {
    pub parent_instance: GObject,
    pub resolver: *mut GXdpProxyResolver,
    pub network_available: gboolean,
}
pub type GProxyResolverPortal = _GProxyResolverPortal;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GProxyResolverPortalClass {
    pub parent_class: GObjectClass,
}
pub type GProxyResolverPortalClass = _GProxyResolverPortalClass;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_proxy_resolver_portal_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GProxyResolverPortal\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GProxyResolverPortalClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_proxy_resolver_portal_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GProxyResolverPortal>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GProxyResolverPortal) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_proxy_resolver_portal_init
                    as unsafe extern "C" fn(*mut GProxyResolverPortal) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GProxyResolverInterface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_proxy_resolver_portal_iface_init
                as unsafe extern "C" fn(*mut GProxyResolverInterface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_proxy_resolver_get_type(),
        &raw const g_implement_interface_info,
    );
    _g_io_modules_ensure_extension_points_registered();
    g_io_extension_point_implement(
        b"gio-proxy-resolver\0" as *const u8 as *const ::core::ffi::c_char,
        g_define_type_id,
        b"portal\0" as *const u8 as *const ::core::ffi::c_char,
        90 as gint,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_proxy_resolver_portal_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_proxy_resolver_portal_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GProxyResolverPortal_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GProxyResolverPortal_private_offset,
        );
    }
    safe_c2rust_g_proxy_resolver_portal_class_init(klass as *mut GProxyResolverPortalClass);
}
static mut safe_c2rust_g_proxy_resolver_portal_parent_class: gpointer = NULL;
static mut safe_c2rust_GProxyResolverPortal_private_offset: gint = 0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_proxy_resolver_portal_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_proxy_resolver_portal_get_type_once();
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
unsafe extern "C" fn safe_c2rust_ensure_resolver_proxy(
    mut resolver: *mut GProxyResolverPortal,
) -> gboolean {
    if !(*resolver).resolver.is_null() {
        return TRUE;
    }
    if glib_should_use_portal() == 0 {
        return FALSE;
    }
    (*resolver).resolver = gxdp_proxy_resolver_proxy_new_for_bus_sync(
        G_BUS_TYPE_SESSION,
        G_DBUS_PROXY_FLAGS_NONE,
        b"org.freedesktop.portal.Desktop\0" as *const u8 as *const gchar,
        b"/org/freedesktop/portal/desktop\0" as *const u8 as *const gchar,
        ::core::ptr::null_mut::<GCancellable>(),
        ::core::ptr::null_mut::<*mut GError>(),
    );
    (*resolver).network_available = glib_network_available_in_sandbox();
    return ((*resolver).resolver != NULL as *mut GXdpProxyResolver) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_proxy_resolver_portal_init(
    mut resolver: *mut GProxyResolverPortal,
) {
}
unsafe extern "C" fn safe_c2rust_g_proxy_resolver_portal_is_supported(
    mut object: *mut GProxyResolver,
) -> gboolean {
    let mut resolver: *mut GProxyResolverPortal =
        object as *mut ::core::ffi::c_void as *mut GProxyResolverPortal;
    let mut name_owner: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut has_portal: gboolean = 0;
    if safe_c2rust_ensure_resolver_proxy(resolver) == 0 {
        return FALSE;
    }
    name_owner = g_dbus_proxy_get_name_owner(
        (*resolver).resolver as *mut ::core::ffi::c_void as *mut GDBusProxy,
    ) as *mut ::core::ffi::c_char;
    has_portal = (name_owner != NULL as *mut ::core::ffi::c_char) as ::core::ffi::c_int as gboolean;
    g_free(name_owner as gpointer);
    return has_portal;
}
static mut safe_c2rust_no_proxy: [*const ::core::ffi::c_char; 2] = [
    b"direct://\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
unsafe extern "C" fn safe_c2rust_g_proxy_resolver_portal_lookup(
    mut proxy_resolver: *mut GProxyResolver,
    mut uri: *const gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut *mut gchar {
    let mut resolver: *mut GProxyResolverPortal =
        proxy_resolver as *mut ::core::ffi::c_void as *mut GProxyResolverPortal;
    let mut proxy: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    safe_c2rust_ensure_resolver_proxy(resolver);
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !(*resolver).resolver.is_null() {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gproxyresolverportal.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            102 as ::core::ffi::c_int,
            G_STRFUNC,
            b"resolver->resolver\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if gxdp_proxy_resolver_call_lookup_sync(
        (*resolver).resolver,
        uri,
        &raw mut proxy,
        cancellable,
        error,
    ) == 0
    {
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    if (*resolver).network_available == 0 {
        g_strfreev(proxy as *mut *mut gchar);
        proxy = g_strdupv(
            &raw mut safe_c2rust_no_proxy as *mut *const ::core::ffi::c_char as *mut *mut gchar,
        ) as *mut *mut ::core::ffi::c_char;
    }
    return proxy as *mut *mut gchar;
}
unsafe extern "C" fn safe_c2rust_lookup_done(
    mut source: *mut GObject,
    mut result: *mut GAsyncResult,
    mut data: gpointer,
) {
    let mut task: *mut GTask = data as *mut GTask;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut proxies: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    if gxdp_proxy_resolver_call_lookup_finish(
        source as *mut ::core::ffi::c_void as *mut GXdpProxyResolver,
        &raw mut proxies,
        result,
        &raw mut error,
    ) == 0
    {
        g_task_return_error(task, error);
    } else {
        g_task_return_pointer(task, proxies as gpointer, None);
    }
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_proxy_resolver_portal_lookup_async(
    mut proxy_resolver: *mut GProxyResolver,
    mut uri: *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut resolver: *mut GProxyResolverPortal =
        proxy_resolver as *mut ::core::ffi::c_void as *mut GProxyResolverPortal;
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    safe_c2rust_ensure_resolver_proxy(resolver);
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !(*resolver).resolver.is_null() {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gproxyresolverportal.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            151 as ::core::ffi::c_int,
            G_STRFUNC,
            b"resolver->resolver\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    task = g_task_new(proxy_resolver as gpointer, cancellable, callback, user_data);
    gxdp_proxy_resolver_call_lookup(
        (*resolver).resolver,
        uri,
        cancellable,
        Some(
            safe_c2rust_lookup_done
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        g_object_ref(task as gpointer) as *mut GTask as gpointer,
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_proxy_resolver_portal_lookup_finish(
    mut proxy_resolver: *mut GProxyResolver,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut *mut gchar {
    let mut resolver: *mut GProxyResolverPortal =
        proxy_resolver as *mut ::core::ffi::c_void as *mut GProxyResolverPortal;
    let mut task: *mut GTask = result as *mut ::core::ffi::c_void as *mut GTask;
    let mut proxies: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    proxies = g_task_propagate_pointer(task, error) as *mut *mut ::core::ffi::c_char;
    if proxies.is_null() {
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    if (*resolver).network_available == 0 {
        g_strfreev(proxies as *mut *mut gchar);
        proxies = g_strdupv(
            &raw mut safe_c2rust_no_proxy as *mut *const ::core::ffi::c_char as *mut *mut gchar,
        ) as *mut *mut ::core::ffi::c_char;
    }
    return proxies as *mut *mut gchar;
}
unsafe extern "C" fn safe_c2rust_g_proxy_resolver_portal_finalize(mut object: *mut GObject) {
    let mut resolver: *mut GProxyResolverPortal =
        object as *mut ::core::ffi::c_void as *mut GProxyResolverPortal;
    let mut _pp: *mut *mut GXdpProxyResolver = &raw mut (*resolver).resolver;
    let mut _ptr: *mut GXdpProxyResolver = *_pp;
    *_pp = ::core::ptr::null_mut::<GXdpProxyResolver>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    (*(safe_c2rust_g_proxy_resolver_portal_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_proxy_resolver_portal_class_init(
    mut resolver_class: *mut GProxyResolverPortalClass,
) {
    let mut object_class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    object_class = resolver_class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*object_class).finalize = Some(
        safe_c2rust_g_proxy_resolver_portal_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
}
unsafe extern "C" fn safe_c2rust_g_proxy_resolver_portal_iface_init(
    mut iface: *mut GProxyResolverInterface,
) {
    (*iface).is_supported = Some(
        safe_c2rust_g_proxy_resolver_portal_is_supported
            as unsafe extern "C" fn(*mut GProxyResolver) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut GProxyResolver) -> gboolean>;
    (*iface).lookup = Some(
        safe_c2rust_g_proxy_resolver_portal_lookup
            as unsafe extern "C" fn(
                *mut GProxyResolver,
                *const gchar,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut *mut gchar,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GProxyResolver,
                *const gchar,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut *mut gchar,
        >;
    (*iface).lookup_async = Some(
        safe_c2rust_g_proxy_resolver_portal_lookup_async
            as unsafe extern "C" fn(
                *mut GProxyResolver,
                *const gchar,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GProxyResolver,
                *const gchar,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*iface).lookup_finish = Some(
        safe_c2rust_g_proxy_resolver_portal_lookup_finish
            as unsafe extern "C" fn(
                *mut GProxyResolver,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut *mut gchar,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GProxyResolver,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut *mut gchar,
        >;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
